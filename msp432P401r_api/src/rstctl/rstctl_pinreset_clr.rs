#[doc = "Register `RSTCTL_PINRESET_CLR` reader"]
pub type R = crate::R<RstctlPinresetClrSpec>;
#[doc = "Register `RSTCTL_PINRESET_CLR` writer"]
pub type W = crate::W<RstctlPinresetClrSpec>;
#[doc = "Field `CLR` writer - Write 1 clears the RSTn/NMI Pin Reset Flag in RSTCTL_PINRESET_STAT"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 clears the RSTn/NMI Pin Reset Flag in RSTCTL_PINRESET_STAT"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<RstctlPinresetClrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Pin Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pinreset_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_pinreset_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlPinresetClrSpec;
impl crate::RegisterSpec for RstctlPinresetClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_pinreset_clr::R`](R) reader structure"]
impl crate::Readable for RstctlPinresetClrSpec {}
#[doc = "`write(|w| ..)` method takes [`rstctl_pinreset_clr::W`](W) writer structure"]
impl crate::Writable for RstctlPinresetClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTCTL_PINRESET_CLR to value 0"]
impl crate::Resettable for RstctlPinresetClrSpec {
    const RESET_VALUE: u32 = 0;
}
