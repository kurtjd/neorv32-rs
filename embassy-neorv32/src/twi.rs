//! Two-Wire Interface (TWI)
//!
//! **Note**: Unfortunately NEORV32's implementation for TWI in hardware appears problematic
//! and makes it difficult to write a driver for it. Specifically, receiving bytes from a device
//! is pretty odd and there is no interrupt for byte received, so an async version of this
//! driver doesn't seem possible and is not provided.
use crate::peripherals::TWI;
use core::marker::PhantomData;
use embassy_hal_internal::{Peri, PeripheralType};
pub use embedded_hal_1::i2c::Operation;

// A hack/workaround for master ACKs (see `read_byte`)
const HI_Z: u8 = 0xFF;

enum Command {
    _Nop,
    Start,
    Stop,
    Data,
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::_Nop => 0b00,
            Command::Start => 0b01,
            Command::Stop => 0b10,
            Command::Data => 0b11,
        }
    }
}

enum Mack {
    Ack,
    Nack,
}

impl From<Mack> for bool {
    fn from(kind: Mack) -> Self {
        match kind {
            Mack::Ack => true,
            Mack::Nack => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Rw {
    Read,
    Write,
}

impl From<Rw> for u8 {
    fn from(rw: Rw) -> Self {
        match rw {
            Rw::Read => 1,
            Rw::Write => 0,
        }
    }
}

/// TWI Error.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Device failed to ack address.
    NackAddr,
    /// Device failed to ack data.
    NackData,
}

/// Two-Wire Interface (TWI) Driver.
pub struct Twi<'d, M: IoMode> {
    reg: &'static crate::pac::twi::RegisterBlock,
    _phantom: PhantomData<&'d M>,
}

// Allows for use in a Mutex (to share safely between harts and tasks)
unsafe impl<'d, M: IoMode> Send for Twi<'d, M> {}

impl<'d, M: IoMode> Twi<'d, M> {
    fn tx_full(&self) -> bool {
        self.reg.ctrl().read().twi_ctrl_tx_full().bit_is_set()
    }

    fn tx_busy(&self) -> bool {
        self.reg.ctrl().read().twi_ctrl_busy().bit_is_set()
    }

    fn device_acked(&self) -> bool {
        self.reg.dcmd().read().twi_dcmd_ack().bit_is_clear()
    }

    fn start(&mut self) {
        while self.tx_full() {}
        // SAFETY: Command enum ensures we are writing valid command
        self.reg
            .dcmd()
            .write(|w| unsafe { w.twi_dcmd_cmd().bits(Command::Start.into()) });
        while self.tx_busy() {}
    }

    fn stop(&mut self) {
        while self.tx_full() {}
        // SAFETY: Command enum ensures we are writing valid command
        self.reg
            .dcmd()
            .write(|w| unsafe { w.twi_dcmd_cmd().bits(Command::Stop.into()) });
        while self.tx_busy() {}
    }

    fn write_byte(&mut self, byte: u8) -> Result<(), Error> {
        while self.tx_full() {}
        // SAFETY: Command enum ensures we are writing valid command
        self.reg.dcmd().write(|w| unsafe {
            w.twi_dcmd_cmd()
                .bits(Command::Data.into())
                .twi_dcmd_ack()
                .clear_bit()
                .twi_dcmd()
                .bits(byte)
        });
        while self.tx_busy() {}

        if self.device_acked() {
            Ok(())
        } else {
            self.stop();
            Err(Error::NackData)
        }
    }

    fn read_byte(&self, ack: Mack) -> u8 {
        // It appears the only way to ACK received bytes is to
        // write to the DCMD reg to turn on controller-issued ACK,
        // and doing so causes the controller to attempt to
        // drive SDA.
        //
        // So we write all high bits so the controller
        // doesn't actually drive the line.
        while self.tx_full() {}
        // SAFETY: Command enum ensures we are writing valid command
        self.reg.dcmd().write(|w| unsafe {
            w.twi_dcmd_cmd()
                .bits(Command::Data.into())
                .twi_dcmd_ack()
                .bit(ack.into())
                .twi_dcmd()
                .bits(HI_Z)
        });
        while self.tx_busy() {}

        self.reg.dcmd().read().twi_dcmd().bits()
    }

    fn write_addr(&mut self, addr: u8, rw: Rw) -> Result<(), Error> {
        self.write_byte(addr | (u8::from(rw)))
            .map_err(|_| Error::NackAddr)
    }

    fn write_raw(&mut self, write: &[u8]) -> Result<(), Error> {
        for &byte in write {
            self.write_byte(byte)?;
        }

        Ok(())
    }

    fn read_raw(&mut self, read: &mut [u8]) {
        let len = read.len();
        for (i, byte) in read.iter_mut().enumerate() {
            let ack = if i < len - 1 { Mack::Ack } else { Mack::Nack };
            *byte = self.read_byte(ack);
        }
    }

