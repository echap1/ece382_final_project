#[doc = "Register `DEVICE_INFO_TAG` reader"]
pub type R = crate::R<DeviceInfoTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Device Info Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`device_info_tag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceInfoTagSpec;
impl crate::RegisterSpec for DeviceInfoTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_info_tag::R`](R) reader structure"]
impl crate::Readable for DeviceInfoTagSpec {}
#[doc = "`reset()` method sets DEVICE_INFO_TAG to value 0x0b"]
impl crate::Resettable for DeviceInfoTagSpec {
    const RESET_VALUE: u32 = 0x0b;
}
