//! SysInfo

// As this is a read-only peripheral this driver is designed to be free-standing for ease of use
fn reg() -> &'static crate::pac::sysinfo::RegisterBlock {
    // SAFETY: We only use this pointer internally and do so safely
    unsafe { &*crate::pac::Sysinfo::ptr() }
}

/// Processor boot mode.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootMode {
    Bootloader,
    CustomAddress,
    ImemImage,
    Unknown,
}

impl BootMode {
    /// Returns the boot made as a static string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bootloader => "Bootloader",
            Self::CustomAddress => "Custom Address",
            Self::ImemImage => "IMEM Image",
            Self::Unknown => "Unknown",
        }
    }
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

/// SoC Configuration.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SocConfig(u32);
impl SocConfig {
    /// Returns raw 32-bit SoC config.
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// Returns true if processor-internal bootloader is implemented.
    pub fn bootloader(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Returns true if external bus interface (XBUS) is implemented.
    pub fn xbus(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Returns true if processor-internal IMEM is implemented.
    pub fn imem(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Returns true if processor-internal DMEM is implemented.
    pub fn dmem(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Returns true if on-chip debugger is implemented.
    pub fn ocd(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Returns true if processor-internal instruction cache is implemented.
    pub fn icache(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Returns true if processor-internal data cache is implemented.
    pub fn dcache(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    /// Returns true if on-chip debugger authentication is implemented.
    pub fn ocd_auth(&self) -> bool {
        self.0 & (1 << 11) != 0
    }

    /// Returns true if processor-internal IMEM is implemented as pre-initialized ROM.
    pub fn imem_as_rom(&self) -> bool {
        self.0 & (1 << 12) != 0
    }

    /// Returns true if TWD is implemented.
    pub fn twd(&self) -> bool {
        self.0 & (1 << 13) != 0
    }

    /// Returns true if DMA is implemented.
    pub fn dma(&self) -> bool {
        self.0 & (1 << 14) != 0
    }

    /// Returns true if GPIO is implemented.
    pub fn gpio(&self) -> bool {
        self.0 & (1 << 15) != 0
    }

    /// Returns true if CLINT is implemented.
    pub fn clint(&self) -> bool {
        self.0 & (1 << 16) != 0
    }

    /// Returns true if UART0 is implemented.
    pub fn uart0(&self) -> bool {
        self.0 & (1 << 17) != 0
    }

    /// Returns true if SPI is implemented.
    pub fn spi(&self) -> bool {
        self.0 & (1 << 18) != 0
    }

    /// Returns true if TWI is implemented.
    pub fn twi(&self) -> bool {
        self.0 & (1 << 19) != 0
    }

    /// Returns true if PWM is implemented.
    pub fn pwm(&self) -> bool {
        self.0 & (1 << 20) != 0
    }

    /// Returns true if WDT is implemented.
    pub fn wdt(&self) -> bool {
        self.0 & (1 << 21) != 0
    }

    /// Returns true if CFS is implemented.
    pub fn cfs(&self) -> bool {
        self.0 & (1 << 22) != 0
    }

    /// Returns true if TRNG is implemented.
    pub fn trng(&self) -> bool {
        self.0 & (1 << 23) != 0
    }

    /// Returns true if SDI is implemented.
    pub fn sdi(&self) -> bool {
        self.0 & (1 << 24) != 0
    }

    /// Returns true if UART1 is implemented.
    pub fn uart1(&self) -> bool {
        self.0 & (1 << 25) != 0
    }

    /// Returns true if NEOLED is implemented.
    pub fn neoled(&self) -> bool {
        self.0 & (1 << 26) != 0
    }

    /// Returns true if TRACER is implemented.
    pub fn tracer(&self) -> bool {
        self.0 & (1 << 27) != 0
    }

    /// Returns true if GPTMR is implemented.
    pub fn gptmr(&self) -> bool {
        self.0 & (1 << 28) != 0
    }

    /// Returns true if SLINK is implemented.
    pub fn slink(&self) -> bool {
        self.0 & (1 << 29) != 0
    }

    /// Returns true if ONEWIRE is implemented.
    pub fn onewire(&self) -> bool {
        self.0 & (1 << 30) != 0
    }

    /// Returns true if NEORV32 is being simulated.
    pub fn simulation(&self) -> bool {
        self.0 & (1 << 31) != 0
    }
}

/// SysInfo Driver
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
