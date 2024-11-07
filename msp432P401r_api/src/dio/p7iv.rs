#[doc = "Register `P7IV` reader"]
pub type R = crate::R<P7ivSpec>;
#[doc = "Port 7 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P7ivEnumRead {
    #[doc = "0: No interrupt pending"]
    P7iv0 = 0,
    #[doc = "2: Interrupt Source: Port 7.0 interrupt; Interrupt Flag: P7IFG0; Interrupt Priority: Highest"]
    P7iv2 = 2,
    #[doc = "4: Interrupt Source: Port 7.1 interrupt; Interrupt Flag: P7IFG1"]
    P7iv4 = 4,
    #[doc = "6: Interrupt Source: Port 7.2 interrupt; Interrupt Flag: P7IFG2"]
    P7iv6 = 6,
    #[doc = "8: Interrupt Source: Port 7.3 interrupt; Interrupt Flag: P7IFG3"]
    P7iv8 = 8,
    #[doc = "10: Interrupt Source: Port 7.4 interrupt; Interrupt Flag: P7IFG4"]
    P7iv10 = 10,
    #[doc = "12: Interrupt Source: Port 7.5 interrupt; Interrupt Flag: P7IFG5"]
    P7iv12 = 12,
    #[doc = "14: Interrupt Source: Port 7.6 interrupt; Interrupt Flag: P7IFG6"]
    P7iv14 = 14,
    #[doc = "16: Interrupt Source: Port 7.7 interrupt; Interrupt Flag: P7IFG7; Interrupt Priority: Lowest"]
    P7iv16 = 16,
}
impl From<P7ivEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: P7ivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P7ivEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for P7ivEnumRead {}
#[doc = "Field `P7IV` reader - Port 7 interrupt vector value"]
pub type P7ivR = crate::FieldReader<P7ivEnumRead>;
impl P7ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P7ivEnumRead> {
        match self.bits {
            0 => Some(P7ivEnumRead::P7iv0),
            2 => Some(P7ivEnumRead::P7iv2),
            4 => Some(P7ivEnumRead::P7iv4),
            6 => Some(P7ivEnumRead::P7iv6),
            8 => Some(P7ivEnumRead::P7iv8),
            10 => Some(P7ivEnumRead::P7iv10),
            12 => Some(P7ivEnumRead::P7iv12),
            14 => Some(P7ivEnumRead::P7iv14),
            16 => Some(P7ivEnumRead::P7iv16),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_p7iv_0(&self) -> bool {
        *self == P7ivEnumRead::P7iv0
    }
    #[doc = "Interrupt Source: Port 7.0 interrupt; Interrupt Flag: P7IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p7iv_2(&self) -> bool {
        *self == P7ivEnumRead::P7iv2
    }
    #[doc = "Interrupt Source: Port 7.1 interrupt; Interrupt Flag: P7IFG1"]
    #[inline(always)]
    pub fn is_p7iv_4(&self) -> bool {
        *self == P7ivEnumRead::P7iv4
    }
    #[doc = "Interrupt Source: Port 7.2 interrupt; Interrupt Flag: P7IFG2"]
    #[inline(always)]
    pub fn is_p7iv_6(&self) -> bool {
        *self == P7ivEnumRead::P7iv6
    }
    #[doc = "Interrupt Source: Port 7.3 interrupt; Interrupt Flag: P7IFG3"]
    #[inline(always)]
    pub fn is_p7iv_8(&self) -> bool {
        *self == P7ivEnumRead::P7iv8
    }
    #[doc = "Interrupt Source: Port 7.4 interrupt; Interrupt Flag: P7IFG4"]
    #[inline(always)]
    pub fn is_p7iv_10(&self) -> bool {
        *self == P7ivEnumRead::P7iv10
    }
    #[doc = "Interrupt Source: Port 7.5 interrupt; Interrupt Flag: P7IFG5"]
    #[inline(always)]
    pub fn is_p7iv_12(&self) -> bool {
        *self == P7ivEnumRead::P7iv12
    }
    #[doc = "Interrupt Source: Port 7.6 interrupt; Interrupt Flag: P7IFG6"]
    #[inline(always)]
    pub fn is_p7iv_14(&self) -> bool {
        *self == P7ivEnumRead::P7iv14
    }
    #[doc = "Interrupt Source: Port 7.7 interrupt; Interrupt Flag: P7IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p7iv_16(&self) -> bool {
        *self == P7ivEnumRead::P7iv16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 7 interrupt vector value"]
    #[inline(always)]
    pub fn p7iv(&self) -> P7ivR {
        P7ivR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 7 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p7iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7ivSpec;
impl crate::RegisterSpec for P7ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p7iv::R`](R) reader structure"]
impl crate::Readable for P7ivSpec {}
#[doc = "`reset()` method sets P7IV to value 0"]
impl crate::Resettable for P7ivSpec {
    const RESET_VALUE: u16 = 0;
}
