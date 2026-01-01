#[doc = "Register `SOC` reader"]
pub type R = crate::R<SocSpec>;
#[doc = "Field `SYSINFO_SOC_BOOTLOADER` reader - Bootloader implemented"]
pub type SysinfoSocBootloaderR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_XBUS` reader - External bus interface implemented"]
pub type SysinfoSocXbusR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IMEM` reader - Processor-internal instruction memory implemented"]
pub type SysinfoSocImemR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_DMEM` reader - Processor-internal data memory implemented"]
pub type SysinfoSocDmemR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_OCD` reader - On-chip debugger implemented"]
pub type SysinfoSocOcdR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_ICACHE` reader - Processor-internal instruction cache implemented"]
pub type SysinfoSocIcacheR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_DCACHE` reader - Processor-internal data cache implemented"]
pub type SysinfoSocDcacheR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_OCD_AUTH` reader - On-chip debugger authentication implemented"]
pub type SysinfoSocOcdAuthR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IMEM_ROM` reader - Processor-internal instruction memory implemented as pre-initialized ROM"]
pub type SysinfoSocImemRomR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_TWD` reader - Two-wire device implemented"]
pub type SysinfoSocIoTwdR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_DMA` reader - Direct memory access controller implemented"]
pub type SysinfoSocIoDmaR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_GPIO` reader - General purpose input/output port unit implemented"]
pub type SysinfoSocIoGpioR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_CLINT` reader - Core local interruptor implemented"]
pub type SysinfoSocIoClintR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_UART0` reader - Primary universal asynchronous receiver/transmitter implemented"]
pub type SysinfoSocIoUart0R = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_SPI` reader - Serial peripheral interface implemented"]
pub type SysinfoSocIoSpiR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_TWI` reader - Two-wire interface implemented"]
pub type SysinfoSocIoTwiR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_PWM` reader - Pulse-width modulation unit implemented"]
pub type SysinfoSocIoPwmR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_WDT` reader - Watchdog timer implemented"]
pub type SysinfoSocIoWdtR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_CFS` reader - Custom functions subsystem implemented"]
pub type SysinfoSocIoCfsR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_TRNG` reader - True random number generator implemented"]
pub type SysinfoSocIoTrngR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_SDI` reader - Serial data interface implemented"]
pub type SysinfoSocIoSdiR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_UART1` reader - Secondary universal asynchronous receiver/transmitter implemented"]
pub type SysinfoSocIoUart1R = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_NEOLED` reader - NeoPixel-compatible smart LED interface implemented"]
pub type SysinfoSocIoNeoledR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_TRACER` reader - Execution tracer implemented"]
pub type SysinfoSocIoTracerR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_GPTMR` reader - General purpose timer implemented"]
pub type SysinfoSocIoGptmrR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_SLINK` reader - Stream link interface implemented"]
pub type SysinfoSocIoSlinkR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_IO_ONEWIRE` reader - 1-wire interface controller implemented"]
pub type SysinfoSocIoOnewireR = crate::BitReader;
#[doc = "Field `SYSINFO_SOC_SIM` reader - Set if this is a simulation"]
pub type SysinfoSocSimR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Bootloader implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_bootloader(&self) -> SysinfoSocBootloaderR {
        SysinfoSocBootloaderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External bus interface implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_xbus(&self) -> SysinfoSocXbusR {
        SysinfoSocXbusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Processor-internal instruction memory implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_imem(&self) -> SysinfoSocImemR {
        SysinfoSocImemR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Processor-internal data memory implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_dmem(&self) -> SysinfoSocDmemR {
        SysinfoSocDmemR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - On-chip debugger implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_ocd(&self) -> SysinfoSocOcdR {
        SysinfoSocOcdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Processor-internal instruction cache implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_icache(&self) -> SysinfoSocIcacheR {
        SysinfoSocIcacheR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Processor-internal data cache implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_dcache(&self) -> SysinfoSocDcacheR {
        SysinfoSocDcacheR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - On-chip debugger authentication implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_ocd_auth(&self) -> SysinfoSocOcdAuthR {
        SysinfoSocOcdAuthR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Processor-internal instruction memory implemented as pre-initialized ROM"]
    #[inline(always)]
    pub fn sysinfo_soc_imem_rom(&self) -> SysinfoSocImemRomR {
        SysinfoSocImemRomR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Two-wire device implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_twd(&self) -> SysinfoSocIoTwdR {
        SysinfoSocIoTwdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Direct memory access controller implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_dma(&self) -> SysinfoSocIoDmaR {
        SysinfoSocIoDmaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - General purpose input/output port unit implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_gpio(&self) -> SysinfoSocIoGpioR {
        SysinfoSocIoGpioR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Core local interruptor implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_clint(&self) -> SysinfoSocIoClintR {
        SysinfoSocIoClintR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Primary universal asynchronous receiver/transmitter implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_uart0(&self) -> SysinfoSocIoUart0R {
        SysinfoSocIoUart0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Serial peripheral interface implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_spi(&self) -> SysinfoSocIoSpiR {
        SysinfoSocIoSpiR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Two-wire interface implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_twi(&self) -> SysinfoSocIoTwiR {
        SysinfoSocIoTwiR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pulse-width modulation unit implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_pwm(&self) -> SysinfoSocIoPwmR {
        SysinfoSocIoPwmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Watchdog timer implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_wdt(&self) -> SysinfoSocIoWdtR {
        SysinfoSocIoWdtR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Custom functions subsystem implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_cfs(&self) -> SysinfoSocIoCfsR {
        SysinfoSocIoCfsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - True random number generator implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_trng(&self) -> SysinfoSocIoTrngR {
        SysinfoSocIoTrngR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Serial data interface implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_sdi(&self) -> SysinfoSocIoSdiR {
        SysinfoSocIoSdiR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Secondary universal asynchronous receiver/transmitter implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_uart1(&self) -> SysinfoSocIoUart1R {
        SysinfoSocIoUart1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - NeoPixel-compatible smart LED interface implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_neoled(&self) -> SysinfoSocIoNeoledR {
        SysinfoSocIoNeoledR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Execution tracer implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_tracer(&self) -> SysinfoSocIoTracerR {
        SysinfoSocIoTracerR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - General purpose timer implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_gptmr(&self) -> SysinfoSocIoGptmrR {
        SysinfoSocIoGptmrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Stream link interface implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_slink(&self) -> SysinfoSocIoSlinkR {
        SysinfoSocIoSlinkR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1-wire interface controller implemented"]
    #[inline(always)]
    pub fn sysinfo_soc_io_onewire(&self) -> SysinfoSocIoOnewireR {
        SysinfoSocIoOnewireR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set if this is a simulation"]
    #[inline(always)]
    pub fn sysinfo_soc_sim(&self) -> SysinfoSocSimR {
        SysinfoSocSimR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SoC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`soc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocSpec;
impl crate::RegisterSpec for SocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc::R`](R) reader structure"]
impl crate::Readable for SocSpec {}
#[doc = "`reset()` method sets SOC to value 0"]
impl crate::Resettable for SocSpec {}
