#[doc = "Register `RTCCTL0` reader"]
pub type R = crate::R<Rtcctl0Spec>;
#[doc = "Register `RTCCTL0` writer"]
pub type W = crate::W<Rtcctl0Spec>;
#[doc = "Real-time clock ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcrdyifg {
    #[doc = "0: RTC cannot be read safely"]
    Rtcrdyifg0 = 0,
    #[doc = "1: RTC can be read safely"]
    Rtcrdyifg1 = 1,
}
impl From<Rtcrdyifg> for bool {
    #[inline(always)]
    fn from(variant: Rtcrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCRDYIFG` reader - Real-time clock ready interrupt flag"]
pub type RtcrdyifgR = crate::BitReader<Rtcrdyifg>;
impl RtcrdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcrdyifg {
        match self.bits {
            false => Rtcrdyifg::Rtcrdyifg0,
            true => Rtcrdyifg::Rtcrdyifg1,
        }
    }
    #[doc = "RTC cannot be read safely"]
    #[inline(always)]
    pub fn is_rtcrdyifg_0(&self) -> bool {
        *self == Rtcrdyifg::Rtcrdyifg0
    }
    #[doc = "RTC can be read safely"]
    #[inline(always)]
    pub fn is_rtcrdyifg_1(&self) -> bool {
        *self == Rtcrdyifg::Rtcrdyifg1
    }
}
#[doc = "Field `RTCRDYIFG` writer - Real-time clock ready interrupt flag"]
pub type RtcrdyifgW<'a, REG> = crate::BitWriter<'a, REG, Rtcrdyifg>;
impl<'a, REG> RtcrdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC cannot be read safely"]
    #[inline(always)]
    pub fn rtcrdyifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcrdyifg::Rtcrdyifg0)
    }
    #[doc = "RTC can be read safely"]
    #[inline(always)]
    pub fn rtcrdyifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcrdyifg::Rtcrdyifg1)
    }
}
#[doc = "Real-time clock alarm interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcaifg {
    #[doc = "0: No time event occurred"]
    Rtcaifg0 = 0,
    #[doc = "1: Time event occurred"]
    Rtcaifg1 = 1,
}
impl From<Rtcaifg> for bool {
    #[inline(always)]
    fn from(variant: Rtcaifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCAIFG` reader - Real-time clock alarm interrupt flag"]
pub type RtcaifgR = crate::BitReader<Rtcaifg>;
impl RtcaifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcaifg {
        match self.bits {
            false => Rtcaifg::Rtcaifg0,
            true => Rtcaifg::Rtcaifg1,
        }
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn is_rtcaifg_0(&self) -> bool {
        *self == Rtcaifg::Rtcaifg0
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn is_rtcaifg_1(&self) -> bool {
        *self == Rtcaifg::Rtcaifg1
    }
}
#[doc = "Field `RTCAIFG` writer - Real-time clock alarm interrupt flag"]
pub type RtcaifgW<'a, REG> = crate::BitWriter<'a, REG, Rtcaifg>;
impl<'a, REG> RtcaifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rtcaifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcaifg::Rtcaifg0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rtcaifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcaifg::Rtcaifg1)
    }
}
#[doc = "Real-time clock time event interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtctevifg {
    #[doc = "0: No time event occurred"]
    Rtctevifg0 = 0,
    #[doc = "1: Time event occurred"]
    Rtctevifg1 = 1,
}
impl From<Rtctevifg> for bool {
    #[inline(always)]
    fn from(variant: Rtctevifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCTEVIFG` reader - Real-time clock time event interrupt flag"]
pub type RtctevifgR = crate::BitReader<Rtctevifg>;
impl RtctevifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtctevifg {
        match self.bits {
            false => Rtctevifg::Rtctevifg0,
            true => Rtctevifg::Rtctevifg1,
        }
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn is_rtctevifg_0(&self) -> bool {
        *self == Rtctevifg::Rtctevifg0
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn is_rtctevifg_1(&self) -> bool {
        *self == Rtctevifg::Rtctevifg1
    }
}
#[doc = "Field `RTCTEVIFG` writer - Real-time clock time event interrupt flag"]
pub type RtctevifgW<'a, REG> = crate::BitWriter<'a, REG, Rtctevifg>;
impl<'a, REG> RtctevifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rtctevifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctevifg::Rtctevifg0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rtctevifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctevifg::Rtctevifg1)
    }
}
#[doc = "32-kHz crystal oscillator fault interrupt flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcofifg {
    #[doc = "0: No interrupt pending"]
    Rtcofifg0 = 0,
    #[doc = "1: Interrupt pending. A 32-kHz crystal oscillator fault occurred after last reset."]
    Rtcofifg1 = 1,
}
impl From<Rtcofifg> for bool {
    #[inline(always)]
    fn from(variant: Rtcofifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCOFIFG` reader - 32-kHz crystal oscillator fault interrupt flag"]
pub type RtcofifgR = crate::BitReader<Rtcofifg>;
impl RtcofifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcofifg {
        match self.bits {
            false => Rtcofifg::Rtcofifg0,
            true => Rtcofifg::Rtcofifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_rtcofifg_0(&self) -> bool {
        *self == Rtcofifg::Rtcofifg0
    }
    #[doc = "Interrupt pending. A 32-kHz crystal oscillator fault occurred after last reset."]
    #[inline(always)]
    pub fn is_rtcofifg_1(&self) -> bool {
        *self == Rtcofifg::Rtcofifg1
    }
}
#[doc = "Field `RTCOFIFG` writer - 32-kHz crystal oscillator fault interrupt flag"]
pub type RtcofifgW<'a, REG> = crate::BitWriter<'a, REG, Rtcofifg>;
impl<'a, REG> RtcofifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn rtcofifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcofifg::Rtcofifg0)
    }
    #[doc = "Interrupt pending. A 32-kHz crystal oscillator fault occurred after last reset."]
    #[inline(always)]
    pub fn rtcofifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcofifg::Rtcofifg1)
    }
}
#[doc = "Real-time clock ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcrdyie {
    #[doc = "0: Interrupt not enabled"]
    Rtcrdyie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Rtcrdyie1 = 1,
}
impl From<Rtcrdyie> for bool {
    #[inline(always)]
    fn from(variant: Rtcrdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCRDYIE` reader - Real-time clock ready interrupt enable"]
pub type RtcrdyieR = crate::BitReader<Rtcrdyie>;
impl RtcrdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcrdyie {
        match self.bits {
            false => Rtcrdyie::Rtcrdyie0,
            true => Rtcrdyie::Rtcrdyie1,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_rtcrdyie_0(&self) -> bool {
        *self == Rtcrdyie::Rtcrdyie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_rtcrdyie_1(&self) -> bool {
        *self == Rtcrdyie::Rtcrdyie1
    }
}
#[doc = "Field `RTCRDYIE` writer - Real-time clock ready interrupt enable"]
pub type RtcrdyieW<'a, REG> = crate::BitWriter<'a, REG, Rtcrdyie>;
impl<'a, REG> RtcrdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtcrdyie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcrdyie::Rtcrdyie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn rtcrdyie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcrdyie::Rtcrdyie1)
    }
}
#[doc = "Real-time clock alarm interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcaie {
    #[doc = "0: Interrupt not enabled"]
    Rtcaie0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    Rtcaie1 = 1,
}
impl From<Rtcaie> for bool {
    #[inline(always)]
    fn from(variant: Rtcaie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCAIE` reader - Real-time clock alarm interrupt enable"]
pub type RtcaieR = crate::BitReader<Rtcaie>;
impl RtcaieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcaie {
        match self.bits {
            false => Rtcaie::Rtcaie0,
            true => Rtcaie::Rtcaie1,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_rtcaie_0(&self) -> bool {
        *self == Rtcaie::Rtcaie0
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn is_rtcaie_1(&self) -> bool {
        *self == Rtcaie::Rtcaie1
    }
}
#[doc = "Field `RTCAIE` writer - Real-time clock alarm interrupt enable"]
pub type RtcaieW<'a, REG> = crate::BitWriter<'a, REG, Rtcaie>;
impl<'a, REG> RtcaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtcaie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcaie::Rtcaie0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rtcaie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcaie::Rtcaie1)
    }
}
#[doc = "Real-time clock time event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtctevie {
    #[doc = "0: Interrupt not enabled"]
    Rtctevie0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    Rtctevie1 = 1,
}
impl From<Rtctevie> for bool {
    #[inline(always)]
    fn from(variant: Rtctevie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCTEVIE` reader - Real-time clock time event interrupt enable"]
pub type RtctevieR = crate::BitReader<Rtctevie>;
impl RtctevieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtctevie {
        match self.bits {
            false => Rtctevie::Rtctevie0,
            true => Rtctevie::Rtctevie1,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_rtctevie_0(&self) -> bool {
        *self == Rtctevie::Rtctevie0
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn is_rtctevie_1(&self) -> bool {
        *self == Rtctevie::Rtctevie1
    }
}
#[doc = "Field `RTCTEVIE` writer - Real-time clock time event interrupt enable"]
pub type RtctevieW<'a, REG> = crate::BitWriter<'a, REG, Rtctevie>;
impl<'a, REG> RtctevieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtctevie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctevie::Rtctevie0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rtctevie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctevie::Rtctevie1)
    }
}
#[doc = "32-kHz crystal oscillator fault interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcofie {
    #[doc = "0: Interrupt not enabled"]
    Rtcofie0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    Rtcofie1 = 1,
}
impl From<Rtcofie> for bool {
    #[inline(always)]
    fn from(variant: Rtcofie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCOFIE` reader - 32-kHz crystal oscillator fault interrupt enable"]
pub type RtcofieR = crate::BitReader<Rtcofie>;
impl RtcofieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcofie {
        match self.bits {
            false => Rtcofie::Rtcofie0,
            true => Rtcofie::Rtcofie1,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_rtcofie_0(&self) -> bool {
        *self == Rtcofie::Rtcofie0
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn is_rtcofie_1(&self) -> bool {
        *self == Rtcofie::Rtcofie1
    }
}
#[doc = "Field `RTCOFIE` writer - 32-kHz crystal oscillator fault interrupt enable"]
pub type RtcofieW<'a, REG> = crate::BitWriter<'a, REG, Rtcofie>;
impl<'a, REG> RtcofieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtcofie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcofie::Rtcofie0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rtcofie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcofie::Rtcofie1)
    }
}
#[doc = "Field `RTCKEY` reader - Real-time clock key"]
pub type RtckeyR = crate::FieldReader;
#[doc = "Field `RTCKEY` writer - Real-time clock key"]
pub type RtckeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Real-time clock ready interrupt flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&self) -> RtcrdyifgR {
        RtcrdyifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real-time clock alarm interrupt flag"]
    #[inline(always)]
    pub fn rtcaifg(&self) -> RtcaifgR {
        RtcaifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real-time clock time event interrupt flag"]
    #[inline(always)]
    pub fn rtctevifg(&self) -> RtctevifgR {
        RtctevifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 32-kHz crystal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&self) -> RtcofifgR {
        RtcofifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real-time clock ready interrupt enable"]
    #[inline(always)]
    pub fn rtcrdyie(&self) -> RtcrdyieR {
        RtcrdyieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real-time clock alarm interrupt enable"]
    #[inline(always)]
    pub fn rtcaie(&self) -> RtcaieR {
        RtcaieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real-time clock time event interrupt enable"]
    #[inline(always)]
    pub fn rtctevie(&self) -> RtctevieR {
        RtctevieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 32-kHz crystal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&self) -> RtcofieR {
        RtcofieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Real-time clock key"]
    #[inline(always)]
    pub fn rtckey(&self) -> RtckeyR {
        RtckeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Real-time clock ready interrupt flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&mut self) -> RtcrdyifgW<Rtcctl0Spec> {
        RtcrdyifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Real-time clock alarm interrupt flag"]
    #[inline(always)]
    pub fn rtcaifg(&mut self) -> RtcaifgW<Rtcctl0Spec> {
        RtcaifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Real-time clock time event interrupt flag"]
    #[inline(always)]
    pub fn rtctevifg(&mut self) -> RtctevifgW<Rtcctl0Spec> {
        RtctevifgW::new(self, 2)
    }
    #[doc = "Bit 3 - 32-kHz crystal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&mut self) -> RtcofifgW<Rtcctl0Spec> {
        RtcofifgW::new(self, 3)
    }
    #[doc = "Bit 4 - Real-time clock ready interrupt enable"]
    #[inline(always)]
    pub fn rtcrdyie(&mut self) -> RtcrdyieW<Rtcctl0Spec> {
        RtcrdyieW::new(self, 4)
    }
    #[doc = "Bit 5 - Real-time clock alarm interrupt enable"]
    #[inline(always)]
    pub fn rtcaie(&mut self) -> RtcaieW<Rtcctl0Spec> {
        RtcaieW::new(self, 5)
    }
    #[doc = "Bit 6 - Real-time clock time event interrupt enable"]
    #[inline(always)]
    pub fn rtctevie(&mut self) -> RtctevieW<Rtcctl0Spec> {
        RtctevieW::new(self, 6)
    }
    #[doc = "Bit 7 - 32-kHz crystal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&mut self) -> RtcofieW<Rtcctl0Spec> {
        RtcofieW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Real-time clock key"]
    #[inline(always)]
    pub fn rtckey(&mut self) -> RtckeyW<Rtcctl0Spec> {
        RtckeyW::new(self, 8)
    }
}
#[doc = "RTCCTL0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcctl0Spec;
impl crate::RegisterSpec for Rtcctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcctl0::R`](R) reader structure"]
impl crate::Readable for Rtcctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`rtcctl0::W`](W) writer structure"]
impl crate::Writable for Rtcctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCCTL0 to value 0x9608"]
impl crate::Resettable for Rtcctl0Spec {
    const RESET_VALUE: u16 = 0x9608;
}
