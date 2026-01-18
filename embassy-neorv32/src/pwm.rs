//! Pulse Width Modulation (PWM)
use crate::sysinfo::SysInfo;
use core::marker::PhantomData;
use critical_section::{self, CriticalSection};
use embassy_hal_internal::{Peri, PeripheralType};

/// PWM error.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The NEORV32 configuration does not support PWM.
    NotSupported,
    /// Invalid duty cycle.
    InvalidDuty,
}

/// A duty cycle percent.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Percent(u8);
impl Percent {
    /// Create a new percent.
    ///
    /// Returns [`Error::InvalidDuty`] if `percent > 100`.
    pub fn new(percent: u8) -> Result<Self, Error> {
        if percent <= 100 {
            Ok(Self(percent))
        } else {
            Err(Error::InvalidDuty)
        }
    }

    /// Get the raw percent value (0-100%).
    pub fn inner(&self) -> u8 {
        self.0
    }
}

/// PWM clock prescaler.
pub enum ClkPrsc {
    /// Divide main CPU clock by 2.
    _2,
    /// Divide main CPU clock by 4.
    _4,
    /// Divide main CPU clock by 8.
    _8,
    /// Divide main CPU clock by 64.
    _64,
    /// Divide main CPU clock by 128.
    _128,
    /// Divide main CPU clock by 1024.
    _1024,
    /// Divide main CPU clock by 2048.
    _2048,
    /// Divide main CPU clock by 4096.
    _4096,
}

impl ClkPrsc {
    fn bits(&self) -> u32 {
        match *self {
            Self::_2 => 0b000,
            Self::_4 => 0b001,
            Self::_8 => 0b010,
            Self::_64 => 0b011,
            Self::_128 => 0b100,
            Self::_1024 => 0b101,
            Self::_2048 => 0b110,
            Self::_4096 => 0b111,
        }
    }

    fn from_bits(raw: u32) -> Self {
        match raw {
            0b000 => Self::_2,
            0b001 => Self::_4,
            0b010 => Self::_8,
            0b011 => Self::_64,
            0b100 => Self::_128,
            0b101 => Self::_1024,
            0b110 => Self::_2048,
            0b111 => Self::_4096,
            _ => unreachable!("Hardware register is only 3 bits wide"),
        }
    }
}

impl From<ClkPrsc> for u16 {
    fn from(clkprsc: ClkPrsc) -> Self {
        match clkprsc {
            ClkPrsc::_2 => 2,
            ClkPrsc::_4 => 4,
            ClkPrsc::_8 => 8,
            ClkPrsc::_64 => 64,
            ClkPrsc::_128 => 128,
            ClkPrsc::_1024 => 1024,
            ClkPrsc::_2048 => 2048,
            ClkPrsc::_4096 => 4096,
        }
    }
}

/// PWM operation mode.
pub enum Mode {
    /// Fast mode.
    Fast,
    /// Phase-correct mode.
    PhaseCorrect,
}

/// PWM driver.
///
/// Must be initialized first with a chosen clock prescaler before initializing individual channels.
pub struct Pwm<'d> {
    reg: &'static crate::pac::pwm::RegisterBlock,
    _phantom: PhantomData<&'d ()>,
}

impl<'d> Pwm<'d> {
    /// Create a new PWM driver instance.
    ///
    /// The given clock prescaler will be applied to all channels and affects which channel
    /// frequencies can be accurately represented.
    ///
    /// To then create a new channel instance, call [`Self::new_channel`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotSupported`] if PWM is not supported.
    pub fn new<T: Instance>(_instance: Peri<'d, T>, clkprsc: ClkPrsc) -> Result<Self, Error> {
        if !crate::sysinfo::SysInfo::soc_config().has_pwm() {
            return Err(Error::NotSupported);
        }

        // SAFETY: Enum ensures we are writing valid prescaler
        T::reg()
            .clkprsc()
            .write(|w| unsafe { w.bits(clkprsc.bits()) });

        Ok(Self {
            reg: T::reg(),
            _phantom: PhantomData,
        })
    }

