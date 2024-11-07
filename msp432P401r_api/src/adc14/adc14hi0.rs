#[doc = "Register `ADC14HI0` reader"]
pub type R = crate::R<Adc14hi0Spec>;
#[doc = "Register `ADC14HI0` writer"]
pub type W = crate::W<Adc14hi0Spec>;
#[doc = "Field `ADC14HI0` reader - High threshold 0"]
pub type Adc14hi0R = crate::FieldReader<u16>;
#[doc = "Field `ADC14HI0` writer - High threshold 0"]
pub type Adc14hi0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - High threshold 0"]
    #[inline(always)]
    pub fn adc14hi0(&self) -> Adc14hi0R {
        Adc14hi0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - High threshold 0"]
    #[inline(always)]
    pub fn adc14hi0(&mut self) -> Adc14hi0W<Adc14hi0Spec> {
        Adc14hi0W::new(self, 0)
    }
}
#[doc = "Window Comparator High Threshold 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14hi0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14hi0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14hi0Spec;
impl crate::RegisterSpec for Adc14hi0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14hi0::R`](R) reader structure"]
impl crate::Readable for Adc14hi0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc14hi0::W`](W) writer structure"]
impl crate::Writable for Adc14hi0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14HI0 to value 0x3fff"]
impl crate::Resettable for Adc14hi0Spec {
    const RESET_VALUE: u32 = 0x3fff;
}
