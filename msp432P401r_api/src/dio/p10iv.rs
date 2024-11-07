#[doc = "Register `P10IV` reader"]
pub type R = crate::R<P10ivSpec>;
#[doc = "Port 10 interrupt vector value"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P10ivEnumRead {
    #[doc = "0: No interrupt pending"]
    P10iv0 = 0,
    #[doc = "2: Interrupt Source: Port 10.0 interrupt; Interrupt Flag: P10IFG0; Interrupt Priority: Highest"]
    P10iv2 = 2,
    #[doc = "4: Interrupt Source: Port 10.1 interrupt; Interrupt Flag: P10IFG1"]
    P10iv4 = 4,
    #[doc = "6: Interrupt Source: Port 10.2 interrupt; Interrupt Flag: P10IFG2"]
    P10iv6 = 6,
    #[doc = "8: Interrupt Source: Port 10.3 interrupt; Interrupt Flag: P10IFG3"]
    P10iv8 = 8,
    #[doc = "10: Interrupt Source: Port 10.4 interrupt; Interrupt Flag: P10IFG4"]
    P10iv10 = 10,
    #[doc = "12: Interrupt Source: Port 10.5 interrupt; Interrupt Flag: P10IFG5"]
    P10iv12 = 12,
    #[doc = "14: Interrupt Source: Port 10.6 interrupt; Interrupt Flag: P10IFG6"]
    P10iv14 = 14,
    #[doc = "16: Interrupt Source: Port 10.7 interrupt; Interrupt Flag: P10IFG7; Interrupt Priority: Lowest"]
    P10iv16 = 16,
}
impl From<P10ivEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: P10ivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P10ivEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for P10ivEnumRead {}
#[doc = "Field `P10IV` reader - Port 10 interrupt vector value"]
pub type P10ivR = crate::FieldReader<P10ivEnumRead>;
impl P10ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P10ivEnumRead> {
        match self.bits {
            0 => Some(P10ivEnumRead::P10iv0),
            2 => Some(P10ivEnumRead::P10iv2),
            4 => Some(P10ivEnumRead::P10iv4),
            6 => Some(P10ivEnumRead::P10iv6),
            8 => Some(P10ivEnumRead::P10iv8),
            10 => Some(P10ivEnumRead::P10iv10),
            12 => Some(P10ivEnumRead::P10iv12),
            14 => Some(P10ivEnumRead::P10iv14),
            16 => Some(P10ivEnumRead::P10iv16),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_p10iv_0(&self) -> bool {
        *self == P10ivEnumRead::P10iv0
    }
    #[doc = "Interrupt Source: Port 10.0 interrupt; Interrupt Flag: P10IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p10iv_2(&self) -> bool {
        *self == P10ivEnumRead::P10iv2
    }
    #[doc = "Interrupt Source: Port 10.1 interrupt; Interrupt Flag: P10IFG1"]
    #[inline(always)]
    pub fn is_p10iv_4(&self) -> bool {
        *self == P10ivEnumRead::P10iv4
    }
    #[doc = "Interrupt Source: Port 10.2 interrupt; Interrupt Flag: P10IFG2"]
    #[inline(always)]
    pub fn is_p10iv_6(&self) -> bool {
        *self == P10ivEnumRead::P10iv6
    }
    #[doc = "Interrupt Source: Port 10.3 interrupt; Interrupt Flag: P10IFG3"]
    #[inline(always)]
    pub fn is_p10iv_8(&self) -> bool {
        *self == P10ivEnumRead::P10iv8
    }
    #[doc = "Interrupt Source: Port 10.4 interrupt; Interrupt Flag: P10IFG4"]
    #[inline(always)]
    pub fn is_p10iv_10(&self) -> bool {
        *self == P10ivEnumRead::P10iv10
    }
    #[doc = "Interrupt Source: Port 10.5 interrupt; Interrupt Flag: P10IFG5"]
    #[inline(always)]
    pub fn is_p10iv_12(&self) -> bool {
        *self == P10ivEnumRead::P10iv12
    }
    #[doc = "Interrupt Source: Port 10.6 interrupt; Interrupt Flag: P10IFG6"]
    #[inline(always)]
    pub fn is_p10iv_14(&self) -> bool {
        *self == P10ivEnumRead::P10iv14
    }
    #[doc = "Interrupt Source: Port 10.7 interrupt; Interrupt Flag: P10IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p10iv_16(&self) -> bool {
        *self == P10ivEnumRead::P10iv16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 10 interrupt vector value"]
    #[inline(always)]
    pub fn p10iv(&self) -> P10ivR {
        P10ivR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 10 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p10iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P10ivSpec;
impl crate::RegisterSpec for P10ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p10iv::R`](R) reader structure"]
impl crate::Readable for P10ivSpec {}
