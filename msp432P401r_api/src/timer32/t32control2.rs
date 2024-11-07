#[doc = "Register `T32CONTROL2` reader"]
pub type R = crate::R<T32control2Spec>;
#[doc = "Register `T32CONTROL2` writer"]
pub type W = crate::W<T32control2Spec>;
#[doc = "Selects one-shot or wrapping counter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oneshot {
    #[doc = "0: wrapping mode"]
    Oneshot0 = 0,
    #[doc = "1: one-shot mode"]
    Oneshot1 = 1,
}
impl From<Oneshot> for bool {
    #[inline(always)]
    fn from(variant: Oneshot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONESHOT` reader - Selects one-shot or wrapping counter mode"]
pub type OneshotR = crate::BitReader<Oneshot>;
impl OneshotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oneshot {
        match self.bits {
            false => Oneshot::Oneshot0,
            true => Oneshot::Oneshot1,
        }
    }
    #[doc = "wrapping mode"]
    #[inline(always)]
    pub fn is_oneshot_0(&self) -> bool {
        *self == Oneshot::Oneshot0
    }
    #[doc = "one-shot mode"]
    #[inline(always)]
    pub fn is_oneshot_1(&self) -> bool {
        *self == Oneshot::Oneshot1
    }
}
#[doc = "Field `ONESHOT` writer - Selects one-shot or wrapping counter mode"]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG, Oneshot>;
impl<'a, REG> OneshotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wrapping mode"]
    #[inline(always)]
    pub fn oneshot_0(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot::Oneshot0)
    }
    #[doc = "one-shot mode"]
    #[inline(always)]
    pub fn oneshot_1(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot::Oneshot1)
    }
}
#[doc = "Selects 16 or 32 bit counter operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    #[doc = "0: 16-bit counter"]
    Size0 = 0,
    #[doc = "1: 32-bit counter"]
    Size1 = 1,
}
impl From<Size> for bool {
    #[inline(always)]
    fn from(variant: Size) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIZE` reader - Selects 16 or 32 bit counter operation"]
pub type SizeR = crate::BitReader<Size>;
impl SizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Size {
        match self.bits {
            false => Size::Size0,
            true => Size::Size1,
        }
    }
    #[doc = "16-bit counter"]
    #[inline(always)]
    pub fn is_size_0(&self) -> bool {
        *self == Size::Size0
    }
    #[doc = "32-bit counter"]
    #[inline(always)]
    pub fn is_size_1(&self) -> bool {
        *self == Size::Size1
    }
}
#[doc = "Field `SIZE` writer - Selects 16 or 32 bit counter operation"]
pub type SizeW<'a, REG> = crate::BitWriter<'a, REG, Size>;
impl<'a, REG> SizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16-bit counter"]
    #[inline(always)]
    pub fn size_0(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size0)
    }
    #[doc = "32-bit counter"]
    #[inline(always)]
    pub fn size_1(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Size1)
    }
}
#[doc = "Prescale bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescale {
    #[doc = "0: 0 stages of prescale, clock is divided by 1"]
    Prescale0 = 0,
    #[doc = "1: 4 stages of prescale, clock is divided by 16"]
    Prescale1 = 1,
    #[doc = "2: 8 stages of prescale, clock is divided by 256"]
    Prescale2 = 2,
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(variant: Prescale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescale {
    type Ux = u8;
}
impl crate::IsEnum for Prescale {}
#[doc = "Field `PRESCALE` reader - Prescale bits"]
pub type PrescaleR = crate::FieldReader<Prescale>;
impl PrescaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prescale> {
        match self.bits {
            0 => Some(Prescale::Prescale0),
            1 => Some(Prescale::Prescale1),
            2 => Some(Prescale::Prescale2),
            _ => None,
        }
    }
    #[doc = "0 stages of prescale, clock is divided by 1"]
    #[inline(always)]
    pub fn is_prescale_0(&self) -> bool {
        *self == Prescale::Prescale0
    }
    #[doc = "4 stages of prescale, clock is divided by 16"]
    #[inline(always)]
    pub fn is_prescale_1(&self) -> bool {
        *self == Prescale::Prescale1
    }
    #[doc = "8 stages of prescale, clock is divided by 256"]
    #[inline(always)]
    pub fn is_prescale_2(&self) -> bool {
        *self == Prescale::Prescale2
    }
}
#[doc = "Field `PRESCALE` writer - Prescale bits"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prescale>;
impl<'a, REG> PrescaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 stages of prescale, clock is divided by 1"]
    #[inline(always)]
    pub fn prescale_0(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::Prescale0)
    }
    #[doc = "4 stages of prescale, clock is divided by 16"]
    #[inline(always)]
    pub fn prescale_1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::Prescale1)
    }
    #[doc = "8 stages of prescale, clock is divided by 256"]
    #[inline(always)]
    pub fn prescale_2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::Prescale2)
    }
}
#[doc = "Interrupt enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ie {
    #[doc = "0: Timer interrupt disabled"]
    Ie0 = 0,
    #[doc = "1: Timer interrupt enabled"]
    Ie1 = 1,
}
impl From<Ie> for bool {
    #[inline(always)]
    fn from(variant: Ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt enable bit"]
pub type IeR = crate::BitReader<Ie>;
impl IeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ie {
        match self.bits {
            false => Ie::Ie0,
            true => Ie::Ie1,
        }
    }
    #[doc = "Timer interrupt disabled"]
    #[inline(always)]
    pub fn is_ie_0(&self) -> bool {
        *self == Ie::Ie0
    }
    #[doc = "Timer interrupt enabled"]
    #[inline(always)]
    pub fn is_ie_1(&self) -> bool {
        *self == Ie::Ie1
    }
}
#[doc = "Field `IE` writer - Interrupt enable bit"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG, Ie>;
impl<'a, REG> IeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer interrupt disabled"]
    #[inline(always)]
    pub fn ie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::Ie0)
    }
    #[doc = "Timer interrupt enabled"]
    #[inline(always)]
    pub fn ie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ie::Ie1)
    }
}
#[doc = "Mode bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Timer is in free-running mode"]
    Mode0 = 0,
    #[doc = "1: Timer is in periodic mode"]
    Mode1 = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Mode bit"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Mode0,
            true => Mode::Mode1,
        }
    }
    #[doc = "Timer is in free-running mode"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == Mode::Mode0
    }
    #[doc = "Timer is in periodic mode"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == Mode::Mode1
    }
}
#[doc = "Field `MODE` writer - Mode bit"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer is in free-running mode"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode0)
    }
    #[doc = "Timer is in periodic mode"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode1)
    }
}
#[doc = "Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Timer disabled"]
    Enable0 = 0,
    #[doc = "1: Timer enabled"]
    Enable1 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable bit"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Enable0,
            true => Enable::Enable1,
        }
    }
    #[doc = "Timer disabled"]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == Enable::Enable0
    }
    #[doc = "Timer enabled"]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == Enable::Enable1
    }
}
#[doc = "Field `ENABLE` writer - Enable bit"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer disabled"]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable0)
    }
    #[doc = "Timer enabled"]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable1)
    }
}
impl R {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode"]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects 16 or 32 bit counter operation"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Prescale bits"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable bit"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mode bit"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable bit"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<T32control2Spec> {
        OneshotW::new(self, 0)
    }
    #[doc = "Bit 1 - Selects 16 or 32 bit counter operation"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<T32control2Spec> {
        SizeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Prescale bits"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PrescaleW<T32control2Spec> {
        PrescaleW::new(self, 2)
    }
    #[doc = "Bit 5 - Interrupt enable bit"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<T32control2Spec> {
        IeW::new(self, 5)
    }
    #[doc = "Bit 6 - Mode bit"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<T32control2Spec> {
        ModeW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable bit"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<T32control2Spec> {
        EnableW::new(self, 7)
    }
}
#[doc = "Timer 2 Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32control2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32control2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T32control2Spec;
impl crate::RegisterSpec for T32control2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t32control2::R`](R) reader structure"]
impl crate::Readable for T32control2Spec {}
#[doc = "`write(|w| ..)` method takes [`t32control2::W`](W) writer structure"]
impl crate::Writable for T32control2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T32CONTROL2 to value 0x20"]
impl crate::Resettable for T32control2Spec {
    const RESET_VALUE: u32 = 0x20;
}
