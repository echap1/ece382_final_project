#[doc = "Register `CSDCOERCAL0` reader"]
pub type R = crate::R<Csdcoercal0Spec>;
#[doc = "Register `CSDCOERCAL0` writer"]
pub type W = crate::W<Csdcoercal0Spec>;
#[doc = "Field `DCO_TCCAL` reader - DCO Temperature compensation calibration"]
pub type DcoTccalR = crate::FieldReader;
#[doc = "Field `DCO_TCCAL` writer - DCO Temperature compensation calibration"]
pub type DcoTccalW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCO_FCAL_RSEL04` reader - DCO frequency calibration for DCO frequency range (DCORSEL) 0 to 4."]
pub type DcoFcalRsel04R = crate::FieldReader<u16>;
#[doc = "Field `DCO_FCAL_RSEL04` writer - DCO frequency calibration for DCO frequency range (DCORSEL) 0 to 4."]
pub type DcoFcalRsel04W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:1 - DCO Temperature compensation calibration"]
    #[inline(always)]
    pub fn dco_tccal(&self) -> DcoTccalR {
        DcoTccalR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:25 - DCO frequency calibration for DCO frequency range (DCORSEL) 0 to 4."]
    #[inline(always)]
    pub fn dco_fcal_rsel04(&self) -> DcoFcalRsel04R {
        DcoFcalRsel04R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - DCO Temperature compensation calibration"]
    #[inline(always)]
    pub fn dco_tccal(&mut self) -> DcoTccalW<Csdcoercal0Spec> {
        DcoTccalW::new(self, 0)
    }
    #[doc = "Bits 16:25 - DCO frequency calibration for DCO frequency range (DCORSEL) 0 to 4."]
    #[inline(always)]
    pub fn dco_fcal_rsel04(&mut self) -> DcoFcalRsel04W<Csdcoercal0Spec> {
        DcoFcalRsel04W::new(self, 16)
    }
}
#[doc = "DCO External Resistor Cailbration 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csdcoercal0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csdcoercal0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csdcoercal0Spec;
impl crate::RegisterSpec for Csdcoercal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csdcoercal0::R`](R) reader structure"]
impl crate::Readable for Csdcoercal0Spec {}
#[doc = "`write(|w| ..)` method takes [`csdcoercal0::W`](W) writer structure"]
impl crate::Writable for Csdcoercal0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSDCOERCAL0 to value 0x0100_0000"]
impl crate::Resettable for Csdcoercal0Spec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
