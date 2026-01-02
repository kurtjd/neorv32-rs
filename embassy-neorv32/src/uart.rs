//! Universal Asynchronous Receiver and Transmitter (UART)
use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::peripherals::{UART0, UART1};
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

/// UART interrupt handler binding.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        // If RX FIFO is not empty, disable RX not empty IRQ and wake RX task
        let rx_nempty_irq_set = T::info()
            .reg
            .ctrl()
            .read()
            .uart_ctrl_irq_rx_nempty()
            .bit_is_set();
        let rx_nempty = T::info()
            .reg
            .ctrl()
            .read()
            .uart_ctrl_rx_nempty()
            .bit_is_set();

        if rx_nempty_irq_set && rx_nempty {
            T::info()
                .reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_irq_rx_nempty().clear_bit());
            T::info().rx_waker.wake();
        }

        // Note: For below, only one TX type IRQ will be active at a time
        // (waiting for a room in the TX fifo or waiting for TX flush),
        // hence it is okay to use the TX waker for both.

        // If TX FIFO is not full, disable TX not full IRQ and wake TX task
        let tx_nfull_irq_set = T::info()
            .reg
            .ctrl()
            .read()
            .uart_ctrl_irq_tx_nfull()
            .bit_is_set();
        let tx_nfull = T::info()
            .reg
            .ctrl()
            .read()
            .uart_ctrl_tx_nfull()
            .bit_is_set();

        if tx_nfull_irq_set && tx_nfull {
            T::info()
                .reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_irq_tx_nfull().clear_bit());
            T::info().tx_waker.wake();
        }

        // If TX FIFO is empty, disable TX empty IRQ and wake TX task
        let tx_empty_irq_set = T::info()
            .reg
            .ctrl()
            .read()
            .uart_ctrl_irq_tx_empty()
            .bit_is_set();
        let tx_empty = T::info()
            .reg
            .ctrl()
            .read()
            .uart_ctrl_tx_empty()
            .bit_is_set();

        if tx_empty_irq_set && tx_empty {
            T::info()
                .reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_irq_tx_empty().clear_bit());
            T::info().tx_waker.wake();
        }
    }
}

/// UART driver.
pub struct Uart<'d, M: IoMode> {
    rx: UartRx<'d, M>,
    tx: UartTx<'d, M>,
}

impl<'d, M: IoMode> Uart<'d, M> {
    fn init<T: Instance>(_instance: Peri<'d, T>, baud_rate: u32, sim: bool, flow_control: bool) {
        // Enable simulation mode if applicable
        if sim {
            T::info()
                .reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_sim_mode().set_bit());
        }

        // Enable flow control if applicable
        if flow_control {
            T::info()
                .reg
                .ctrl()
                .modify(|_, w| w.uart_ctrl_hwfc_en().set_bit());
        }

        let cpu_freq = crate::sysinfo::SysInfo::clock_freq();
        let mut baud_div = cpu_freq / (2 * baud_rate);
        let mut prsc_sel = 0;

        // Calculate clock prescaler and baud rate prescaler
        // See: https://github.com/stnolting/neorv32/blob/main/sw/lib/source/neorv32_uart.c#L47
        while baud_div >= 0x3ff {
            if prsc_sel == 2 || prsc_sel == 4 {
                baud_div >>= 3;
            } else {
                baud_div >>= 1;
            }
            prsc_sel += 1;
        }

        // Set the clock and baudrate prescalers
        T::info().reg.ctrl().modify(|_, w| unsafe {
            w.uart_ctrl_prsc()
                .bits(prsc_sel & 0b111)
                .uart_ctrl_baud()
                .bits((baud_div as u16 - 1) & 0x3ff)
        });

