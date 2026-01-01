#[doc = "Register `CLKPRSC` reader"]
pub type R = crate::R<ClkprscSpec>;
#[doc = "Register `CLKPRSC` writer"]
pub type W = crate::W<ClkprscSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock prescaler select\n\nYou can [`read`](crate::Reg::read) this register and get [`clkprsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkprsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkprscSpec;
impl crate::RegisterSpec for ClkprscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkprsc::R`](R) reader structure"]
impl crate::Readable for ClkprscSpec {}
#[doc = "`write(|w| ..)` method takes [`clkprsc::W`](W) writer structure"]
impl crate::Writable for ClkprscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKPRSC to value 0"]
impl crate::Resettable for ClkprscSpec {}
