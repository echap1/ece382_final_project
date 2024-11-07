#[doc = "Register `RSTCTL_PCMRESET_STAT` reader"]
pub type R = crate::R<RstctlPcmresetStatSpec>;
#[doc = "Field `LPM35` reader - Indicates if POR was caused by PCM due to an exit from LPM3.5"]
pub type Lpm35R = crate::BitReader;
#[doc = "Field `LPM45` reader - Indicates if POR was caused by PCM due to an exit from LPM4.5"]
pub type Lpm45R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if POR was caused by PCM due to an exit from LPM3.5"]
    #[inline(always)]
    pub fn lpm35(&self) -> Lpm35R {
        Lpm35R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if POR was caused by PCM due to an exit from LPM4.5"]
    #[inline(always)]
    pub fn lpm45(&self) -> Lpm45R {
        Lpm45R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PCM Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pcmreset_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlPcmresetStatSpec;
impl crate::RegisterSpec for RstctlPcmresetStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_pcmreset_stat::R`](R) reader structure"]
impl crate::Readable for RstctlPcmresetStatSpec {}
#[doc = "`reset()` method sets RSTCTL_PCMRESET_STAT to value 0"]
impl crate::Resettable for RstctlPcmresetStatSpec {
    const RESET_VALUE: u32 = 0;
}
