#[doc = "Register `FLCTL_IE` reader"]
pub type R = crate::R<FlctlIeSpec>;
#[doc = "Register `FLCTL_IE` writer"]
pub type W = crate::W<FlctlIeSpec>;
#[doc = "Field `RDBRST` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type RdbrstR = crate::BitReader;
#[doc = "Field `RDBRST` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type RdbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVPRE` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type AvpreR = crate::BitReader;
#[doc = "Field `AVPRE` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type AvpreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVPST` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type AvpstR = crate::BitReader;
#[doc = "Field `AVPST` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type AvpstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PrgR = crate::BitReader;
#[doc = "Field `PRG` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRGB` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PrgbR = crate::BitReader;
#[doc = "Field `PRGB` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PrgbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASE` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type EraseR = crate::BitReader;
#[doc = "Field `ERASE` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMRK` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type BmrkR = crate::BitReader;
#[doc = "Field `BMRK` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type BmrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_ERR` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PrgErrR = crate::BitReader;
#[doc = "Field `PRG_ERR` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub type PrgErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&self) -> RdbrstR {
        RdbrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&self) -> AvpreR {
        AvpreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&self) -> AvpstR {
        AvpstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&self) -> PrgR {
        PrgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&self) -> PrgbR {
        PrgbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&self) -> BmrkR {
        BmrkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg_err(&self) -> PrgErrR {
        PrgErrR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&mut self) -> RdbrstW<FlctlIeSpec> {
        RdbrstW::new(self, 0)
    }
    #[doc = "Bit 1 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&mut self) -> AvpreW<FlctlIeSpec> {
        AvpreW::new(self, 1)
    }
    #[doc = "Bit 2 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&mut self) -> AvpstW<FlctlIeSpec> {
        AvpstW::new(self, 2)
    }
    #[doc = "Bit 3 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&mut self) -> PrgW<FlctlIeSpec> {
        PrgW::new(self, 3)
    }
    #[doc = "Bit 4 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&mut self) -> PrgbW<FlctlIeSpec> {
        PrgbW::new(self, 4)
    }
    #[doc = "Bit 5 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<FlctlIeSpec> {
        EraseW::new(self, 5)
    }
    #[doc = "Bit 8 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&mut self) -> BmrkW<FlctlIeSpec> {
        BmrkW::new(self, 8)
    }
    #[doc = "Bit 9 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg_err(&mut self) -> PrgErrW<FlctlIeSpec> {
        PrgErrW::new(self, 9)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlIeSpec;
impl crate::RegisterSpec for FlctlIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_ie::R`](R) reader structure"]
impl crate::Readable for FlctlIeSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_ie::W`](W) writer structure"]
impl crate::Writable for FlctlIeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_IE to value 0"]
impl crate::Resettable for FlctlIeSpec {
    const RESET_VALUE: u32 = 0;
}