    fn new_inner<T: Instance>(
        _instance: Peri<'d, T>,
        twi_freq: u32,
        clock_stretch_en: bool,
    ) -> Self {
        let cpu_freq = crate::sysinfo::SysInfo::clock_freq() as u64;
        let mut cdiv = (cpu_freq / (4 * twi_freq as u64 * 2)) - 1;
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
            .modify(|_, w| unsafe { w.twi_ctrl_prsc().bits(psc) });

        // Set clock divider
        // SAFETY: We've ensured cdiv can fit in 4 bits
        T::reg()
            .ctrl()
            .modify(|_, w| unsafe { w.twi_ctrl_cdiv().bits(cdiv as u8) });

        // Set clock stretching enable
        T::reg()
            .ctrl()
            .modify(|_, w| w.twi_ctrl_clkstr().bit(clock_stretch_en));

        // Enable TWI
        T::reg().ctrl().modify(|_, w| w.twi_ctrl_en().set_bit());

        Self {
            reg: T::reg(),
            _phantom: PhantomData,
        }
    }

    /// Perform a blocking read from specified address on the TWI bus into buffer.
    ///
    /// # Errors
    ///
    /// Returns [Error::NackAddr] if address is not acknowledged.
    pub fn blocking_read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Error> {
        self.blocking_transaction(address, &mut [Operation::Read(read)])
    }

    /// Perform a blocking write to specified address on the TWI bus from buffer.
    ///
    /// # Errors
    ///
    /// Returns [Error::NackAddr] or [Error::NackData] if address/data is not acknowledged.
    pub fn blocking_write(&mut self, address: u8, write: &[u8]) -> Result<(), Error> {
        self.blocking_transaction(address, &mut [Operation::Write(write)])
    }

    /// Perform a blocking write to specified address on the TWI bus from write buffer,
    /// followed by a blocking read into read buffer.
    ///
    /// # Errors
    ///
    /// Returns [Error::NackAddr] or [Error::NackData] if address/data is not acknowledged.
    pub fn blocking_write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Error> {
        self.blocking_transaction(
            address,
            &mut [Operation::Write(write), Operation::Read(read)],
        )
    }

    /// Perform given list of blocking operations to/from specified address on the TWI bus.
    ///
    /// # Errors
    ///
    /// Returns [Error::NackAddr] or [Error::NackData] if address/data is not acknowledged.
    pub fn blocking_transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        let mut prev = None;

        for operation in operations {
            match operation {
                Operation::Write(data) => {
                    if prev != Some(Rw::Write) {
                        self.start();
                        self.write_addr(address, Rw::Write)?;
                        prev = Some(Rw::Write);
                    }

                    self.write_raw(data)?;
                }
                Operation::Read(data) => {
                    if prev != Some(Rw::Read) {
                        self.start();
                        self.write_addr(address, Rw::Read)?;
                        prev = Some(Rw::Read);
                    }

                    self.read_raw(data);
                }
            }
        }

        self.stop();
        Ok(())
    }
}

impl<'d> Twi<'d, Blocking> {
    /// Returns a new instance of a blocking TWI driver with given frequency (in Hz).
    pub fn new_blocking<T: Instance>(
        _instance: Peri<'d, T>,
        twi_freq: u32,
        clock_stretch_en: bool,
    ) -> Self {
        Self::new_inner(_instance, twi_freq, clock_stretch_en)
    }
}

trait SealedIoMode {}

/// TWI IO mode.
#[allow(private_bounds)]
pub trait IoMode: SealedIoMode {}

/// Blocking TWI.
pub struct Blocking;
impl SealedIoMode for Blocking {}
impl IoMode for Blocking {}

trait SealedInstance {
    fn reg() -> &'static crate::pac::twi::RegisterBlock;
}

/// A valid TWI peripheral.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {}
impl SealedInstance for TWI {
    fn reg() -> &'static crate::pac::twi::RegisterBlock {
        // SAFETY: We own the TWI peripheral and are sure to use it safely
        unsafe { &*crate::pac::Twi::ptr() }
    }
}
impl Instance for TWI {}

impl embedded_hal_1::i2c::Error for Error {
    fn kind(&self) -> embedded_hal_1::i2c::ErrorKind {
        match *self {
            Self::NackAddr => embedded_hal_1::i2c::ErrorKind::NoAcknowledge(
                embedded_hal_1::i2c::NoAcknowledgeSource::Address,
            ),
            Self::NackData => embedded_hal_1::i2c::ErrorKind::NoAcknowledge(
                embedded_hal_1::i2c::NoAcknowledgeSource::Data,
            ),
        }
    }
}

impl<'d, M: IoMode> embedded_hal_1::i2c::ErrorType for Twi<'d, M> {
    type Error = Error;
}

impl<'d, M: IoMode> embedded_hal_1::i2c::I2c for Twi<'d, M> {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.blocking_transaction(address, operations)
    }
}
