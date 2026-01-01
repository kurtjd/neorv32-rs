#[doc = "Register `CSR0` reader"]
pub type R = crate::R<Csr0Spec>;
#[doc = "Register `CSR0` writer"]
pub type W = crate::W<Csr0Spec>;
#[doc = "Field `ENABLE` reader - Per-slice enable bit"]
pub type EnableR = crate::FieldReader<u16>;
#[doc = "Field `ENABLE` writer - Per-slice enable bit"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MODE` reader - Per-slice mode bit (0 = single-shot mode, 1 = continuous mode)"]
pub type ModeR = crate::FieldReader<u16>;
#[doc = "Field `MODE` writer - Per-slice mode bit (0 = single-shot mode, 1 = continuous mode)"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Per-slice enable bit"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Per-slice mode bit (0 = single-shot mode, 1 = continuous mode)"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Per-slice enable bit"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Csr0Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Per-slice mode bit (0 = single-shot mode, 1 = continuous mode)"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Csr0Spec> {
        ModeW::new(self, 16)
    }
}
#[doc = "Control and status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr0Spec;
impl crate::RegisterSpec for Csr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr0::R`](R) reader structure"]
impl crate::Readable for Csr0Spec {}
#[doc = "`write(|w| ..)` method takes [`csr0::W`](W) writer structure"]
impl crate::Writable for Csr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR0 to value 0"]
impl crate::Resettable for Csr0Spec {}