    /// Create a new instance of a PWM channel driver with given mode and frequency and enables it.
    ///
    /// Depending on main clock frequency and prescaler, not all frequencies can be represented exactly.
    ///
    /// # Panics
    ///
    /// Panics if `pwm_freq == 0`, or `pwm_freq` is too small or too large to be represented.
    pub fn new_channel<T: ChannelInstance>(
        &self,
        _instance: Peri<'d, T>,
        mode: Mode,
        pwm_freq: u32,
        invert_polarity: bool,
    ) -> PwmChan<'d> {
        PwmChan::new(_instance, self.reg, mode, pwm_freq, invert_polarity)
    }
}

/// PWM channel driver.
///
/// **Note**: The PWM channel will be disabled when dropped.
pub struct PwmChan<'d> {
    reg: &'static crate::pac::pwm::RegisterBlock,
    channel: usize,
    _phantom: PhantomData<&'d ()>,
}

// Allows for use in a Mutex (to share safely between harts and tasks)
unsafe impl<'d> Send for PwmChan<'d> {}

impl<'d> PwmChan<'d> {
    fn new<T: ChannelInstance>(
        _instance: Peri<'d, T>,
        reg: &'static crate::pac::pwm::RegisterBlock,
        mode: Mode,
        pwm_freq: u32,
        invert_polarity: bool,
    ) -> Self {
        let mut pwm = Self {
            reg,
            channel: T::channel(),
            _phantom: PhantomData,
        };

        pwm.set_freq(pwm_freq);

        // These all modify config registers, which all PWM channels share, hence the CS here
        critical_section::with(|cs| {
            if invert_polarity {
                pwm.invert_polarity(cs);
            }
            pwm.set_mode(cs, mode);
            pwm.enable(cs);
        });

        pwm
    }

    fn enable(&mut self, _cs: CriticalSection) {
        // SAFETY: Bit mask preserves previous channel values
        self.reg
            .enable()
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.channel)) });
    }

    fn disable(&mut self, _cs: CriticalSection) {
        // SAFETY: Bit mask preserves previous channel values
        self.reg
            .enable()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.channel)) });
    }

    fn invert_polarity(&mut self, _cs: CriticalSection) {
        // SAFETY: Bit mask preserves previous channel values
        self.reg
            .polarity()
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.channel)) });
    }

    fn set_mode(&mut self, _cs: CriticalSection, mode: Mode) {
        // SAFETY: Bit mask preserves previous channel values
        self.reg.mode().modify(|r, w| unsafe {
            w.bits(match mode {
                Mode::Fast => r.bits() & !(1 << self.channel),
                Mode::PhaseCorrect => r.bits() | (1 << self.channel),
            })
        });
    }

    fn mode(&self) -> Mode {
        let mode_raw = self.reg.mode().read().bits() & (1 << self.channel);
        if mode_raw != 0 {
            Mode::PhaseCorrect
        } else {
            Mode::Fast
        }
    }

    fn top(&self) -> u16 {
        self.reg.channel(self.channel).topcmp().read().top().bits()
    }

    fn cmp(&self) -> u16 {
        self.reg.channel(self.channel).topcmp().read().cmp().bits()
    }

    fn clkprsc(&self) -> ClkPrsc {
        ClkPrsc::from_bits(self.reg.clkprsc().read().bits())
    }

    fn duty_cycle(&self) -> Percent {
        let cmp = self.cmp() as u32;
        let top = self.top() as u32;
        let denom = top + 1;
        let percent = ((100 * cmp + (denom / 2)) / denom) as u8;

        Percent::new(percent).expect("Infallible")
    }

    fn set_freq(&mut self, pwm_freq: u32) {
        assert!(pwm_freq != 0);

        let clkprsc = u16::from(self.clkprsc()) as u64;
        let cpuclk = SysInfo::clock_freq() as u64;

        let new_top = match self.mode() {
            Mode::Fast => (cpuclk / (clkprsc * pwm_freq as u64)) - 1,
            Mode::PhaseCorrect => cpuclk / (2 * clkprsc * pwm_freq as u64),
        };

        assert!(new_top <= u16::MAX as u64);

        // SAFETY: We've ensured a valid TOP value above
        self.reg
            .channel(self.channel)
            .topcmp()
            .modify(|_, w| unsafe { w.top().bits(new_top as u16) });
    }

    /// Set the PWM channel duty cycle in percent.
    pub fn set_duty_cycle(&mut self, percent: Percent) {
        let percent = u32::from(percent.inner());
        let top = self.top() as u32;
        let denom = 100;
        let cmp = (((percent) * (top + 1) + (denom / 2)) / denom) as u16;

        // SAFETY: We've ensured a valid CMP value above
        self.reg
            .channel(self.channel)
            .topcmp()
            .modify(|_, w| unsafe { w.cmp().bits(cmp) });
    }
}

