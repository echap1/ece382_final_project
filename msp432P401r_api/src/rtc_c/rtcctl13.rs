#[doc = "Register `RTCCTL13` reader"]
pub type R = crate::R<Rtcctl13Spec>;
#[doc = "Register `RTCCTL13` writer"]
pub type W = crate::W<Rtcctl13Spec>;
#[doc = "Real-time clock time event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtctev {
    #[doc = "0: Minute changed"]
    Rtctev0 = 0,
    #[doc = "1: Hour changed"]
    Rtctev1 = 1,
    #[doc = "2: Every day at midnight (00:00)"]
    Rtctev2 = 2,
    #[doc = "3: Every day at noon (12:00)"]
    Rtctev3 = 3,
}
impl From<Rtctev> for u8 {
    #[inline(always)]
    fn from(variant: Rtctev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtctev {
    type Ux = u8;
}
impl crate::IsEnum for Rtctev {}
#[doc = "Field `RTCTEV` reader - Real-time clock time event"]
pub type RtctevR = crate::FieldReader<Rtctev>;
impl RtctevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtctev {
        match self.bits {
            0 => Rtctev::Rtctev0,
            1 => Rtctev::Rtctev1,
            2 => Rtctev::Rtctev2,
            3 => Rtctev::Rtctev3,
            _ => unreachable!(),
        }
    }
    #[doc = "Minute changed"]
    #[inline(always)]
    pub fn is_rtctev_0(&self) -> bool {
        *self == Rtctev::Rtctev0
    }
    #[doc = "Hour changed"]
    #[inline(always)]
    pub fn is_rtctev_1(&self) -> bool {
        *self == Rtctev::Rtctev1
    }
    #[doc = "Every day at midnight (00:00)"]
    #[inline(always)]
    pub fn is_rtctev_2(&self) -> bool {
        *self == Rtctev::Rtctev2
    }
    #[doc = "Every day at noon (12:00)"]
    #[inline(always)]
    pub fn is_rtctev_3(&self) -> bool {
        *self == Rtctev::Rtctev3
    }
}
#[doc = "Field `RTCTEV` writer - Real-time clock time event"]
pub type RtctevW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtctev, crate::Safe>;
impl<'a, REG> RtctevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minute changed"]
    #[inline(always)]
    pub fn rtctev_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::Rtctev0)
    }
    #[doc = "Hour changed"]
    #[inline(always)]
    pub fn rtctev_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::Rtctev1)
    }
    #[doc = "Every day at midnight (00:00)"]
    #[inline(always)]
    pub fn rtctev_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::Rtctev2)
    }
    #[doc = "Every day at noon (12:00)"]
    #[inline(always)]
    pub fn rtctev_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rtctev::Rtctev3)
    }
}
#[doc = "Real-time clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcssel {
    #[doc = "0: BCLK"]
    Rtcssel0 = 0,
}
impl From<Rtcssel> for u8 {
    #[inline(always)]
    fn from(variant: Rtcssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcssel {
    type Ux = u8;
}
impl crate::IsEnum for Rtcssel {}
#[doc = "Field `RTCSSEL` reader - Real-time clock source select"]
pub type RtcsselR = crate::FieldReader<Rtcssel>;
impl RtcsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rtcssel> {
        match self.bits {
            0 => Some(Rtcssel::Rtcssel0),
            _ => None,
        }
    }
    #[doc = "BCLK"]
    #[inline(always)]
    pub fn is_rtcssel_0(&self) -> bool {
        *self == Rtcssel::Rtcssel0
    }
}
#[doc = "Field `RTCSSEL` writer - Real-time clock source select"]
pub type RtcsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtcssel>;
impl<'a, REG> RtcsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BCLK"]
    #[inline(always)]
    pub fn rtcssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcssel::Rtcssel0)
    }
}
#[doc = "Real-time clock ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcrdyEnumRead {
    #[doc = "0: RTC time values in transition"]
    Rtcrdy0 = 0,
    #[doc = "1: RTC time values safe for reading. This bit indicates when the real-time clock time values are safe for reading."]
    Rtcrdy1 = 1,
}
impl From<RtcrdyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: RtcrdyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCRDY` reader - Real-time clock ready"]
pub type RtcrdyR = crate::BitReader<RtcrdyEnumRead>;
impl RtcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcrdyEnumRead {
        match self.bits {
            false => RtcrdyEnumRead::Rtcrdy0,
            true => RtcrdyEnumRead::Rtcrdy1,
        }
    }
    #[doc = "RTC time values in transition"]
    #[inline(always)]
    pub fn is_rtcrdy_0(&self) -> bool {
        *self == RtcrdyEnumRead::Rtcrdy0
    }
    #[doc = "RTC time values safe for reading. This bit indicates when the real-time clock time values are safe for reading."]
    #[inline(always)]
    pub fn is_rtcrdy_1(&self) -> bool {
        *self == RtcrdyEnumRead::Rtcrdy1
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcmodeEnumRead {
    #[doc = "1: Calendar mode. Always reads a value of 1."]
    Rtcmode1 = 1,
}
impl From<RtcmodeEnumRead> for bool {
    #[inline(always)]
    fn from(variant: RtcmodeEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCMODE` reader - "]
