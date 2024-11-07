#[doc = "Register `CSCTL0` reader"]
pub type R = crate::R<Csctl0Spec>;
#[doc = "Register `CSCTL0` writer"]
pub type W = crate::W<Csctl0Spec>;
#[doc = "Field `DCOTUNE` reader - DCO frequency tuning select"]
pub type DcotuneR = crate::FieldReader<u16>;
#[doc = "Field `DCOTUNE` writer - DCO frequency tuning select"]
pub type DcotuneW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "DCO frequency range select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcorsel {
    #[doc = "0: Nominal DCO Frequency Range (MHz): 1 to 2"]
    Dcorsel0 = 0,
    #[doc = "1: Nominal DCO Frequency Range (MHz): 2 to 4"]
    Dcorsel1 = 1,
    #[doc = "2: Nominal DCO Frequency Range (MHz): 4 to 8"]
    Dcorsel2 = 2,
    #[doc = "3: Nominal DCO Frequency Range (MHz): 8 to 16"]
    Dcorsel3 = 3,
    #[doc = "4: Nominal DCO Frequency Range (MHz): 16 to 32"]
    Dcorsel4 = 4,
    #[doc = "5: Nominal DCO Frequency Range (MHz): 32 to 64"]
    Dcorsel5 = 5,
}
impl From<Dcorsel> for u8 {
    #[inline(always)]
    fn from(variant: Dcorsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcorsel {
    type Ux = u8;
}
impl crate::IsEnum for Dcorsel {}
#[doc = "Field `DCORSEL` reader - DCO frequency range select"]
pub type DcorselR = crate::FieldReader<Dcorsel>;
impl DcorselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dcorsel> {
        match self.bits {
            0 => Some(Dcorsel::Dcorsel0),
            1 => Some(Dcorsel::Dcorsel1),
            2 => Some(Dcorsel::Dcorsel2),
            3 => Some(Dcorsel::Dcorsel3),
            4 => Some(Dcorsel::Dcorsel4),
            5 => Some(Dcorsel::Dcorsel5),
            _ => None,
        }
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 1 to 2"]
    #[inline(always)]
    pub fn is_dcorsel_0(&self) -> bool {
        *self == Dcorsel::Dcorsel0
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 2 to 4"]
    #[inline(always)]
    pub fn is_dcorsel_1(&self) -> bool {
        *self == Dcorsel::Dcorsel1
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 4 to 8"]
    #[inline(always)]
    pub fn is_dcorsel_2(&self) -> bool {
        *self == Dcorsel::Dcorsel2
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 8 to 16"]
    #[inline(always)]
    pub fn is_dcorsel_3(&self) -> bool {
        *self == Dcorsel::Dcorsel3
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 16 to 32"]
    #[inline(always)]
    pub fn is_dcorsel_4(&self) -> bool {
        *self == Dcorsel::Dcorsel4
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 32 to 64"]
    #[inline(always)]
    pub fn is_dcorsel_5(&self) -> bool {
        *self == Dcorsel::Dcorsel5
    }
}
#[doc = "Field `DCORSEL` writer - DCO frequency range select"]
pub type DcorselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dcorsel>;
impl<'a, REG> DcorselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Nominal DCO Frequency Range (MHz): 1 to 2"]
    #[inline(always)]
    pub fn dcorsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel0)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 2 to 4"]
    #[inline(always)]
    pub fn dcorsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel1)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 4 to 8"]
    #[inline(always)]
    pub fn dcorsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel2)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 8 to 16"]
    #[inline(always)]
    pub fn dcorsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel3)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 16 to 32"]
    #[inline(always)]
    pub fn dcorsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel4)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 32 to 64"]
    #[inline(always)]
    pub fn dcorsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel5)
    }
}
#[doc = "Enables the DCO external resistor mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcores {
    #[doc = "0: Internal resistor mode"]
    Dcores0 = 0,
    #[doc = "1: External resistor mode"]
    Dcores1 = 1,
}
impl From<Dcores> for bool {
    #[inline(always)]
    fn from(variant: Dcores) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCORES` reader - Enables the DCO external resistor mode"]
pub type DcoresR = crate::BitReader<Dcores>;
impl DcoresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcores {
        match self.bits {
            false => Dcores::Dcores0,
            true => Dcores::Dcores1,
        }
    }
    #[doc = "Internal resistor mode"]
    #[inline(always)]
    pub fn is_dcores_0(&self) -> bool {
        *self == Dcores::Dcores0
    }
    #[doc = "External resistor mode"]
    #[inline(always)]
    pub fn is_dcores_1(&self) -> bool {
        *self == Dcores::Dcores1
    }
}
#[doc = "Field `DCORES` writer - Enables the DCO external resistor mode"]
pub type DcoresW<'a, REG> = crate::BitWriter<'a, REG, Dcores>;
impl<'a, REG> DcoresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal resistor mode"]
    #[inline(always)]
    pub fn dcores_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcores::Dcores0)
    }
    #[doc = "External resistor mode"]
    #[inline(always)]
    pub fn dcores_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcores::Dcores1)
    }
}
#[doc = "Enables the DCO oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcoen {
    #[doc = "0: DCO is on if it is used as a source for MCLK, HSMCLK , or SMCLK and clock is requested, otherwise it is disabled."]
    Dcoen0 = 0,
    #[doc = "1: DCO is on"]
    Dcoen1 = 1,
}
impl From<Dcoen> for bool {
    #[inline(always)]
    fn from(variant: Dcoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCOEN` reader - Enables the DCO oscillator"]
pub type DcoenR = crate::BitReader<Dcoen>;
impl DcoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcoen {
        match self.bits {
            false => Dcoen::Dcoen0,
            true => Dcoen::Dcoen1,
        }
    }
    #[doc = "DCO is on if it is used as a source for MCLK, HSMCLK , or SMCLK and clock is requested, otherwise it is disabled."]
    #[inline(always)]
    pub fn is_dcoen_0(&self) -> bool {
        *self == Dcoen::Dcoen0
    }
    #[doc = "DCO is on"]
    #[inline(always)]
    pub fn is_dcoen_1(&self) -> bool {
        *self == Dcoen::Dcoen1
    }
}
#[doc = "Field `DCOEN` writer - Enables the DCO oscillator"]
pub type DcoenW<'a, REG> = crate::BitWriter<'a, REG, Dcoen>;
impl<'a, REG> DcoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DCO is on if it is used as a source for MCLK, HSMCLK , or SMCLK and clock is requested, otherwise it is disabled."]
    #[inline(always)]
    pub fn dcoen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoen::Dcoen0)
    }
    #[doc = "DCO is on"]
    #[inline(always)]
    pub fn dcoen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoen::Dcoen1)
    }
}
impl R {
    #[doc = "Bits 0:9 - DCO frequency tuning select"]
    #[inline(always)]
    pub fn dcotune(&self) -> DcotuneR {
        DcotuneR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18 - DCO frequency range select"]
    #[inline(always)]
    pub fn dcorsel(&self) -> DcorselR {
        DcorselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 22 - Enables the DCO external resistor mode"]
    #[inline(always)]
    pub fn dcores(&self) -> DcoresR {
        DcoresR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enables the DCO oscillator"]
    #[inline(always)]
    pub fn dcoen(&self) -> DcoenR {
        DcoenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - DCO frequency tuning select"]
    #[inline(always)]
    pub fn dcotune(&mut self) -> DcotuneW<Csctl0Spec> {
        DcotuneW::new(self, 0)
    }
    #[doc = "Bits 16:18 - DCO frequency range select"]
    #[inline(always)]
    pub fn dcorsel(&mut self) -> DcorselW<Csctl0Spec> {
        DcorselW::new(self, 16)
    }
    #[doc = "Bit 22 - Enables the DCO external resistor mode"]
    #[inline(always)]
    pub fn dcores(&mut self) -> DcoresW<Csctl0Spec> {
        DcoresW::new(self, 22)
    }
    #[doc = "Bit 23 - Enables the DCO oscillator"]
    #[inline(always)]
    pub fn dcoen(&mut self) -> DcoenW<Csctl0Spec> {
        DcoenW::new(self, 23)
    }
}
#[doc = "Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl0Spec;
impl crate::RegisterSpec for Csctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csctl0::R`](R) reader structure"]
impl crate::Readable for Csctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl0::W`](W) writer structure"]
impl crate::Writable for Csctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSCTL0 to value 0x0001_0000"]
impl crate::Resettable for Csctl0Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
