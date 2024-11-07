#[doc = "Register `T32BGLOAD1` reader"]
pub type R = crate::R<T32bgload1Spec>;
#[doc = "Register `T32BGLOAD1` writer"]
pub type W = crate::W<T32bgload1Spec>;
#[doc = "Field `BGLOAD` reader - Value from which the counter decrements"]
pub type BgloadR = crate::FieldReader<u32>;
#[doc = "Field `BGLOAD` writer - Value from which the counter decrements"]
pub type BgloadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value from which the counter decrements"]
    #[inline(always)]
    pub fn bgload(&self) -> BgloadR {
        BgloadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value from which the counter decrements"]
    #[inline(always)]
    pub fn bgload(&mut self) -> BgloadW<T32bgload1Spec> {
        BgloadW::new(self, 0)
    }
}
#[doc = "Timer 1 Background Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32bgload1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32bgload1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T32bgload1Spec;
impl crate::RegisterSpec for T32bgload1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t32bgload1::R`](R) reader structure"]
impl crate::Readable for T32bgload1Spec {}
#[doc = "`write(|w| ..)` method takes [`t32bgload1::W`](W) writer structure"]
impl crate::Writable for T32bgload1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T32BGLOAD1 to value 0"]
impl crate::Resettable for T32bgload1Spec {
    const RESET_VALUE: u32 = 0;
}
