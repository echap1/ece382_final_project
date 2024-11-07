#[doc = "Register `DCOER_FCAL_RSEL04` reader"]
pub type R = crate::R<DcoerFcalRsel04Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DCO ER mode: Frequency calibration for DCORSEL 0 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoer_fcal_rsel04::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcoerFcalRsel04Spec;
impl crate::RegisterSpec for DcoerFcalRsel04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcoer_fcal_rsel04::R`](R) reader structure"]
impl crate::Readable for DcoerFcalRsel04Spec {}
#[doc = "`reset()` method sets DCOER_FCAL_RSEL04 to value 0"]
impl crate::Resettable for DcoerFcalRsel04Spec {
    const RESET_VALUE: u32 = 0;
}
