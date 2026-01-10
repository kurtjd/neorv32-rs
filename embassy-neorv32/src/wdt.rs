//! Watchdog Timer (WDT)
use crate::peripherals::WDT;
use core::marker::PhantomData;
use embassy_hal_internal::{Peri, PeripheralType};

/// Cause of last hardware reset.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResetCause {
    /// Reset caused by external reset signal pin.
    External,
    /// Reset caused by on-chip debugger.
    OnChipDebugger,
    /// Reset caused by watchdog timeout.
    Timeout,
    /// Reset caused by illegal watchdog access.
    IllegalAccess,
    /// Reset caused by unknown source.
    Unknown,
}

impl From<u8> for ResetCause {
    fn from(value: u8) -> Self {
        match value {
            0b00 => Self::External,
            0b01 => Self::OnChipDebugger,
            0b10 => Self::Timeout,
            0b11 => Self::IllegalAccess,
            _ => Self::Unknown,
        }
    }
}

/// WDT error.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The NEORV32 configuration does not support WDT.
    NotSupported,
}

/// Watchdog Timer (WDT) Driver.
pub struct Wdt<'d, M: LockMode> {
    reg: &'static crate::pac::wdt::RegisterBlock,
    _phantom: PhantomData<&'d M>,
}

impl<'d> Wdt<'d, Unlocked> {
    /// Returns a new unlocked WDT with timeout set to 24-bit max.
    ///
    /// Caller should configure timeout and then enable the WDT.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotSupported`] if WDT is not supported.
    pub fn new<T: Instance>(_instance: Peri<'d, T>) -> Result<Self, Error> {
        if !crate::sysinfo::SysInfo::soc_config().wdt() {
            return Err(Error::NotSupported);
        }

        let wdt = Self {
            reg: T::reg(),
            _phantom: PhantomData,
        };

        // Set timeout to max so WDT does not immediately reset if user calls `enable` before `set_timeout`
        wdt.set_timeout(0xffffff);
        Ok(wdt)
    }

    /// Enable WDT.
    pub fn enable(&self) {
        self.reg.ctrl().modify(|_, w| w.wdt_ctrl_en().set_bit());
    }

    /// Disable WDT.
    ///
    /// Resets the internal timeout counter to 0.
    pub fn disable(&self) {
        self.reg.ctrl().modify(|_, w| w.wdt_ctrl_en().clear_bit());
    }

    /// Returns true if WDT is enabled.
    pub fn enabled(&self) -> bool {
        self.reg.ctrl().read().wdt_ctrl_en().bit_is_set()
    }

    /// Sets 24-bit WDT timeout value.
    ///
    /// WDT counter is clocked at 1/4096 of CPU clock frequency.
    pub fn set_timeout(&self, timeout: u32) {
        // SAFETY: Any u32 is a valid timeout
        self.reg
            .ctrl()
            .modify(|_, w| unsafe { w.wdt_ctrl_timeout().bits(timeout) });
    }

    /// Sets WDT timeout value in milliseconds (ms).
    ///
    /// Millisecond precision may not be possible depending on configured main clock frequency.
    pub fn set_timeout_ms(&self, timeout_ms: u32) {
        let wdt_clock_freq: u32 = crate::sysinfo::SysInfo::clock_freq() / 4096;
        let ticks_per_ms: u32 = wdt_clock_freq / 1000;
        let timeout = timeout_ms * ticks_per_ms;
        self.set_timeout(timeout);
    }

    /// Returns a locked WDT which prevents illegal access.
    ///
    /// The only way to unlock the WDT is via system reset.
    #[must_use]
    pub fn lock(self) -> Wdt<'d, Locked> {
        self.reg.ctrl().modify(|_, w| w.wdt_ctrl_lock().set_bit());
        Wdt {
            reg: self.reg,
            _phantom: PhantomData,
        }
    }
}

impl<'d, M: LockMode> Wdt<'d, M> {
    /// Resets WDT timeout counter.
    pub fn feed(&self) {
        const PASSWORD: u32 = 0x709D1AB3;
        // SAFETY: We write the valid password
        self.reg
            .reset()
            .write(|w| unsafe { w.wdt_reset().bits(PASSWORD) });
    }

    /// Returns the cause of the last hardware reset.
    pub fn reset_cause(&self) -> ResetCause {
        let cause_raw = self.reg.ctrl().read().wdt_ctrl_rcause().bits();
        ResetCause::from(cause_raw)
    }

    /// Forces a hardware reset by feeding an incorrect password to the WDT.
    pub fn force_hw_reset(&self) {
        // WDT must be enabled for illegal access resets to trigger
        // It also appears that the WDT must be locked as well for incorrect password to trigger reset
        self.reg
            .ctrl()
            .modify(|_, w| w.wdt_ctrl_en().set_bit().wdt_ctrl_lock().set_bit());

        // Feed incorrect password to trigger reset
        // SAFETY: We intentionally feed a bad password so this is defined behavior
        self.reg
            .reset()
            .write(|w| unsafe { w.wdt_reset().bits(0xDEADBEEF) });
    }
}

trait SealedLockMode {}

/// WDT lock mode.
#[allow(private_bounds)]
pub trait LockMode: SealedLockMode {}

/// WDT is unlocked and all registers can be written to.
pub struct Unlocked;
impl SealedLockMode for Unlocked {}
impl LockMode for Unlocked {}

/// WDT is locked and certain registers cannot be written to.
///
/// Attempting to circumvent the HAL and writing anyway will trigger reset.
pub struct Locked;
impl SealedLockMode for Locked {}
impl LockMode for Locked {}

trait SealedInstance {
    fn reg() -> &'static crate::pac::wdt::RegisterBlock;
}

/// A valid WDT peripheral.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {}
impl SealedInstance for WDT {
    fn reg() -> &'static crate::pac::wdt::RegisterBlock {
        // SAFETY: We only use this ptr internally and ensure we do so safely
        unsafe { &*crate::pac::Wdt::ptr() }
    }
}
impl Instance for WDT {}
