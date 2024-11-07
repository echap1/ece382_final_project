#[doc = "Register `P9IV` reader"]
pub type R = crate::R<P9ivSpec>;
#[doc = "Port 9 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P9ivEnumRead {
    #[doc = "0: No interrupt pending"]
    P9iv0 = 0,
    #[doc = "2: Interrupt Source: Port 9.0 interrupt; Interrupt Flag: P9IFG0; Interrupt Priority: Highest"]
    P9iv2 = 2,
    #[doc = "4: Interrupt Source: Port 9.1 interrupt; Interrupt Flag: P9IFG1"]
    P9iv4 = 4,
    #[doc = "6: Interrupt Source: Port 9.2 interrupt; Interrupt Flag: P9IFG2"]
    P9iv6 = 6,
    #[doc = "8: Interrupt Source: Port 9.3 interrupt; Interrupt Flag: P9IFG3"]
    P9iv8 = 8,
    #[doc = "10: Interrupt Source: Port 9.4 interrupt; Interrupt Flag: P9IFG4"]
    P9iv10 = 10,
    #[doc = "12: Interrupt Source: Port 9.5 interrupt; Interrupt Flag: P9IFG5"]
    P9iv12 = 12,
    #[doc = "14: Interrupt Source: Port 9.6 interrupt; Interrupt Flag: P9IFG6"]
    P9iv14 = 14,
    #[doc = "16: Interrupt Source: Port 9.7 interrupt; Interrupt Flag: P9IFG7; Interrupt Priority: Lowest"]
    P9iv16 = 16,
}
impl From<P9ivEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: P9ivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P9ivEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for P9ivEnumRead {}
#[doc = "Field `P9IV` reader - Port 9 interrupt vector value"]
pub type P9ivR = crate::FieldReader<P9ivEnumRead>;
impl P9ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P9ivEnumRead> {
        match self.bits {
            0 => Some(P9ivEnumRead::P9iv0),
            2 => Some(P9ivEnumRead::P9iv2),
            4 => Some(P9ivEnumRead::P9iv4),
            6 => Some(P9ivEnumRead::P9iv6),
            8 => Some(P9ivEnumRead::P9iv8),
            10 => Some(P9ivEnumRead::P9iv10),
            12 => Some(P9ivEnumRead::P9iv12),
            14 => Some(P9ivEnumRead::P9iv14),
            16 => Some(P9ivEnumRead::P9iv16),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_p9iv_0(&self) -> bool {
        *self == P9ivEnumRead::P9iv0
    }
    #[doc = "Interrupt Source: Port 9.0 interrupt; Interrupt Flag: P9IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p9iv_2(&self) -> bool {
        *self == P9ivEnumRead::P9iv2
    }
    #[doc = "Interrupt Source: Port 9.1 interrupt; Interrupt Flag: P9IFG1"]
    #[inline(always)]
    pub fn is_p9iv_4(&self) -> bool {
        *self == P9ivEnumRead::P9iv4
    }
    #[doc = "Interrupt Source: Port 9.2 interrupt; Interrupt Flag: P9IFG2"]
    #[inline(always)]
    pub fn is_p9iv_6(&self) -> bool {
        *self == P9ivEnumRead::P9iv6
    }
    #[doc = "Interrupt Source: Port 9.3 interrupt; Interrupt Flag: P9IFG3"]
    #[inline(always)]
    pub fn is_p9iv_8(&self) -> bool {
        *self == P9ivEnumRead::P9iv8
    }
    #[doc = "Interrupt Source: Port 9.4 interrupt; Interrupt Flag: P9IFG4"]
    #[inline(always)]
    pub fn is_p9iv_10(&self) -> bool {
        *self == P9ivEnumRead::P9iv10
    }
    #[doc = "Interrupt Source: Port 9.5 interrupt; Interrupt Flag: P9IFG5"]
    #[inline(always)]
    pub fn is_p9iv_12(&self) -> bool {
        *self == P9ivEnumRead::P9iv12
    }
    #[doc = "Interrupt Source: Port 9.6 interrupt; Interrupt Flag: P9IFG6"]
    #[inline(always)]
    pub fn is_p9iv_14(&self) -> bool {
        *self == P9ivEnumRead::P9iv14
    }
    #[doc = "Interrupt Source: Port 9.7 interrupt; Interrupt Flag: P9IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p9iv_16(&self) -> bool {
        *self == P9ivEnumRead::P9iv16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 9 interrupt vector value"]
    #[inline(always)]
    pub fn p9iv(&self) -> P9ivR {
        P9ivR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 9 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p9iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P9ivSpec;
impl crate::RegisterSpec for P9ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p9iv::R`](R) reader structure"]
impl crate::Readable for P9ivSpec {}
#[doc = "`reset()` method sets P9IV to value 0"]
impl crate::Resettable for P9ivSpec {
    const RESET_VALUE: u16 = 0;
}
