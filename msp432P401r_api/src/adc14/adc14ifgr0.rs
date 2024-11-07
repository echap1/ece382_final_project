#[doc = "Register `ADC14IFGR0` reader"]
pub type R = crate::R<Adc14ifgr0Spec>;
#[doc = "ADC14MEM0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg0EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg0_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg0_1 = 1,
}
impl From<Adc14ifg0EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg0EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG0` reader - ADC14MEM0 interrupt flag"]
pub type Adc14ifg0R = crate::BitReader<Adc14ifg0EnumRead>;
impl Adc14ifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg0EnumRead {
        match self.bits {
            false => Adc14ifg0EnumRead::Adc14ifg0_0,
            true => Adc14ifg0EnumRead::Adc14ifg0_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg0_0(&self) -> bool {
        *self == Adc14ifg0EnumRead::Adc14ifg0_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg0_1(&self) -> bool {
        *self == Adc14ifg0EnumRead::Adc14ifg0_1
    }
}
#[doc = "ADC14MEM1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg1EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg1_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg1_1 = 1,
}
impl From<Adc14ifg1EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg1EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG1` reader - ADC14MEM1 interrupt flag"]
pub type Adc14ifg1R = crate::BitReader<Adc14ifg1EnumRead>;
impl Adc14ifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg1EnumRead {
        match self.bits {
            false => Adc14ifg1EnumRead::Adc14ifg1_0,
            true => Adc14ifg1EnumRead::Adc14ifg1_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg1_0(&self) -> bool {
        *self == Adc14ifg1EnumRead::Adc14ifg1_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg1_1(&self) -> bool {
        *self == Adc14ifg1EnumRead::Adc14ifg1_1
    }
}
#[doc = "ADC14MEM2 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg2EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg2_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg2_1 = 1,
}
impl From<Adc14ifg2EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg2EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG2` reader - ADC14MEM2 interrupt flag"]
pub type Adc14ifg2R = crate::BitReader<Adc14ifg2EnumRead>;
impl Adc14ifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg2EnumRead {
        match self.bits {
            false => Adc14ifg2EnumRead::Adc14ifg2_0,
            true => Adc14ifg2EnumRead::Adc14ifg2_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg2_0(&self) -> bool {
        *self == Adc14ifg2EnumRead::Adc14ifg2_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg2_1(&self) -> bool {
        *self == Adc14ifg2EnumRead::Adc14ifg2_1
    }
}
#[doc = "ADC14MEM3 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg3EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg3_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg3_1 = 1,
}
impl From<Adc14ifg3EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg3EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG3` reader - ADC14MEM3 interrupt flag"]
pub type Adc14ifg3R = crate::BitReader<Adc14ifg3EnumRead>;
impl Adc14ifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg3EnumRead {
        match self.bits {
            false => Adc14ifg3EnumRead::Adc14ifg3_0,
            true => Adc14ifg3EnumRead::Adc14ifg3_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg3_0(&self) -> bool {
        *self == Adc14ifg3EnumRead::Adc14ifg3_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg3_1(&self) -> bool {
        *self == Adc14ifg3EnumRead::Adc14ifg3_1
    }
}
#[doc = "ADC14MEM4 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg4EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg4_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg4_1 = 1,
}
impl From<Adc14ifg4EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg4EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG4` reader - ADC14MEM4 interrupt flag"]
pub type Adc14ifg4R = crate::BitReader<Adc14ifg4EnumRead>;
impl Adc14ifg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg4EnumRead {
        match self.bits {
            false => Adc14ifg4EnumRead::Adc14ifg4_0,
            true => Adc14ifg4EnumRead::Adc14ifg4_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg4_0(&self) -> bool {
        *self == Adc14ifg4EnumRead::Adc14ifg4_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg4_1(&self) -> bool {
        *self == Adc14ifg4EnumRead::Adc14ifg4_1
    }
}
#[doc = "ADC14MEM5 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg5EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg5_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg5_1 = 1,
}
impl From<Adc14ifg5EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg5EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG5` reader - ADC14MEM5 interrupt flag"]
pub type Adc14ifg5R = crate::BitReader<Adc14ifg5EnumRead>;
impl Adc14ifg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg5EnumRead {
        match self.bits {
            false => Adc14ifg5EnumRead::Adc14ifg5_0,
            true => Adc14ifg5EnumRead::Adc14ifg5_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg5_0(&self) -> bool {
        *self == Adc14ifg5EnumRead::Adc14ifg5_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg5_1(&self) -> bool {
        *self == Adc14ifg5EnumRead::Adc14ifg5_1
    }
}
#[doc = "ADC14MEM6 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg6EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg6_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg6_1 = 1,
}
impl From<Adc14ifg6EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg6EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG6` reader - ADC14MEM6 interrupt flag"]
pub type Adc14ifg6R = crate::BitReader<Adc14ifg6EnumRead>;
impl Adc14ifg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg6EnumRead {
        match self.bits {
            false => Adc14ifg6EnumRead::Adc14ifg6_0,
            true => Adc14ifg6EnumRead::Adc14ifg6_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg6_0(&self) -> bool {
        *self == Adc14ifg6EnumRead::Adc14ifg6_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg6_1(&self) -> bool {
        *self == Adc14ifg6EnumRead::Adc14ifg6_1
    }
}
#[doc = "ADC14MEM7 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg7EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg7_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg7_1 = 1,
}
impl From<Adc14ifg7EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg7EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG7` reader - ADC14MEM7 interrupt flag"]
pub type Adc14ifg7R = crate::BitReader<Adc14ifg7EnumRead>;
impl Adc14ifg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg7EnumRead {
        match self.bits {
            false => Adc14ifg7EnumRead::Adc14ifg7_0,
            true => Adc14ifg7EnumRead::Adc14ifg7_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg7_0(&self) -> bool {
        *self == Adc14ifg7EnumRead::Adc14ifg7_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg7_1(&self) -> bool {
        *self == Adc14ifg7EnumRead::Adc14ifg7_1
    }
}
#[doc = "ADC14MEM8 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg8EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg8_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg8_1 = 1,
}
impl From<Adc14ifg8EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg8EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG8` reader - ADC14MEM8 interrupt flag"]
pub type Adc14ifg8R = crate::BitReader<Adc14ifg8EnumRead>;
impl Adc14ifg8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg8EnumRead {
        match self.bits {
            false => Adc14ifg8EnumRead::Adc14ifg8_0,
            true => Adc14ifg8EnumRead::Adc14ifg8_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg8_0(&self) -> bool {
        *self == Adc14ifg8EnumRead::Adc14ifg8_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg8_1(&self) -> bool {
        *self == Adc14ifg8EnumRead::Adc14ifg8_1
    }
}
#[doc = "ADC14MEM9 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg9EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg9_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg9_1 = 1,
}
impl From<Adc14ifg9EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg9EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG9` reader - ADC14MEM9 interrupt flag"]
pub type Adc14ifg9R = crate::BitReader<Adc14ifg9EnumRead>;
impl Adc14ifg9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg9EnumRead {
        match self.bits {
            false => Adc14ifg9EnumRead::Adc14ifg9_0,
            true => Adc14ifg9EnumRead::Adc14ifg9_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg9_0(&self) -> bool {
        *self == Adc14ifg9EnumRead::Adc14ifg9_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg9_1(&self) -> bool {
        *self == Adc14ifg9EnumRead::Adc14ifg9_1
    }
}
#[doc = "ADC14MEM10 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg10EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg10_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg10_1 = 1,
}
impl From<Adc14ifg10EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg10EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG10` reader - ADC14MEM10 interrupt flag"]
pub type Adc14ifg10R = crate::BitReader<Adc14ifg10EnumRead>;
impl Adc14ifg10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg10EnumRead {
        match self.bits {
            false => Adc14ifg10EnumRead::Adc14ifg10_0,
            true => Adc14ifg10EnumRead::Adc14ifg10_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg10_0(&self) -> bool {
        *self == Adc14ifg10EnumRead::Adc14ifg10_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg10_1(&self) -> bool {
        *self == Adc14ifg10EnumRead::Adc14ifg10_1
    }
}
#[doc = "ADC14MEM11 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg11EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg11_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg11_1 = 1,
}
impl From<Adc14ifg11EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg11EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG11` reader - ADC14MEM11 interrupt flag"]
pub type Adc14ifg11R = crate::BitReader<Adc14ifg11EnumRead>;
impl Adc14ifg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg11EnumRead {
        match self.bits {
            false => Adc14ifg11EnumRead::Adc14ifg11_0,
            true => Adc14ifg11EnumRead::Adc14ifg11_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg11_0(&self) -> bool {
        *self == Adc14ifg11EnumRead::Adc14ifg11_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg11_1(&self) -> bool {
        *self == Adc14ifg11EnumRead::Adc14ifg11_1
    }
}
#[doc = "ADC14MEM12 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg12EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg12_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg12_1 = 1,
}
impl From<Adc14ifg12EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg12EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG12` reader - ADC14MEM12 interrupt flag"]
pub type Adc14ifg12R = crate::BitReader<Adc14ifg12EnumRead>;
impl Adc14ifg12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg12EnumRead {
        match self.bits {
            false => Adc14ifg12EnumRead::Adc14ifg12_0,
            true => Adc14ifg12EnumRead::Adc14ifg12_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg12_0(&self) -> bool {
        *self == Adc14ifg12EnumRead::Adc14ifg12_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg12_1(&self) -> bool {
        *self == Adc14ifg12EnumRead::Adc14ifg12_1
    }
}
#[doc = "ADC14MEM13 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg13EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg13_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg13_1 = 1,
}
impl From<Adc14ifg13EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg13EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG13` reader - ADC14MEM13 interrupt flag"]
pub type Adc14ifg13R = crate::BitReader<Adc14ifg13EnumRead>;
impl Adc14ifg13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg13EnumRead {
        match self.bits {
            false => Adc14ifg13EnumRead::Adc14ifg13_0,
            true => Adc14ifg13EnumRead::Adc14ifg13_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg13_0(&self) -> bool {
        *self == Adc14ifg13EnumRead::Adc14ifg13_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg13_1(&self) -> bool {
        *self == Adc14ifg13EnumRead::Adc14ifg13_1
    }
}
#[doc = "ADC14MEM14 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg14EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg14_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg14_1 = 1,
}
impl From<Adc14ifg14EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg14EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG14` reader - ADC14MEM14 interrupt flag"]
pub type Adc14ifg14R = crate::BitReader<Adc14ifg14EnumRead>;
impl Adc14ifg14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg14EnumRead {
        match self.bits {
            false => Adc14ifg14EnumRead::Adc14ifg14_0,
            true => Adc14ifg14EnumRead::Adc14ifg14_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg14_0(&self) -> bool {
        *self == Adc14ifg14EnumRead::Adc14ifg14_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg14_1(&self) -> bool {
        *self == Adc14ifg14EnumRead::Adc14ifg14_1
    }
}
#[doc = "ADC14MEM15 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg15EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg15_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg15_1 = 1,
}
impl From<Adc14ifg15EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg15EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG15` reader - ADC14MEM15 interrupt flag"]
pub type Adc14ifg15R = crate::BitReader<Adc14ifg15EnumRead>;
impl Adc14ifg15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg15EnumRead {
        match self.bits {
            false => Adc14ifg15EnumRead::Adc14ifg15_0,
            true => Adc14ifg15EnumRead::Adc14ifg15_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg15_0(&self) -> bool {
        *self == Adc14ifg15EnumRead::Adc14ifg15_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg15_1(&self) -> bool {
        *self == Adc14ifg15EnumRead::Adc14ifg15_1
    }
}
#[doc = "ADC14MEM16 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg16EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg16_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg16_1 = 1,
}
impl From<Adc14ifg16EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg16EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG16` reader - ADC14MEM16 interrupt flag"]
pub type Adc14ifg16R = crate::BitReader<Adc14ifg16EnumRead>;
impl Adc14ifg16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg16EnumRead {
        match self.bits {
            false => Adc14ifg16EnumRead::Adc14ifg16_0,
            true => Adc14ifg16EnumRead::Adc14ifg16_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg16_0(&self) -> bool {
        *self == Adc14ifg16EnumRead::Adc14ifg16_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg16_1(&self) -> bool {
        *self == Adc14ifg16EnumRead::Adc14ifg16_1
    }
}
#[doc = "ADC14MEM17 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg17EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg17_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg17_1 = 1,
}
impl From<Adc14ifg17EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg17EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG17` reader - ADC14MEM17 interrupt flag"]
pub type Adc14ifg17R = crate::BitReader<Adc14ifg17EnumRead>;
impl Adc14ifg17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg17EnumRead {
        match self.bits {
            false => Adc14ifg17EnumRead::Adc14ifg17_0,
            true => Adc14ifg17EnumRead::Adc14ifg17_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg17_0(&self) -> bool {
        *self == Adc14ifg17EnumRead::Adc14ifg17_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg17_1(&self) -> bool {
        *self == Adc14ifg17EnumRead::Adc14ifg17_1
    }
}
#[doc = "ADC14MEM18 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg18EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg18_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg18_1 = 1,
}
impl From<Adc14ifg18EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg18EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG18` reader - ADC14MEM18 interrupt flag"]
pub type Adc14ifg18R = crate::BitReader<Adc14ifg18EnumRead>;
impl Adc14ifg18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg18EnumRead {
        match self.bits {
            false => Adc14ifg18EnumRead::Adc14ifg18_0,
            true => Adc14ifg18EnumRead::Adc14ifg18_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg18_0(&self) -> bool {
        *self == Adc14ifg18EnumRead::Adc14ifg18_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg18_1(&self) -> bool {
        *self == Adc14ifg18EnumRead::Adc14ifg18_1
    }
}
#[doc = "ADC14MEM19 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg19EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg19_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg19_1 = 1,
}
impl From<Adc14ifg19EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg19EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG19` reader - ADC14MEM19 interrupt flag"]
pub type Adc14ifg19R = crate::BitReader<Adc14ifg19EnumRead>;
impl Adc14ifg19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg19EnumRead {
        match self.bits {
            false => Adc14ifg19EnumRead::Adc14ifg19_0,
            true => Adc14ifg19EnumRead::Adc14ifg19_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg19_0(&self) -> bool {
        *self == Adc14ifg19EnumRead::Adc14ifg19_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg19_1(&self) -> bool {
        *self == Adc14ifg19EnumRead::Adc14ifg19_1
    }
}
#[doc = "ADC14MEM20 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg20EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg20_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg20_1 = 1,
}
impl From<Adc14ifg20EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg20EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG20` reader - ADC14MEM20 interrupt flag"]
pub type Adc14ifg20R = crate::BitReader<Adc14ifg20EnumRead>;
impl Adc14ifg20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg20EnumRead {
        match self.bits {
            false => Adc14ifg20EnumRead::Adc14ifg20_0,
            true => Adc14ifg20EnumRead::Adc14ifg20_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg20_0(&self) -> bool {
        *self == Adc14ifg20EnumRead::Adc14ifg20_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg20_1(&self) -> bool {
        *self == Adc14ifg20EnumRead::Adc14ifg20_1
    }
}
#[doc = "ADC14MEM21 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg21EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg21_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg21_1 = 1,
}
impl From<Adc14ifg21EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg21EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG21` reader - ADC14MEM21 interrupt flag"]
pub type Adc14ifg21R = crate::BitReader<Adc14ifg21EnumRead>;
impl Adc14ifg21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg21EnumRead {
        match self.bits {
            false => Adc14ifg21EnumRead::Adc14ifg21_0,
            true => Adc14ifg21EnumRead::Adc14ifg21_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg21_0(&self) -> bool {
        *self == Adc14ifg21EnumRead::Adc14ifg21_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg21_1(&self) -> bool {
        *self == Adc14ifg21EnumRead::Adc14ifg21_1
    }
}
#[doc = "ADC14MEM22 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg22EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg22_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg22_1 = 1,
}
impl From<Adc14ifg22EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg22EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG22` reader - ADC14MEM22 interrupt flag"]
pub type Adc14ifg22R = crate::BitReader<Adc14ifg22EnumRead>;
impl Adc14ifg22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg22EnumRead {
        match self.bits {
            false => Adc14ifg22EnumRead::Adc14ifg22_0,
            true => Adc14ifg22EnumRead::Adc14ifg22_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg22_0(&self) -> bool {
        *self == Adc14ifg22EnumRead::Adc14ifg22_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg22_1(&self) -> bool {
        *self == Adc14ifg22EnumRead::Adc14ifg22_1
    }
}
#[doc = "ADC14MEM23 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg23EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg23_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg23_1 = 1,
}
impl From<Adc14ifg23EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg23EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG23` reader - ADC14MEM23 interrupt flag"]
pub type Adc14ifg23R = crate::BitReader<Adc14ifg23EnumRead>;
impl Adc14ifg23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg23EnumRead {
        match self.bits {
            false => Adc14ifg23EnumRead::Adc14ifg23_0,
            true => Adc14ifg23EnumRead::Adc14ifg23_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg23_0(&self) -> bool {
        *self == Adc14ifg23EnumRead::Adc14ifg23_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg23_1(&self) -> bool {
        *self == Adc14ifg23EnumRead::Adc14ifg23_1
    }
}
#[doc = "ADC14MEM24 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg24EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg24_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg24_1 = 1,
}
impl From<Adc14ifg24EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg24EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG24` reader - ADC14MEM24 interrupt flag"]
pub type Adc14ifg24R = crate::BitReader<Adc14ifg24EnumRead>;
impl Adc14ifg24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg24EnumRead {
        match self.bits {
            false => Adc14ifg24EnumRead::Adc14ifg24_0,
            true => Adc14ifg24EnumRead::Adc14ifg24_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg24_0(&self) -> bool {
        *self == Adc14ifg24EnumRead::Adc14ifg24_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg24_1(&self) -> bool {
        *self == Adc14ifg24EnumRead::Adc14ifg24_1
    }
}
#[doc = "ADC14MEM25 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg25EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg25_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg25_1 = 1,
}
impl From<Adc14ifg25EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg25EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG25` reader - ADC14MEM25 interrupt flag"]
pub type Adc14ifg25R = crate::BitReader<Adc14ifg25EnumRead>;
impl Adc14ifg25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg25EnumRead {
        match self.bits {
            false => Adc14ifg25EnumRead::Adc14ifg25_0,
            true => Adc14ifg25EnumRead::Adc14ifg25_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg25_0(&self) -> bool {
        *self == Adc14ifg25EnumRead::Adc14ifg25_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg25_1(&self) -> bool {
        *self == Adc14ifg25EnumRead::Adc14ifg25_1
    }
}
#[doc = "ADC14MEM26 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg26EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg26_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg26_1 = 1,
}
impl From<Adc14ifg26EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg26EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG26` reader - ADC14MEM26 interrupt flag"]
pub type Adc14ifg26R = crate::BitReader<Adc14ifg26EnumRead>;
impl Adc14ifg26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg26EnumRead {
        match self.bits {
            false => Adc14ifg26EnumRead::Adc14ifg26_0,
            true => Adc14ifg26EnumRead::Adc14ifg26_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg26_0(&self) -> bool {
        *self == Adc14ifg26EnumRead::Adc14ifg26_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg26_1(&self) -> bool {
        *self == Adc14ifg26EnumRead::Adc14ifg26_1
    }
}
#[doc = "ADC14MEM27 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg27EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg27_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg27_1 = 1,
}
impl From<Adc14ifg27EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg27EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG27` reader - ADC14MEM27 interrupt flag"]
pub type Adc14ifg27R = crate::BitReader<Adc14ifg27EnumRead>;
impl Adc14ifg27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg27EnumRead {
        match self.bits {
            false => Adc14ifg27EnumRead::Adc14ifg27_0,
            true => Adc14ifg27EnumRead::Adc14ifg27_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg27_0(&self) -> bool {
        *self == Adc14ifg27EnumRead::Adc14ifg27_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg27_1(&self) -> bool {
        *self == Adc14ifg27EnumRead::Adc14ifg27_1
    }
}
#[doc = "ADC14MEM28 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg28EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg28_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg28_1 = 1,
}
impl From<Adc14ifg28EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg28EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG28` reader - ADC14MEM28 interrupt flag"]
pub type Adc14ifg28R = crate::BitReader<Adc14ifg28EnumRead>;
impl Adc14ifg28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg28EnumRead {
        match self.bits {
            false => Adc14ifg28EnumRead::Adc14ifg28_0,
            true => Adc14ifg28EnumRead::Adc14ifg28_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg28_0(&self) -> bool {
        *self == Adc14ifg28EnumRead::Adc14ifg28_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg28_1(&self) -> bool {
        *self == Adc14ifg28EnumRead::Adc14ifg28_1
    }
}
#[doc = "ADC14MEM29 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg29EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg29_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg29_1 = 1,
}
impl From<Adc14ifg29EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg29EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG29` reader - ADC14MEM29 interrupt flag"]
pub type Adc14ifg29R = crate::BitReader<Adc14ifg29EnumRead>;
impl Adc14ifg29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg29EnumRead {
        match self.bits {
            false => Adc14ifg29EnumRead::Adc14ifg29_0,
            true => Adc14ifg29EnumRead::Adc14ifg29_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg29_0(&self) -> bool {
        *self == Adc14ifg29EnumRead::Adc14ifg29_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg29_1(&self) -> bool {
        *self == Adc14ifg29EnumRead::Adc14ifg29_1
    }
}
#[doc = "ADC14MEM30 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg30EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg30_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg30_1 = 1,
}
impl From<Adc14ifg30EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg30EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG30` reader - ADC14MEM30 interrupt flag"]
pub type Adc14ifg30R = crate::BitReader<Adc14ifg30EnumRead>;
impl Adc14ifg30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg30EnumRead {
        match self.bits {
            false => Adc14ifg30EnumRead::Adc14ifg30_0,
            true => Adc14ifg30EnumRead::Adc14ifg30_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg30_0(&self) -> bool {
        *self == Adc14ifg30EnumRead::Adc14ifg30_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg30_1(&self) -> bool {
        *self == Adc14ifg30EnumRead::Adc14ifg30_1
    }
}
#[doc = "ADC14MEM31 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ifg31EnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ifg31_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ifg31_1 = 1,
}
impl From<Adc14ifg31EnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ifg31EnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IFG31` reader - ADC14MEM31 interrupt flag"]
pub type Adc14ifg31R = crate::BitReader<Adc14ifg31EnumRead>;
impl Adc14ifg31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ifg31EnumRead {
        match self.bits {
            false => Adc14ifg31EnumRead::Adc14ifg31_0,
            true => Adc14ifg31EnumRead::Adc14ifg31_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg31_0(&self) -> bool {
        *self == Adc14ifg31EnumRead::Adc14ifg31_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ifg31_1(&self) -> bool {
        *self == Adc14ifg31EnumRead::Adc14ifg31_1
    }
}
impl R {
    #[doc = "Bit 0 - ADC14MEM0 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg0(&self) -> Adc14ifg0R {
        Adc14ifg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC14MEM1 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg1(&self) -> Adc14ifg1R {
        Adc14ifg1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC14MEM2 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg2(&self) -> Adc14ifg2R {
        Adc14ifg2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC14MEM3 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg3(&self) -> Adc14ifg3R {
        Adc14ifg3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC14MEM4 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg4(&self) -> Adc14ifg4R {
        Adc14ifg4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC14MEM5 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg5(&self) -> Adc14ifg5R {
        Adc14ifg5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC14MEM6 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg6(&self) -> Adc14ifg6R {
        Adc14ifg6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC14MEM7 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg7(&self) -> Adc14ifg7R {
        Adc14ifg7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC14MEM8 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg8(&self) -> Adc14ifg8R {
        Adc14ifg8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC14MEM9 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg9(&self) -> Adc14ifg9R {
        Adc14ifg9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC14MEM10 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg10(&self) -> Adc14ifg10R {
        Adc14ifg10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC14MEM11 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg11(&self) -> Adc14ifg11R {
        Adc14ifg11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC14MEM12 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg12(&self) -> Adc14ifg12R {
        Adc14ifg12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC14MEM13 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg13(&self) -> Adc14ifg13R {
        Adc14ifg13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC14MEM14 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg14(&self) -> Adc14ifg14R {
        Adc14ifg14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC14MEM15 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg15(&self) -> Adc14ifg15R {
        Adc14ifg15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC14MEM16 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg16(&self) -> Adc14ifg16R {
        Adc14ifg16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC14MEM17 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg17(&self) -> Adc14ifg17R {
        Adc14ifg17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC14MEM18 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg18(&self) -> Adc14ifg18R {
        Adc14ifg18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC14MEM19 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg19(&self) -> Adc14ifg19R {
        Adc14ifg19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC14MEM20 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg20(&self) -> Adc14ifg20R {
        Adc14ifg20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADC14MEM21 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg21(&self) -> Adc14ifg21R {
        Adc14ifg21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC14MEM22 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg22(&self) -> Adc14ifg22R {
        Adc14ifg22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADC14MEM23 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg23(&self) -> Adc14ifg23R {
        Adc14ifg23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC14MEM24 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg24(&self) -> Adc14ifg24R {
        Adc14ifg24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADC14MEM25 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg25(&self) -> Adc14ifg25R {
        Adc14ifg25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ADC14MEM26 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg26(&self) -> Adc14ifg26R {
        Adc14ifg26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC14MEM27 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg27(&self) -> Adc14ifg27R {
        Adc14ifg27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC14MEM28 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg28(&self) -> Adc14ifg28R {
        Adc14ifg28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC14MEM29 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg29(&self) -> Adc14ifg29R {
        Adc14ifg29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ADC14MEM30 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg30(&self) -> Adc14ifg30R {
        Adc14ifg30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC14MEM31 interrupt flag"]
    #[inline(always)]
    pub fn adc14ifg31(&self) -> Adc14ifg31R {
        Adc14ifg31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Flag 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ifgr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14ifgr0Spec;
impl crate::RegisterSpec for Adc14ifgr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14ifgr0::R`](R) reader structure"]
impl crate::Readable for Adc14ifgr0Spec {}
#[doc = "`reset()` method sets ADC14IFGR0 to value 0"]
impl crate::Resettable for Adc14ifgr0Spec {
    const RESET_VALUE: u32 = 0;
}