        // Enable UART
        T::info()
            .reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_en().set_bit());
    }

    fn new_inner<T: Instance>() -> Self {
        let rx = UartRx::new_inner::<T>();
        let tx = UartTx::new_inner::<T>();
        Self { rx, tx }
    }

    /// Reads a byte from RX FIFO, blocking if empty.
    pub fn blocking_read_byte(&self) -> u8 {
        self.rx.blocking_read_byte()
    }

    /// Reads bytes from RX FIFO until buffer is full, blocking if empty.
    pub fn blocking_read(&self, buf: &mut [u8]) {
        self.rx.blocking_read(buf);
    }

    /// Writes a byte to TX FIFO, blocking if full.
    pub fn blocking_write_byte(&mut self, byte: u8) {
        self.tx.blocking_write_byte(byte);
    }

    /// Writes bytes to TX FIFO, blocking if full.
    pub fn blocking_write(&mut self, bytes: &[u8]) {
        self.tx.blocking_write(bytes);
    }

    /// Blocks until all TX complete.
    pub fn blocking_flush(&mut self) {
        self.tx.blocking_flush();
    }

    /// Splits the UART driver into separate [UartRx] and [UartTx] drivers.
    ///
    /// Helpful for sharing the UART among receiver/transmitter tasks.
    pub fn split(self) -> (UartRx<'d, M>, UartTx<'d, M>) {
        (self.rx, self.tx)
    }

    /// Splits the UART driver into separate [UartRx] and [UartTx] drivers by mutable reference.
    ///
    /// Helpful for sharing the UART among receiver/transmitter tasks without destroying the original [Uart] instance.
    pub fn split_ref(&mut self) -> (&mut UartRx<'d, M>, &mut UartTx<'d, M>) {
        (&mut self.rx, &mut self.tx)
    }
}

impl<'d> Uart<'d, Blocking> {
    /// Creates a new blocking UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_blocking<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
    ) -> Self {
        Self::init(_instance, baud_rate, sim, flow_control);
        Self::new_inner::<T>()
    }
}

impl<'d> Uart<'d, Async> {
    /// Creates a new async UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_async<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        Self::init(_instance, baud_rate, sim, flow_control);
        // SAFETY: It is valid to enable UART interrupt here
        unsafe { T::Interrupt::enable() }
        Self::new_inner::<T>()
    }

    /// Reads a byte from RX FIFO.
    pub fn read_byte(&mut self) -> impl Future<Output = u8> {
        self.rx.read_byte()
    }

    /// Reads bytes from RX FIFO until buffer is full.
    pub fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = ()> {
        self.rx.read(buf)
    }

    /// Writes a byte to TX FIFO.
    pub fn write_byte(&mut self, byte: u8) -> impl Future<Output = ()> {
        self.tx.write_byte(byte)
    }

    /// Writes bytes from buffer to TX FIFO.
    pub fn write(&mut self, bytes: &[u8]) -> impl Future<Output = ()> {
        self.tx.write(bytes)
    }

    /// Waits until all TX complete.
    pub fn flush(&mut self) -> impl Future<Output = ()> {
        self.tx.flush()
    }
}

/// RX-only UART driver.
pub struct UartRx<'d, M: IoMode> {
    info: Info,
    _phantom: PhantomData<&'d M>,
}

// Allows for use in a Mutex (to share safely between harts and tasks)
unsafe impl<'d, M: IoMode> Send for UartRx<'d, M> {}

impl<'d, M: IoMode> UartRx<'d, M> {
    fn new_inner<T: Instance>() -> Self {
        // Mark RX as active
        T::info().active.rx.store(true, Ordering::SeqCst);

        Self {
            info: T::info(),
            _phantom: PhantomData,
        }
    }

    fn read_unchecked(&self) -> u8 {
        self.info.reg.data().read().bits() as u8
    }

    fn read_until_empty(&self, buf: &mut [u8]) -> usize {
        let mut n = 0;
        for byte in buf {
            *byte = self.read_unchecked();
            n += 1;

            if self.fifo_empty() {
                break;
            }
        }

        n
    }

    fn enable_irq_rx_nempty(&mut self) {
        self.info
            .reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_rx_nempty().set_bit());
    }

    fn fifo_empty(&self) -> bool {
        self.info
            .reg
            .ctrl()
            .read()
            .uart_ctrl_rx_nempty()
            .bit_is_clear()
    }

    /// Reads a byte from RX FIFO, blocking if empty.
    pub fn blocking_read_byte(&self) -> u8 {
        while self.fifo_empty() {}
        self.read_unchecked()
    }

    /// Reads bytes from RX FIFO until buffer is full, blocking if empty.
    pub fn blocking_read(&self, buf: &mut [u8]) {
        for byte in buf {
            *byte = self.blocking_read_byte();
        }
    }
}

