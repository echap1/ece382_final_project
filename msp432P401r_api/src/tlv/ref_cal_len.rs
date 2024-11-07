#[doc = "Register `REF_CAL_LEN` reader"]
pub type R = crate::R<RefCalLenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "REF Calibration Length\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_cal_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefCalLenSpec;
impl crate::RegisterSpec for RefCalLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_cal_len::R`](R) reader structure"]
impl crate::Readable for RefCalLenSpec {}
#[doc = "`reset()` method sets REF_CAL_LEN to value 0"]
impl crate::Resettable for RefCalLenSpec {
    const RESET_VALUE: u32 = 0;
}
