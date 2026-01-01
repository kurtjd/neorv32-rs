#[doc = "Register `POLARITY` reader"]
pub type R = crate::R<PolaritySpec>;
#[doc = "Register `POLARITY` writer"]
pub type W = crate::W<PolaritySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel polarity\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PolaritySpec;
impl crate::RegisterSpec for PolaritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polarity::R`](R) reader structure"]
impl crate::Readable for PolaritySpec {}
#[doc = "`write(|w| ..)` method takes [`polarity::W`](W) writer structure"]
impl crate::Writable for PolaritySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POLARITY to value 0"]
impl crate::Resettable for PolaritySpec {}
