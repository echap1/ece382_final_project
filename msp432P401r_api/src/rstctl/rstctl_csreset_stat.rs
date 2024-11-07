#[doc = "Register `RSTCTL_CSRESET_STAT` reader"]
pub type R = crate::R<RstctlCsresetStatSpec>;
#[doc = "Field `DCOR_SHT` reader - Indicates if POR was caused by DCO short circuit fault in the external resistor mode"]
pub type DcorShtR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if POR was caused by DCO short circuit fault in the external resistor mode"]
    #[inline(always)]
    pub fn dcor_sht(&self) -> DcorShtR {
        DcorShtR::new((self.bits & 1) != 0)
    }
}
#[doc = "CS Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_csreset_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlCsresetStatSpec;
impl crate::RegisterSpec for RstctlCsresetStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_csreset_stat::R`](R) reader structure"]
impl crate::Readable for RstctlCsresetStatSpec {}
#[doc = "`reset()` method sets RSTCTL_CSRESET_STAT to value 0"]
impl crate::Resettable for RstctlCsresetStatSpec {
    const RESET_VALUE: u32 = 0;
}
