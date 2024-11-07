#[doc = "Register `T32RIS1` reader"]
pub type R = crate::R<T32ris1Spec>;
#[doc = "Field `RAW_IFG` reader - Raw interrupt status"]
pub type RawIfgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw interrupt status"]
    #[inline(always)]
    pub fn raw_ifg(&self) -> RawIfgR {
        RawIfgR::new((self.bits & 1) != 0)
    }
}
#[doc = "Timer 1 Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32ris1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T32ris1Spec;
impl crate::RegisterSpec for T32ris1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t32ris1::R`](R) reader structure"]
impl crate::Readable for T32ris1Spec {}
#[doc = "`reset()` method sets T32RIS1 to value 0"]
impl crate::Resettable for T32ris1Spec {
    const RESET_VALUE: u32 = 0;
}
