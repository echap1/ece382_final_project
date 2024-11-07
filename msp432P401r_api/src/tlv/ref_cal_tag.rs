#[doc = "Register `REF_CAL_TAG` reader"]
pub type R = crate::R<RefCalTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "REF Calibration Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_cal_tag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefCalTagSpec;
impl crate::RegisterSpec for RefCalTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_cal_tag::R`](R) reader structure"]
impl crate::Readable for RefCalTagSpec {}
#[doc = "`reset()` method sets REF_CAL_TAG to value 0x08"]
impl crate::Resettable for RefCalTagSpec {
    const RESET_VALUE: u32 = 0x08;
}
