#[doc = "Register `RTCPS0CTL` reader"]
pub type R = crate::R<Rtcps0ctlSpec>;
#[doc = "Register `RTCPS0CTL` writer"]
pub type W = crate::W<Rtcps0ctlSpec>;
#[doc = "Prescale timer 0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt0psifg {
    #[doc = "0: No time event occurred"]
    Rt0psifg0 = 0,
    #[doc = "1: Time event occurred"]
    Rt0psifg1 = 1,
}
impl From<Rt0psifg> for bool {
    #[inline(always)]
    fn from(variant: Rt0psifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT0PSIFG` reader - Prescale timer 0 interrupt flag"]
pub type Rt0psifgR = crate::BitReader<Rt0psifg>;
impl Rt0psifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt0psifg {
        match self.bits {
            false => Rt0psifg::Rt0psifg0,
            true => Rt0psifg::Rt0psifg1,
        }
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn is_rt0psifg_0(&self) -> bool {
        *self == Rt0psifg::Rt0psifg0
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn is_rt0psifg_1(&self) -> bool {
        *self == Rt0psifg::Rt0psifg1
    }
}
#[doc = "Field `RT0PSIFG` writer - Prescale timer 0 interrupt flag"]
pub type Rt0psifgW<'a, REG> = crate::BitWriter<'a, REG, Rt0psifg>;
impl<'a, REG> Rt0psifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rt0psifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psifg::Rt0psifg0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rt0psifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psifg::Rt0psifg1)
    }
}
#[doc = "Prescale timer 0 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt0psie {
    #[doc = "0: Interrupt not enabled"]
    Rt0psie0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    Rt0psie1 = 1,
}
impl From<Rt0psie> for bool {
    #[inline(always)]
    fn from(variant: Rt0psie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT0PSIE` reader - Prescale timer 0 interrupt enable"]
pub type Rt0psieR = crate::BitReader<Rt0psie>;
impl Rt0psieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt0psie {
        match self.bits {
            false => Rt0psie::Rt0psie0,
            true => Rt0psie::Rt0psie1,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_rt0psie_0(&self) -> bool {
        *self == Rt0psie::Rt0psie0
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn is_rt0psie_1(&self) -> bool {
        *self == Rt0psie::Rt0psie1
    }
}
#[doc = "Field `RT0PSIE` writer - Prescale timer 0 interrupt enable"]
pub type Rt0psieW<'a, REG> = crate::BitWriter<'a, REG, Rt0psie>;
impl<'a, REG> Rt0psieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rt0psie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psie::Rt0psie0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rt0psie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0psie::Rt0psie1)
    }
}
#[doc = "Prescale timer 0 interrupt interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rt0ip {
    #[doc = "0: Divide by 2"]
    Rt0ip0 = 0,
    #[doc = "1: Divide by 4"]
    Rt0ip1 = 1,
    #[doc = "2: Divide by 8"]
    Rt0ip2 = 2,
    #[doc = "3: Divide by 16"]
    Rt0ip3 = 3,
    #[doc = "4: Divide by 32"]
    Rt0ip4 = 4,
    #[doc = "5: Divide by 64"]
    Rt0ip5 = 5,
    #[doc = "6: Divide by 128"]
    Rt0ip6 = 6,
    #[doc = "7: Divide by 256"]
    Rt0ip7 = 7,
}
impl From<Rt0ip> for u8 {
    #[inline(always)]
    fn from(variant: Rt0ip) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rt0ip {
    type Ux = u8;
}
impl crate::IsEnum for Rt0ip {}
#[doc = "Field `RT0IP` reader - Prescale timer 0 interrupt interval"]
pub type Rt0ipR = crate::FieldReader<Rt0ip>;
impl Rt0ipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt0ip {
        match self.bits {
            0 => Rt0ip::Rt0ip0,
            1 => Rt0ip::Rt0ip1,
            2 => Rt0ip::Rt0ip2,
            3 => Rt0ip::Rt0ip3,
            4 => Rt0ip::Rt0ip4,
            5 => Rt0ip::Rt0ip5,
            6 => Rt0ip::Rt0ip6,
            7 => Rt0ip::Rt0ip7,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_rt0ip_0(&self) -> bool {
        *self == Rt0ip::Rt0ip0
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_rt0ip_1(&self) -> bool {
        *self == Rt0ip::Rt0ip1
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_rt0ip_2(&self) -> bool {
        *self == Rt0ip::Rt0ip2
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_rt0ip_3(&self) -> bool {
        *self == Rt0ip::Rt0ip3
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_rt0ip_4(&self) -> bool {
        *self == Rt0ip::Rt0ip4
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_rt0ip_5(&self) -> bool {
        *self == Rt0ip::Rt0ip5
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_rt0ip_6(&self) -> bool {
        *self == Rt0ip::Rt0ip6
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_rt0ip_7(&self) -> bool {
        *self == Rt0ip::Rt0ip7
    }
}
#[doc = "Field `RT0IP` writer - Prescale timer 0 interrupt interval"]
pub type Rt0ipW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rt0ip, crate::Safe>;
impl<'a, REG> Rt0ipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn rt0ip_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Rt0ip0)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn rt0ip_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Rt0ip1)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn rt0ip_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Rt0ip2)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn rt0ip_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Rt0ip3)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn rt0ip_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Rt0ip4)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn rt0ip_5(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Rt0ip5)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn rt0ip_6(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Rt0ip6)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn rt0ip_7(self) -> &'a mut crate::W<REG> {
        self.variant(Rt0ip::Rt0ip7)
    }
}
impl R {
    #[doc = "Bit 0 - Prescale timer 0 interrupt flag"]
    #[inline(always)]
    pub fn rt0psifg(&self) -> Rt0psifgR {
        Rt0psifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prescale timer 0 interrupt enable"]
    #[inline(always)]
    pub fn rt0psie(&self) -> Rt0psieR {
        Rt0psieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn rt0ip(&self) -> Rt0ipR {
        Rt0ipR::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Prescale timer 0 interrupt flag"]
    #[inline(always)]
    pub fn rt0psifg(&mut self) -> Rt0psifgW<Rtcps0ctlSpec> {
        Rt0psifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Prescale timer 0 interrupt enable"]
    #[inline(always)]
    pub fn rt0psie(&mut self) -> Rt0psieW<Rtcps0ctlSpec> {
        Rt0psieW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn rt0ip(&mut self) -> Rt0ipW<Rtcps0ctlSpec> {
        Rt0ipW::new(self, 2)
    }
}
#[doc = "Real-Time Clock Prescale Timer 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcps0ctlSpec;
impl crate::RegisterSpec for Rtcps0ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcps0ctl::R`](R) reader structure"]
impl crate::Readable for Rtcps0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcps0ctl::W`](W) writer structure"]
impl crate::Writable for Rtcps0ctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCPS0CTL to value 0"]
impl crate::Resettable for Rtcps0ctlSpec {
    const RESET_VALUE: u16 = 0;
}