impl<'d> UartRx<'d, Blocking> {
    /// Creates a new RX-only blocking UART driver with given baud rate.
    ///
    /// Enables hardware flow control if `flow_control` is true.
    pub fn new_blocking<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        flow_control: bool,
    ) -> Self {
        Uart::<Blocking>::init(_instance, baud_rate, false, flow_control);
        Self::new_inner::<T>()
    }
}

impl<'d> UartRx<'d, Async> {
    async fn wait_fifo_nempty(&mut self) {
        poll_fn(|cx| {
            self.info.rx_waker.register(cx.waker());
            if !self.fifo_empty() {
                Poll::Ready(())
            } else {
                // CS used here since interrupt modifies register
                critical_section::with(|_| self.enable_irq_rx_nempty());
                Poll::Pending
            }
        })
        .await
    }

    /// Creates a new RX-only async UART driver with given baud rate.
    ///
    /// Enables hardware flow control if `flow_control` is true.
    pub fn new_async<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        flow_control: bool,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        Uart::<Async>::init(_instance, baud_rate, false, flow_control);
        // SAFETY: It is valid to enable UART interrupt here
        unsafe { T::Interrupt::enable() }
        Self::new_inner::<T>()
    }

    /// Reads a byte from RX FIFO.
    pub async fn read_byte(&mut self) -> u8 {
        self.wait_fifo_nempty().await;
        self.read_unchecked()
    }

    /// Reads bytes from RX FIFO until buffer is full.
    pub async fn read(&mut self, buf: &mut [u8]) {
        for byte in buf {
            *byte = self.read_byte().await;
        }
    }
}

impl<'d, M: IoMode> Drop for UartRx<'d, M> {
    fn drop(&mut self) {
        self.info.active.rx.store(false, Ordering::SeqCst);
        drop_rx_tx(&self.info);
    }
}

/// TX-only UART driver.
pub struct UartTx<'d, M: IoMode> {
    info: Info,
    _phantom: PhantomData<&'d M>,
}

// Allows for use in a Mutex (to share safely between harts and tasks)
unsafe impl<'d, M: IoMode> Send for UartTx<'d, M> {}

impl<'d, M: IoMode> UartTx<'d, M> {
    fn new_inner<T: Instance>() -> Self {
        // Mark TX as active
        T::info().active.rx.store(true, Ordering::SeqCst);

        Self {
            info: T::info(),
            _phantom: PhantomData,
        }
    }

    fn write_unchecked(&mut self, byte: u8) {
        // SAFETY: We are just writing a byte, the MSB bits are read-only
        self.info
            .reg
            .data()
            .write(|w| unsafe { w.bits(byte as u32) });
    }

    fn write_until_full(&mut self, buf: &[u8]) -> usize {
        // But then only write bytes that can fit into FIFO
        let mut n = 0;
        for byte in buf {
            self.write_unchecked(*byte);
            n += 1;

            if self.fifo_full() {
                break;
            }
        }

        // And finally return the number of bytes actually written
        n
    }

    fn enable_irq_tx_nfull(&mut self) {
        self.info
            .reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_tx_nfull().set_bit());
    }

    fn enable_irq_tx_empty(&mut self) {
        self.info
            .reg
            .ctrl()
            .modify(|_, w| w.uart_ctrl_irq_tx_empty().set_bit());
    }

    fn fifo_full(&self) -> bool {
        self.info
            .reg
            .ctrl()
            .read()
            .uart_ctrl_tx_nfull()
            .bit_is_clear()
    }

    fn fifo_empty(&self) -> bool {
        self.info
            .reg
            .ctrl()
            .read()
            .uart_ctrl_irq_tx_empty()
            .bit_is_set()
    }

    /// Writes a byte to TX FIFO, blocking if full.
    pub fn blocking_write_byte(&mut self, byte: u8) {
        while self.fifo_full() {}
        self.write_unchecked(byte);
    }

    /// Writes bytes to TX FIFO, blocking if full.
    pub fn blocking_write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.blocking_write_byte(*byte);
        }
    }

    /// Blocks until all TX complete.
    pub fn blocking_flush(&mut self) {
        while self.info.reg.ctrl().read().uart_ctrl_tx_busy().bit_is_set() {}
    }
}

