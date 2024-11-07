#[doc = "Register `RSTCTL_REBOOTRESET_CLR` reader"]
pub type R = crate::R<RstctlRebootresetClrSpec>;
#[doc = "Register `RSTCTL_REBOOTRESET_CLR` writer"]
pub type W = crate::W<RstctlRebootresetClrSpec>;
#[doc = "Field `CLR` writer - Write 1 clears the Reboot Reset Flag in RSTCTL_REBOOTRESET_STAT"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 clears the Reboot Reset Flag in RSTCTL_REBOOTRESET_STAT"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<RstctlRebootresetClrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Reboot Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_rebootreset_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_rebootreset_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlRebootresetClrSpec;
impl crate::RegisterSpec for RstctlRebootresetClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_rebootreset_clr::R`](R) reader structure"]
impl crate::Readable for RstctlRebootresetClrSpec {}
#[doc = "`write(|w| ..)` method takes [`rstctl_rebootreset_clr::W`](W) writer structure"]
impl crate::Writable for RstctlRebootresetClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTCTL_REBOOTRESET_CLR to value 0"]
impl crate::Resettable for RstctlRebootresetClrSpec {
    const RESET_VALUE: u32 = 0;
}
