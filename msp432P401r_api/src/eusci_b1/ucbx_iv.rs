#[doc = "Register `UCBxIV` reader"]
pub type R = crate::R<UcbxIvSpec>;
#[doc = "eUSCI_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum UcivEnumRead {
    #[doc = "0: No interrupt pending"]
    Uciv0 = 0,
    #[doc = "2: Interrupt Source: Arbitration lost; Interrupt Flag: UCALIFG; Interrupt Priority: Highest"]
    Uciv2 = 2,
    #[doc = "4: Interrupt Source: Not acknowledgment; Interrupt Flag: UCNACKIFG"]
    Uciv4 = 4,
    #[doc = "6: Interrupt Source: Start condition received; Interrupt Flag: UCSTTIFG"]
    Uciv6 = 6,
    #[doc = "8: Interrupt Source: Stop condition received; Interrupt Flag: UCSTPIFG"]
    Uciv8 = 8,
    #[doc = "10: Interrupt Source: Slave 3 Data received; Interrupt Flag: UCRXIFG3"]
    Uciv10 = 10,
    #[doc = "12: Interrupt Source: Slave 3 Transmit buffer empty; Interrupt Flag: UCTXIFG3"]
    Uciv12 = 12,
    #[doc = "14: Interrupt Source: Slave 2 Data received; Interrupt Flag: UCRXIFG2"]
    Uciv14 = 14,
    #[doc = "16: Interrupt Source: Slave 2 Transmit buffer empty; Interrupt Flag: UCTXIFG2"]
    Uciv16 = 16,
    #[doc = "18: Interrupt Source: Slave 1 Data received; Interrupt Flag: UCRXIFG1"]
    Uciv18 = 18,
    #[doc = "20: Interrupt Source: Slave 1 Transmit buffer empty; Interrupt Flag: UCTXIFG1"]
    Uciv20 = 20,
    #[doc = "22: Interrupt Source: Data received; Interrupt Flag: UCRXIFG0"]
    Uciv22 = 22,
    #[doc = "24: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG0"]
    Uciv24 = 24,
    #[doc = "26: Interrupt Source: Byte counter zero; Interrupt Flag: UCBCNTIFG"]
    Uciv26 = 26,
    #[doc = "28: Interrupt Source: Clock low timeout; Interrupt Flag: UCCLTOIFG"]
    Uciv28 = 28,
    #[doc = "30: Interrupt Source: Nineth bit position; Interrupt Flag: UCBIT9IFG; Priority: Lowest"]
    Uciv30 = 30,
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
#[doc = "Field `UCIV` reader - eUSCI_B interrupt vector value"]
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
            10 => Some(UcivEnumRead::Uciv10),
            12 => Some(UcivEnumRead::Uciv12),
            14 => Some(UcivEnumRead::Uciv14),
            16 => Some(UcivEnumRead::Uciv16),
            18 => Some(UcivEnumRead::Uciv18),
            20 => Some(UcivEnumRead::Uciv20),
            22 => Some(UcivEnumRead::Uciv22),
            24 => Some(UcivEnumRead::Uciv24),
            26 => Some(UcivEnumRead::Uciv26),
            28 => Some(UcivEnumRead::Uciv28),
            30 => Some(UcivEnumRead::Uciv30),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_uciv_0(&self) -> bool {
        *self == UcivEnumRead::Uciv0
    }
    #[doc = "Interrupt Source: Arbitration lost; Interrupt Flag: UCALIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_uciv_2(&self) -> bool {
        *self == UcivEnumRead::Uciv2
    }
    #[doc = "Interrupt Source: Not acknowledgment; Interrupt Flag: UCNACKIFG"]
    #[inline(always)]
    pub fn is_uciv_4(&self) -> bool {
        *self == UcivEnumRead::Uciv4
    }
    #[doc = "Interrupt Source: Start condition received; Interrupt Flag: UCSTTIFG"]
    #[inline(always)]
    pub fn is_uciv_6(&self) -> bool {
        *self == UcivEnumRead::Uciv6
    }
    #[doc = "Interrupt Source: Stop condition received; Interrupt Flag: UCSTPIFG"]
    #[inline(always)]
    pub fn is_uciv_8(&self) -> bool {
        *self == UcivEnumRead::Uciv8
    }
    #[doc = "Interrupt Source: Slave 3 Data received; Interrupt Flag: UCRXIFG3"]
    #[inline(always)]
    pub fn is_uciv_10(&self) -> bool {
        *self == UcivEnumRead::Uciv10
    }
    #[doc = "Interrupt Source: Slave 3 Transmit buffer empty; Interrupt Flag: UCTXIFG3"]
    #[inline(always)]
    pub fn is_uciv_12(&self) -> bool {
        *self == UcivEnumRead::Uciv12
    }
    #[doc = "Interrupt Source: Slave 2 Data received; Interrupt Flag: UCRXIFG2"]
    #[inline(always)]
    pub fn is_uciv_14(&self) -> bool {
        *self == UcivEnumRead::Uciv14
    }
    #[doc = "Interrupt Source: Slave 2 Transmit buffer empty; Interrupt Flag: UCTXIFG2"]
    #[inline(always)]
    pub fn is_uciv_16(&self) -> bool {
        *self == UcivEnumRead::Uciv16
    }
    #[doc = "Interrupt Source: Slave 1 Data received; Interrupt Flag: UCRXIFG1"]
    #[inline(always)]
    pub fn is_uciv_18(&self) -> bool {
        *self == UcivEnumRead::Uciv18
    }
    #[doc = "Interrupt Source: Slave 1 Transmit buffer empty; Interrupt Flag: UCTXIFG1"]
    #[inline(always)]
    pub fn is_uciv_20(&self) -> bool {
        *self == UcivEnumRead::Uciv20
    }
    #[doc = "Interrupt Source: Data received; Interrupt Flag: UCRXIFG0"]
    #[inline(always)]
    pub fn is_uciv_22(&self) -> bool {
        *self == UcivEnumRead::Uciv22
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG0"]
    #[inline(always)]
    pub fn is_uciv_24(&self) -> bool {
        *self == UcivEnumRead::Uciv24
    }
    #[doc = "Interrupt Source: Byte counter zero; Interrupt Flag: UCBCNTIFG"]
    #[inline(always)]
    pub fn is_uciv_26(&self) -> bool {
        *self == UcivEnumRead::Uciv26
    }
    #[doc = "Interrupt Source: Clock low timeout; Interrupt Flag: UCCLTOIFG"]
    #[inline(always)]
    pub fn is_uciv_28(&self) -> bool {
        *self == UcivEnumRead::Uciv28
    }
    #[doc = "Interrupt Source: Nineth bit position; Interrupt Flag: UCBIT9IFG; Priority: Lowest"]
    #[inline(always)]
    pub fn is_uciv_30(&self) -> bool {
        *self == UcivEnumRead::Uciv30
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_B interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UcivR {
        UcivR::new(self.bits)
    }
}
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcbxIvSpec;
impl crate::RegisterSpec for UcbxIvSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucbx_iv::R`](R) reader structure"]
impl crate::Readable for UcbxIvSpec {}
#[doc = "`reset()` method sets UCBxIV to value 0"]
impl crate::Resettable for UcbxIvSpec {
    const RESET_VALUE: u16 = 0;
}