pub type RtcmodeR = crate::BitReader<RtcmodeEnumRead>;
impl RtcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RtcmodeEnumRead> {
        match self.bits {
            true => Some(RtcmodeEnumRead::Rtcmode1),
            _ => None,
        }
    }
    #[doc = "Calendar mode. Always reads a value of 1."]
    #[inline(always)]
    pub fn is_rtcmode_1(&self) -> bool {
        *self == RtcmodeEnumRead::Rtcmode1
    }
}
#[doc = "Real-time clock hold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtchold {
    #[doc = "0: Real-time clock is operational"]
    Rtchold0 = 0,
    #[doc = "1: When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care"]
    Rtchold1 = 1,
}
impl From<Rtchold> for bool {
    #[inline(always)]
    fn from(variant: Rtchold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCHOLD` reader - Real-time clock hold"]
pub type RtcholdR = crate::BitReader<Rtchold>;
impl RtcholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtchold {
        match self.bits {
            false => Rtchold::Rtchold0,
            true => Rtchold::Rtchold1,
        }
    }
    #[doc = "Real-time clock is operational"]
    #[inline(always)]
    pub fn is_rtchold_0(&self) -> bool {
        *self == Rtchold::Rtchold0
    }
    #[doc = "When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care"]
    #[inline(always)]
    pub fn is_rtchold_1(&self) -> bool {
        *self == Rtchold::Rtchold1
    }
}
#[doc = "Field `RTCHOLD` writer - Real-time clock hold"]
pub type RtcholdW<'a, REG> = crate::BitWriter<'a, REG, Rtchold>;
impl<'a, REG> RtcholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Real-time clock is operational"]
    #[inline(always)]
    pub fn rtchold_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtchold::Rtchold0)
    }
    #[doc = "When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care"]
    #[inline(always)]
    pub fn rtchold_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtchold::Rtchold1)
    }
}
#[doc = "Real-time clock BCD select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcbcd {
    #[doc = "0: Binary (hexadecimal) code selected"]
    Rtcbcd0 = 0,
    #[doc = "1: Binary coded decimal (BCD) code selected"]
    Rtcbcd1 = 1,
}
impl From<Rtcbcd> for bool {
    #[inline(always)]
    fn from(variant: Rtcbcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCBCD` reader - Real-time clock BCD select"]
pub type RtcbcdR = crate::BitReader<Rtcbcd>;
impl RtcbcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcbcd {
        match self.bits {
            false => Rtcbcd::Rtcbcd0,
            true => Rtcbcd::Rtcbcd1,
        }
    }
    #[doc = "Binary (hexadecimal) code selected"]
    #[inline(always)]
    pub fn is_rtcbcd_0(&self) -> bool {
        *self == Rtcbcd::Rtcbcd0
    }
    #[doc = "Binary coded decimal (BCD) code selected"]
    #[inline(always)]
    pub fn is_rtcbcd_1(&self) -> bool {
        *self == Rtcbcd::Rtcbcd1
    }
}
#[doc = "Field `RTCBCD` writer - Real-time clock BCD select"]
pub type RtcbcdW<'a, REG> = crate::BitWriter<'a, REG, Rtcbcd>;
impl<'a, REG> RtcbcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Binary (hexadecimal) code selected"]
    #[inline(always)]
    pub fn rtcbcd_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcbcd::Rtcbcd0)
    }
    #[doc = "Binary coded decimal (BCD) code selected"]
    #[inline(always)]
    pub fn rtcbcd_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcbcd::Rtcbcd1)
    }
}
#[doc = "Real-time clock calibration frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtccalf {
    #[doc = "0: No frequency output to RTCCLK pin"]
    Rtccalf0 = 0,
    #[doc = "1: 512 Hz"]
    Rtccalf1 = 1,
    #[doc = "2: 256 Hz"]
    Rtccalf2 = 2,
    #[doc = "3: 1 Hz"]
    Rtccalf3 = 3,
}
impl From<Rtccalf> for u8 {
    #[inline(always)]
    fn from(variant: Rtccalf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtccalf {
    type Ux = u8;
}
impl crate::IsEnum for Rtccalf {}
#[doc = "Field `RTCCALF` reader - Real-time clock calibration frequency"]
pub type RtccalfR = crate::FieldReader<Rtccalf>;
impl RtccalfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtccalf {
        match self.bits {
            0 => Rtccalf::Rtccalf0,
            1 => Rtccalf::Rtccalf1,
            2 => Rtccalf::Rtccalf2,
            3 => Rtccalf::Rtccalf3,
            _ => unreachable!(),
        }
    }
    #[doc = "No frequency output to RTCCLK pin"]
    #[inline(always)]
    pub fn is_rtccalf_0(&self) -> bool {
        *self == Rtccalf::Rtccalf0
    }
    #[doc = "512 Hz"]
    #[inline(always)]
    pub fn is_rtccalf_1(&self) -> bool {
        *self == Rtccalf::Rtccalf1
    }
    #[doc = "256 Hz"]
    #[inline(always)]
    pub fn is_rtccalf_2(&self) -> bool {
        *self == Rtccalf::Rtccalf2
    }
    #[doc = "1 Hz"]
    #[inline(always)]
    pub fn is_rtccalf_3(&self) -> bool {
        *self == Rtccalf::Rtccalf3
    }
}
#[doc = "Field `RTCCALF` writer - Real-time clock calibration frequency"]
pub type RtccalfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtccalf, crate::Safe>;
impl<'a, REG> RtccalfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No frequency output to RTCCLK pin"]
    #[inline(always)]
    pub fn rtccalf_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalf::Rtccalf0)
    }
    #[doc = "512 Hz"]
    #[inline(always)]
    pub fn rtccalf_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalf::Rtccalf1)
    }
    #[doc = "256 Hz"]
    #[inline(always)]
    pub fn rtccalf_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalf::Rtccalf2)
    }
    #[doc = "1 Hz"]
    #[inline(always)]
    pub fn rtccalf_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccalf::Rtccalf3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Real-time clock time event"]
    #[inline(always)]
    pub fn rtctev(&self) -> RtctevR {
        RtctevR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcssel(&self) -> RtcsselR {
        RtcsselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Real-time clock ready"]
    #[inline(always)]
    pub fn rtcrdy(&self) -> RtcrdyR {
        RtcrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtcmode(&self) -> RtcmodeR {
        RtcmodeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real-time clock hold"]
    #[inline(always)]
    pub fn rtchold(&self) -> RtcholdR {
        RtcholdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Real-time clock BCD select"]
    #[inline(always)]
    pub fn rtcbcd(&self) -> RtcbcdR {
        RtcbcdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Real-time clock calibration frequency"]
    #[inline(always)]
    pub fn rtccalf(&self) -> RtccalfR {
        RtccalfR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Real-time clock time event"]
    #[inline(always)]
    pub fn rtctev(&mut self) -> RtctevW<Rtcctl13Spec> {
        RtctevW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcssel(&mut self) -> RtcsselW<Rtcctl13Spec> {
        RtcsselW::new(self, 2)
    }
    #[doc = "Bit 6 - Real-time clock hold"]
    #[inline(always)]
    pub fn rtchold(&mut self) -> RtcholdW<Rtcctl13Spec> {
        RtcholdW::new(self, 6)
    }
    #[doc = "Bit 7 - Real-time clock BCD select"]
    #[inline(always)]
    pub fn rtcbcd(&mut self) -> RtcbcdW<Rtcctl13Spec> {
        RtcbcdW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Real-time clock calibration frequency"]
    #[inline(always)]
    pub fn rtccalf(&mut self) -> RtccalfW<Rtcctl13Spec> {
        RtccalfW::new(self, 8)
    }
}
#[doc = "RTCCTL13 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcctl13Spec;
impl crate::RegisterSpec for Rtcctl13Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcctl13::R`](R) reader structure"]
impl crate::Readable for Rtcctl13Spec {}
#[doc = "`write(|w| ..)` method takes [`rtcctl13::W`](W) writer structure"]
impl crate::Writable for Rtcctl13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCCTL13 to value 0x70"]
impl crate::Resettable for Rtcctl13Spec {
    const RESET_VALUE: u16 = 0x70;
}
