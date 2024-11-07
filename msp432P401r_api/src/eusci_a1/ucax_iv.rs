#[doc = "Register `UCAxIV` reader"]
pub type R = crate::R<UcaxIvSpec>;
#[doc = "eUSCI_A interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum UcivEnumRead {
    #[doc = "0: No interrupt pending"]
    Uciv0 = 0,
    #[doc = "2: Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    Uciv2 = 2,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    Uciv4 = 4,
    #[doc = "6: Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    Uciv6 = 6,
    #[doc = "8: Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    Uciv8 = 8,
}
impl From<UcivEnumRead> for u16 {
    #[inline(always)]
    fn from(variant: UcivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UcivEnumRead {
    type Ux = u16;
}
impl crate::IsEnum for UcivEnumRead {}
#[doc = "Field `UCIV` reader - eUSCI_A interrupt vector value"]
pub type UcivR = crate::FieldReader<UcivEnumRead>;
impl UcivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UcivEnumRead> {
        match self.bits {
            0 => Some(UcivEnumRead::Uciv0),
            2 => Some(UcivEnumRead::Uciv2),
            4 => Some(UcivEnumRead::Uciv4),
            6 => Some(UcivEnumRead::Uciv6),
            8 => Some(UcivEnumRead::Uciv8),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_uciv_0(&self) -> bool {
        *self == UcivEnumRead::Uciv0
    }
    #[doc = "Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_uciv_2(&self) -> bool {
        *self == UcivEnumRead::Uciv2
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    #[inline(always)]
    pub fn is_uciv_4(&self) -> bool {
        *self == UcivEnumRead::Uciv4
    }
    #[doc = "Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    #[inline(always)]
    pub fn is_uciv_6(&self) -> bool {
        *self == UcivEnumRead::Uciv6
    }
    #[doc = "Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_uciv_8(&self) -> bool {
        *self == UcivEnumRead::Uciv8
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_A interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UcivR {
        UcivR::new(self.bits)
    }
}
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcaxIvSpec;
impl crate::RegisterSpec for UcaxIvSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucax_iv::R`](R) reader structure"]
impl crate::Readable for UcaxIvSpec {}
#[doc = "`reset()` method sets UCAxIV to value 0"]
impl crate::Resettable for UcaxIvSpec {
    const RESET_VALUE: u16 = 0;
}
