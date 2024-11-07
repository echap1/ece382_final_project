#[doc = "Register `RSTCTL_PCMRESET_CLR` reader"]
pub type R = crate::R<RstctlPcmresetClrSpec>;
#[doc = "Register `RSTCTL_PCMRESET_CLR` writer"]
pub type W = crate::W<RstctlPcmresetClrSpec>;
#[doc = "Field `CLR` writer - Write 1 clears all PCM Reset Flags in the RSTCTL_PCMRESET_STAT"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 clears all PCM Reset Flags in the RSTCTL_PCMRESET_STAT"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<RstctlPcmresetClrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "PCM Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pcmreset_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_pcmreset_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlPcmresetClrSpec;
impl crate::RegisterSpec for RstctlPcmresetClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_pcmreset_clr::R`](R) reader structure"]
impl crate::Readable for RstctlPcmresetClrSpec {}
#[doc = "`write(|w| ..)` method takes [`rstctl_pcmreset_clr::W`](W) writer structure"]
impl crate::Writable for RstctlPcmresetClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTCTL_PCMRESET_CLR to value 0"]
impl crate::Resettable for RstctlPcmresetClrSpec {
    const RESET_VALUE: u32 = 0;
}
