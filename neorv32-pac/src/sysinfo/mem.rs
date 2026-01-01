#[doc = "Register `MEM` reader"]
pub type R = crate::R<MemSpec>;
#[doc = "Field `SYSINFO_MISC_IMEM` reader - log2(IMEM size in bytes)"]
pub type SysinfoMiscImemR = crate::FieldReader;
#[doc = "Field `SYSINFO_MISC_DMEM` reader - log2(DMEM size in bytes)"]
pub type SysinfoMiscDmemR = crate::FieldReader;
#[doc = "Field `SYSINFO_MISC_HART` reader - Number of physical CPU cores"]
pub type SysinfoMiscHartR = crate::FieldReader;
#[doc = "Field `SYSINFO_MISC_BOOT` reader - Boot mode configuration select"]
pub type SysinfoMiscBootR = crate::FieldReader;
#[doc = "Field `SYSINFO_MISC_ITMO` reader - log2(internal bus timeout cycles)"]
pub type SysinfoMiscItmoR = crate::FieldReader;
#[doc = "Field `SYSINFO_MISC_ETMO` reader - log2(external bus timeout cycles)"]
pub type SysinfoMiscEtmoR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - log2(IMEM size in bytes)"]
    #[inline(always)]
    pub fn sysinfo_misc_imem(&self) -> SysinfoMiscImemR {
        SysinfoMiscImemR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - log2(DMEM size in bytes)"]
    #[inline(always)]
    pub fn sysinfo_misc_dmem(&self) -> SysinfoMiscDmemR {
        SysinfoMiscDmemR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Number of physical CPU cores"]
    #[inline(always)]
    pub fn sysinfo_misc_hart(&self) -> SysinfoMiscHartR {
        SysinfoMiscHartR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Boot mode configuration select"]
    #[inline(always)]
    pub fn sysinfo_misc_boot(&self) -> SysinfoMiscBootR {
        SysinfoMiscBootR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:26 - log2(internal bus timeout cycles)"]
    #[inline(always)]
    pub fn sysinfo_misc_itmo(&self) -> SysinfoMiscItmoR {
        SysinfoMiscItmoR::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - log2(external bus timeout cycles)"]
    #[inline(always)]
    pub fn sysinfo_misc_etmo(&self) -> SysinfoMiscEtmoR {
        SysinfoMiscEtmoR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[doc = "Miscellaneous system configurations\n\nYou can [`read`](crate::Reg::read) this register and get [`mem::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSpec;
impl crate::RegisterSpec for MemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem::R`](R) reader structure"]
impl crate::Readable for MemSpec {}
#[doc = "`reset()` method sets MEM to value 0"]
impl crate::Resettable for MemSpec {}
