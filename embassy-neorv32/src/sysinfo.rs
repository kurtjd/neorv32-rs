//! SysInfo
//!
//! As this is a read-only peripheral, this driver is designed to be free-standing for ease of use.
//! All functions can be called directly on [`SysInfo`] without needing to instantiate a singleton.

/// Processor boot mode.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootMode {
    /// Processor-internal BOOTROM as pre-initialized ROM.
    Bootloader,
    /// User-defined address.
    CustomAddress,
    /// Processor-internal IMEM as pre-initialized ROM.
    ImemImage,
    /// Unrecognized boot mode.
    Unknown,
}

impl From<u8> for BootMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Bootloader,
            1 => Self::CustomAddress,
            2 => Self::ImemImage,
            _ => Self::Unknown,
        }
    }
}

/// SoC configuration.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SocConfig(u32);

impl SocConfig {
    #[inline(always)]
    fn supported(&self, i: u32) -> bool {
        self.0 & (1 << i) != 0
    }

    /// Returns raw 32-bit SoC config.
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// Returns true if processor-internal bootloader is implemented.
    pub fn bootloader(&self) -> bool {
        self.supported(0)
    }

    /// Returns true if external bus interface (XBUS) is implemented.
    pub fn xbus(&self) -> bool {
        self.supported(1)
    }

    /// Returns true if processor-internal IMEM is implemented.
    pub fn imem(&self) -> bool {
        self.supported(2)
    }

    /// Returns true if processor-internal DMEM is implemented.
    pub fn dmem(&self) -> bool {
        self.supported(3)
    }

    /// Returns true if on-chip debugger is implemented.
    pub fn ocd(&self) -> bool {
        self.supported(4)
    }

    /// Returns true if processor-internal instruction cache is implemented.
    pub fn icache(&self) -> bool {
        self.supported(5)
    }

    /// Returns true if processor-internal data cache is implemented.
    pub fn dcache(&self) -> bool {
        self.supported(6)
    }

    /// Returns true if on-chip debugger authentication is implemented.
    pub fn ocd_auth(&self) -> bool {
        self.supported(11)
    }

    /// Returns true if processor-internal IMEM is implemented as pre-initialized ROM.
    pub fn imem_as_rom(&self) -> bool {
        self.supported(12)
    }

    /// Returns true if TWD is implemented.
    pub fn twd(&self) -> bool {
        self.supported(13)
    }

    /// Returns true if DMA is implemented.
    pub fn dma(&self) -> bool {
        self.supported(14)
    }

    /// Returns true if GPIO is implemented.
    pub fn gpio(&self) -> bool {
        self.supported(15)
    }

    /// Returns true if CLINT is implemented.
    pub fn clint(&self) -> bool {
        self.supported(16)
    }

    /// Returns true if UART0 is implemented.
    pub fn uart0(&self) -> bool {
        self.supported(17)
    }

    /// Returns true if SPI is implemented.
    pub fn spi(&self) -> bool {
        self.supported(18)
    }

    /// Returns true if TWI is implemented.
    pub fn twi(&self) -> bool {
        self.supported(19)
    }

    /// Returns true if PWM is implemented.
    pub fn pwm(&self) -> bool {
        self.supported(20)
    }

    /// Returns true if WDT is implemented.
    pub fn wdt(&self) -> bool {
        self.supported(21)
    }

    /// Returns true if CFS is implemented.
    pub fn cfs(&self) -> bool {
        self.supported(22)
    }

    /// Returns true if TRNG is implemented.
    pub fn trng(&self) -> bool {
        self.supported(23)
    }

    /// Returns true if SDI is implemented.
    pub fn sdi(&self) -> bool {
        self.supported(24)
    }

    /// Returns true if UART1 is implemented.
    pub fn uart1(&self) -> bool {
        self.supported(25)
    }

    /// Returns true if NEOLED is implemented.
    pub fn neoled(&self) -> bool {
        self.supported(26)
    }

    /// Returns true if TRACER is implemented.
    pub fn tracer(&self) -> bool {
        self.supported(27)
    }

    /// Returns true if GPTMR is implemented.
    pub fn gptmr(&self) -> bool {
        self.supported(28)
    }

    /// Returns true if SLINK is implemented.
    pub fn slink(&self) -> bool {
        self.supported(29)
    }

    /// Returns true if ONEWIRE is implemented.
    pub fn onewire(&self) -> bool {
        self.supported(30)
    }

    /// Returns true if NEORV32 is being simulated.
    pub fn simulation(&self) -> bool {
        self.supported(31)
    }
}

/// SysInfo driver
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SysInfo;

impl SysInfo {
    /// Returns the main CPU clock frequency (Hz).
    pub fn clock_freq() -> u32 {
        reg().clk().read().bits()
    }

    /// Returns the IMEM size in bytes.
    pub fn imem_size() -> u32 {
        1 << reg().mem().read().sysinfo_misc_imem().bits()
    }

    /// Returns the DMEM size in bytes.
    pub fn dmem_size() -> u32 {
        1 << reg().mem().read().sysinfo_misc_dmem().bits()
    }

    /// Returns the number of harts (cores).
    pub fn num_harts() -> u8 {
        reg().mem().read().sysinfo_misc_hart().bits()
    }

    /// Returns the boot mode configuration.
    pub fn boot_mode() -> BootMode {
        let raw = reg().mem().read().sysinfo_misc_boot().bits();
        BootMode::from(raw)
    }

    /// Returns the number of internal bus timeout cycles.
    pub fn bus_itmo_cycles() -> u32 {
        1 << reg().mem().read().sysinfo_misc_itmo().bits()
    }

    /// Returns the number of external bus timeout cycles.
    pub fn bus_etmo_cycles() -> u32 {
        1 << reg().mem().read().sysinfo_misc_etmo().bits()
    }

    /// Returns the SoC config.
    ///
    /// Additional methods can be called to check if SoC features are implemented.
    pub fn soc_config() -> SocConfig {
        SocConfig(reg().soc().read().bits())
    }
}

fn reg() -> &'static crate::pac::sysinfo::RegisterBlock {
    // SAFETY: We only use this pointer internally and do so safely
    unsafe { &*crate::pac::Sysinfo::ptr() }
}
