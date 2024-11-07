#[doc = "Register `P5IV` reader"]
pub type R = crate::R<P5ivSpec>;
#[doc = "Port 5 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P5ivEnumRead {
    #[doc = "0: No interrupt pending"]
    P5iv0 = 0,
    #[doc = "2: Interrupt Source: Port 5.0 interrupt; Interrupt Flag: P5IFG0; Interrupt Priority: Highest"]
    P5iv2 = 2,
    #[doc = "4: Interrupt Source: Port 5.1 interrupt; Interrupt Flag: P5IFG1"]
    P5iv4 = 4,
    #[doc = "6: Interrupt Source: Port 5.2 interrupt; Interrupt Flag: P5IFG2"]
    P5iv6 = 6,
    #[doc = "8: Interrupt Source: Port 5.3 interrupt; Interrupt Flag: P5IFG3"]
    P5iv8 = 8,
    #[doc = "10: Interrupt Source: Port 5.4 interrupt; Interrupt Flag: P5IFG4"]
    P5iv10 = 10,
    #[doc = "12: Interrupt Source: Port 5.5 interrupt; Interrupt Flag: P5IFG5"]
    P5iv12 = 12,
    #[doc = "14: Interrupt Source: Port 5.6 interrupt; Interrupt Flag: P5IFG6"]
    P5iv14 = 14,
    #[doc = "16: Interrupt Source: Port 5.7 interrupt; Interrupt Flag: P5IFG7; Interrupt Priority: Lowest"]
    P5iv16 = 16,
}
impl From<P5ivEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: P5ivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P5ivEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for P5ivEnumRead {}
#[doc = "Field `P5IV` reader - Port 5 interrupt vector value"]
pub type P5ivR = crate::FieldReader<P5ivEnumRead>;
impl P5ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P5ivEnumRead> {
        match self.bits {
            0 => Some(P5ivEnumRead::P5iv0),
            2 => Some(P5ivEnumRead::P5iv2),
            4 => Some(P5ivEnumRead::P5iv4),
            6 => Some(P5ivEnumRead::P5iv6),
            8 => Some(P5ivEnumRead::P5iv8),
            10 => Some(P5ivEnumRead::P5iv10),
            12 => Some(P5ivEnumRead::P5iv12),
            14 => Some(P5ivEnumRead::P5iv14),
            16 => Some(P5ivEnumRead::P5iv16),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_p5iv_0(&self) -> bool {
        *self == P5ivEnumRead::P5iv0
    }
    #[doc = "Interrupt Source: Port 5.0 interrupt; Interrupt Flag: P5IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p5iv_2(&self) -> bool {
        *self == P5ivEnumRead::P5iv2
    }
    #[doc = "Interrupt Source: Port 5.1 interrupt; Interrupt Flag: P5IFG1"]
    #[inline(always)]
    pub fn is_p5iv_4(&self) -> bool {
        *self == P5ivEnumRead::P5iv4
    }
    #[doc = "Interrupt Source: Port 5.2 interrupt; Interrupt Flag: P5IFG2"]
    #[inline(always)]
    pub fn is_p5iv_6(&self) -> bool {
        *self == P5ivEnumRead::P5iv6
    }
    #[doc = "Interrupt Source: Port 5.3 interrupt; Interrupt Flag: P5IFG3"]
    #[inline(always)]
    pub fn is_p5iv_8(&self) -> bool {
        *self == P5ivEnumRead::P5iv8
    }
    #[doc = "Interrupt Source: Port 5.4 interrupt; Interrupt Flag: P5IFG4"]
    #[inline(always)]
    pub fn is_p5iv_10(&self) -> bool {
        *self == P5ivEnumRead::P5iv10
    }
    #[doc = "Interrupt Source: Port 5.5 interrupt; Interrupt Flag: P5IFG5"]
    #[inline(always)]
    pub fn is_p5iv_12(&self) -> bool {
        *self == P5ivEnumRead::P5iv12
    }
    #[doc = "Interrupt Source: Port 5.6 interrupt; Interrupt Flag: P5IFG6"]
    #[inline(always)]
    pub fn is_p5iv_14(&self) -> bool {
        *self == P5ivEnumRead::P5iv14
    }
    #[doc = "Interrupt Source: Port 5.7 interrupt; Interrupt Flag: P5IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p5iv_16(&self) -> bool {
        *self == P5ivEnumRead::P5iv16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 5 interrupt vector value"]
    #[inline(always)]
    pub fn p5iv(&self) -> P5ivR {
        P5ivR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 5 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p5iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5ivSpec;
impl crate::RegisterSpec for P5ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p5iv::R`](R) reader structure"]
impl crate::Readable for P5ivSpec {}
#[doc = "`reset()` method sets P5IV to value 0"]
impl crate::Resettable for P5ivSpec {
    const RESET_VALUE: u16 = 0;
}
