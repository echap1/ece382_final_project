#[doc = "Register `T32INTCLR2` writer"]
pub type W = crate::W<T32intclr2Spec>;
#[doc = "Field `INTCLR` writer - Write clears the interrupt output"]
pub type IntclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Write clears the interrupt output"]
    #[inline(always)]
    pub fn intclr(&mut self) -> IntclrW<T32intclr2Spec> {
        IntclrW::new(self, 0)
    }
}
#[doc = "Timer 2 Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32intclr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T32intclr2Spec;
impl crate::RegisterSpec for T32intclr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`t32intclr2::W`](W) writer structure"]
impl crate::Writable for T32intclr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T32INTCLR2 to value 0"]
impl crate::Resettable for T32intclr2Spec {
    const RESET_VALUE: u32 = 0;
}
