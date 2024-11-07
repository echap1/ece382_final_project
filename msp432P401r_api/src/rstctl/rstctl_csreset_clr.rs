#[doc = "Register `RSTCTL_CSRESET_CLR` reader"]
pub type R = crate::R<RstctlCsresetClrSpec>;
#[doc = "Register `RSTCTL_CSRESET_CLR` writer"]
pub type W = crate::W<RstctlCsresetClrSpec>;
#[doc = "Field `CLR` writer - Write 1 clears the DCOR_SHT Flag in RSTCTL_CSRESET_STAT as well as DCOR_SHTIFG flag in CSIFG register of clock system"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 clears the DCOR_SHT Flag in RSTCTL_CSRESET_STAT as well as DCOR_SHTIFG flag in CSIFG register of clock system"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<RstctlCsresetClrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "CS Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_csreset_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_csreset_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlCsresetClrSpec;
impl crate::RegisterSpec for RstctlCsresetClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_csreset_clr::R`](R) reader structure"]
impl crate::Readable for RstctlCsresetClrSpec {}
#[doc = "`write(|w| ..)` method takes [`rstctl_csreset_clr::W`](W) writer structure"]
impl crate::Writable for RstctlCsresetClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTCTL_CSRESET_CLR to value 0"]
impl crate::Resettable for RstctlCsresetClrSpec {
    const RESET_VALUE: u32 = 0;
}
