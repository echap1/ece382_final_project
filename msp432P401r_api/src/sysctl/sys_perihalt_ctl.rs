#[doc = "Register `SYS_PERIHALT_CTL` reader"]
pub type R = crate::R<SysPerihaltCtlSpec>;
#[doc = "Register `SYS_PERIHALT_CTL` writer"]
pub type W = crate::W<SysPerihaltCtlSpec>;
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltT16_0 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltT16_0_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltT16_0_1 = 1,
}
impl From<HaltT16_0> for bool {
    #[inline(always)]
    fn from(variant: HaltT16_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_T16_0` reader - Freezes IP operation when CPU is halted"]
pub type HaltT16_0R = crate::BitReader<HaltT16_0>;
impl HaltT16_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltT16_0 {
        match self.bits {
            false => HaltT16_0::HaltT16_0_0,
            true => HaltT16_0::HaltT16_0_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_t16_0_0(&self) -> bool {
        *self == HaltT16_0::HaltT16_0_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_t16_0_1(&self) -> bool {
        *self == HaltT16_0::HaltT16_0_1
    }
}
#[doc = "Field `HALT_T16_0` writer - Freezes IP operation when CPU is halted"]
pub type HaltT16_0W<'a, REG> = crate::BitWriter<'a, REG, HaltT16_0>;
impl<'a, REG> HaltT16_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_0_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltT16_0::HaltT16_0_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_0_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltT16_0::HaltT16_0_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltT16_1 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltT16_1_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltT16_1_1 = 1,
}
impl From<HaltT16_1> for bool {
    #[inline(always)]
    fn from(variant: HaltT16_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_T16_1` reader - Freezes IP operation when CPU is halted"]
pub type HaltT16_1R = crate::BitReader<HaltT16_1>;
impl HaltT16_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltT16_1 {
        match self.bits {
            false => HaltT16_1::HaltT16_1_0,
            true => HaltT16_1::HaltT16_1_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_t16_1_0(&self) -> bool {
        *self == HaltT16_1::HaltT16_1_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_t16_1_1(&self) -> bool {
        *self == HaltT16_1::HaltT16_1_1
    }
}
#[doc = "Field `HALT_T16_1` writer - Freezes IP operation when CPU is halted"]
pub type HaltT16_1W<'a, REG> = crate::BitWriter<'a, REG, HaltT16_1>;
impl<'a, REG> HaltT16_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_1_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltT16_1::HaltT16_1_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_1_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltT16_1::HaltT16_1_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltT16_2 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltT16_2_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltT16_2_1 = 1,
}
impl From<HaltT16_2> for bool {
    #[inline(always)]
    fn from(variant: HaltT16_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_T16_2` reader - Freezes IP operation when CPU is halted"]
pub type HaltT16_2R = crate::BitReader<HaltT16_2>;
impl HaltT16_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltT16_2 {
        match self.bits {
            false => HaltT16_2::HaltT16_2_0,
            true => HaltT16_2::HaltT16_2_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_t16_2_0(&self) -> bool {
        *self == HaltT16_2::HaltT16_2_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_t16_2_1(&self) -> bool {
        *self == HaltT16_2::HaltT16_2_1
    }
}
#[doc = "Field `HALT_T16_2` writer - Freezes IP operation when CPU is halted"]
pub type HaltT16_2W<'a, REG> = crate::BitWriter<'a, REG, HaltT16_2>;
impl<'a, REG> HaltT16_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_2_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltT16_2::HaltT16_2_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_2_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltT16_2::HaltT16_2_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltT16_3 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltT16_3_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltT16_3_1 = 1,
}
impl From<HaltT16_3> for bool {
    #[inline(always)]
    fn from(variant: HaltT16_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_T16_3` reader - Freezes IP operation when CPU is halted"]
pub type HaltT16_3R = crate::BitReader<HaltT16_3>;
impl HaltT16_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltT16_3 {
        match self.bits {
            false => HaltT16_3::HaltT16_3_0,
            true => HaltT16_3::HaltT16_3_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_t16_3_0(&self) -> bool {
        *self == HaltT16_3::HaltT16_3_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_t16_3_1(&self) -> bool {
        *self == HaltT16_3::HaltT16_3_1
    }
}
#[doc = "Field `HALT_T16_3` writer - Freezes IP operation when CPU is halted"]
pub type HaltT16_3W<'a, REG> = crate::BitWriter<'a, REG, HaltT16_3>;
impl<'a, REG> HaltT16_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_3_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltT16_3::HaltT16_3_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_3_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltT16_3::HaltT16_3_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltT32_0 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltT32_0_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltT32_0_1 = 1,
}
impl From<HaltT32_0> for bool {
    #[inline(always)]
    fn from(variant: HaltT32_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_T32_0` reader - Freezes IP operation when CPU is halted"]
pub type HaltT32_0R = crate::BitReader<HaltT32_0>;
impl HaltT32_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltT32_0 {
        match self.bits {
            false => HaltT32_0::HaltT32_0_0,
            true => HaltT32_0::HaltT32_0_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_t32_0_0(&self) -> bool {
        *self == HaltT32_0::HaltT32_0_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_t32_0_1(&self) -> bool {
        *self == HaltT32_0::HaltT32_0_1
    }
}
#[doc = "Field `HALT_T32_0` writer - Freezes IP operation when CPU is halted"]
pub type HaltT32_0W<'a, REG> = crate::BitWriter<'a, REG, HaltT32_0>;
impl<'a, REG> HaltT32_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_t32_0_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltT32_0::HaltT32_0_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t32_0_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltT32_0::HaltT32_0_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltEUa0 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltEUa0_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltEUa0_1 = 1,
}
impl From<HaltEUa0> for bool {
    #[inline(always)]
    fn from(variant: HaltEUa0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUA0` reader - Freezes IP operation when CPU is halted"]
pub type HaltEUa0R = crate::BitReader<HaltEUa0>;
impl HaltEUa0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltEUa0 {
        match self.bits {
            false => HaltEUa0::HaltEUa0_0,
            true => HaltEUa0::HaltEUa0_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ua0_0(&self) -> bool {
        *self == HaltEUa0::HaltEUa0_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ua0_1(&self) -> bool {
        *self == HaltEUa0::HaltEUa0_1
    }
}
#[doc = "Field `HALT_eUA0` writer - Freezes IP operation when CPU is halted"]
pub type HaltEUa0W<'a, REG> = crate::BitWriter<'a, REG, HaltEUa0>;
impl<'a, REG> HaltEUa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua0_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUa0::HaltEUa0_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua0_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUa0::HaltEUa0_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltEUa1 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltEUa1_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltEUa1_1 = 1,
}
impl From<HaltEUa1> for bool {
    #[inline(always)]
    fn from(variant: HaltEUa1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUA1` reader - Freezes IP operation when CPU is halted"]
pub type HaltEUa1R = crate::BitReader<HaltEUa1>;
impl HaltEUa1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltEUa1 {
        match self.bits {
            false => HaltEUa1::HaltEUa1_0,
            true => HaltEUa1::HaltEUa1_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ua1_0(&self) -> bool {
        *self == HaltEUa1::HaltEUa1_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ua1_1(&self) -> bool {
        *self == HaltEUa1::HaltEUa1_1
    }
}
#[doc = "Field `HALT_eUA1` writer - Freezes IP operation when CPU is halted"]
pub type HaltEUa1W<'a, REG> = crate::BitWriter<'a, REG, HaltEUa1>;
impl<'a, REG> HaltEUa1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua1_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUa1::HaltEUa1_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua1_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUa1::HaltEUa1_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltEUa2 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltEUa2_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltEUa2_1 = 1,
}
impl From<HaltEUa2> for bool {
    #[inline(always)]
    fn from(variant: HaltEUa2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUA2` reader - Freezes IP operation when CPU is halted"]
pub type HaltEUa2R = crate::BitReader<HaltEUa2>;
impl HaltEUa2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltEUa2 {
        match self.bits {
            false => HaltEUa2::HaltEUa2_0,
            true => HaltEUa2::HaltEUa2_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ua2_0(&self) -> bool {
        *self == HaltEUa2::HaltEUa2_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ua2_1(&self) -> bool {
        *self == HaltEUa2::HaltEUa2_1
    }
}
#[doc = "Field `HALT_eUA2` writer - Freezes IP operation when CPU is halted"]
pub type HaltEUa2W<'a, REG> = crate::BitWriter<'a, REG, HaltEUa2>;
impl<'a, REG> HaltEUa2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua2_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUa2::HaltEUa2_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua2_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUa2::HaltEUa2_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltEUa3 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltEUa3_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltEUa3_1 = 1,
}
impl From<HaltEUa3> for bool {
    #[inline(always)]
    fn from(variant: HaltEUa3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUA3` reader - Freezes IP operation when CPU is halted"]
pub type HaltEUa3R = crate::BitReader<HaltEUa3>;
impl HaltEUa3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltEUa3 {
        match self.bits {
            false => HaltEUa3::HaltEUa3_0,
            true => HaltEUa3::HaltEUa3_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ua3_0(&self) -> bool {
        *self == HaltEUa3::HaltEUa3_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ua3_1(&self) -> bool {
        *self == HaltEUa3::HaltEUa3_1
    }
}
#[doc = "Field `HALT_eUA3` writer - Freezes IP operation when CPU is halted"]
pub type HaltEUa3W<'a, REG> = crate::BitWriter<'a, REG, HaltEUa3>;
impl<'a, REG> HaltEUa3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua3_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUa3::HaltEUa3_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua3_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUa3::HaltEUa3_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltEUb0 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltEUb0_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltEUb0_1 = 1,
}
impl From<HaltEUb0> for bool {
    #[inline(always)]
    fn from(variant: HaltEUb0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUB0` reader - Freezes IP operation when CPU is halted"]
pub type HaltEUb0R = crate::BitReader<HaltEUb0>;
impl HaltEUb0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltEUb0 {
        match self.bits {
            false => HaltEUb0::HaltEUb0_0,
            true => HaltEUb0::HaltEUb0_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ub0_0(&self) -> bool {
        *self == HaltEUb0::HaltEUb0_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ub0_1(&self) -> bool {
        *self == HaltEUb0::HaltEUb0_1
    }
}
#[doc = "Field `HALT_eUB0` writer - Freezes IP operation when CPU is halted"]
pub type HaltEUb0W<'a, REG> = crate::BitWriter<'a, REG, HaltEUb0>;
impl<'a, REG> HaltEUb0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub0_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUb0::HaltEUb0_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub0_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUb0::HaltEUb0_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltEUb1 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltEUb1_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltEUb1_1 = 1,
}
impl From<HaltEUb1> for bool {
    #[inline(always)]
    fn from(variant: HaltEUb1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUB1` reader - Freezes IP operation when CPU is halted"]
pub type HaltEUb1R = crate::BitReader<HaltEUb1>;
impl HaltEUb1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltEUb1 {
        match self.bits {
            false => HaltEUb1::HaltEUb1_0,
            true => HaltEUb1::HaltEUb1_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ub1_0(&self) -> bool {
        *self == HaltEUb1::HaltEUb1_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ub1_1(&self) -> bool {
        *self == HaltEUb1::HaltEUb1_1
    }
}
#[doc = "Field `HALT_eUB1` writer - Freezes IP operation when CPU is halted"]
pub type HaltEUb1W<'a, REG> = crate::BitWriter<'a, REG, HaltEUb1>;
impl<'a, REG> HaltEUb1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub1_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUb1::HaltEUb1_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub1_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUb1::HaltEUb1_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltEUb2 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltEUb2_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltEUb2_1 = 1,
}
impl From<HaltEUb2> for bool {
    #[inline(always)]
    fn from(variant: HaltEUb2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUB2` reader - Freezes IP operation when CPU is halted"]
pub type HaltEUb2R = crate::BitReader<HaltEUb2>;
impl HaltEUb2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltEUb2 {
        match self.bits {
            false => HaltEUb2::HaltEUb2_0,
            true => HaltEUb2::HaltEUb2_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ub2_0(&self) -> bool {
        *self == HaltEUb2::HaltEUb2_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ub2_1(&self) -> bool {
        *self == HaltEUb2::HaltEUb2_1
    }
}
#[doc = "Field `HALT_eUB2` writer - Freezes IP operation when CPU is halted"]
pub type HaltEUb2W<'a, REG> = crate::BitWriter<'a, REG, HaltEUb2>;
impl<'a, REG> HaltEUb2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub2_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUb2::HaltEUb2_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub2_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUb2::HaltEUb2_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltEUb3 {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltEUb3_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltEUb3_1 = 1,
}
impl From<HaltEUb3> for bool {
    #[inline(always)]
    fn from(variant: HaltEUb3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUB3` reader - Freezes IP operation when CPU is halted"]
pub type HaltEUb3R = crate::BitReader<HaltEUb3>;
impl HaltEUb3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltEUb3 {
        match self.bits {
            false => HaltEUb3::HaltEUb3_0,
            true => HaltEUb3::HaltEUb3_1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ub3_0(&self) -> bool {
        *self == HaltEUb3::HaltEUb3_0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_e_ub3_1(&self) -> bool {
        *self == HaltEUb3::HaltEUb3_1
    }
}
#[doc = "Field `HALT_eUB3` writer - Freezes IP operation when CPU is halted"]
pub type HaltEUb3W<'a, REG> = crate::BitWriter<'a, REG, HaltEUb3>;
impl<'a, REG> HaltEUb3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub3_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUb3::HaltEUb3_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub3_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltEUb3::HaltEUb3_1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltAdc {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltAdc0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltAdc1 = 1,
}
impl From<HaltAdc> for bool {
    #[inline(always)]
    fn from(variant: HaltAdc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_ADC` reader - Freezes IP operation when CPU is halted"]
pub type HaltAdcR = crate::BitReader<HaltAdc>;
impl HaltAdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltAdc {
        match self.bits {
            false => HaltAdc::HaltAdc0,
            true => HaltAdc::HaltAdc1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_adc_0(&self) -> bool {
        *self == HaltAdc::HaltAdc0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_adc_1(&self) -> bool {
        *self == HaltAdc::HaltAdc1
    }
}
#[doc = "Field `HALT_ADC` writer - Freezes IP operation when CPU is halted"]
pub type HaltAdcW<'a, REG> = crate::BitWriter<'a, REG, HaltAdc>;
impl<'a, REG> HaltAdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_adc_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltAdc::HaltAdc0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_adc_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltAdc::HaltAdc1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltWdt {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltWdt0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltWdt1 = 1,
}
impl From<HaltWdt> for bool {
    #[inline(always)]
    fn from(variant: HaltWdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_WDT` reader - Freezes IP operation when CPU is halted"]
pub type HaltWdtR = crate::BitReader<HaltWdt>;
impl HaltWdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltWdt {
        match self.bits {
            false => HaltWdt::HaltWdt0,
            true => HaltWdt::HaltWdt1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_wdt_0(&self) -> bool {
        *self == HaltWdt::HaltWdt0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_wdt_1(&self) -> bool {
        *self == HaltWdt::HaltWdt1
    }
}
#[doc = "Field `HALT_WDT` writer - Freezes IP operation when CPU is halted"]
pub type HaltWdtW<'a, REG> = crate::BitWriter<'a, REG, HaltWdt>;
impl<'a, REG> HaltWdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_wdt_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltWdt::HaltWdt0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_wdt_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltWdt::HaltWdt1)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HaltDma {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HaltDma0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HaltDma1 = 1,
}
impl From<HaltDma> for bool {
    #[inline(always)]
    fn from(variant: HaltDma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_DMA` reader - Freezes IP operation when CPU is halted"]
pub type HaltDmaR = crate::BitReader<HaltDma>;
impl HaltDmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HaltDma {
        match self.bits {
            false => HaltDma::HaltDma0,
            true => HaltDma::HaltDma1,
        }
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_dma_0(&self) -> bool {
        *self == HaltDma::HaltDma0
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn is_halt_dma_1(&self) -> bool {
        *self == HaltDma::HaltDma1
    }
}
#[doc = "Field `HALT_DMA` writer - Freezes IP operation when CPU is halted"]
pub type HaltDmaW<'a, REG> = crate::BitWriter<'a, REG, HaltDma>;
impl<'a, REG> HaltDmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_dma_0(self) -> &'a mut crate::W<REG> {
        self.variant(HaltDma::HaltDma0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_dma_1(self) -> &'a mut crate::W<REG> {
        self.variant(HaltDma::HaltDma1)
    }
}
impl R {
    #[doc = "Bit 0 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_0(&self) -> HaltT16_0R {
        HaltT16_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_1(&self) -> HaltT16_1R {
        HaltT16_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_2(&self) -> HaltT16_2R {
        HaltT16_2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_3(&self) -> HaltT16_3R {
        HaltT16_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t32_0(&self) -> HaltT32_0R {
        HaltT32_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua0(&self) -> HaltEUa0R {
        HaltEUa0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua1(&self) -> HaltEUa1R {
        HaltEUa1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua2(&self) -> HaltEUa2R {
        HaltEUa2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua3(&self) -> HaltEUa3R {
        HaltEUa3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub0(&self) -> HaltEUb0R {
        HaltEUb0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub1(&self) -> HaltEUb1R {
        HaltEUb1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub2(&self) -> HaltEUb2R {
        HaltEUb2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub3(&self) -> HaltEUb3R {
        HaltEUb3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_adc(&self) -> HaltAdcR {
        HaltAdcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_wdt(&self) -> HaltWdtR {
        HaltWdtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_dma(&self) -> HaltDmaR {
        HaltDmaR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_0(&mut self) -> HaltT16_0W<SysPerihaltCtlSpec> {
        HaltT16_0W::new(self, 0)
    }
    #[doc = "Bit 1 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_1(&mut self) -> HaltT16_1W<SysPerihaltCtlSpec> {
        HaltT16_1W::new(self, 1)
    }
    #[doc = "Bit 2 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_2(&mut self) -> HaltT16_2W<SysPerihaltCtlSpec> {
        HaltT16_2W::new(self, 2)
    }
    #[doc = "Bit 3 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_3(&mut self) -> HaltT16_3W<SysPerihaltCtlSpec> {
        HaltT16_3W::new(self, 3)
    }
    #[doc = "Bit 4 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t32_0(&mut self) -> HaltT32_0W<SysPerihaltCtlSpec> {
        HaltT32_0W::new(self, 4)
    }
    #[doc = "Bit 5 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua0(&mut self) -> HaltEUa0W<SysPerihaltCtlSpec> {
        HaltEUa0W::new(self, 5)
    }
    #[doc = "Bit 6 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua1(&mut self) -> HaltEUa1W<SysPerihaltCtlSpec> {
        HaltEUa1W::new(self, 6)
    }
    #[doc = "Bit 7 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua2(&mut self) -> HaltEUa2W<SysPerihaltCtlSpec> {
        HaltEUa2W::new(self, 7)
    }
    #[doc = "Bit 8 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua3(&mut self) -> HaltEUa3W<SysPerihaltCtlSpec> {
        HaltEUa3W::new(self, 8)
    }
    #[doc = "Bit 9 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub0(&mut self) -> HaltEUb0W<SysPerihaltCtlSpec> {
        HaltEUb0W::new(self, 9)
    }
    #[doc = "Bit 10 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub1(&mut self) -> HaltEUb1W<SysPerihaltCtlSpec> {
        HaltEUb1W::new(self, 10)
    }
    #[doc = "Bit 11 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub2(&mut self) -> HaltEUb2W<SysPerihaltCtlSpec> {
        HaltEUb2W::new(self, 11)
    }
    #[doc = "Bit 12 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub3(&mut self) -> HaltEUb3W<SysPerihaltCtlSpec> {
        HaltEUb3W::new(self, 12)
    }
    #[doc = "Bit 13 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_adc(&mut self) -> HaltAdcW<SysPerihaltCtlSpec> {
        HaltAdcW::new(self, 13)
    }
    #[doc = "Bit 14 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_wdt(&mut self) -> HaltWdtW<SysPerihaltCtlSpec> {
        HaltWdtW::new(self, 14)
    }
    #[doc = "Bit 15 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_dma(&mut self) -> HaltDmaW<SysPerihaltCtlSpec> {
        HaltDmaW::new(self, 15)
    }
}
#[doc = "Peripheral Halt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_perihalt_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_perihalt_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysPerihaltCtlSpec;
impl crate::RegisterSpec for SysPerihaltCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_perihalt_ctl::R`](R) reader structure"]
impl crate::Readable for SysPerihaltCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_perihalt_ctl::W`](W) writer structure"]
impl crate::Writable for SysPerihaltCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_PERIHALT_CTL to value 0x4000"]
impl crate::Resettable for SysPerihaltCtlSpec {
    const RESET_VALUE: u32 = 0x4000;
}
