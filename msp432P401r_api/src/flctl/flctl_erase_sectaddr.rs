#[doc = "Register `FLCTL_ERASE_SECTADDR` reader"]
pub type R = crate::R<FlctlEraseSectaddrSpec>;
#[doc = "Register `FLCTL_ERASE_SECTADDR` writer"]
pub type W = crate::W<FlctlEraseSectaddrSpec>;
#[doc = "Field `SECT_ADDRESS` reader - Address of Sector being Erased"]
pub type SectAddressR = crate::FieldReader<u32>;
#[doc = "Field `SECT_ADDRESS` writer - Address of Sector being Erased"]
pub type SectAddressW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Address of Sector being Erased"]
    #[inline(always)]
    pub fn sect_address(&self) -> SectAddressR {
        SectAddressR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Address of Sector being Erased"]
    #[inline(always)]
    pub fn sect_address(&mut self) -> SectAddressW<FlctlEraseSectaddrSpec> {
        SectAddressW::new(self, 0)
    }
}
#[doc = "Erase Sector Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_erase_sectaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_erase_sectaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlEraseSectaddrSpec;
impl crate::RegisterSpec for FlctlEraseSectaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_erase_sectaddr::R`](R) reader structure"]
impl crate::Readable for FlctlEraseSectaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_erase_sectaddr::W`](W) writer structure"]
impl crate::Writable for FlctlEraseSectaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_ERASE_SECTADDR to value 0"]
impl crate::Resettable for FlctlEraseSectaddrSpec {
    const RESET_VALUE: u32 = 0;
}
