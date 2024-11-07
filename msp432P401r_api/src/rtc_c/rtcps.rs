#[doc = "Register `RTCPS` reader"]
pub type R = crate::R<RtcpsSpec>;
#[doc = "Register `RTCPS` writer"]
pub type W = crate::W<RtcpsSpec>;
#[doc = "Field `RT0PS` reader - Prescale timer 0 counter value"]
pub type Rt0psR = crate::FieldReader;
#[doc = "Field `RT0PS` writer - Prescale timer 0 counter value"]
pub type Rt0psW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RT1PS` reader - Prescale timer 1 counter value"]
pub type Rt1psR = crate::FieldReader;
#[doc = "Field `RT1PS` writer - Prescale timer 1 counter value"]
pub type Rt1psW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Prescale timer 0 counter value"]
    #[inline(always)]
    pub fn rt0ps(&self) -> Rt0psR {
        Rt0psR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Prescale timer 1 counter value"]
    #[inline(always)]
    pub fn rt1ps(&self) -> Rt1psR {
        Rt1psR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescale timer 0 counter value"]
    #[inline(always)]
    pub fn rt0ps(&mut self) -> Rt0psW<RtcpsSpec> {
        Rt0psW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Prescale timer 1 counter value"]
    #[inline(always)]
    pub fn rt1ps(&mut self) -> Rt1psW<RtcpsSpec> {
        Rt1psW::new(self, 8)
    }
}
#[doc = "Real-Time Clock Prescale Timer Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcpsSpec;
impl crate::RegisterSpec for RtcpsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcps::R`](R) reader structure"]
impl crate::Readable for RtcpsSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcps::W`](W) writer structure"]
impl crate::Writable for RtcpsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCPS to value 0"]
impl crate::Resettable for RtcpsSpec {
    const RESET_VALUE: u16 = 0;
}
