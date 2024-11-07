#[doc = "Register `P6IV` reader"]
pub type R = crate::R<P6ivSpec>;
#[doc = "Port 6 interrupt vector value"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P6ivEnumRead {
    #[doc = "0: No interrupt pending"]
    P6iv0 = 0,
    #[doc = "2: Interrupt Source: Port 6.0 interrupt; Interrupt Flag: P6IFG0; Interrupt Priority: Highest"]
    P6iv2 = 2,
    #[doc = "4: Interrupt Source: Port 6.1 interrupt; Interrupt Flag: P6IFG1"]
    P6iv4 = 4,
    #[doc = "6: Interrupt Source: Port 6.2 interrupt; Interrupt Flag: P6IFG2"]
    P6iv6 = 6,
    #[doc = "8: Interrupt Source: Port 6.3 interrupt; Interrupt Flag: P6IFG3"]
    P6iv8 = 8,
    #[doc = "10: Interrupt Source: Port 6.4 interrupt; Interrupt Flag: P6IFG4"]
    P6iv10 = 10,
    #[doc = "12: Interrupt Source: Port 6.5 interrupt; Interrupt Flag: P6IFG5"]
    P6iv12 = 12,
    #[doc = "14: Interrupt Source: Port 6.6 interrupt; Interrupt Flag: P6IFG6"]
    P6iv14 = 14,
    #[doc = "16: Interrupt Source: Port 6.7 interrupt; Interrupt Flag: P6IFG7; Interrupt Priority: Lowest"]
    P6iv16 = 16,
}
impl From<P6ivEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: P6ivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P6ivEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for P6ivEnumRead {}
#[doc = "Field `P6IV` reader - Port 6 interrupt vector value"]
pub type P6ivR = crate::FieldReader<P6ivEnumRead>;
impl P6ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P6ivEnumRead> {
        match self.bits {
            0 => Some(P6ivEnumRead::P6iv0),
            2 => Some(P6ivEnumRead::P6iv2),
            4 => Some(P6ivEnumRead::P6iv4),
            6 => Some(P6ivEnumRead::P6iv6),
            8 => Some(P6ivEnumRead::P6iv8),
            10 => Some(P6ivEnumRead::P6iv10),
            12 => Some(P6ivEnumRead::P6iv12),
            14 => Some(P6ivEnumRead::P6iv14),
            16 => Some(P6ivEnumRead::P6iv16),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_p6iv_0(&self) -> bool {
        *self == P6ivEnumRead::P6iv0
    }
    #[doc = "Interrupt Source: Port 6.0 interrupt; Interrupt Flag: P6IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p6iv_2(&self) -> bool {
        *self == P6ivEnumRead::P6iv2
    }
    #[doc = "Interrupt Source: Port 6.1 interrupt; Interrupt Flag: P6IFG1"]
    #[inline(always)]
    pub fn is_p6iv_4(&self) -> bool {
        *self == P6ivEnumRead::P6iv4
    }
    #[doc = "Interrupt Source: Port 6.2 interrupt; Interrupt Flag: P6IFG2"]
    #[inline(always)]
    pub fn is_p6iv_6(&self) -> bool {
        *self == P6ivEnumRead::P6iv6
    }
    #[doc = "Interrupt Source: Port 6.3 interrupt; Interrupt Flag: P6IFG3"]
    #[inline(always)]
    pub fn is_p6iv_8(&self) -> bool {
        *self == P6ivEnumRead::P6iv8
    }
    #[doc = "Interrupt Source: Port 6.4 interrupt; Interrupt Flag: P6IFG4"]
    #[inline(always)]
    pub fn is_p6iv_10(&self) -> bool {
        *self == P6ivEnumRead::P6iv10
    }
    #[doc = "Interrupt Source: Port 6.5 interrupt; Interrupt Flag: P6IFG5"]
    #[inline(always)]
    pub fn is_p6iv_12(&self) -> bool {
        *self == P6ivEnumRead::P6iv12
    }
    #[doc = "Interrupt Source: Port 6.6 interrupt; Interrupt Flag: P6IFG6"]
    #[inline(always)]
    pub fn is_p6iv_14(&self) -> bool {
        *self == P6ivEnumRead::P6iv14
    }
    #[doc = "Interrupt Source: Port 6.7 interrupt; Interrupt Flag: P6IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p6iv_16(&self) -> bool {
        *self == P6ivEnumRead::P6iv16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 6 interrupt vector value"]
    #[inline(always)]
    pub fn p6iv(&self) -> P6ivR {
        P6ivR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 6 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p6iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6ivSpec;
impl crate::RegisterSpec for P6ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p6iv::R`](R) reader structure"]
impl crate::Readable for P6ivSpec {}
