#[doc = "Register `RTCAMINHR` reader"]
pub type R = crate::R<RtcaminhrSpec>;
#[doc = "Register `RTCAMINHR` writer"]
pub type W = crate::W<RtcaminhrSpec>;
#[doc = "Field `Minutes` reader - Minutes (0 to 59)"]
pub type MinutesR = crate::FieldReader;
#[doc = "Field `Minutes` writer - Minutes (0 to 59)"]
pub type MinutesW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MINAE` reader - Alarm enable"]
pub type MinaeR = crate::BitReader;
#[doc = "Field `MINAE` writer - Alarm enable"]
pub type MinaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Hours` reader - Hours (0 to 23)"]
pub type HoursR = crate::FieldReader;
#[doc = "Field `Hours` writer - Hours (0 to 23)"]
pub type HoursW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HOURAE` reader - Alarm enable"]
pub type HouraeR = crate::BitReader;
#[doc = "Field `HOURAE` writer - Alarm enable"]
pub type HouraeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&self) -> MinutesR {
        MinutesR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&self) -> MinaeR {
        MinaeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&self) -> HoursR {
        HoursR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&self) -> HouraeR {
        HouraeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&mut self) -> MinutesW<RtcaminhrSpec> {
        MinutesW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&mut self) -> MinaeW<RtcaminhrSpec> {
        MinaeW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&mut self) -> HoursW<RtcaminhrSpec> {
        HoursW::new(self, 8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&mut self) -> HouraeW<RtcaminhrSpec> {
        HouraeW::new(self, 15)
    }
}
#[doc = "RTCMINHR - Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcaminhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcaminhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcaminhrSpec;
impl crate::RegisterSpec for RtcaminhrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcaminhr::R`](R) reader structure"]
impl crate::Readable for RtcaminhrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcaminhr::W`](W) writer structure"]
impl crate::Writable for RtcaminhrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCAMINHR to value 0"]
impl crate::Resettable for RtcaminhrSpec {
    const RESET_VALUE: u16 = 0;
}
