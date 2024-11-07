#[doc = "Register `ADC14IFGR1` reader"]
pub type R = crate::R<Adc14ifgr1Spec>;
#[doc = "Interrupt flag for ADC14MEMx within comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14inifgEnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14inifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14inifg1 = 1,
}
impl From<Adc14inifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14inifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14INIFG` reader - Interrupt flag for ADC14MEMx within comparator window"]
pub type Adc14inifgR = crate::BitReader<Adc14inifgEnumRead>;
impl Adc14inifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14inifgEnumRead {
        match self.bits {
            false => Adc14inifgEnumRead::Adc14inifg0,
            true => Adc14inifgEnumRead::Adc14inifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14inifg_0(&self) -> bool {
        *self == Adc14inifgEnumRead::Adc14inifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14inifg_1(&self) -> bool {
        *self == Adc14inifgEnumRead::Adc14inifg1
    }
}
#[doc = "Interrupt flag for ADC14MEMx below comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14loifgEnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14loifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14loifg1 = 1,
}
impl From<Adc14loifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14loifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14LOIFG` reader - Interrupt flag for ADC14MEMx below comparator window"]
pub type Adc14loifgR = crate::BitReader<Adc14loifgEnumRead>;
impl Adc14loifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14loifgEnumRead {
        match self.bits {
            false => Adc14loifgEnumRead::Adc14loifg0,
            true => Adc14loifgEnumRead::Adc14loifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14loifg_0(&self) -> bool {
        *self == Adc14loifgEnumRead::Adc14loifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14loifg_1(&self) -> bool {
        *self == Adc14loifgEnumRead::Adc14loifg1
    }
}
#[doc = "Interrupt flag for ADC14MEMx above comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14hiifgEnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14hiifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14hiifg1 = 1,
}
impl From<Adc14hiifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14hiifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14HIIFG` reader - Interrupt flag for ADC14MEMx above comparator window"]
pub type Adc14hiifgR = crate::BitReader<Adc14hiifgEnumRead>;
impl Adc14hiifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14hiifgEnumRead {
        match self.bits {
            false => Adc14hiifgEnumRead::Adc14hiifg0,
            true => Adc14hiifgEnumRead::Adc14hiifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14hiifg_0(&self) -> bool {
        *self == Adc14hiifgEnumRead::Adc14hiifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14hiifg_1(&self) -> bool {
        *self == Adc14hiifgEnumRead::Adc14hiifg1
    }
}
#[doc = "ADC14MEMx overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ovifgEnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14ovifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14ovifg1 = 1,
}
impl From<Adc14ovifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14ovifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14OVIFG` reader - ADC14MEMx overflow interrupt flag"]
pub type Adc14ovifgR = crate::BitReader<Adc14ovifgEnumRead>;
impl Adc14ovifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ovifgEnumRead {
        match self.bits {
            false => Adc14ovifgEnumRead::Adc14ovifg0,
            true => Adc14ovifgEnumRead::Adc14ovifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ovifg_0(&self) -> bool {
        *self == Adc14ovifgEnumRead::Adc14ovifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14ovifg_1(&self) -> bool {
        *self == Adc14ovifgEnumRead::Adc14ovifg1
    }
}
#[doc = "ADC14 conversion time overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14tovifgEnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14tovifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14tovifg1 = 1,
}
impl From<Adc14tovifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14tovifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14TOVIFG` reader - ADC14 conversion time overflow interrupt flag"]
pub type Adc14tovifgR = crate::BitReader<Adc14tovifgEnumRead>;
impl Adc14tovifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14tovifgEnumRead {
        match self.bits {
            false => Adc14tovifgEnumRead::Adc14tovifg0,
            true => Adc14tovifgEnumRead::Adc14tovifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14tovifg_0(&self) -> bool {
        *self == Adc14tovifgEnumRead::Adc14tovifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14tovifg_1(&self) -> bool {
        *self == Adc14tovifgEnumRead::Adc14tovifg1
    }
}
#[doc = "ADC14 local buffered reference ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14rdyifgEnumRead {
    #[doc = "0: No interrupt pending"]
    Adc14rdyifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adc14rdyifg1 = 1,
}
impl From<Adc14rdyifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14rdyifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14RDYIFG` reader - ADC14 local buffered reference ready interrupt flag"]
pub type Adc14rdyifgR = crate::BitReader<Adc14rdyifgEnumRead>;
impl Adc14rdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14rdyifgEnumRead {
        match self.bits {
            false => Adc14rdyifgEnumRead::Adc14rdyifg0,
            true => Adc14rdyifgEnumRead::Adc14rdyifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14rdyifg_0(&self) -> bool {
        *self == Adc14rdyifgEnumRead::Adc14rdyifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adc14rdyifg_1(&self) -> bool {
        *self == Adc14rdyifgEnumRead::Adc14rdyifg1
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt flag for ADC14MEMx within comparator window"]
    #[inline(always)]
    pub fn adc14inifg(&self) -> Adc14inifgR {
        Adc14inifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for ADC14MEMx below comparator window"]
    #[inline(always)]
    pub fn adc14loifg(&self) -> Adc14loifgR {
        Adc14loifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for ADC14MEMx above comparator window"]
    #[inline(always)]
    pub fn adc14hiifg(&self) -> Adc14hiifgR {
        Adc14hiifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC14MEMx overflow interrupt flag"]
    #[inline(always)]
    pub fn adc14ovifg(&self) -> Adc14ovifgR {
        Adc14ovifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC14 conversion time overflow interrupt flag"]
    #[inline(always)]
    pub fn adc14tovifg(&self) -> Adc14tovifgR {
        Adc14tovifgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC14 local buffered reference ready interrupt flag"]
    #[inline(always)]
    pub fn adc14rdyifg(&self) -> Adc14rdyifgR {
        Adc14rdyifgR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Interrupt Flag 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ifgr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14ifgr1Spec;
impl crate::RegisterSpec for Adc14ifgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14ifgr1::R`](R) reader structure"]
impl crate::Readable for Adc14ifgr1Spec {}
#[doc = "`reset()` method sets ADC14IFGR1 to value 0"]
impl crate::Resettable for Adc14ifgr1Spec {
    const RESET_VALUE: u32 = 0;
}
