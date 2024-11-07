#[doc = "Register `RTCOCAL` reader"]
pub type R = crate::R<RtcocalSpec>;
#[doc = "Register `RTCOCAL` writer"]
pub type W = crate::W<RtcocalSpec>;
#[doc = "Field `RTCOCAL` reader - Real-time clock offset error calibration"]
pub type RtcocalR = crate::FieldReader;
#[doc = "Field `RTCOCAL` writer - Real-time clock offset error calibration"]
pub type RtcocalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Real-time clock offset error calibration sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcocals {
    #[doc = "0: Down calibration. Frequency adjusted down."]
    Rtcocals0 = 0,
    #[doc = "1: Up calibration. Frequency adjusted up."]
    Rtcocals1 = 1,
}
impl From<Rtcocals> for bool {
    #[inline(always)]
    fn from(variant: Rtcocals) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCOCALS` reader - Real-time clock offset error calibration sign"]
pub type RtcocalsR = crate::BitReader<Rtcocals>;
impl RtcocalsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcocals {
        match self.bits {
            false => Rtcocals::Rtcocals0,
            true => Rtcocals::Rtcocals1,
        }
    }
    #[doc = "Down calibration. Frequency adjusted down."]
    #[inline(always)]
    pub fn is_rtcocals_0(&self) -> bool {
        *self == Rtcocals::Rtcocals0
    }
    #[doc = "Up calibration. Frequency adjusted up."]
    #[inline(always)]
    pub fn is_rtcocals_1(&self) -> bool {
        *self == Rtcocals::Rtcocals1
    }
}
#[doc = "Field `RTCOCALS` writer - Real-time clock offset error calibration sign"]
pub type RtcocalsW<'a, REG> = crate::BitWriter<'a, REG, Rtcocals>;
impl<'a, REG> RtcocalsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Down calibration. Frequency adjusted down."]
    #[inline(always)]
    pub fn rtcocals_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcocals::Rtcocals0)
    }
    #[doc = "Up calibration. Frequency adjusted up."]
    #[inline(always)]
    pub fn rtcocals_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcocals::Rtcocals1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration"]
    #[inline(always)]
    pub fn rtcocal(&self) -> RtcocalR {
        RtcocalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign"]
    #[inline(always)]
    pub fn rtcocals(&self) -> RtcocalsR {
        RtcocalsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration"]
    #[inline(always)]
    pub fn rtcocal(&mut self) -> RtcocalW<RtcocalSpec> {
        RtcocalW::new(self, 0)
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign"]
    #[inline(always)]
    pub fn rtcocals(&mut self) -> RtcocalsW<RtcocalSpec> {
        RtcocalsW::new(self, 15)
    }
}
#[doc = "RTCOCAL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcocal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcocal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcocalSpec;
impl crate::RegisterSpec for RtcocalSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcocal::R`](R) reader structure"]
impl crate::Readable for RtcocalSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcocal::W`](W) writer structure"]
impl crate::Writable for RtcocalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCOCAL to value 0"]
impl crate::Resettable for RtcocalSpec {
    const RESET_VALUE: u16 = 0;
}