impl<'d> UartTx<'d, Blocking> {
    /// Creates a new TX-only blocking UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_blocking<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
    ) -> Self {
        Uart::<Blocking>::init(_instance, baud_rate, sim, flow_control);
        Self::new_inner::<T>()
    }
}

impl<'d> UartTx<'d, Async> {
    async fn wait_fifo_nfull(&mut self) {
        poll_fn(|cx| {
            self.info.tx_waker.register(cx.waker());
            if !self.fifo_full() {
                Poll::Ready(())
            } else {
                // CS used here since interrupt modifies register
                critical_section::with(|_| self.enable_irq_tx_nfull());
                Poll::Pending
            }
        })
        .await
    }

    /// Creates a new TX-only async UART driver with given baud rate.
    ///
    /// Enables simulation mode if `sim` is true and hardware flow control if `flow_control` is true.
    pub fn new_async<T: Instance>(
        _instance: Peri<'d, T>,
        baud_rate: u32,
        sim: bool,
        flow_control: bool,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        Uart::<Async>::init(_instance, baud_rate, sim, flow_control);
        // SAFETY: It is valid to enable UART interrupt here
        unsafe { T::Interrupt::enable() }
        Self::new_inner::<T>()
    }

    /// Writes a byte to TX FIFO.
    pub async fn write_byte(&mut self, byte: u8) {
        self.wait_fifo_nfull().await;
        self.write_unchecked(byte);
    }

    /// Writes bytes from buffer to TX FIFO.
    pub async fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte).await;
        }
    }

    /// Waits until all TX complete.
    pub async fn flush(&mut self) {
        poll_fn(|cx| {
            self.info.tx_waker.register(cx.waker());
            if self.fifo_empty() {
                Poll::Ready(())
            } else {
                // CS used here since interrupt modifies register
                critical_section::with(|_| self.enable_irq_tx_empty());
                Poll::Pending
            }
        })
        .await
    }
}

impl<'d, M: IoMode> Drop for UartTx<'d, M> {
    fn drop(&mut self) {
        self.info.active.tx.store(false, Ordering::SeqCst);
        drop_rx_tx(&self.info);
    }
}

fn drop_rx_tx(info: &Info) {
    // Only disable UART if both Rx and Tx have been dropped
    if !info.active.rx.load(Ordering::SeqCst) && !info.active.tx.load(Ordering::SeqCst) {
        info.reg.ctrl().modify(|_, w| w.uart_ctrl_en().clear_bit());
    }
}

// Serves as a "reference-counter" so we know when Uart is completely dropped
// Use two AtomicBools instead of AtomicU8 since fetch_add/fetch_sub are not available without A extension
struct Active {
    rx: AtomicBool,
    tx: AtomicBool,
}

impl Active {
    const fn new() -> Self {
        Self {
            rx: AtomicBool::new(false),
            tx: AtomicBool::new(false),
        }
    }
}

struct Info {
    reg: &'static crate::pac::uart0::RegisterBlock,
    active: &'static Active,
    rx_waker: &'static AtomicWaker,
    tx_waker: &'static AtomicWaker,
}

trait SealedIoMode {}

/// UART IO mode.
#[allow(private_bounds)]
pub trait IoMode: SealedIoMode {}

/// Blocking UART.
pub struct Blocking;
impl SealedIoMode for Blocking {}
impl IoMode for Blocking {}

/// Async UART.
pub struct Async;
impl SealedIoMode for Async {}
impl IoMode for Async {}

trait SealedInstance {
    fn info() -> Info;
}

/// A valid UART peripheral.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    type Interrupt: Interrupt;
}

