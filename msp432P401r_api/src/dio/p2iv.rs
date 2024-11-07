#[doc = "Register `P2IV` reader"]
pub type R = crate::R<P2ivSpec>;
#[doc = "Port 2 interrupt vector value"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2ivEnumRead {
    #[doc = "0: No interrupt pending"]
    P2iv0 = 0,
    #[doc = "2: Interrupt Source: Port 2.0 interrupt; Interrupt Flag: P2IFG0; Interrupt Priority: Highest"]
    P2iv2 = 2,
    #[doc = "4: Interrupt Source: Port 2.1 interrupt; Interrupt Flag: P2IFG1"]
    P2iv4 = 4,
    #[doc = "6: Interrupt Source: Port 2.2 interrupt; Interrupt Flag: P2IFG2"]
    P2iv6 = 6,
    #[doc = "8: Interrupt Source: Port 2.3 interrupt; Interrupt Flag: P2IFG3"]
    P2iv8 = 8,
    #[doc = "10: Interrupt Source: Port 2.4 interrupt; Interrupt Flag: P2IFG4"]
    P2iv10 = 10,
    #[doc = "12: Interrupt Source: Port 2.5 interrupt; Interrupt Flag: P2IFG5"]
    P2iv12 = 12,
    #[doc = "14: Interrupt Source: Port 2.6 interrupt; Interrupt Flag: P2IFG6"]
    P2iv14 = 14,
    #[doc = "16: Interrupt Source: Port 2.7 interrupt; Interrupt Flag: P2IFG7; Interrupt Priority: Lowest"]
    P2iv16 = 16,
}
impl From<P2ivEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: P2ivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2ivEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for P2ivEnumRead {}
#[doc = "Field `P2IV` reader - Port 2 interrupt vector value"]
pub type P2ivR = crate::FieldReader<P2ivEnumRead>;
impl P2ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P2ivEnumRead> {
        match self.bits {
            0 => Some(P2ivEnumRead::P2iv0),
            2 => Some(P2ivEnumRead::P2iv2),
            4 => Some(P2ivEnumRead::P2iv4),
            6 => Some(P2ivEnumRead::P2iv6),
            8 => Some(P2ivEnumRead::P2iv8),
            10 => Some(P2ivEnumRead::P2iv10),
            12 => Some(P2ivEnumRead::P2iv12),
            14 => Some(P2ivEnumRead::P2iv14),
            16 => Some(P2ivEnumRead::P2iv16),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_p2iv_0(&self) -> bool {
        *self == P2ivEnumRead::P2iv0
    }
    #[doc = "Interrupt Source: Port 2.0 interrupt; Interrupt Flag: P2IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p2iv_2(&self) -> bool {
        *self == P2ivEnumRead::P2iv2
    }
    #[doc = "Interrupt Source: Port 2.1 interrupt; Interrupt Flag: P2IFG1"]
    #[inline(always)]
    pub fn is_p2iv_4(&self) -> bool {
        *self == P2ivEnumRead::P2iv4
    }
    #[doc = "Interrupt Source: Port 2.2 interrupt; Interrupt Flag: P2IFG2"]
    #[inline(always)]
    pub fn is_p2iv_6(&self) -> bool {
        *self == P2ivEnumRead::P2iv6
    }
    #[doc = "Interrupt Source: Port 2.3 interrupt; Interrupt Flag: P2IFG3"]
    #[inline(always)]
    pub fn is_p2iv_8(&self) -> bool {
        *self == P2ivEnumRead::P2iv8
    }
    #[doc = "Interrupt Source: Port 2.4 interrupt; Interrupt Flag: P2IFG4"]
    #[inline(always)]
    pub fn is_p2iv_10(&self) -> bool {
        *self == P2ivEnumRead::P2iv10
    }
    #[doc = "Interrupt Source: Port 2.5 interrupt; Interrupt Flag: P2IFG5"]
    #[inline(always)]
    pub fn is_p2iv_12(&self) -> bool {
        *self == P2ivEnumRead::P2iv12
    }
    #[doc = "Interrupt Source: Port 2.6 interrupt; Interrupt Flag: P2IFG6"]
    #[inline(always)]
    pub fn is_p2iv_14(&self) -> bool {
        *self == P2ivEnumRead::P2iv14
    }
    #[doc = "Interrupt Source: Port 2.7 interrupt; Interrupt Flag: P2IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p2iv_16(&self) -> bool {
        *self == P2ivEnumRead::P2iv16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 2 interrupt vector value"]
    #[inline(always)]
    pub fn p2iv(&self) -> P2ivR {
        P2ivR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 2 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2ivSpec;
impl crate::RegisterSpec for P2ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p2iv::R`](R) reader structure"]
impl crate::Readable for P2ivSpec {}
