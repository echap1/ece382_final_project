#[doc = "Register `FLASH_INFO_TAG` reader"]
pub type R = crate::R<FlashInfoTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Flash Info Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_info_tag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashInfoTagSpec;
impl crate::RegisterSpec for FlashInfoTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_info_tag::R`](R) reader structure"]
impl crate::Readable for FlashInfoTagSpec {}
#[doc = "`reset()` method sets FLASH_INFO_TAG to value 0x04"]
impl crate::Resettable for FlashInfoTagSpec {
    const RESET_VALUE: u32 = 0x04;
}
