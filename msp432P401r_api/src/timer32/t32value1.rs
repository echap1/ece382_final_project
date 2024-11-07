#[doc = "Register `T32VALUE1` reader"]
pub type R = crate::R<T32value1Spec>;
#[doc = "Field `VALUE` reader - Current value"]
pub type ValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
#[doc = "Timer 1 Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32value1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T32value1Spec;
impl crate::RegisterSpec for T32value1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t32value1::R`](R) reader structure"]
impl crate::Readable for T32value1Spec {}
#[doc = "`reset()` method sets T32VALUE1 to value 0xffff_ffff"]
impl crate::Resettable for T32value1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
