#[doc = "Register `T32MIS1` reader"]
pub type R = crate::R<T32mis1Spec>;
#[doc = "Field `IFG` reader - Enabled interrupt status"]
pub type IfgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enabled interrupt status"]
    #[inline(always)]
    pub fn ifg(&self) -> IfgR {
        IfgR::new((self.bits & 1) != 0)
    }
}
#[doc = "Timer 1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32mis1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T32mis1Spec;
impl crate::RegisterSpec for T32mis1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t32mis1::R`](R) reader structure"]
impl crate::Readable for T32mis1Spec {}
#[doc = "`reset()` method sets T32MIS1 to value 0"]
impl crate::Resettable for T32mis1Spec {
    const RESET_VALUE: u32 = 0;
}
