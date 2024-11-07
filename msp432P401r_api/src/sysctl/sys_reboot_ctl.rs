#[doc = "Register `SYS_REBOOT_CTL` reader"]
pub type R = crate::R<SysRebootCtlSpec>;
#[doc = "Register `SYS_REBOOT_CTL` writer"]
pub type W = crate::W<SysRebootCtlSpec>;
#[doc = "Field `REBOOT` reader - Write 1 initiates a Reboot of the device"]
pub type RebootR = crate::BitReader;
#[doc = "Field `REBOOT` writer - Write 1 initiates a Reboot of the device"]
pub type RebootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKEY` writer - Key to enable writes to bit 0"]
pub type WkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Write 1 initiates a Reboot of the device"]
    #[inline(always)]
    pub fn reboot(&self) -> RebootR {
        RebootR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 initiates a Reboot of the device"]
    #[inline(always)]
    pub fn reboot(&mut self) -> RebootW<SysRebootCtlSpec> {
        RebootW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Key to enable writes to bit 0"]
    #[inline(always)]
    pub fn wkey(&mut self) -> WkeyW<SysRebootCtlSpec> {
        WkeyW::new(self, 8)
    }
}
#[doc = "Reboot Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_reboot_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_reboot_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysRebootCtlSpec;
impl crate::RegisterSpec for SysRebootCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_reboot_ctl::R`](R) reader structure"]
impl crate::Readable for SysRebootCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_reboot_ctl::W`](W) writer structure"]
impl crate::Writable for SysRebootCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_REBOOT_CTL to value 0xfe"]
impl crate::Resettable for SysRebootCtlSpec {
    const RESET_VALUE: u32 = 0xfe;
}
