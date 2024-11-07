#[doc = "Register `T32INTCLR1` writer"]
pub type W = crate::W<T32intclr1Spec>;
#[doc = "Field `INTCLR` writer - Write clears interrupt output"]
pub type IntclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Write clears interrupt output"]
    #[inline(always)]
    pub fn intclr(&mut self) -> IntclrW<T32intclr1Spec> {
        IntclrW::new(self, 0)
    }
}
#[doc = "Timer 1 Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32intclr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T32intclr1Spec;
impl crate::RegisterSpec for T32intclr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`t32intclr1::W`](W) writer structure"]
impl crate::Writable for T32intclr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T32INTCLR1 to value 0"]
impl crate::Resettable for T32intclr1Spec {
    const RESET_VALUE: u32 = 0;
}
