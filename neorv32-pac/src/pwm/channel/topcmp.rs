#[doc = "Register `TOPCMP` reader"]
pub type R = crate::R<TopcmpSpec>;
#[doc = "Register `TOPCMP` writer"]
pub type W = crate::W<TopcmpSpec>;
#[doc = "Field `CMP` reader - Channel counter compare value"]
pub type CmpR = crate::FieldReader<u16>;
#[doc = "Field `CMP` writer - Channel counter compare value"]
pub type CmpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOP` reader - Channel counter top/wrap value"]
pub type TopR = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Channel counter top/wrap value"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel counter compare value"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Channel counter top/wrap value"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel counter compare value"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<'_, TopcmpSpec> {
        CmpW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Channel counter top/wrap value"]
    #[inline(always)]
    pub fn top(&mut self) -> TopW<'_, TopcmpSpec> {
        TopW::new(self, 16)
    }
}
#[doc = "Channel counter full word access alias\n\nYou can [`read`](crate::Reg::read) this register and get [`topcmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`topcmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TopcmpSpec;
impl crate::RegisterSpec for TopcmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`topcmp::R`](R) reader structure"]
impl crate::Readable for TopcmpSpec {}
#[doc = "`write(|w| ..)` method takes [`topcmp::W`](W) writer structure"]
impl crate::Writable for TopcmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOPCMP to value 0"]
impl crate::Resettable for TopcmpSpec {}
