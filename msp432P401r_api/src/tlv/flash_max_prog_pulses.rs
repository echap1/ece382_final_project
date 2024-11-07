#[doc = "Register `FLASH_MAX_PROG_PULSES` reader"]
pub type R = crate::R<FlashMaxProgPulsesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Flash Maximum Programming Pulses\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_max_prog_pulses::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashMaxProgPulsesSpec;
impl crate::RegisterSpec for FlashMaxProgPulsesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_max_prog_pulses::R`](R) reader structure"]
impl crate::Readable for FlashMaxProgPulsesSpec {}
#[doc = "`reset()` method sets FLASH_MAX_PROG_PULSES to value 0"]
impl crate::Resettable for FlashMaxProgPulsesSpec {
    const RESET_VALUE: u32 = 0;
}
