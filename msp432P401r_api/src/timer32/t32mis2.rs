#[doc = "Register `T32MIS2` reader"]
pub type R = crate::R<T32mis2Spec>;
#[doc = "Field `IFG` reader - Enabled interrupt status"]
pub type IfgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enabled interrupt status"]
    #[inline(always)]
    pub fn ifg(&self) -> IfgR {
        IfgR::new((self.bits & 1) != 0)
    }
}
#[doc = "Timer 2 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32mis2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T32mis2Spec;
impl crate::RegisterSpec for T32mis2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t32mis2::R`](R) reader structure"]
impl crate::Readable for T32mis2Spec {}
#[doc = "`reset()` method sets T32MIS2 to value 0"]
impl crate::Resettable for T32mis2Spec {
    const RESET_VALUE: u32 = 0;
}
