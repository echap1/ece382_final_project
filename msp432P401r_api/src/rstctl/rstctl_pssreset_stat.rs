#[doc = "Register `RSTCTL_PSSRESET_STAT` reader"]
pub type R = crate::R<RstctlPssresetStatSpec>;
#[doc = "Field `SVSL` reader - Indicates if POR was caused by an SVSL trip condition in the PSS"]
pub type SvslR = crate::BitReader;
#[doc = "Field `SVSMH` reader - Indicates if POR was caused by an SVSMH trip condition int the PSS"]
pub type SvsmhR = crate::BitReader;
#[doc = "Field `BGREF` reader - Indicates if POR was caused by a BGREF not okay condition in the PSS"]
pub type BgrefR = crate::BitReader;
#[doc = "Field `VCCDET` reader - Indicates if POR was caused by a VCCDET trip condition in the PSS"]
pub type VccdetR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if POR was caused by an SVSL trip condition in the PSS"]
    #[inline(always)]
    pub fn svsl(&self) -> SvslR {
        SvslR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if POR was caused by an SVSMH trip condition int the PSS"]
    #[inline(always)]
    pub fn svsmh(&self) -> SvsmhR {
        SvsmhR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates if POR was caused by a BGREF not okay condition in the PSS"]
    #[inline(always)]
    pub fn bgref(&self) -> BgrefR {
        BgrefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates if POR was caused by a VCCDET trip condition in the PSS"]
    #[inline(always)]
    pub fn vccdet(&self) -> VccdetR {
        VccdetR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PSS Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pssreset_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlPssresetStatSpec;
impl crate::RegisterSpec for RstctlPssresetStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_pssreset_stat::R`](R) reader structure"]
impl crate::Readable for RstctlPssresetStatSpec {}
#[doc = "`reset()` method sets RSTCTL_PSSRESET_STAT to value 0x0f"]
impl crate::Resettable for RstctlPssresetStatSpec {
    const RESET_VALUE: u32 = 0x0f;
}
