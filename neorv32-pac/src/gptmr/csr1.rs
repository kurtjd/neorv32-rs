#[doc = "Register `CSR1` reader"]
pub type R = crate::R<Csr1Spec>;
#[doc = "Register `CSR1` writer"]
pub type W = crate::W<Csr1Spec>;
#[doc = "Field `IRQ` reader - Per-slice interrupt-pending bit; write 0 to clear"]
pub type IrqR = crate::FieldReader<u16>;
#[doc = "Field `IRQ` writer - Per-slice interrupt-pending bit; write 0 to clear"]
pub type IrqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRSC` reader - Global clock prescaler select"]
pub type PrscR = crate::FieldReader;
#[doc = "Field `PRSC` writer - Global clock prescaler select"]
pub type PrscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - Per-slice interrupt-pending bit; write 0 to clear"]
    #[inline(always)]
    pub fn irq(&self) -> IrqR {
        IrqR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Global clock prescaler select"]
    #[inline(always)]
    pub fn prsc(&self) -> PrscR {
        PrscR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Per-slice interrupt-pending bit; write 0 to clear"]
    #[inline(always)]
    pub fn irq(&mut self) -> IrqW<'_, Csr1Spec> {
        IrqW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Global clock prescaler select"]
    #[inline(always)]
    pub fn prsc(&mut self) -> PrscW<'_, Csr1Spec> {
        PrscW::new(self, 16)
    }
}
#[doc = "Control and status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`csr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr1Spec;
impl crate::RegisterSpec for Csr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr1::R`](R) reader structure"]
impl crate::Readable for Csr1Spec {}
#[doc = "`write(|w| ..)` method takes [`csr1::W`](W) writer structure"]
impl crate::Writable for Csr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR1 to value 0"]
impl crate::Resettable for Csr1Spec {}
