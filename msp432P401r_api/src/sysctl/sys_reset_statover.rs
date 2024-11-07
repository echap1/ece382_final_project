#[doc = "Register `SYS_RESET_STATOVER` reader"]
pub type R = crate::R<SysResetStatoverSpec>;
#[doc = "Register `SYS_RESET_STATOVER` writer"]
pub type W = crate::W<SysResetStatoverSpec>;
#[doc = "Field `SOFT` reader - Indicates if SOFT Reset is active"]
pub type SoftR = crate::BitReader;
#[doc = "Field `HARD` reader - Indicates if HARD Reset is active"]
pub type HardR = crate::BitReader;
#[doc = "Field `REBOOT` reader - Indicates if Reboot Reset is active"]
pub type RebootR = crate::BitReader;
#[doc = "Field `SOFT_OVER` reader - SOFT_Reset overwrite request"]
pub type SoftOverR = crate::BitReader;
#[doc = "Field `SOFT_OVER` writer - SOFT_Reset overwrite request"]
pub type SoftOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HARD_OVER` reader - HARD_Reset overwrite request"]
pub type HardOverR = crate::BitReader;
#[doc = "Field `HARD_OVER` writer - HARD_Reset overwrite request"]
pub type HardOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBT_OVER` reader - Reboot Reset overwrite request"]
pub type RbtOverR = crate::BitReader;
#[doc = "Field `RBT_OVER` writer - Reboot Reset overwrite request"]
pub type RbtOverW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates if SOFT Reset is active"]
    #[inline(always)]
    pub fn soft(&self) -> SoftR {
        SoftR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if HARD Reset is active"]
    #[inline(always)]
    pub fn hard(&self) -> HardR {
        HardR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates if Reboot Reset is active"]
    #[inline(always)]
    pub fn reboot(&self) -> RebootR {
        RebootR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SOFT_Reset overwrite request"]
    #[inline(always)]
    pub fn soft_over(&self) -> SoftOverR {
        SoftOverR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HARD_Reset overwrite request"]
    #[inline(always)]
    pub fn hard_over(&self) -> HardOverR {
        HardOverR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reboot Reset overwrite request"]
    #[inline(always)]
    pub fn rbt_over(&self) -> RbtOverR {
        RbtOverR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SOFT_Reset overwrite request"]
    #[inline(always)]
    pub fn soft_over(&mut self) -> SoftOverW<SysResetStatoverSpec> {
        SoftOverW::new(self, 8)
    }
    #[doc = "Bit 9 - HARD_Reset overwrite request"]
    #[inline(always)]
    pub fn hard_over(&mut self) -> HardOverW<SysResetStatoverSpec> {
        HardOverW::new(self, 9)
    }
    #[doc = "Bit 10 - Reboot Reset overwrite request"]
    #[inline(always)]
    pub fn rbt_over(&mut self) -> RbtOverW<SysResetStatoverSpec> {
        RbtOverW::new(self, 10)
    }
}
#[doc = "Reset Status and Override Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_reset_statover::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_reset_statover::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysResetStatoverSpec;
impl crate::RegisterSpec for SysResetStatoverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_reset_statover::R`](R) reader structure"]
impl crate::Readable for SysResetStatoverSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_reset_statover::W`](W) writer structure"]
impl crate::Writable for SysResetStatoverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_RESET_STATOVER to value 0"]
impl crate::Resettable for SysResetStatoverSpec {
    const RESET_VALUE: u32 = 0;
}
