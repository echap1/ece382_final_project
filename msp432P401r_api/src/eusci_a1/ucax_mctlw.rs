#[doc = "Register `UCAxMCTLW` reader"]
pub type R = crate::R<UcaxMctlwSpec>;
#[doc = "Register `UCAxMCTLW` writer"]
pub type W = crate::W<UcaxMctlwSpec>;
#[doc = "Oversampling mode enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucos16 {
    #[doc = "0: Disabled"]
    Ucos16_0 = 0,
    #[doc = "1: Enabled"]
    Ucos16_1 = 1,
}
impl From<Ucos16> for bool {
    #[inline(always)]
    fn from(variant: Ucos16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCOS16` reader - Oversampling mode enabled"]
pub type Ucos16R = crate::BitReader<Ucos16>;
impl Ucos16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucos16 {
        match self.bits {
            false => Ucos16::Ucos16_0,
            true => Ucos16::Ucos16_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_ucos16_0(&self) -> bool {
        *self == Ucos16::Ucos16_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_ucos16_1(&self) -> bool {
        *self == Ucos16::Ucos16_1
    }
}
#[doc = "Field `UCOS16` writer - Oversampling mode enabled"]
pub type Ucos16W<'a, REG> = crate::BitWriter<'a, REG, Ucos16>;
impl<'a, REG> Ucos16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn ucos16_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucos16::Ucos16_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ucos16_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucos16::Ucos16_1)
    }
}
#[doc = "Field `UCBRF` reader - First modulation stage select"]
pub type UcbrfR = crate::FieldReader;
#[doc = "Field `UCBRF` writer - First modulation stage select"]
pub type UcbrfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UCBRS` reader - Second modulation stage select"]
pub type UcbrsR = crate::FieldReader;
#[doc = "Field `UCBRS` writer - Second modulation stage select"]
pub type UcbrsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Oversampling mode enabled"]
    #[inline(always)]
    pub fn ucos16(&self) -> Ucos16R {
        Ucos16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - First modulation stage select"]
    #[inline(always)]
    pub fn ucbrf(&self) -> UcbrfR {
        UcbrfR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Second modulation stage select"]
    #[inline(always)]
    pub fn ucbrs(&self) -> UcbrsR {
        UcbrsR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oversampling mode enabled"]
    #[inline(always)]
    pub fn ucos16(&mut self) -> Ucos16W<UcaxMctlwSpec> {
        Ucos16W::new(self, 0)
    }
    #[doc = "Bits 4:7 - First modulation stage select"]
    #[inline(always)]
    pub fn ucbrf(&mut self) -> UcbrfW<UcaxMctlwSpec> {
        UcbrfW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Second modulation stage select"]
    #[inline(always)]
    pub fn ucbrs(&mut self) -> UcbrsW<UcaxMctlwSpec> {
        UcbrsW::new(self, 8)
    }
}
#[doc = "eUSCI_Ax Modulation Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_mctlw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_mctlw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcaxMctlwSpec;
impl crate::RegisterSpec for UcaxMctlwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucax_mctlw::R`](R) reader structure"]
impl crate::Readable for UcaxMctlwSpec {}
#[doc = "`write(|w| ..)` method takes [`ucax_mctlw::W`](W) writer structure"]
impl crate::Writable for UcaxMctlwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCAxMCTLW to value 0"]
impl crate::Resettable for UcaxMctlwSpec {
    const RESET_VALUE: u16 = 0;
}
