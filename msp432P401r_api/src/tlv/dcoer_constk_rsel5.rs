#[doc = "Register `DCOER_CONSTK_RSEL5` reader"]
pub type R = crate::R<DcoerConstkRsel5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DCO ER mode: DCO Constant (K) for DCORSEL 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoer_constk_rsel5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcoerConstkRsel5Spec;
impl crate::RegisterSpec for DcoerConstkRsel5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcoer_constk_rsel5::R`](R) reader structure"]
impl crate::Readable for DcoerConstkRsel5Spec {}
#[doc = "`reset()` method sets DCOER_CONSTK_RSEL5 to value 0"]
impl crate::Resettable for DcoerConstkRsel5Spec {
    const RESET_VALUE: u32 = 0;
}
