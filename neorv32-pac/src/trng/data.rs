#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Field `TRNG_DATA` reader - Random data"]
pub type TrngDataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Random data"]
    #[inline(always)]
    pub fn trng_data(&self) -> TrngDataR {
        TrngDataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Random data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {}
