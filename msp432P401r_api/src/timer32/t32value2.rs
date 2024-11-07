#[doc = "Register `T32VALUE2` reader"]
pub type R = crate::R<T32value2Spec>;
#[doc = "Field `VALUE` reader - Current value of the decrementing counter"]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current value of the decrementing counter"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "Timer 2 Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32value2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T32value2Spec;
impl crate::RegisterSpec for T32value2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t32value2::R`](R) reader structure"]
impl crate::Readable for T32value2Spec {}
#[doc = "`reset()` method sets T32VALUE2 to value 0xffff_ffff"]
impl crate::Resettable for T32value2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
