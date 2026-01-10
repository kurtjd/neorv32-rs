//! Serial Peripheral Interface (SPI)
use crate::dma;
use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::peripherals::SPI;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;
use embassy_hal_internal::drop::OnDrop;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
pub use embedded_hal_1::spi::{MODE_0, MODE_1, MODE_2, MODE_3, Mode, Phase, Polarity};

// Dummy value for writes when we are only interested in the read value
const DUMMY: u8 = 0xFF;

/// SPI interrupt handler binding.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        // We disable the interrupt since it is level triggered and there is no apparent way to acknowledge it
        T::Interrupt::disable();
        T::waker().wake();
    }
}

/// SPI error.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The NEORV32 configuration does not support SPI.
    NotSupported,
    /// A DMA bus error occurred.
    DmaBusError,
}

/// Serial Peripheral Interface (SPI) Driver.
pub struct Spi<'d, M: IoMode> {
    reg: &'static crate::pac::spi::RegisterBlock,
    waker: &'static AtomicWaker,
    dma: Option<dma::Dma<'d>>,
    _phantom: PhantomData<&'d M>,
}

// Allows for use in a Mutex (to share safely between harts and tasks)
unsafe impl<'d, M: IoMode> Send for Spi<'d, M> {}

impl<'d, M: IoMode> Spi<'d, M> {
    fn tx_fifo_full(&self) -> bool {
        self.reg.ctrl().read().spi_ctrl_tx_full().bit_is_set()
    }

    fn fifo_depth(&self) -> usize {
        // Value in register is log2 of fifo depth
        1 << self.reg.ctrl().read().spi_ctrl_fifo().bits()
    }

    // This is mainly used in the OnDrop closure, so it can't take a reference to self
    // Therefore it takes a reference to the register block instead
    fn clear_fifos(reg: &'static crate::pac::spi::RegisterBlock) {
        // Flush to ensure TX is complete, then disable and reenable to clear FIFOs
        while reg.ctrl().read().spi_ctrl_busy().bit_is_set() {}
        reg.ctrl().modify(|_, w| w.spi_ctrl_en().clear_bit());
        reg.ctrl().modify(|_, w| w.spi_ctrl_en().set_bit());
    }

    fn busy(&self) -> bool {
        self.reg.ctrl().read().spi_ctrl_busy().bit_is_set()
    }

    fn read_byte(&self) -> u8 {
        self.reg.data().read().spi_data().bits()
    }

    fn write_byte(&mut self, byte: u8) {
        // SAFETY: We ensure data bit is cleared and any value we drite to data is valid
        self.reg
            .data()
            .write(|w| unsafe { w.spi_data_cmd().clear_bit().spi_data().bits(byte) });
    }

    fn blocking_read_chunk(&mut self, chunk: &mut [u8]) {
        for byte in chunk.iter_mut() {
            *byte = self.read_byte();
        }
    }

    fn blocking_write_chunk(&mut self, chunk: &[u8]) {
        for byte in chunk.iter().copied() {
            self.write_byte(byte);
        }
    }

