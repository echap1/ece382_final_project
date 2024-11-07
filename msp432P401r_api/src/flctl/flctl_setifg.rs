#[doc = "Register `FLCTL_SETIFG` writer"]
pub type W = crate::W<FlctlSetifgSpec>;
#[doc = "Field `RDBRST` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type RdbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVPRE` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type AvpreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVPST` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type AvpstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type PrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRGB` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type PrgbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASE` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMRK` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type BmrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRG_ERR` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type PrgErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&mut self) -> RdbrstW<FlctlSetifgSpec> {
        RdbrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&mut self) -> AvpreW<FlctlSetifgSpec> {
        AvpreW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&mut self) -> AvpstW<FlctlSetifgSpec> {
        AvpstW::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&mut self) -> PrgW<FlctlSetifgSpec> {
        PrgW::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&mut self) -> PrgbW<FlctlSetifgSpec> {
        PrgbW::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<FlctlSetifgSpec> {
        EraseW::new(self, 5)
    }
    #[doc = "Bit 8 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&mut self) -> BmrkW<FlctlSetifgSpec> {
        BmrkW::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg_err(&mut self) -> PrgErrW<FlctlSetifgSpec> {
        PrgErrW::new(self, 9)
    }
}
#[doc = "Set Interrupt Flag Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_setifg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlSetifgSpec;
impl crate::RegisterSpec for FlctlSetifgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flctl_setifg::W`](W) writer structure"]
impl crate::Writable for FlctlSetifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_SETIFG to value 0"]
impl crate::Resettable for FlctlSetifgSpec {
    const RESET_VALUE: u32 = 0;
}
