#[doc = "Register `DCOER_FCAL_RSEL5` reader"]
pub type R = crate::R<DcoerFcalRsel5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DCO ER mode: Frequency calibration for DCORSEL 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoer_fcal_rsel5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcoerFcalRsel5Spec;
impl crate::RegisterSpec for DcoerFcalRsel5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcoer_fcal_rsel5::R`](R) reader structure"]
impl crate::Readable for DcoerFcalRsel5Spec {}
#[doc = "`reset()` method sets DCOER_FCAL_RSEL5 to value 0"]
impl crate::Resettable for DcoerFcalRsel5Spec {
    const RESET_VALUE: u32 = 0;
}
