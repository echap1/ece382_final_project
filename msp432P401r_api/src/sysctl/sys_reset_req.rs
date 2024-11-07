#[doc = "Register `SYS_RESET_REQ` reader"]
pub type R = crate::R<SysResetReqSpec>;
#[doc = "Register `SYS_RESET_REQ` writer"]
pub type W = crate::W<SysResetReqSpec>;
#[doc = "Field `POR` writer - Generate POR"]
pub type PorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REBOOT` writer - Generate Reboot_Reset"]
pub type RebootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKEY` writer - Write key"]
pub type WkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bit 0 - Generate POR"]
    #[inline(always)]
    pub fn por(&mut self) -> PorW<SysResetReqSpec> {
        PorW::new(self, 0)
    }
    #[doc = "Bit 1 - Generate Reboot_Reset"]
    #[inline(always)]
    pub fn reboot(&mut self) -> RebootW<SysResetReqSpec> {
        RebootW::new(self, 1)
    }
    #[doc = "Bits 8:15 - Write key"]
    #[inline(always)]
    pub fn wkey(&mut self) -> WkeyW<SysResetReqSpec> {
        WkeyW::new(self, 8)
    }
}
#[doc = "Reset Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_reset_req::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_reset_req::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysResetReqSpec;
impl crate::RegisterSpec for SysResetReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_reset_req::R`](R) reader structure"]
impl crate::Readable for SysResetReqSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_reset_req::W`](W) writer structure"]
impl crate::Writable for SysResetReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
