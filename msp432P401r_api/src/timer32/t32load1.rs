#[doc = "Register `T32LOAD1` reader"]
pub type R = crate::R<T32load1Spec>;
#[doc = "Register `T32LOAD1` writer"]
pub type W = crate::W<T32load1Spec>;
#[doc = "Field `LOAD` reader - The value from which the Timer 1 counter decrements"]
pub type LoadR = crate::FieldReader<u32>;
#[doc = "Field `LOAD` writer - The value from which the Timer 1 counter decrements"]
pub type LoadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The value from which the Timer 1 counter decrements"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The value from which the Timer 1 counter decrements"]
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<T32load1Spec> {
        LoadW::new(self, 0)
    }
}
#[doc = "Timer 1 Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32load1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32load1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T32load1Spec;
impl crate::RegisterSpec for T32load1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t32load1::R`](R) reader structure"]
impl crate::Readable for T32load1Spec {}
#[doc = "`write(|w| ..)` method takes [`t32load1::W`](W) writer structure"]
impl crate::Writable for T32load1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T32LOAD1 to value 0"]
impl crate::Resettable for T32load1Spec {
    const RESET_VALUE: u32 = 0;
}