impl<'d> Drop for PwmChan<'d> {
    fn drop(&mut self) {
        critical_section::with(|cs| self.disable(cs));
    }
}

trait SealedInstance {
    fn reg() -> &'static crate::pac::pwm::RegisterBlock;
}

/// A valid PWM instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {}

impl SealedInstance for crate::peripherals::PWM {
    fn reg() -> &'static crate::pac::pwm::RegisterBlock {
        // SAFETY: We own the PWM peripheral and use it safely
        unsafe { &*crate::pac::Pwm::ptr() }
    }
}
impl Instance for crate::peripherals::PWM {}

trait SealedChannelInstance {
    fn channel() -> usize;
}

/// A valid PWM channel.
#[allow(private_bounds)]
pub trait ChannelInstance: SealedChannelInstance + PeripheralType {}

macro_rules! impl_chan {
    ($periph:ident, $ch:expr) => {
        impl SealedChannelInstance for crate::peripherals::$periph {
            fn channel() -> usize {
                $ch
            }
        }
        impl ChannelInstance for crate::peripherals::$periph {}
    };
}

impl_chan!(PWMCHAN0, 0);
impl_chan!(PWMCHAN1, 1);
impl_chan!(PWMCHAN2, 2);
impl_chan!(PWMCHAN3, 3);
impl_chan!(PWMCHAN4, 4);
impl_chan!(PWMCHAN5, 5);
impl_chan!(PWMCHAN6, 6);
impl_chan!(PWMCHAN7, 7);
impl_chan!(PWMCHAN8, 8);
impl_chan!(PWMCHAN9, 9);
impl_chan!(PWMCHAN10, 10);
impl_chan!(PWMCHAN11, 11);
impl_chan!(PWMCHAN12, 12);
impl_chan!(PWMCHAN13, 13);
impl_chan!(PWMCHAN14, 14);
impl_chan!(PWMCHAN15, 15);
impl_chan!(PWMCHAN16, 16);
impl_chan!(PWMCHAN17, 17);
impl_chan!(PWMCHAN18, 18);
impl_chan!(PWMCHAN19, 19);
impl_chan!(PWMCHAN20, 20);
impl_chan!(PWMCHAN21, 21);
impl_chan!(PWMCHAN22, 22);
impl_chan!(PWMCHAN23, 23);
impl_chan!(PWMCHAN24, 24);
impl_chan!(PWMCHAN25, 25);
impl_chan!(PWMCHAN26, 26);
impl_chan!(PWMCHAN27, 27);
impl_chan!(PWMCHAN28, 28);
impl_chan!(PWMCHAN29, 29);
impl_chan!(PWMCHAN30, 30);
impl_chan!(PWMCHAN31, 31);

impl<'d> embedded_hal_02::PwmPin for PwmChan<'d> {
    type Duty = Percent;

    fn enable(&mut self) {
        critical_section::with(|cs| self.enable(cs));
    }

    fn disable(&mut self) {
        critical_section::with(|cs| self.disable(cs));
    }

    fn get_duty(&self) -> Self::Duty {
        self.duty_cycle()
    }

    fn get_max_duty(&self) -> Self::Duty {
        Percent(100)
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        self.set_duty_cycle(duty);
    }
}

impl embedded_hal_1::pwm::Error for Error {
    fn kind(&self) -> embedded_hal_1::pwm::ErrorKind {
        embedded_hal_1::pwm::ErrorKind::Other
    }
}

impl<'d> embedded_hal_1::pwm::ErrorType for PwmChan<'d> {
    type Error = Error;
}

impl<'d> embedded_hal_1::pwm::SetDutyCycle for PwmChan<'d> {
    fn max_duty_cycle(&self) -> u16 {
        100
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        let percent = Percent::new(duty as u8)?;
        self.set_duty_cycle(percent);
        Ok(())
    }
}
