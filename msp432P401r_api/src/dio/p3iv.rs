#[doc = "Register `P3IV` reader"]
pub type R = crate::R<P3ivSpec>;
#[doc = "Port 3 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P3ivEnumRead {
    #[doc = "0: No interrupt pending"]
    P3iv0 = 0,
    #[doc = "2: Interrupt Source: Port 3.0 interrupt; Interrupt Flag: P3IFG0; Interrupt Priority: Highest"]
    P3iv2 = 2,
    #[doc = "4: Interrupt Source: Port 3.1 interrupt; Interrupt Flag: P3IFG1"]
    P3iv4 = 4,
    #[doc = "6: Interrupt Source: Port 3.2 interrupt; Interrupt Flag: P3IFG2"]
    P3iv6 = 6,
    #[doc = "8: Interrupt Source: Port 3.3 interrupt; Interrupt Flag: P3IFG3"]
    P3iv8 = 8,
    #[doc = "10: Interrupt Source: Port 3.4 interrupt; Interrupt Flag: P3IFG4"]
    P3iv10 = 10,
    #[doc = "12: Interrupt Source: Port 3.5 interrupt; Interrupt Flag: P3IFG5"]
    P3iv12 = 12,
    #[doc = "14: Interrupt Source: Port 3.6 interrupt; Interrupt Flag: P3IFG6"]
    P3iv14 = 14,
    #[doc = "16: Interrupt Source: Port 3.7 interrupt; Interrupt Flag: P3IFG7; Interrupt Priority: Lowest"]
    P3iv16 = 16,
}
impl From<P3ivEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: P3ivEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P3ivEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for P3ivEnumRead {}
#[doc = "Field `P3IV` reader - Port 3 interrupt vector value"]
pub type P3ivR = crate::FieldReader<P3ivEnumRead>;
impl P3ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P3ivEnumRead> {
        match self.bits {
            0 => Some(P3ivEnumRead::P3iv0),
            2 => Some(P3ivEnumRead::P3iv2),
            4 => Some(P3ivEnumRead::P3iv4),
            6 => Some(P3ivEnumRead::P3iv6),
            8 => Some(P3ivEnumRead::P3iv8),
            10 => Some(P3ivEnumRead::P3iv10),
            12 => Some(P3ivEnumRead::P3iv12),
            14 => Some(P3ivEnumRead::P3iv14),
            16 => Some(P3ivEnumRead::P3iv16),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_p3iv_0(&self) -> bool {
        *self == P3ivEnumRead::P3iv0
    }
    #[doc = "Interrupt Source: Port 3.0 interrupt; Interrupt Flag: P3IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p3iv_2(&self) -> bool {
        *self == P3ivEnumRead::P3iv2
    }
    #[doc = "Interrupt Source: Port 3.1 interrupt; Interrupt Flag: P3IFG1"]
    #[inline(always)]
    pub fn is_p3iv_4(&self) -> bool {
        *self == P3ivEnumRead::P3iv4
    }
    #[doc = "Interrupt Source: Port 3.2 interrupt; Interrupt Flag: P3IFG2"]
    #[inline(always)]
    pub fn is_p3iv_6(&self) -> bool {
        *self == P3ivEnumRead::P3iv6
    }
    #[doc = "Interrupt Source: Port 3.3 interrupt; Interrupt Flag: P3IFG3"]
    #[inline(always)]
    pub fn is_p3iv_8(&self) -> bool {
        *self == P3ivEnumRead::P3iv8
    }
    #[doc = "Interrupt Source: Port 3.4 interrupt; Interrupt Flag: P3IFG4"]
    #[inline(always)]
    pub fn is_p3iv_10(&self) -> bool {
        *self == P3ivEnumRead::P3iv10
    }
    #[doc = "Interrupt Source: Port 3.5 interrupt; Interrupt Flag: P3IFG5"]
    #[inline(always)]
    pub fn is_p3iv_12(&self) -> bool {
        *self == P3ivEnumRead::P3iv12
    }
    #[doc = "Interrupt Source: Port 3.6 interrupt; Interrupt Flag: P3IFG6"]
    #[inline(always)]
    pub fn is_p3iv_14(&self) -> bool {
        *self == P3ivEnumRead::P3iv14
    }
    #[doc = "Interrupt Source: Port 3.7 interrupt; Interrupt Flag: P3IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p3iv_16(&self) -> bool {
        *self == P3ivEnumRead::P3iv16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 3 interrupt vector value"]
    #[inline(always)]
    pub fn p3iv(&self) -> P3ivR {
        P3ivR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 3 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p3iv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3ivSpec;
impl crate::RegisterSpec for P3ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p3iv::R`](R) reader structure"]
impl crate::Readable for P3ivSpec {}
#[doc = "`reset()` method sets P3IV to value 0"]
impl crate::Resettable for P3ivSpec {
    const RESET_VALUE: u16 = 0;
}
