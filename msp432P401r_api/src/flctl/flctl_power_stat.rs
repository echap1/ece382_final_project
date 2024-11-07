#[doc = "Register `FLCTL_POWER_STAT` reader"]
pub type R = crate::R<FlctlPowerStatSpec>;
#[doc = "Flash power status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pstat {
    #[doc = "0: Flash IP in power-down mode"]
    Pstat0 = 0,
    #[doc = "1: Flash IP Vdd domain power-up in progress"]
    Pstat1 = 1,
    #[doc = "2: PSS LDO_GOOD, IREF_OK and VREF_OK check in progress"]
    Pstat2 = 2,
    #[doc = "3: Flash IP SAFE_LV check in progress"]
    Pstat3 = 3,
    #[doc = "4: Flash IP Active"]
    Pstat4 = 4,
    #[doc = "5: Flash IP Active in Low-Frequency Active and Low-Frequency LPM0 modes."]
    Pstat5 = 5,
    #[doc = "6: Flash IP in Standby mode"]
    Pstat6 = 6,
    #[doc = "7: Flash IP in Current mirror boost state"]
    Pstat7 = 7,
}
impl From<Pstat> for u8 {
    #[inline(always)]
    fn from(variant: Pstat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pstat {
    type Ux = u8;
}
impl crate::IsEnum for Pstat {}
#[doc = "Field `PSTAT` reader - Flash power status"]
pub type PstatR = crate::FieldReader<Pstat>;
impl PstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pstat {
        match self.bits {
            0 => Pstat::Pstat0,
            1 => Pstat::Pstat1,
            2 => Pstat::Pstat2,
            3 => Pstat::Pstat3,
            4 => Pstat::Pstat4,
            5 => Pstat::Pstat5,
            6 => Pstat::Pstat6,
            7 => Pstat::Pstat7,
            _ => unreachable!(),
        }
    }
    #[doc = "Flash IP in power-down mode"]
    #[inline(always)]
    pub fn is_pstat_0(&self) -> bool {
        *self == Pstat::Pstat0
    }
    #[doc = "Flash IP Vdd domain power-up in progress"]
    #[inline(always)]
    pub fn is_pstat_1(&self) -> bool {
        *self == Pstat::Pstat1
    }
    #[doc = "PSS LDO_GOOD, IREF_OK and VREF_OK check in progress"]
    #[inline(always)]
    pub fn is_pstat_2(&self) -> bool {
        *self == Pstat::Pstat2
    }
    #[doc = "Flash IP SAFE_LV check in progress"]
    #[inline(always)]
    pub fn is_pstat_3(&self) -> bool {
        *self == Pstat::Pstat3
    }
    #[doc = "Flash IP Active"]
    #[inline(always)]
    pub fn is_pstat_4(&self) -> bool {
        *self == Pstat::Pstat4
    }
    #[doc = "Flash IP Active in Low-Frequency Active and Low-Frequency LPM0 modes."]
    #[inline(always)]
    pub fn is_pstat_5(&self) -> bool {
        *self == Pstat::Pstat5
    }
    #[doc = "Flash IP in Standby mode"]
    #[inline(always)]
    pub fn is_pstat_6(&self) -> bool {
        *self == Pstat::Pstat6
    }
    #[doc = "Flash IP in Current mirror boost state"]
    #[inline(always)]
    pub fn is_pstat_7(&self) -> bool {
        *self == Pstat::Pstat7
    }
}
#[doc = "PSS FLDO GOOD status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldostat {
    #[doc = "0: FLDO not GOOD"]
    Ldostat0 = 0,
    #[doc = "1: FLDO GOOD"]
    Ldostat1 = 1,
}
impl From<Ldostat> for bool {
    #[inline(always)]
    fn from(variant: Ldostat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDOSTAT` reader - PSS FLDO GOOD status"]
pub type LdostatR = crate::BitReader<Ldostat>;
impl LdostatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldostat {
        match self.bits {
            false => Ldostat::Ldostat0,
            true => Ldostat::Ldostat1,
        }
    }
    #[doc = "FLDO not GOOD"]
    #[inline(always)]
    pub fn is_ldostat_0(&self) -> bool {
        *self == Ldostat::Ldostat0
    }
    #[doc = "FLDO GOOD"]
    #[inline(always)]
    pub fn is_ldostat_1(&self) -> bool {
        *self == Ldostat::Ldostat1
    }
}
#[doc = "PSS VREF stable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrefstat {
    #[doc = "0: Flash LDO not stable"]
    Vrefstat0 = 0,
    #[doc = "1: Flash LDO stable"]
    Vrefstat1 = 1,
}
impl From<Vrefstat> for bool {
    #[inline(always)]
    fn from(variant: Vrefstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFSTAT` reader - PSS VREF stable status"]
pub type VrefstatR = crate::BitReader<Vrefstat>;
impl VrefstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrefstat {
        match self.bits {
            false => Vrefstat::Vrefstat0,
            true => Vrefstat::Vrefstat1,
        }
    }
    #[doc = "Flash LDO not stable"]
    #[inline(always)]
    pub fn is_vrefstat_0(&self) -> bool {
        *self == Vrefstat::Vrefstat0
    }
    #[doc = "Flash LDO stable"]
    #[inline(always)]
    pub fn is_vrefstat_1(&self) -> bool {
        *self == Vrefstat::Vrefstat1
    }
}
#[doc = "PSS IREF stable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irefstat {
    #[doc = "0: IREF not stable"]
    Irefstat0 = 0,
    #[doc = "1: IREF stable"]
    Irefstat1 = 1,
}
impl From<Irefstat> for bool {
    #[inline(always)]
    fn from(variant: Irefstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFSTAT` reader - PSS IREF stable status"]
pub type IrefstatR = crate::BitReader<Irefstat>;
impl IrefstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irefstat {
        match self.bits {
            false => Irefstat::Irefstat0,
            true => Irefstat::Irefstat1,
        }
    }
    #[doc = "IREF not stable"]
    #[inline(always)]
    pub fn is_irefstat_0(&self) -> bool {
        *self == Irefstat::Irefstat0
    }
    #[doc = "IREF stable"]
    #[inline(always)]
    pub fn is_irefstat_1(&self) -> bool {
        *self == Irefstat::Irefstat1
    }
}
#[doc = "PSS trim done status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trimstat {
    #[doc = "0: PSS trim not complete"]
    Trimstat0 = 0,
    #[doc = "1: PSS trim complete"]
    Trimstat1 = 1,
}
impl From<Trimstat> for bool {
    #[inline(always)]
    fn from(variant: Trimstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIMSTAT` reader - PSS trim done status"]
pub type TrimstatR = crate::BitReader<Trimstat>;
impl TrimstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trimstat {
        match self.bits {
            false => Trimstat::Trimstat0,
            true => Trimstat::Trimstat1,
        }
    }
    #[doc = "PSS trim not complete"]
    #[inline(always)]
    pub fn is_trimstat_0(&self) -> bool {
        *self == Trimstat::Trimstat0
    }
    #[doc = "PSS trim complete"]
    #[inline(always)]
    pub fn is_trimstat_1(&self) -> bool {
        *self == Trimstat::Trimstat1
    }
}
#[doc = "Indicates if Flash is being accessed in 2T mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rd2t {
    #[doc = "0: Flash reads are in 1T mode"]
    Rd2t0 = 0,
    #[doc = "1: Flash reads are in 2T mode"]
    Rd2t1 = 1,
}
impl From<Rd2t> for bool {
    #[inline(always)]
    fn from(variant: Rd2t) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_2T` reader - Indicates if Flash is being accessed in 2T mode"]
pub type Rd2tR = crate::BitReader<Rd2t>;
impl Rd2tR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rd2t {
        match self.bits {
            false => Rd2t::Rd2t0,
            true => Rd2t::Rd2t1,
        }
    }
    #[doc = "Flash reads are in 1T mode"]
    #[inline(always)]
    pub fn is_rd_2t_0(&self) -> bool {
        *self == Rd2t::Rd2t0
    }
    #[doc = "Flash reads are in 2T mode"]
    #[inline(always)]
    pub fn is_rd_2t_1(&self) -> bool {
        *self == Rd2t::Rd2t1
    }
}
impl R {
    #[doc = "Bits 0:2 - Flash power status"]
    #[inline(always)]
    pub fn pstat(&self) -> PstatR {
        PstatR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - PSS FLDO GOOD status"]
    #[inline(always)]
    pub fn ldostat(&self) -> LdostatR {
        LdostatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PSS VREF stable status"]
    #[inline(always)]
    pub fn vrefstat(&self) -> VrefstatR {
        VrefstatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PSS IREF stable status"]
    #[inline(always)]
    pub fn irefstat(&self) -> IrefstatR {
        IrefstatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PSS trim done status"]
    #[inline(always)]
    pub fn trimstat(&self) -> TrimstatR {
        TrimstatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates if Flash is being accessed in 2T mode"]
    #[inline(always)]
    pub fn rd_2t(&self) -> Rd2tR {
        Rd2tR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Power Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_power_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlPowerStatSpec;
impl crate::RegisterSpec for FlctlPowerStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_power_stat::R`](R) reader structure"]
impl crate::Readable for FlctlPowerStatSpec {}
#[doc = "`reset()` method sets FLCTL_POWER_STAT to value 0x80"]
impl crate::Resettable for FlctlPowerStatSpec {
    const RESET_VALUE: u32 = 0x80;
}
