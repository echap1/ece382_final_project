#[doc = "Register `P8IV` reader"]
pub type R = crate::R<P8ivSpec>;
#[doc = "Port 8 interrupt vector value"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P8ivEnumRead {
    #[doc = "0: No interrupt pending"]
    P8iv0 = 0,
    #[doc = "2: Interrupt Source: Port 8.0 interrupt; Interrupt Flag: P8IFG0; Interrupt Priority: Highest"]
    P8iv2 = 2,
    #[doc = "4: Interrupt Source: Port 8.1 interrupt; Interrupt Flag: P8IFG1"]
    P8iv4 = 4,
    #[doc = "6: Interrupt Source: Port 8.2 interrupt; Interrupt Flag: P8IFG2"]
    P8iv6 = 6,
    #[doc = "8: Interrupt Source: Port 8.3 interrupt; Interrupt Flag: P8IFG3"]
    P8iv8 = 8,
    #[doc = "10: Interrupt Source: Port 8.4 interrupt; Interrupt Flag: P8IFG4"]
    P8iv10 = 10,
    #[doc = "12: Interrupt Source: Port 8.5 interrupt; Interrupt Flag: P8IFG5"]
    P8iv12 = 12,
    #[doc = "14: Interrupt Source: Port 8.6 interrupt; Interrupt Flag: P8IFG6"]
    P8iv14 = 14,
    #[doc = "16: Interrupt Source: Port 8.7 interrupt; Interrupt Flag: P8IFG7; Interrupt Priority: Lowest"]
    P8iv16 = 16,
}
impl From<P8ivEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: P8ivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P8ivEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for P8ivEnumRead {}
#[doc = "Field `P8IV` reader - Port 8 interrupt vector value"]
pub type P8ivR = crate::FieldReader<P8ivEnumRead>;
impl P8ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P8ivEnumRead> {
        match self.bits {
            0 => Some(P8ivEnumRead::P8iv0),
            2 => Some(P8ivEnumRead::P8iv2),
            4 => Some(P8ivEnumRead::P8iv4),
            6 => Some(P8ivEnumRead::P8iv6),
            8 => Some(P8ivEnumRead::P8iv8),
            10 => Some(P8ivEnumRead::P8iv10),
            12 => Some(P8ivEnumRead::P8iv12),
            14 => Some(P8ivEnumRead::P8iv14),
            16 => Some(P8ivEnumRead::P8iv16),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_p8iv_0(&self) -> bool {
        *self == P8ivEnumRead::P8iv0
    }
    #[doc = "Interrupt Source: Port 8.0 interrupt; Interrupt Flag: P8IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p8iv_2(&self) -> bool {
        *self == P8ivEnumRead::P8iv2
    }
    #[doc = "Interrupt Source: Port 8.1 interrupt; Interrupt Flag: P8IFG1"]
    #[inline(always)]
    pub fn is_p8iv_4(&self) -> bool {
        *self == P8ivEnumRead::P8iv4
    }
    #[doc = "Interrupt Source: Port 8.2 interrupt; Interrupt Flag: P8IFG2"]
    #[inline(always)]
    pub fn is_p8iv_6(&self) -> bool {
        *self == P8ivEnumRead::P8iv6
    }
    #[doc = "Interrupt Source: Port 8.3 interrupt; Interrupt Flag: P8IFG3"]
    #[inline(always)]
    pub fn is_p8iv_8(&self) -> bool {
        *self == P8ivEnumRead::P8iv8
    }
    #[doc = "Interrupt Source: Port 8.4 interrupt; Interrupt Flag: P8IFG4"]
    #[inline(always)]
    pub fn is_p8iv_10(&self) -> bool {
        *self == P8ivEnumRead::P8iv10
    }
    #[doc = "Interrupt Source: Port 8.5 interrupt; Interrupt Flag: P8IFG5"]
    #[inline(always)]
    pub fn is_p8iv_12(&self) -> bool {
        *self == P8ivEnumRead::P8iv12
    }
    #[doc = "Interrupt Source: Port 8.6 interrupt; Interrupt Flag: P8IFG6"]
    #[inline(always)]
    pub fn is_p8iv_14(&self) -> bool {
        *self == P8ivEnumRead::P8iv14
    }
    #[doc = "Interrupt Source: Port 8.7 interrupt; Interrupt Flag: P8IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p8iv_16(&self) -> bool {
        *self == P8ivEnumRead::P8iv16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 8 interrupt vector value"]
    #[inline(always)]
    pub fn p8iv(&self) -> P8ivR {
        P8ivR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 8 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p8iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8ivSpec;
impl crate::RegisterSpec for P8ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p8iv::R`](R) reader structure"]
impl crate::Readable for P8ivSpec {}
