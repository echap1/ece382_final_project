#[doc = "Register `CS_CAL_TAG` reader"]
pub type R = crate::R<CsCalTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Clock System Calibration Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`cs_cal_tag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsCalTagSpec;
impl crate::RegisterSpec for CsCalTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs_cal_tag::R`](R) reader structure"]
impl crate::Readable for CsCalTagSpec {}
#[doc = "`reset()` method sets CS_CAL_TAG to value 0x03"]
impl crate::Resettable for CsCalTagSpec {
    const RESET_VALUE: u32 = 0x03;
}
