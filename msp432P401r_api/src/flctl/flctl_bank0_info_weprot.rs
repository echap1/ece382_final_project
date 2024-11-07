#[doc = "Register `FLCTL_BANK0_INFO_WEPROT` reader"]
pub type R = crate::R<FlctlBank0InfoWeprotSpec>;
#[doc = "Register `FLCTL_BANK0_INFO_WEPROT` writer"]
pub type W = crate::W<FlctlBank0InfoWeprotSpec>;
#[doc = "Field `PROT0` reader - Protects Sector 0 from program or erase"]
pub type Prot0R = crate::BitReader;
#[doc = "Field `PROT0` writer - Protects Sector 0 from program or erase"]
pub type Prot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROT1` reader - Protects Sector 1 from program or erase"]
pub type Prot1R = crate::BitReader;
#[doc = "Field `PROT1` writer - Protects Sector 1 from program or erase"]
pub type Prot1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase"]
    #[inline(always)]
    pub fn prot0(&self) -> Prot0R {
        Prot0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase"]
    #[inline(always)]
    pub fn prot1(&self) -> Prot1R {
        Prot1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase"]
    #[inline(always)]
    pub fn prot0(&mut self) -> Prot0W<FlctlBank0InfoWeprotSpec> {
        Prot0W::new(self, 0)
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase"]
    #[inline(always)]
    pub fn prot1(&mut self) -> Prot1W<FlctlBank0InfoWeprotSpec> {
        Prot1W::new(self, 1)
    }
}
#[doc = "Information Memory Bank0 Write/Erase Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bank0_info_weprot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bank0_info_weprot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlBank0InfoWeprotSpec;
impl crate::RegisterSpec for FlctlBank0InfoWeprotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_bank0_info_weprot::R`](R) reader structure"]
impl crate::Readable for FlctlBank0InfoWeprotSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_bank0_info_weprot::W`](W) writer structure"]
impl crate::Writable for FlctlBank0InfoWeprotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_BANK0_INFO_WEPROT to value 0x03"]
impl crate::Resettable for FlctlBank0InfoWeprotSpec {
    const RESET_VALUE: u32 = 0x03;
}