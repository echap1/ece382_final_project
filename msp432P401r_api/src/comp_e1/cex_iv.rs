#[doc = "Register `CExIV` reader"]
pub type R = crate::R<CexIvSpec>;
#[doc = "Comparator interrupt vector word register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CeivEnumRead {
    #[doc = "0: No interrupt pending"]
    Ceiv0 = 0,
    #[doc = "2: Interrupt Source: CEOUT interrupt; Interrupt Flag: CEIFG; Interrupt Priority: Highest"]
    Ceiv2 = 2,
    #[doc = "4: Interrupt Source: CEOUT interrupt inverted polarity; Interrupt Flag: CEIIFG"]
    Ceiv4 = 4,
    #[doc = "10: Interrupt Source: Comparator ready interrupt; Interrupt Flag: CERDYIFG; Interrupt Priority: Lowest"]
    Ceiv10 = 10,
}
impl From<CeivEnumRead> for u16 {
    #[inline(always)]
    fn from(variant: CeivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CeivEnumRead {
    type Ux = u16;
}
impl crate::IsEnum for CeivEnumRead {}
#[doc = "Field `CEIV` reader - Comparator interrupt vector word register"]
pub type CeivR = crate::FieldReader<CeivEnumRead>;
impl CeivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CeivEnumRead> {
        match self.bits {
            0 => Some(CeivEnumRead::Ceiv0),
            2 => Some(CeivEnumRead::Ceiv2),
            4 => Some(CeivEnumRead::Ceiv4),
            10 => Some(CeivEnumRead::Ceiv10),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ceiv_0(&self) -> bool {
        *self == CeivEnumRead::Ceiv0
    }
    #[doc = "Interrupt Source: CEOUT interrupt; Interrupt Flag: CEIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_ceiv_2(&self) -> bool {
        *self == CeivEnumRead::Ceiv2
    }
    #[doc = "Interrupt Source: CEOUT interrupt inverted polarity; Interrupt Flag: CEIIFG"]
    #[inline(always)]
    pub fn is_ceiv_4(&self) -> bool {
        *self == CeivEnumRead::Ceiv4
    }
    #[doc = "Interrupt Source: Comparator ready interrupt; Interrupt Flag: CERDYIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_ceiv_10(&self) -> bool {
        *self == CeivEnumRead::Ceiv10
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator interrupt vector word register"]
    #[inline(always)]
    pub fn ceiv(&self) -> CeivR {
        CeivR::new(self.bits)
    }
}
#[doc = "Comparator Interrupt Vector Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cex_iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CexIvSpec;
impl crate::RegisterSpec for CexIvSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cex_iv::R`](R) reader structure"]
impl crate::Readable for CexIvSpec {}
#[doc = "`reset()` method sets CExIV to value 0"]
impl crate::Resettable for CexIvSpec {
    const RESET_VALUE: u16 = 0;
}
