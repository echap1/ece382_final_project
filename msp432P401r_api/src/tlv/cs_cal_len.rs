#[doc = "Register `CS_CAL_LEN` reader"]
pub type R = crate::R<CsCalLenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Clock System Calibration Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cs_cal_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsCalLenSpec;
impl crate::RegisterSpec for CsCalLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs_cal_len::R`](R) reader structure"]
impl crate::Readable for CsCalLenSpec {}
#[doc = "`reset()` method sets CS_CAL_LEN to value 0"]
impl crate::Resettable for CsCalLenSpec {
    const RESET_VALUE: u32 = 0;
}
