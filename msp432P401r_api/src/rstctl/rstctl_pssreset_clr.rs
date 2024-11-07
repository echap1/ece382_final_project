#[doc = "Register `RSTCTL_PSSRESET_CLR` reader"]
pub type R = crate::R<RstctlPssresetClrSpec>;
#[doc = "Register `RSTCTL_PSSRESET_CLR` writer"]
pub type W = crate::W<RstctlPssresetClrSpec>;
#[doc = "Field `CLR` writer - Write 1 clears all PSS Reset Flags in the RSTCTL_PSSRESET_STAT"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 clears all PSS Reset Flags in the RSTCTL_PSSRESET_STAT"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<RstctlPssresetClrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "PSS Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pssreset_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_pssreset_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlPssresetClrSpec;
impl crate::RegisterSpec for RstctlPssresetClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_pssreset_clr::R`](R) reader structure"]
impl crate::Readable for RstctlPssresetClrSpec {}
#[doc = "`write(|w| ..)` method takes [`rstctl_pssreset_clr::W`](W) writer structure"]
impl crate::Writable for RstctlPssresetClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTCTL_PSSRESET_CLR to value 0"]
impl crate::Resettable for RstctlPssresetClrSpec {
    const RESET_VALUE: u32 = 0;
}