    fn new_inner<T: Instance>(
        _instance: Peri<'d, T>,
        spi_freq: u32,
        mode: Mode,
    ) -> Result<Self, Error> {
        if !crate::sysinfo::SysInfo::soc_config().spi() {
            return Err(Error::NotSupported);
        }

        // Configure clock phase and polarity
        match mode.polarity {
            Polarity::IdleLow => T::reg().ctrl().modify(|_, w| w.spi_ctrl_cpol().clear_bit()),
            Polarity::IdleHigh => T::reg().ctrl().modify(|_, w| w.spi_ctrl_cpol().set_bit()),
        };
        match mode.phase {
            Phase::CaptureOnFirstTransition => {
                T::reg().ctrl().modify(|_, w| w.spi_ctrl_cpha().clear_bit());
            }
            Phase::CaptureOnSecondTransition => {
                T::reg().ctrl().modify(|_, w| w.spi_ctrl_cpha().set_bit());
            }
        }

        let cpu_freq = crate::sysinfo::SysInfo::clock_freq() as u64;
        let mut cdiv = (cpu_freq / (2 * spi_freq as u64 * 2)) - 1;
        let mut psc = 0;

        // Calculate prescaler and divider similar to UART
        // See: https://github.com/stnolting/neorv32/blob/main/sw/lib/source/neorv32_uart.c#L47
        // Revisit: There are some ways to improve this and make it more accurate
        while cdiv > 0xf {
            if psc == 2 || psc == 4 {
                cdiv >>= 3;
            } else {
                cdiv >>= 1;
            }
            psc += 1;
        }

        // Set clock prescaler
        // SAFETY: We are writing a valid psc value
        T::reg()
            .ctrl()
            .modify(|_, w| unsafe { w.spi_ctrl_prsc().bits(psc) });

        // Set clock divider
        // SAFETY: We've ensured cdiv can fit in 4 bits
        T::reg()
            .ctrl()
            .modify(|_, w| unsafe { w.spi_ctrl_cdiv().bits(cdiv as u8) });

        // Enable SPI
        T::reg().ctrl().modify(|_, w| w.spi_ctrl_en().set_bit());

        Ok(Self {
            reg: T::reg(),
            waker: T::waker(),
            dma: None,
            _phantom: PhantomData,
        })
    }

    /// Perform a blocking read on the SPI bus.
    ///
    /// This will write whatever is currently in `data` as dummy values.
    pub fn blocking_read(&mut self, data: &mut [u8]) {
        self.blocking_transfer_in_place(data);
    }

    /// Perform a blocking write on the SPI bus.
    pub fn blocking_write(&mut self, data: &[u8]) {
        for chunk in data.chunks(self.fifo_depth()) {
            self.blocking_write_chunk(chunk);
            self.blocking_flush();
        }

        // The rx fifo needs to be empty for subsequent transfers.
        // Instead of wasting cycles draining it for values we don't care about,
        // quickly clear it instead.
        Self::clear_fifos(self.reg);
    }

    /// Perform a blocking transfer on the SPI bus.
    ///
    /// If `write` is larger, those reads will be discarded.
    /// If `read` is larger, dummy values will be written.
    pub fn blocking_transfer(&mut self, read: &mut [u8], write: &[u8]) {
        // Revisit: Is there a clean way to write this with iterators like transfer_in_place?
        let mut wi = 0;
        let mut ri = 0;
        let len = read.len().max(write.len());

        while ri < len {
            while wi < len && !self.tx_fifo_full() {
                let byte = write.get(wi).copied().unwrap_or(DUMMY);
                self.write_byte(byte);
                wi += 1;
            }

            self.blocking_flush();

            while ri < wi {
                let byte = self.read_byte();
                if let Some(r) = read.get_mut(ri) {
                    *r = byte;
                }
                ri += 1;
            }
        }
    }

    /// Perform a blocking transfer in place on the SPI bus.
    pub fn blocking_transfer_in_place(&mut self, data: &mut [u8]) {
        for chunk in data.chunks_mut(self.fifo_depth()) {
            self.blocking_write_chunk(chunk);
            self.blocking_flush();
            self.blocking_read_chunk(chunk);
        }
    }

    /// Blocks until the SPI bus is idle and all transfers have completed.
    pub fn blocking_flush(&self) {
        while self.busy() {}
    }
}

impl<'d> Spi<'d, Blocking> {
    /// Returns a new instance of a blocking SPI driver with given frequency (in Hz) and mode.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotSupported`] if SPI is not supported.
    pub fn new_blocking<T: Instance>(
        _instance: Peri<'d, T>,
        spi_freq: u32,
        mode: Mode,
    ) -> Result<Self, Error> {
        Self::new_inner(_instance, spi_freq, mode)
    }
}

impl<'d> Spi<'d, Async> {
    async fn read_chunk(&mut self, chunk: &mut [u8]) -> Result<(), Error> {
        // If DMA available, use it to transfer data from RX FIFO to buffer
        if let Some(dma) = &mut self.dma {
            // SAFETY: The PAC ensures the data register pointer is not-null and properly aligned,
            // and the DMA controller takes care to only transfer the 8 LSB
            let src = unsafe { self.reg.data().as_ptr().as_ref().unwrap_unchecked() };
            dma.read(src, chunk, false)
                .await
                .map_err(|_| Error::DmaBusError)?;

        // Otherwise, manually read each byte into buffer
        } else {
            for byte in chunk.iter_mut() {
                *byte = self.read_byte();
            }
        }

        Ok(())
    }

