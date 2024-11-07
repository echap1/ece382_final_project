#[doc = "Register `ADC14LO1` reader"]
pub type R = crate::R<Adc14lo1Spec>;
#[doc = "Register `ADC14LO1` writer"]
pub type W = crate::W<Adc14lo1Spec>;
#[doc = "Field `ADC14LO1` reader - Low threshold 1"]
pub type Adc14lo1R = crate::FieldReader<u16>;
#[doc = "Field `ADC14LO1` writer - Low threshold 1"]
pub type Adc14lo1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low threshold 1"]
    #[inline(always)]
    pub fn adc14lo1(&self) -> Adc14lo1R {
        Adc14lo1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low threshold 1"]
    #[inline(always)]
    pub fn adc14lo1(&mut self) -> Adc14lo1W<Adc14lo1Spec> {
        Adc14lo1W::new(self, 0)
    }
}
#[doc = "Window Comparator Low Threshold 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14lo1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14lo1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14lo1Spec;
impl crate::RegisterSpec for Adc14lo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14lo1::R`](R) reader structure"]
impl crate::Readable for Adc14lo1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc14lo1::W`](W) writer structure"]
impl crate::Writable for Adc14lo1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14LO1 to value 0"]
impl crate::Resettable for Adc14lo1Spec {
    const RESET_VALUE: u32 = 0;
}
