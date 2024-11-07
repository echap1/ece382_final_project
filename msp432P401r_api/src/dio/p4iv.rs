#[doc = "Register `P4IV` reader"]
pub type R = crate::R<P4ivSpec>;
#[doc = "Port 4 interrupt vector value"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P4ivEnumRead {
    #[doc = "0: No interrupt pending"]
    P4iv0 = 0,
    #[doc = "2: Interrupt Source: Port 4.0 interrupt; Interrupt Flag: P4IFG0; Interrupt Priority: Highest"]
    P4iv2 = 2,
    #[doc = "4: Interrupt Source: Port 4.1 interrupt; Interrupt Flag: P4IFG1"]
    P4iv4 = 4,
    #[doc = "6: Interrupt Source: Port 4.2 interrupt; Interrupt Flag: P4IFG2"]
    P4iv6 = 6,
    #[doc = "8: Interrupt Source: Port 4.3 interrupt; Interrupt Flag: P4IFG3"]
    P4iv8 = 8,
    #[doc = "10: Interrupt Source: Port 4.4 interrupt; Interrupt Flag: P4IFG4"]
    P4iv10 = 10,
    #[doc = "12: Interrupt Source: Port 4.5 interrupt; Interrupt Flag: P4IFG5"]
    P4iv12 = 12,
    #[doc = "14: Interrupt Source: Port 4.6 interrupt; Interrupt Flag: P4IFG6"]
    P4iv14 = 14,
    #[doc = "16: Interrupt Source: Port 4.7 interrupt; Interrupt Flag: P4IFG7; Interrupt Priority: Lowest"]
    P4iv16 = 16,
}
impl From<P4ivEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: P4ivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P4ivEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for P4ivEnumRead {}
#[doc = "Field `P4IV` reader - Port 4 interrupt vector value"]
pub type P4ivR = crate::FieldReader<P4ivEnumRead>;
impl P4ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P4ivEnumRead> {
        match self.bits {
            0 => Some(P4ivEnumRead::P4iv0),
            2 => Some(P4ivEnumRead::P4iv2),
            4 => Some(P4ivEnumRead::P4iv4),
            6 => Some(P4ivEnumRead::P4iv6),
            8 => Some(P4ivEnumRead::P4iv8),
            10 => Some(P4ivEnumRead::P4iv10),
            12 => Some(P4ivEnumRead::P4iv12),
            14 => Some(P4ivEnumRead::P4iv14),
            16 => Some(P4ivEnumRead::P4iv16),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_p4iv_0(&self) -> bool {
        *self == P4ivEnumRead::P4iv0
    }
    #[doc = "Interrupt Source: Port 4.0 interrupt; Interrupt Flag: P4IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p4iv_2(&self) -> bool {
        *self == P4ivEnumRead::P4iv2
    }
    #[doc = "Interrupt Source: Port 4.1 interrupt; Interrupt Flag: P4IFG1"]
    #[inline(always)]
    pub fn is_p4iv_4(&self) -> bool {
        *self == P4ivEnumRead::P4iv4
    }
    #[doc = "Interrupt Source: Port 4.2 interrupt; Interrupt Flag: P4IFG2"]
    #[inline(always)]
    pub fn is_p4iv_6(&self) -> bool {
        *self == P4ivEnumRead::P4iv6
    }
    #[doc = "Interrupt Source: Port 4.3 interrupt; Interrupt Flag: P4IFG3"]
    #[inline(always)]
    pub fn is_p4iv_8(&self) -> bool {
        *self == P4ivEnumRead::P4iv8
    }
    #[doc = "Interrupt Source: Port 4.4 interrupt; Interrupt Flag: P4IFG4"]
    #[inline(always)]
    pub fn is_p4iv_10(&self) -> bool {
        *self == P4ivEnumRead::P4iv10
    }
    #[doc = "Interrupt Source: Port 4.5 interrupt; Interrupt Flag: P4IFG5"]
    #[inline(always)]
    pub fn is_p4iv_12(&self) -> bool {
        *self == P4ivEnumRead::P4iv12
    }
    #[doc = "Interrupt Source: Port 4.6 interrupt; Interrupt Flag: P4IFG6"]
    #[inline(always)]
    pub fn is_p4iv_14(&self) -> bool {
        *self == P4ivEnumRead::P4iv14
    }
    #[doc = "Interrupt Source: Port 4.7 interrupt; Interrupt Flag: P4IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p4iv_16(&self) -> bool {
        *self == P4ivEnumRead::P4iv16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 4 interrupt vector value"]
    #[inline(always)]
    pub fn p4iv(&self) -> P4ivR {
        P4ivR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 4 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p4iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4ivSpec;
impl crate::RegisterSpec for P4ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p4iv::R`](R) reader structure"]
impl crate::Readable for P4ivSpec {}
