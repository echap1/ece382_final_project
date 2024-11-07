#[doc = "Register `FLASH_INFO_LEN` reader"]
pub type R = crate::R<FlashInfoLenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Flash Info Length\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_info_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashInfoLenSpec;
impl crate::RegisterSpec for FlashInfoLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_info_len::R`](R) reader structure"]
impl crate::Readable for FlashInfoLenSpec {}
#[doc = "`reset()` method sets FLASH_INFO_LEN to value 0"]
impl crate::Resettable for FlashInfoLenSpec {
    const RESET_VALUE: u32 = 0;
}
