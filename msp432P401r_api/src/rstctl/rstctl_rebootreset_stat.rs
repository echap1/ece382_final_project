#[doc = "Register `RSTCTL_REBOOTRESET_STAT` reader"]
pub type R = crate::R<RstctlRebootresetStatSpec>;
#[doc = "Field `REBOOT` reader - Indicates if Reboot reset was caused by the SYSCTL module."]
pub type RebootR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if Reboot reset was caused by the SYSCTL module."]
    #[inline(always)]
    pub fn reboot(&self) -> RebootR {
        RebootR::new((self.bits & 1) != 0)
    }
}
#[doc = "Reboot Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_rebootreset_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlRebootresetStatSpec;
impl crate::RegisterSpec for RstctlRebootresetStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_rebootreset_stat::R`](R) reader structure"]
impl crate::Readable for RstctlRebootresetStatSpec {}
#[doc = "`reset()` method sets RSTCTL_REBOOTRESET_STAT to value 0"]
impl crate::Resettable for RstctlRebootresetStatSpec {
    const RESET_VALUE: u32 = 0;
}
