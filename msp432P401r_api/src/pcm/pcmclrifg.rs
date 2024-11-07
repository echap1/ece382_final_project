#[doc = "Register `PCMCLRIFG` writer"]
pub type W = crate::W<PcmclrifgSpec>;
#[doc = "Field `CLR_LPM_INVALID_TR_IFG` writer - Clear LPM invalid transition flag"]
pub type ClrLpmInvalidTrIfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_LPM_INVALID_CLK_IFG` writer - Clear LPM invalid clock flag"]
pub type ClrLpmInvalidClkIfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_AM_INVALID_TR_IFG` writer - Clear active mode invalid transition flag"]
pub type ClrAmInvalidTrIfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_DCDC_ERROR_IFG` writer - Clear DC-DC error flag"]
pub type ClrDcdcErrorIfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear LPM invalid transition flag"]
    #[inline(always)]
    pub fn clr_lpm_invalid_tr_ifg(&mut self) -> ClrLpmInvalidTrIfgW<PcmclrifgSpec> {
        ClrLpmInvalidTrIfgW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear LPM invalid clock flag"]
    #[inline(always)]
    pub fn clr_lpm_invalid_clk_ifg(&mut self) -> ClrLpmInvalidClkIfgW<PcmclrifgSpec> {
        ClrLpmInvalidClkIfgW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear active mode invalid transition flag"]
    #[inline(always)]
    pub fn clr_am_invalid_tr_ifg(&mut self) -> ClrAmInvalidTrIfgW<PcmclrifgSpec> {
        ClrAmInvalidTrIfgW::new(self, 2)
    }
    #[doc = "Bit 6 - Clear DC-DC error flag"]
    #[inline(always)]
    pub fn clr_dcdc_error_ifg(&mut self) -> ClrDcdcErrorIfgW<PcmclrifgSpec> {
        ClrDcdcErrorIfgW::new(self, 6)
    }
}
#[doc = "Clear Interrupt Flag Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmclrifg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcmclrifgSpec;
impl crate::RegisterSpec for PcmclrifgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pcmclrifg::W`](W) writer structure"]
impl crate::Writable for PcmclrifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCMCLRIFG to value 0"]
impl crate::Resettable for PcmclrifgSpec {
    const RESET_VALUE: u32 = 0;
}
