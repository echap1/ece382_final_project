#[doc = "Register `RTCIV` reader"]
pub type R = crate::R<RtcivSpec>;
#[doc = "Real-time clock interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum RtcivEnumRead {
    #[doc = "0: No interrupt pending"]
    Rtciv0 = 0,
    #[doc = "2: Interrupt Source: RTC oscillator failure; Interrupt Flag: RTCOFIFG; Interrupt Priority: Highest"]
    Rtciv2 = 2,
    #[doc = "4: Interrupt Source: RTC ready; Interrupt Flag: RTCRDYIFG"]
    Rtciv4 = 4,
    #[doc = "6: Interrupt Source: RTC interval timer; Interrupt Flag: RTCTEVIFG"]
    Rtciv6 = 6,
    #[doc = "8: Interrupt Source: RTC user alarm; Interrupt Flag: RTCAIFG"]
    Rtciv8 = 8,
    #[doc = "10: Interrupt Source: RTC prescaler 0; Interrupt Flag: RT0PSIFG"]
    Rtciv10 = 10,
    #[doc = "12: Interrupt Source: RTC prescaler 1; Interrupt Flag: RT1PSIFG"]
    Rtciv12 = 12,
}
impl From<RtcivEnumRead> for u16 {
    #[inline(always)]
    fn from(variant: RtcivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RtcivEnumRead {
    type Ux = u16;
}
impl crate::IsEnum for RtcivEnumRead {}
#[doc = "Field `RTCIV` reader - Real-time clock interrupt vector value"]
pub type RtcivR = crate::FieldReader<RtcivEnumRead>;
impl RtcivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RtcivEnumRead> {
        match self.bits {
            0 => Some(RtcivEnumRead::Rtciv0),
            2 => Some(RtcivEnumRead::Rtciv2),
            4 => Some(RtcivEnumRead::Rtciv4),
            6 => Some(RtcivEnumRead::Rtciv6),
            8 => Some(RtcivEnumRead::Rtciv8),
            10 => Some(RtcivEnumRead::Rtciv10),
            12 => Some(RtcivEnumRead::Rtciv12),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_rtciv_0(&self) -> bool {
        *self == RtcivEnumRead::Rtciv0
    }
    #[doc = "Interrupt Source: RTC oscillator failure; Interrupt Flag: RTCOFIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_rtciv_2(&self) -> bool {
        *self == RtcivEnumRead::Rtciv2
    }
    #[doc = "Interrupt Source: RTC ready; Interrupt Flag: RTCRDYIFG"]
    #[inline(always)]
    pub fn is_rtciv_4(&self) -> bool {
        *self == RtcivEnumRead::Rtciv4
    }
    #[doc = "Interrupt Source: RTC interval timer; Interrupt Flag: RTCTEVIFG"]
    #[inline(always)]
    pub fn is_rtciv_6(&self) -> bool {
        *self == RtcivEnumRead::Rtciv6
    }
    #[doc = "Interrupt Source: RTC user alarm; Interrupt Flag: RTCAIFG"]
    #[inline(always)]
    pub fn is_rtciv_8(&self) -> bool {
        *self == RtcivEnumRead::Rtciv8
    }
    #[doc = "Interrupt Source: RTC prescaler 0; Interrupt Flag: RT0PSIFG"]
    #[inline(always)]
    pub fn is_rtciv_10(&self) -> bool {
        *self == RtcivEnumRead::Rtciv10
    }
    #[doc = "Interrupt Source: RTC prescaler 1; Interrupt Flag: RT1PSIFG"]
    #[inline(always)]
    pub fn is_rtciv_12(&self) -> bool {
        *self == RtcivEnumRead::Rtciv12
    }
}
impl R {
    #[doc = "Bits 0:15 - Real-time clock interrupt vector value"]
    #[inline(always)]
    pub fn rtciv(&self) -> RtcivR {
        RtcivR::new(self.bits)
    }
}
#[doc = "Real-Time Clock Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtciv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcivSpec;
impl crate::RegisterSpec for RtcivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtciv::R`](R) reader structure"]
impl crate::Readable for RtcivSpec {}
#[doc = "`reset()` method sets RTCIV to value 0"]
impl crate::Resettable for RtcivSpec {
    const RESET_VALUE: u16 = 0;
}
