#[doc = "Register `ADC14LO0` reader"]
pub type R = crate::R<Adc14lo0Spec>;
#[doc = "Register `ADC14LO0` writer"]
pub type W = crate::W<Adc14lo0Spec>;
#[doc = "Field `ADC14LO0` reader - Low threshold 0"]
pub type Adc14lo0R = crate::FieldReader<u16>;
#[doc = "Field `ADC14LO0` writer - Low threshold 0"]
pub type Adc14lo0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low threshold 0"]
    #[inline(always)]
    pub fn adc14lo0(&self) -> Adc14lo0R {
        Adc14lo0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low threshold 0"]
    #[inline(always)]
    pub fn adc14lo0(&mut self) -> Adc14lo0W<Adc14lo0Spec> {
        Adc14lo0W::new(self, 0)
    }
}
#[doc = "Window Comparator Low Threshold 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14lo0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14lo0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14lo0Spec;
impl crate::RegisterSpec for Adc14lo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14lo0::R`](R) reader structure"]
impl crate::Readable for Adc14lo0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc14lo0::W`](W) writer structure"]
impl crate::Writable for Adc14lo0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14LO0 to value 0"]
impl crate::Resettable for Adc14lo0Spec {
    const RESET_VALUE: u32 = 0;
}
