#[doc = "Register `PMAPCTL` reader"]
pub type R = crate::R<PmapctlSpec>;
#[doc = "Register `PMAPCTL` writer"]
pub type W = crate::W<PmapctlSpec>;
#[doc = "Port mapping lock bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmaplockedEnumRead {
    #[doc = "0: Access to mapping registers is granted"]
    Pmaplocked0 = 0,
    #[doc = "1: Access to mapping registers is locked"]
    Pmaplocked1 = 1,
}
impl From<PmaplockedEnumRead> for bool {
    #[inline(always)]
    fn from(variant: PmaplockedEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAPLOCKED` reader - Port mapping lock bit"]
pub type PmaplockedR = crate::BitReader<PmaplockedEnumRead>;
impl PmaplockedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmaplockedEnumRead {
        match self.bits {
            false => PmaplockedEnumRead::Pmaplocked0,
            true => PmaplockedEnumRead::Pmaplocked1,
        }
    }
    #[doc = "Access to mapping registers is granted"]
    #[inline(always)]
    pub fn is_pmaplocked_0(&self) -> bool {
        *self == PmaplockedEnumRead::Pmaplocked0
    }
    #[doc = "Access to mapping registers is locked"]
    #[inline(always)]
    pub fn is_pmaplocked_1(&self) -> bool {
        *self == PmaplockedEnumRead::Pmaplocked1
    }
}
#[doc = "Port mapping reconfiguration control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmaprecfg {
    #[doc = "0: Configuration allowed only once"]
    Pmaprecfg0 = 0,
    #[doc = "1: Allow reconfiguration of port mapping"]
    Pmaprecfg1 = 1,
}
impl From<Pmaprecfg> for bool {
    #[inline(always)]
    fn from(variant: Pmaprecfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAPRECFG` reader - Port mapping reconfiguration control bit"]
pub type PmaprecfgR = crate::BitReader<Pmaprecfg>;
impl PmaprecfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmaprecfg {
        match self.bits {
            false => Pmaprecfg::Pmaprecfg0,
            true => Pmaprecfg::Pmaprecfg1,
        }
    }
    #[doc = "Configuration allowed only once"]
    #[inline(always)]
    pub fn is_pmaprecfg_0(&self) -> bool {
        *self == Pmaprecfg::Pmaprecfg0
    }
    #[doc = "Allow reconfiguration of port mapping"]
    #[inline(always)]
    pub fn is_pmaprecfg_1(&self) -> bool {
        *self == Pmaprecfg::Pmaprecfg1
    }
}
#[doc = "Field `PMAPRECFG` writer - Port mapping reconfiguration control bit"]
pub type PmaprecfgW<'a, REG> = crate::BitWriter<'a, REG, Pmaprecfg>;
impl<'a, REG> PmaprecfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configuration allowed only once"]
    #[inline(always)]
    pub fn pmaprecfg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmaprecfg::Pmaprecfg0)
    }
    #[doc = "Allow reconfiguration of port mapping"]
    #[inline(always)]
    pub fn pmaprecfg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmaprecfg::Pmaprecfg1)
    }
}
impl R {
    #[doc = "Bit 0 - Port mapping lock bit"]
    #[inline(always)]
    pub fn pmaplocked(&self) -> PmaplockedR {
        PmaplockedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port mapping reconfiguration control bit"]
    #[inline(always)]
    pub fn pmaprecfg(&self) -> PmaprecfgR {
        PmaprecfgR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Port mapping reconfiguration control bit"]
    #[inline(always)]
    pub fn pmaprecfg(&mut self) -> PmaprecfgW<PmapctlSpec> {
        PmaprecfgW::new(self, 1)
    }
}
#[doc = "Port Mapping Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmapctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmapctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmapctlSpec;
impl crate::RegisterSpec for PmapctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmapctl::R`](R) reader structure"]
impl crate::Readable for PmapctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pmapctl::W`](W) writer structure"]
impl crate::Writable for PmapctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMAPCTL to value 0x01"]
impl crate::Resettable for PmapctlSpec {
    const RESET_VALUE: u16 = 0x01;
}
