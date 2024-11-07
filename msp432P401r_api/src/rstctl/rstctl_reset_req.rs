#[doc = "Register `RSTCTL_RESET_REQ` reader"]
pub type R = crate::R<RstctlResetReqSpec>;
#[doc = "Register `RSTCTL_RESET_REQ` writer"]
pub type W = crate::W<RstctlResetReqSpec>;
#[doc = "Field `SOFT_REQ` writer - Soft Reset request"]
pub type SoftReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARD_REQ` writer - Hard Reset request"]
pub type HardReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTKEY` writer - Write key to unlock reset request bits"]
pub type RstkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bit 0 - Soft Reset request"]
    #[inline(always)]
    pub fn soft_req(&mut self) -> SoftReqW<RstctlResetReqSpec> {
        SoftReqW::new(self, 0)
    }
    #[doc = "Bit 1 - Hard Reset request"]
    #[inline(always)]
    pub fn hard_req(&mut self) -> HardReqW<RstctlResetReqSpec> {
        HardReqW::new(self, 1)
    }
    #[doc = "Bits 8:15 - Write key to unlock reset request bits"]
    #[inline(always)]
    pub fn rstkey(&mut self) -> RstkeyW<RstctlResetReqSpec> {
        RstkeyW::new(self, 8)
    }
}
#[doc = "Reset Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_reset_req::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_reset_req::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlResetReqSpec;
impl crate::RegisterSpec for RstctlResetReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_reset_req::R`](R) reader structure"]
impl crate::Readable for RstctlResetReqSpec {}
#[doc = "`write(|w| ..)` method takes [`rstctl_reset_req::W`](W) writer structure"]
impl crate::Writable for RstctlResetReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTCTL_RESET_REQ to value 0"]
impl crate::Resettable for RstctlResetReqSpec {
    const RESET_VALUE: u32 = 0;
}
