#[doc = "Register `ADC14HI1` reader"]
pub type R = crate::R<Adc14hi1Spec>;
#[doc = "Register `ADC14HI1` writer"]
pub type W = crate::W<Adc14hi1Spec>;
#[doc = "Field `ADC14HI1` reader - High threshold 1"]
pub type Adc14hi1R = crate::FieldReader<u16>;
#[doc = "Field `ADC14HI1` writer - High threshold 1"]
pub type Adc14hi1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - High threshold 1"]
    #[inline(always)]
    pub fn adc14hi1(&self) -> Adc14hi1R {
        Adc14hi1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - High threshold 1"]
    #[inline(always)]
    pub fn adc14hi1(&mut self) -> Adc14hi1W<Adc14hi1Spec> {
        Adc14hi1W::new(self, 0)
    }
}
#[doc = "Window Comparator High Threshold 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14hi1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14hi1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14hi1Spec;
impl crate::RegisterSpec for Adc14hi1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14hi1::R`](R) reader structure"]
impl crate::Readable for Adc14hi1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc14hi1::W`](W) writer structure"]
impl crate::Writable for Adc14hi1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14HI1 to value 0x3fff"]
impl crate::Resettable for Adc14hi1Spec {
    const RESET_VALUE: u32 = 0x3fff;
}
