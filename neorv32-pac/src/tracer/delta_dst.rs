#[doc = "Register `DELTA_DST` reader"]
pub type R = crate::R<DeltaDstSpec>;
impl core::fmt::Debug for crate::generic::Reg<DeltaDstSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Trace data: delta destination address + trap-entry bit\n\nYou can [`read`](crate::Reg::read) this register and get [`delta_dst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct DeltaDstSpec;
impl crate::RegisterSpec for DeltaDstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delta_dst::R`](R) reader structure"]
impl crate::Readable for DeltaDstSpec {}
#[doc = "`reset()` method sets DELTA_DST to value 0"]
impl crate::Resettable for DeltaDstSpec {}
