#[doc = "Register `RTCADOWDAY` reader"]
pub type R = crate::R<RtcadowdaySpec>;
#[doc = "Register `RTCADOWDAY` writer"]
pub type W = crate::W<RtcadowdaySpec>;
#[doc = "Field `DayofWeek` reader - Day of week (0 to 6)"]
pub type DayofWeekR = crate::FieldReader;
#[doc = "Field `DayofWeek` writer - Day of week (0 to 6)"]
pub type DayofWeekW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DOWAE` reader - Alarm enable"]
pub type DowaeR = crate::BitReader;
#[doc = "Field `DOWAE` writer - Alarm enable"]
pub type DowaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DayofMonth` reader - Day of month (1 to 28, 29, 30, 31)"]
pub type DayofMonthR = crate::FieldReader;
#[doc = "Field `DayofMonth` writer - Day of month (1 to 28, 29, 30, 31)"]
pub type DayofMonthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DAYAE` reader - Alarm enable"]
pub type DayaeR = crate::BitReader;
#[doc = "Field `DAYAE` writer - Alarm enable"]
pub type DayaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayof_week(&self) -> DayofWeekR {
        DayofWeekR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn dowae(&self) -> DowaeR {
        DowaeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn dayof_month(&self) -> DayofMonthR {
        DayofMonthR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn dayae(&self) -> DayaeR {
        DayaeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayof_week(&mut self) -> DayofWeekW<RtcadowdaySpec> {
        DayofWeekW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn dowae(&mut self) -> DowaeW<RtcadowdaySpec> {
        DowaeW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn dayof_month(&mut self) -> DayofMonthW<RtcadowdaySpec> {
        DayofMonthW::new(self, 8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn dayae(&mut self) -> DayaeW<RtcadowdaySpec> {
        DayaeW::new(self, 15)
    }
}
#[doc = "RTCADOWDAY - Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcadowday::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcadowday::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcadowdaySpec;
impl crate::RegisterSpec for RtcadowdaySpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcadowday::R`](R) reader structure"]
impl crate::Readable for RtcadowdaySpec {}
#[doc = "`write(|w| ..)` method takes [`rtcadowday::W`](W) writer structure"]
impl crate::Writable for RtcadowdaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCADOWDAY to value 0"]
impl crate::Resettable for RtcadowdaySpec {
    const RESET_VALUE: u16 = 0;
}
