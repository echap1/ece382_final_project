#[doc = "Register `WAFER_ID` reader"]
pub type R = crate::R<WaferIdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Wafer ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaferIdSpec;
impl crate::RegisterSpec for WaferIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wafer_id::R`](R) reader structure"]
impl crate::Readable for WaferIdSpec {}
#[doc = "`reset()` method sets WAFER_ID to value 0"]
impl crate::Resettable for WaferIdSpec {
    const RESET_VALUE: u32 = 0;
}