macro_rules! impl_instance {
    ($periph:ident, $rb:ident) => {
        impl SealedInstance for $periph {
            // Note: uart0 and uart1 can both share uart0::RegisterBlock
            // PAC is able to coerce uart1::ptr() to it with correct base address
            fn info() -> Info {
                static RX_WAKER: AtomicWaker = AtomicWaker::new();
                static TX_WAKER: AtomicWaker = AtomicWaker::new();
                static ACTIVE: Active = Active::new();

                Info {
                    reg: unsafe { &*crate::pac::$rb::ptr() },
                    active: &ACTIVE,
                    rx_waker: &RX_WAKER,
                    tx_waker: &TX_WAKER,
                }
            }
        }
        impl Instance for $periph {
            type Interrupt = crate::interrupt::typelevel::$periph;
        }
    };
}

impl_instance!(UART0, Uart0);
impl_instance!(UART1, Uart1);

// Convenience for writing formatted strings to UART
impl<'d, M: IoMode> core::fmt::Write for UartTx<'d, M> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.blocking_write(s.as_bytes());
        Ok(())
    }
}

impl<'d, M: IoMode> embedded_hal_02::blocking::serial::Write<u8> for Uart<'d, M> {
    type Error = core::convert::Infallible;

    fn bwrite_all(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        self.blocking_write(buffer);
        Ok(())
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        self.blocking_flush();
        Ok(())
    }
}

impl<'d, M: IoMode> embedded_hal_02::blocking::serial::Write<u8> for UartTx<'d, M> {
    type Error = core::convert::Infallible;

    fn bwrite_all(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        self.blocking_write(buffer);
        Ok(())
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        self.blocking_flush();
        Ok(())
    }
}

impl<'d, M: IoMode> embedded_io::ErrorType for Uart<'d, M> {
    type Error = core::convert::Infallible;
}

impl<'d, M: IoMode> embedded_io::Read for Uart<'d, M> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        embedded_io::Read::read(&mut self.rx, buf)
    }
}

impl<'d> embedded_io_async::Read for Uart<'d, Async> {
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize, Self::Error>> {
        embedded_io_async::Read::read(&mut self.rx, buf)
    }
}

impl<'d, M: IoMode> embedded_io::Write for Uart<'d, M> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        embedded_io::Write::write(&mut self.tx, buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        embedded_io::Write::flush(&mut self.tx)
    }
}

impl<'d> embedded_io_async::Write for Uart<'d, Async> {
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = Result<usize, Self::Error>> {
        embedded_io_async::Write::write(&mut self.tx, buf)
    }

    fn flush(&mut self) -> impl Future<Output = Result<(), Self::Error>> {
        embedded_io_async::Write::flush(&mut self.tx)
    }
}

impl<'d, M: IoMode> embedded_io::ErrorType for UartTx<'d, M> {
    type Error = core::convert::Infallible;
}

impl<'d, M: IoMode> embedded_io::Write for UartTx<'d, M> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        // Immediately return if empty buffer without blocking
        if buf.is_empty() {
            return Ok(0);
        }

        // Must block until at least the first byte can be written
        while self.fifo_full() {}

        // But then only write bytes that can fit into FIFO
        Ok(self.write_until_full(buf))
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.blocking_flush();
        Ok(())
    }
}

impl<'d> embedded_io_async::Write for UartTx<'d, Async> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        // Immediately return if empty buffer without blocking
        if buf.is_empty() {
            return Ok(0);
        }

        // Must block until at least the first byte can be written
        self.wait_fifo_nfull().await;

        // But then only write bytes that can fit into FIFO
        Ok(self.write_until_full(buf))
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush().await;
        Ok(())
    }
}

impl<'d, M: IoMode> embedded_io::ErrorType for UartRx<'d, M> {
    type Error = core::convert::Infallible;
}

impl<'d, M: IoMode> embedded_io::Read for UartRx<'d, M> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        // Immediately return if empty buffer without blocking
        if buf.is_empty() {
            return Ok(0);
        }

        // Must block until at least one byte is available
        while self.fifo_empty() {}

        // But then only read bytes that are immediately available
        Ok(self.read_until_empty(buf))
    }
}

impl<'d> embedded_io_async::Read for UartRx<'d, Async> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        // Immediately return if empty buffer without blocking
        if buf.is_empty() {
            return Ok(0);
        }

        // Must wait until at least one byte is available
        self.wait_fifo_nempty().await;

        // But then only read bytes that are immediately available
        Ok(self.read_until_empty(buf))
    }
}
