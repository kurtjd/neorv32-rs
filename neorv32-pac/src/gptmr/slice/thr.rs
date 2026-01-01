#[doc = "Register `THR` reader"]
pub type R = crate::R<ThrSpec>;
#[doc = "Register `THR` writer"]
pub type W = crate::W<ThrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThrSpec;
impl crate::RegisterSpec for ThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thr::R`](R) reader structure"]
impl crate::Readable for ThrSpec {}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for ThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for ThrSpec {}
