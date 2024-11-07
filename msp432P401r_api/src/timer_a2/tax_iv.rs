#[doc = "Register `TAxIV` reader"]
pub type R = crate::R<TaxIvSpec>;
#[doc = "TimerA interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TaivEnumRead {
    #[doc = "0: No interrupt pending"]
    Taiv0 = 0,
    #[doc = "2: Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest"]
    Taiv2 = 2,
    #[doc = "4: Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG"]
    Taiv4 = 4,
    #[doc = "6: Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG"]
    Taiv6 = 6,
    #[doc = "8: Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG"]
    Taiv8 = 8,
    #[doc = "10: Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG"]
    Taiv10 = 10,
    #[doc = "12: Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG"]
    Taiv12 = 12,
    #[doc = "14: Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest"]
    Taiv14 = 14,
}
impl From<TaivEnumRead> for u16 {
    #[inline(always)]
    fn from(variant: TaivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TaivEnumRead {
    type Ux = u16;
}
impl crate::IsEnum for TaivEnumRead {}
#[doc = "Field `TAIV` reader - TimerA interrupt vector value"]
pub type TaivR = crate::FieldReader<TaivEnumRead>;
impl TaivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TaivEnumRead> {
        match self.bits {
            0 => Some(TaivEnumRead::Taiv0),
            2 => Some(TaivEnumRead::Taiv2),
            4 => Some(TaivEnumRead::Taiv4),
            6 => Some(TaivEnumRead::Taiv6),
            8 => Some(TaivEnumRead::Taiv8),
            10 => Some(TaivEnumRead::Taiv10),
            12 => Some(TaivEnumRead::Taiv12),
            14 => Some(TaivEnumRead::Taiv14),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_taiv_0(&self) -> bool {
        *self == TaivEnumRead::Taiv0
    }
    #[doc = "Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_taiv_2(&self) -> bool {
        *self == TaivEnumRead::Taiv2
    }
    #[doc = "Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG"]
    #[inline(always)]
    pub fn is_taiv_4(&self) -> bool {
        *self == TaivEnumRead::Taiv4
    }
    #[doc = "Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG"]
    #[inline(always)]
    pub fn is_taiv_6(&self) -> bool {
        *self == TaivEnumRead::Taiv6
    }
    #[doc = "Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG"]
    #[inline(always)]
    pub fn is_taiv_8(&self) -> bool {
        *self == TaivEnumRead::Taiv8
    }
    #[doc = "Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG"]
    #[inline(always)]
    pub fn is_taiv_10(&self) -> bool {
        *self == TaivEnumRead::Taiv10
    }
    #[doc = "Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG"]
    #[inline(always)]
    pub fn is_taiv_12(&self) -> bool {
        *self == TaivEnumRead::Taiv12
    }
    #[doc = "Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_taiv_14(&self) -> bool {
        *self == TaivEnumRead::Taiv14
    }
}
impl R {
    #[doc = "Bits 0:15 - TimerA interrupt vector value"]
    #[inline(always)]
    pub fn taiv(&self) -> TaivR {
        TaivR::new(self.bits)
    }
}
#[doc = "TimerAx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaxIvSpec;
impl crate::RegisterSpec for TaxIvSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tax_iv::R`](R) reader structure"]
impl crate::Readable for TaxIvSpec {}
#[doc = "`reset()` method sets TAxIV to value 0"]
impl crate::Resettable for TaxIvSpec {
    const RESET_VALUE: u16 = 0;
}