    async fn write_chunk(&mut self, chunk: &[u8]) -> Result<(), Error> {
        // If DMA available, use it to transfer data from buffer to TX FIFO
        if let Some(dma) = &mut self.dma {
            // SAFETY: The PAC ensures the data register pointer is not-null and properly aligned,
            // and the DMA controller takes care of zero-extending the byte to 32 bits, which helpfully
            // also marks the write as a DATA byte
            let dst = unsafe { self.reg.data().as_ptr().as_mut().unwrap_unchecked() };
            dma.write(chunk, dst, false)
                .await
                .map_err(|_| Error::DmaBusError)?;

        // Otherwise, manually write each byte to TX FIFO
        } else {
            for byte in chunk.iter().copied() {
                self.write_byte(byte);
            }
        }

        Ok(())
    }

    /// Returns a new instance of an async SPI driver with given frequency (in Hz) and mode.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotSupported`] if SPI is not supported.
    pub fn new_async<T: Instance>(
        _instance: Peri<'d, T>,
        spi_freq: u32,
        mode: Mode,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Result<Self, Error> {
        Self::new_inner(_instance, spi_freq, mode)
    }

    /// Perform a read on the SPI bus.
    ///
    /// This will write whatever is currently in `data` as dummy values.
    ///
    /// # Errors
    ///
    /// Returns [Error::DmaBusError] if DMA transfer fails.
    pub async fn read(&mut self, data: &mut [u8]) -> Result<(), Error> {
        self.transfer_in_place(data).await
    }

    /// Perform a write on the SPI bus.
    ///
    /// # Errors
    ///
    /// Returns [Error::DmaBusError] if DMA transfer fails.
    pub async fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        let reg = self.reg;
        let _drop_guard = OnDrop::new(|| Self::clear_fifos(reg));

        for chunk in data.chunks(self.fifo_depth()) {
            self.write_chunk(chunk).await?;
            self.flush().await;
        }

        // The rx fifo needs to be empty for subsequent transfers.
        // Instead of wasting cycles draining it for values we don't care about,
        // quickly clear it instead. So always want drop_guard to run to clear rx fifo.
        Ok(())
    }

    /// Perform a transfer on the SPI bus.
    ///
    /// If `write` is larger, those reads will be discarded.
    /// If `read` is larger, dummy values will be written.
    ///
    /// # Errors
    ///
    /// Returns [Error::DmaBusError] if DMA transfer fails.
    pub async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        // Revisit: Is there a clean way to write this with iterators like transfer_in_place?
        // As it stands this isn't very chunkable for DMA
        let reg = self.reg;
        let drop_guard = OnDrop::new(|| Self::clear_fifos(reg));

        let mut wi = 0;
        let mut ri = 0;
        let len = read.len().max(write.len());

        while ri < len {
            while wi < len && !self.tx_fifo_full() {
                let byte = write.get(wi).copied().unwrap_or(DUMMY);
                self.write_byte(byte);
                wi += 1;
            }

            self.flush().await;

            while ri < wi {
                let byte = self.read_byte();
                if let Some(r) = read.get_mut(ri) {
                    *r = byte;
                }
                ri += 1;
            }
        }

