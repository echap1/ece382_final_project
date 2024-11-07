#[doc = "Register `PCMIFG` reader"]
pub type R = crate::R<PcmifgSpec>;
#[doc = "Field `LPM_INVALID_TR_IFG` reader - LPM invalid transition flag"]
pub type LpmInvalidTrIfgR = crate::BitReader;
#[doc = "Field `LPM_INVALID_CLK_IFG` reader - LPM invalid clock flag"]
pub type LpmInvalidClkIfgR = crate::BitReader;
#[doc = "Field `AM_INVALID_TR_IFG` reader - Active mode invalid transition flag"]
pub type AmInvalidTrIfgR = crate::BitReader;
#[doc = "Field `DCDC_ERROR_IFG` reader - DC-DC error flag"]
pub type DcdcErrorIfgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - LPM invalid transition flag"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ifg(&self) -> LpmInvalidTrIfgR {
        LpmInvalidTrIfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM invalid clock flag"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ifg(&self) -> LpmInvalidClkIfgR {
        LpmInvalidClkIfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active mode invalid transition flag"]
    #[inline(always)]
    pub fn am_invalid_tr_ifg(&self) -> AmInvalidTrIfgR {
        AmInvalidTrIfgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - DC-DC error flag"]
    #[inline(always)]
    pub fn dcdc_error_ifg(&self) -> DcdcErrorIfgR {
        DcdcErrorIfgR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmifg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcmifgSpec;
impl crate::RegisterSpec for PcmifgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcmifg::R`](R) reader structure"]
impl crate::Readable for PcmifgSpec {}
#[doc = "`reset()` method sets PCMIFG to value 0"]
impl crate::Resettable for PcmifgSpec {
    const RESET_VALUE: u32 = 0;
}
