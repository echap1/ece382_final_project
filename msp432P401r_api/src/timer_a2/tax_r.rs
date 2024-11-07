#[doc = "Register `TAxR` reader"]
pub type R = crate::R<TaxRSpec>;
#[doc = "Register `TAxR` writer"]
pub type W = crate::W<TaxRSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TimerA register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tax_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaxRSpec;
impl crate::RegisterSpec for TaxRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tax_r::R`](R) reader structure"]
impl crate::Readable for TaxRSpec {}
#[doc = "`write(|w| ..)` method takes [`tax_r::W`](W) writer structure"]
impl crate::Writable for TaxRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TAxR to value 0"]
impl crate::Resettable for TaxRSpec {
    const RESET_VALUE: u16 = 0;
}
