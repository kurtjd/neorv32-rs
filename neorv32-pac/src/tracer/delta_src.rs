#[doc = "Register `DELTA_SRC` reader"]
pub type R = crate::R<DeltaSrcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Trace data: delta source address + first-packet bit\n\nYou can [`read`](crate::Reg::read) this register and get [`delta_src::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeltaSrcSpec;
impl crate::RegisterSpec for DeltaSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delta_src::R`](R) reader structure"]
impl crate::Readable for DeltaSrcSpec {}
#[doc = "`reset()` method sets DELTA_SRC to value 0"]
impl crate::Resettable for DeltaSrcSpec {}
