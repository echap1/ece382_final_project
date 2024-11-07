#[doc = "Register `CSDCOERCAL1` reader"]
pub type R = crate::R<Csdcoercal1Spec>;
#[doc = "Register `CSDCOERCAL1` writer"]
pub type W = crate::W<Csdcoercal1Spec>;
#[doc = "Field `DCO_FCAL_RSEL5` reader - DCO frequency calibration for DCO frequency range (DCORSEL) 5."]
pub type DcoFcalRsel5R = crate::FieldReader<u16>;
#[doc = "Field `DCO_FCAL_RSEL5` writer - DCO frequency calibration for DCO frequency range (DCORSEL) 5."]
pub type DcoFcalRsel5W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DCO frequency calibration for DCO frequency range (DCORSEL) 5."]
    #[inline(always)]
    pub fn dco_fcal_rsel5(&self) -> DcoFcalRsel5R {
        DcoFcalRsel5R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DCO frequency calibration for DCO frequency range (DCORSEL) 5."]
    #[inline(always)]
    pub fn dco_fcal_rsel5(&mut self) -> DcoFcalRsel5W<Csdcoercal1Spec> {
        DcoFcalRsel5W::new(self, 0)
    }
}
#[doc = "DCO External Resistor Calibration 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csdcoercal1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csdcoercal1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csdcoercal1Spec;
impl crate::RegisterSpec for Csdcoercal1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csdcoercal1::R`](R) reader structure"]
impl crate::Readable for Csdcoercal1Spec {}
#[doc = "`write(|w| ..)` method takes [`csdcoercal1::W`](W) writer structure"]
impl crate::Writable for Csdcoercal1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSDCOERCAL1 to value 0x0100"]
impl crate::Resettable for Csdcoercal1Spec {
    const RESET_VALUE: u32 = 0x0100;
}
