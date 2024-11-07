#[doc = "Register `FLASH_MAX_ERASE_PULSES` reader"]
pub type R = crate::R<FlashMaxErasePulsesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Flash Maximum Erase Pulses\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_max_erase_pulses::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashMaxErasePulsesSpec;
impl crate::RegisterSpec for FlashMaxErasePulsesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_max_erase_pulses::R`](R) reader structure"]
impl crate::Readable for FlashMaxErasePulsesSpec {}
#[doc = "`reset()` method sets FLASH_MAX_ERASE_PULSES to value 0"]
impl crate::Resettable for FlashMaxErasePulsesSpec {
    const RESET_VALUE: u32 = 0;
}