        drop_guard.defuse();
        Ok(())
    }

    /// Perform a transfer in place on the SPI bus.
    ///
    /// # Errors
    ///
    /// Returns [Error::DmaBusError] if DMA transfer fails.
    pub async fn transfer_in_place(&mut self, data: &mut [u8]) -> Result<(), Error> {
        let reg = self.reg;
        let drop_guard = OnDrop::new(|| Self::clear_fifos(reg));

        for chunk in data.chunks_mut(self.fifo_depth()) {
            self.write_chunk(chunk).await?;
            self.flush().await;
            self.read_chunk(chunk).await?;
        }

        drop_guard.defuse();
        Ok(())
    }

    /// Waits until the SPI bus is idle and all transfers have completed.
    pub async fn flush(&mut self) {
        poll_fn(|cx| {
            self.waker.register(cx.waker());

            if !self.busy() {
                Poll::Ready(())
            } else {
                // SAFETY: It is valid to enable inetrrupts here, since it is level triggered
                // and if the bus becomes not busy between the above check and here, we won't miss it
                unsafe { crate::enable_periph_irq!(SPI) }
                Poll::Pending
            }
        })
        .await
    }

    /// Gives the DMA controller to the SPI driver.
    /// It can later be retrieved via [Self::take_dma] for use with other peripherals.
    ///
    /// This is for flexibility purposes as there is only one DMA channel available.
    /// If no DMA is provided, data must be manually copied to/from FIFOs.
    ///
    /// However, for small FIFO depths, and/or small transfer sizes,
    /// this would be more efficient as there is overhead in setting up the DMA transfer.
    pub fn give_dma(&mut self, dma: dma::Dma<'d>) {
        let _ = self.dma.replace(dma);
    }

    /// Retrieves the DMA controller if available, allowing it to be used by other peripherals again.
    ///
    /// See [Self::give_dma] for the implications of this.
    pub fn take_dma(&mut self) -> Option<dma::Dma<'d>> {
        self.dma.take()
    }
}

impl<'d, M: IoMode> Drop for Spi<'d, M> {
    fn drop(&mut self) {
        self.blocking_flush();
        self.reg.ctrl().modify(|_, w| w.spi_ctrl_en().clear_bit());
    }
}

trait SealedIoMode {}

/// SPI IO mode.
#[allow(private_bounds)]
pub trait IoMode: SealedIoMode {}

/// Blocking SPI.
pub struct Blocking;
impl SealedIoMode for Blocking {}
impl IoMode for Blocking {}

/// Async SPI.
pub struct Async;
impl SealedIoMode for Async {}
impl IoMode for Async {}

trait SealedInstance {
    fn reg() -> &'static crate::pac::spi::RegisterBlock;
    fn waker() -> &'static AtomicWaker;
}

/// A valid SPI peripheral.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    type Interrupt: Interrupt;
}
impl SealedInstance for SPI {
    fn reg() -> &'static crate::pac::spi::RegisterBlock {
        // SAFETY: We own the SPI peripheral and are sure to use it safely
        unsafe { &*crate::pac::Spi::ptr() }
    }

    fn waker() -> &'static AtomicWaker {
        static WAKER: AtomicWaker = AtomicWaker::new();
        &WAKER
    }
}
impl Instance for SPI {
    type Interrupt = crate::interrupt::typelevel::SPI;
}

impl<'d, M: IoMode> embedded_hal_02::blocking::spi::Transfer<u8> for Spi<'d, M> {
    type Error = core::convert::Infallible;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.blocking_transfer_in_place(words);
        Ok(words)
    }
}

impl<'d, M: IoMode> embedded_hal_02::blocking::spi::Write<u8> for Spi<'d, M> {
    type Error = core::convert::Infallible;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.blocking_write(words);
        Ok(())
    }
}

impl embedded_hal_1::spi::Error for Error {
    fn kind(&self) -> embedded_hal_1::spi::ErrorKind {
        embedded_hal_1::spi::ErrorKind::Other
    }
}

impl<'d, M: IoMode> embedded_hal_1::spi::ErrorType for Spi<'d, M> {
    type Error = Error;
}

impl<'d, M: IoMode> embedded_hal_1::spi::SpiBus<u8> for Spi<'d, M> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.blocking_read(words);
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.blocking_write(words);
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.blocking_transfer(read, write);
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.blocking_transfer_in_place(words);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.blocking_flush();
        Ok(())
    }
}

impl<'d> embedded_hal_async::spi::SpiBus<u8> for Spi<'d, Async> {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        (*self).read(words).await
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        (*self).write(words).await
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        (*self).transfer(read, write).await
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        (*self).transfer_in_place(words).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        (*self).flush().await;
        Ok(())
    }
}
