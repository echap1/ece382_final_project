#[doc = "Register `RTCPS1CTL` reader"]
pub type R = crate::R<Rtcps1ctlSpec>;
#[doc = "Register `RTCPS1CTL` writer"]
pub type W = crate::W<Rtcps1ctlSpec>;
#[doc = "Prescale timer 1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt1psifg {
    #[doc = "0: No time event occurred"]
    Rt1psifg0 = 0,
    #[doc = "1: Time event occurred"]
    Rt1psifg1 = 1,
}
impl From<Rt1psifg> for bool {
    #[inline(always)]
    fn from(variant: Rt1psifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT1PSIFG` reader - Prescale timer 1 interrupt flag"]
pub type Rt1psifgR = crate::BitReader<Rt1psifg>;
impl Rt1psifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1psifg {
        match self.bits {
            false => Rt1psifg::Rt1psifg0,
            true => Rt1psifg::Rt1psifg1,
        }
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn is_rt1psifg_0(&self) -> bool {
        *self == Rt1psifg::Rt1psifg0
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn is_rt1psifg_1(&self) -> bool {
        *self == Rt1psifg::Rt1psifg1
    }
}
#[doc = "Field `RT1PSIFG` writer - Prescale timer 1 interrupt flag"]
pub type Rt1psifgW<'a, REG> = crate::BitWriter<'a, REG, Rt1psifg>;
impl<'a, REG> Rt1psifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rt1psifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psifg::Rt1psifg0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rt1psifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psifg::Rt1psifg1)
    }
}
#[doc = "Prescale timer 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rt1psie {
    #[doc = "0: Interrupt not enabled"]
    Rt1psie0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    Rt1psie1 = 1,
}
impl From<Rt1psie> for bool {
    #[inline(always)]
    fn from(variant: Rt1psie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT1PSIE` reader - Prescale timer 1 interrupt enable"]
pub type Rt1psieR = crate::BitReader<Rt1psie>;
impl Rt1psieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1psie {
        match self.bits {
            false => Rt1psie::Rt1psie0,
            true => Rt1psie::Rt1psie1,
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn is_rt1psie_0(&self) -> bool {
        *self == Rt1psie::Rt1psie0
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn is_rt1psie_1(&self) -> bool {
        *self == Rt1psie::Rt1psie1
    }
}
#[doc = "Field `RT1PSIE` writer - Prescale timer 1 interrupt enable"]
pub type Rt1psieW<'a, REG> = crate::BitWriter<'a, REG, Rt1psie>;
impl<'a, REG> Rt1psieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rt1psie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psie::Rt1psie0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rt1psie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1psie::Rt1psie1)
    }
}
#[doc = "Prescale timer 1 interrupt interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rt1ip {
    #[doc = "0: Divide by 2"]
    Rt1ip0 = 0,
    #[doc = "1: Divide by 4"]
    Rt1ip1 = 1,
    #[doc = "2: Divide by 8"]
    Rt1ip2 = 2,
    #[doc = "3: Divide by 16"]
    Rt1ip3 = 3,
    #[doc = "4: Divide by 32"]
    Rt1ip4 = 4,
    #[doc = "5: Divide by 64"]
    Rt1ip5 = 5,
    #[doc = "6: Divide by 128"]
    Rt1ip6 = 6,
    #[doc = "7: Divide by 256"]
    Rt1ip7 = 7,
}
impl From<Rt1ip> for u8 {
    #[inline(always)]
    fn from(variant: Rt1ip) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rt1ip {
    type Ux = u8;
}
impl crate::IsEnum for Rt1ip {}
#[doc = "Field `RT1IP` reader - Prescale timer 1 interrupt interval"]
pub type Rt1ipR = crate::FieldReader<Rt1ip>;
impl Rt1ipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rt1ip {
        match self.bits {
            0 => Rt1ip::Rt1ip0,
            1 => Rt1ip::Rt1ip1,
            2 => Rt1ip::Rt1ip2,
            3 => Rt1ip::Rt1ip3,
            4 => Rt1ip::Rt1ip4,
            5 => Rt1ip::Rt1ip5,
            6 => Rt1ip::Rt1ip6,
            7 => Rt1ip::Rt1ip7,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_rt1ip_0(&self) -> bool {
        *self == Rt1ip::Rt1ip0
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_rt1ip_1(&self) -> bool {
        *self == Rt1ip::Rt1ip1
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_rt1ip_2(&self) -> bool {
        *self == Rt1ip::Rt1ip2
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_rt1ip_3(&self) -> bool {
        *self == Rt1ip::Rt1ip3
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_rt1ip_4(&self) -> bool {
        *self == Rt1ip::Rt1ip4
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_rt1ip_5(&self) -> bool {
        *self == Rt1ip::Rt1ip5
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_rt1ip_6(&self) -> bool {
        *self == Rt1ip::Rt1ip6
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_rt1ip_7(&self) -> bool {
        *self == Rt1ip::Rt1ip7
    }
}
#[doc = "Field `RT1IP` writer - Prescale timer 1 interrupt interval"]
pub type Rt1ipW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rt1ip, crate::Safe>;
impl<'a, REG> Rt1ipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn rt1ip_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Rt1ip0)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn rt1ip_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Rt1ip1)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn rt1ip_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Rt1ip2)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn rt1ip_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Rt1ip3)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn rt1ip_4(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Rt1ip4)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn rt1ip_5(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Rt1ip5)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn rt1ip_6(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Rt1ip6)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn rt1ip_7(self) -> &'a mut crate::W<REG> {
        self.variant(Rt1ip::Rt1ip7)
    }
}
impl R {
    #[doc = "Bit 0 - Prescale timer 1 interrupt flag"]
    #[inline(always)]
    pub fn rt1psifg(&self) -> Rt1psifgR {
        Rt1psifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prescale timer 1 interrupt enable"]
    #[inline(always)]
    pub fn rt1psie(&self) -> Rt1psieR {
        Rt1psieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn rt1ip(&self) -> Rt1ipR {
        Rt1ipR::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Prescale timer 1 interrupt flag"]
    #[inline(always)]
    pub fn rt1psifg(&mut self) -> Rt1psifgW<Rtcps1ctlSpec> {
        Rt1psifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Prescale timer 1 interrupt enable"]
    #[inline(always)]
    pub fn rt1psie(&mut self) -> Rt1psieW<Rtcps1ctlSpec> {
        Rt1psieW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn rt1ip(&mut self) -> Rt1ipW<Rtcps1ctlSpec> {
        Rt1ipW::new(self, 2)
    }
}
#[doc = "Real-Time Clock Prescale Timer 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps1ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps1ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcps1ctlSpec;
impl crate::RegisterSpec for Rtcps1ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcps1ctl::R`](R) reader structure"]
impl crate::Readable for Rtcps1ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcps1ctl::W`](W) writer structure"]
impl crate::Writable for Rtcps1ctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCPS1CTL to value 0"]
impl crate::Resettable for Rtcps1ctlSpec {
    const RESET_VALUE: u16 = 0;
}
