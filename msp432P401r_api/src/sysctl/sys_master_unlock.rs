#[doc = "Register `SYS_MASTER_UNLOCK` reader"]
pub type R = crate::R<SysMasterUnlockSpec>;
#[doc = "Register `SYS_MASTER_UNLOCK` writer"]
pub type W = crate::W<SysMasterUnlockSpec>;
#[doc = "Field `UNLKEY` reader - Unlock Key"]
pub type UnlkeyR = crate::FieldReader<u16>;
#[doc = "Field `UNLKEY` writer - Unlock Key"]
pub type UnlkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Unlock Key"]
    #[inline(always)]
    pub fn unlkey(&self) -> UnlkeyR {
        UnlkeyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unlock Key"]
    #[inline(always)]
    pub fn unlkey(&mut self) -> UnlkeyW<SysMasterUnlockSpec> {
        UnlkeyW::new(self, 0)
    }
}
#[doc = "Master Unlock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_master_unlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_master_unlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysMasterUnlockSpec;
impl crate::RegisterSpec for SysMasterUnlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_master_unlock::R`](R) reader structure"]
impl crate::Readable for SysMasterUnlockSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_master_unlock::W`](W) writer structure"]
impl crate::Writable for SysMasterUnlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_MASTER_UNLOCK to value 0"]
impl crate::Resettable for SysMasterUnlockSpec {
    const RESET_VALUE: u32 = 0;
}
