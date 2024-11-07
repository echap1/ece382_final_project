#[doc = "Register `P1IV` reader"]
pub type R = crate::R<P1ivSpec>;
#[doc = "Port 1 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1ivEnumRead {
    #[doc = "0: No interrupt pending"]
    P1iv0 = 0,
    #[doc = "2: Interrupt Source: Port 1.0 interrupt; Interrupt Flag: P1IFG0; Interrupt Priority: Highest"]
    P1iv2 = 2,
    #[doc = "4: Interrupt Source: Port 1.1 interrupt; Interrupt Flag: P1IFG1"]
    P1iv4 = 4,
    #[doc = "6: Interrupt Source: Port 1.2 interrupt; Interrupt Flag: P1IFG2"]
    P1iv6 = 6,
    #[doc = "8: Interrupt Source: Port 1.3 interrupt; Interrupt Flag: P1IFG3"]
    P1iv8 = 8,
    #[doc = "10: Interrupt Source: Port 1.4 interrupt; Interrupt Flag: P1IFG4"]
    P1iv10 = 10,
    #[doc = "12: Interrupt Source: Port 1.5 interrupt; Interrupt Flag: P1IFG5"]
    P1iv12 = 12,
    #[doc = "14: Interrupt Source: Port 1.6 interrupt; Interrupt Flag: P1IFG6"]
    P1iv14 = 14,
    #[doc = "16: Interrupt Source: Port 1.7 interrupt; Interrupt Flag: P1IFG7; Interrupt Priority: Lowest"]
    P1iv16 = 16,
}
impl From<P1ivEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: P1ivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1ivEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for P1ivEnumRead {}
#[doc = "Field `P1IV` reader - Port 1 interrupt vector value"]
pub type P1ivR = crate::FieldReader<P1ivEnumRead>;
impl P1ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P1ivEnumRead> {
        match self.bits {
            0 => Some(P1ivEnumRead::P1iv0),
            2 => Some(P1ivEnumRead::P1iv2),
            4 => Some(P1ivEnumRead::P1iv4),
            6 => Some(P1ivEnumRead::P1iv6),
            8 => Some(P1ivEnumRead::P1iv8),
            10 => Some(P1ivEnumRead::P1iv10),
            12 => Some(P1ivEnumRead::P1iv12),
            14 => Some(P1ivEnumRead::P1iv14),
            16 => Some(P1ivEnumRead::P1iv16),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_p1iv_0(&self) -> bool {
        *self == P1ivEnumRead::P1iv0
    }
    #[doc = "Interrupt Source: Port 1.0 interrupt; Interrupt Flag: P1IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p1iv_2(&self) -> bool {
        *self == P1ivEnumRead::P1iv2
    }
    #[doc = "Interrupt Source: Port 1.1 interrupt; Interrupt Flag: P1IFG1"]
    #[inline(always)]
    pub fn is_p1iv_4(&self) -> bool {
        *self == P1ivEnumRead::P1iv4
    }
    #[doc = "Interrupt Source: Port 1.2 interrupt; Interrupt Flag: P1IFG2"]
    #[inline(always)]
    pub fn is_p1iv_6(&self) -> bool {
        *self == P1ivEnumRead::P1iv6
    }
    #[doc = "Interrupt Source: Port 1.3 interrupt; Interrupt Flag: P1IFG3"]
    #[inline(always)]
    pub fn is_p1iv_8(&self) -> bool {
        *self == P1ivEnumRead::P1iv8
    }
    #[doc = "Interrupt Source: Port 1.4 interrupt; Interrupt Flag: P1IFG4"]
    #[inline(always)]
    pub fn is_p1iv_10(&self) -> bool {
        *self == P1ivEnumRead::P1iv10
    }
    #[doc = "Interrupt Source: Port 1.5 interrupt; Interrupt Flag: P1IFG5"]
    #[inline(always)]
    pub fn is_p1iv_12(&self) -> bool {
        *self == P1ivEnumRead::P1iv12
    }
    #[doc = "Interrupt Source: Port 1.6 interrupt; Interrupt Flag: P1IFG6"]
    #[inline(always)]
    pub fn is_p1iv_14(&self) -> bool {
        *self == P1ivEnumRead::P1iv14
    }
    #[doc = "Interrupt Source: Port 1.7 interrupt; Interrupt Flag: P1IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p1iv_16(&self) -> bool {
        *self == P1ivEnumRead::P1iv16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 1 interrupt vector value"]
    #[inline(always)]
    pub fn p1iv(&self) -> P1ivR {
        P1ivR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 1 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1ivSpec;
impl crate::RegisterSpec for P1ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p1iv::R`](R) reader structure"]
impl crate::Readable for P1ivSpec {}
#[doc = "`reset()` method sets P1IV to value 0"]
impl crate::Resettable for P1ivSpec {
    const RESET_VALUE: u16 = 0;
}
