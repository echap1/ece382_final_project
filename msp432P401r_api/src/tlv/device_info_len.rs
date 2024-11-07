#[doc = "Register `DEVICE_INFO_LEN` reader"]
pub type R = crate::R<DeviceInfoLenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Device Info Length\n\nYou can [`read`](crate::Reg::read) this register and get [`device_info_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceInfoLenSpec;
impl crate::RegisterSpec for DeviceInfoLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_info_len::R`](R) reader structure"]
impl crate::Readable for DeviceInfoLenSpec {}
#[doc = "`reset()` method sets DEVICE_INFO_LEN to value 0"]
impl crate::Resettable for DeviceInfoLenSpec {
    const RESET_VALUE: u32 = 0;
}
