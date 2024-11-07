#[doc = "Register `RSTCTL_PINRESET_STAT` reader"]
pub type R = crate::R<RstctlPinresetStatSpec>;
#[doc = "Field `RSTNMI` reader - POR was caused by RSTn/NMI pin based reset event"]
pub type RstnmiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - POR was caused by RSTn/NMI pin based reset event"]
    #[inline(always)]
    pub fn rstnmi(&self) -> RstnmiR {
        RstnmiR::new((self.bits & 1) != 0)
    }
}
#[doc = "Pin Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pinreset_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlPinresetStatSpec;
impl crate::RegisterSpec for RstctlPinresetStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_pinreset_stat::R`](R) reader structure"]
impl crate::Readable for RstctlPinresetStatSpec {}
#[doc = "`reset()` method sets RSTCTL_PINRESET_STAT to value 0"]
impl crate::Resettable for RstctlPinresetStatSpec {
    const RESET_VALUE: u32 = 0;
}
