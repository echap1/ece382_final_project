#[doc = "Register `TAxCCR[%s]` reader"]
pub type R = crate::R<TaxCcrSpec>;
#[doc = "Register `TAxCCR[%s]` writer"]
pub type W = crate::W<TaxCcrSpec>;
#[doc = "Field `TAxR` reader - TimerA register"]
pub type TaxRR = crate::FieldReader<u16>;
#[doc = "Field `TAxR` writer - TimerA register"]
pub type TaxRW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - TimerA register"]
    #[inline(always)]
    pub fn tax_r(&self) -> TaxRR {
        TaxRR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - TimerA register"]
    #[inline(always)]
    pub fn tax_r(&mut self) -> TaxRW<TaxCcrSpec> {
        TaxRW::new(self, 0)
    }
}
#[doc = "Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tax_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaxCcrSpec;
impl crate::RegisterSpec for TaxCcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tax_ccr::R`](R) reader structure"]
impl crate::Readable for TaxCcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tax_ccr::W`](W) writer structure"]
impl crate::Writable for TaxCcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TAxCCR[%s]
to value 0"]
impl crate::Resettable for TaxCcrSpec {
    const RESET_VALUE: u16 = 0;
}
