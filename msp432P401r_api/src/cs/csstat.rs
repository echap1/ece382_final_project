#[doc = "Register `CSSTAT` reader"]
pub type R = crate::R<CsstatSpec>;
#[doc = "DCO status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcoOnEnumRead {
    #[doc = "0: Inactive"]
    DcoOn0 = 0,
    #[doc = "1: Active"]
    DcoOn1 = 1,
}
impl From<DcoOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: DcoOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCO_ON` reader - DCO status"]
pub type DcoOnR = crate::BitReader<DcoOnEnumRead>;
impl DcoOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcoOnEnumRead {
        match self.bits {
            false => DcoOnEnumRead::DcoOn0,
            true => DcoOnEnumRead::DcoOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_dco_on_0(&self) -> bool {
        *self == DcoOnEnumRead::DcoOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_dco_on_1(&self) -> bool {
        *self == DcoOnEnumRead::DcoOn1
    }
}
#[doc = "DCO bias status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcobiasOnEnumRead {
    #[doc = "0: Inactive"]
    DcobiasOn0 = 0,
    #[doc = "1: Active"]
    DcobiasOn1 = 1,
}
impl From<DcobiasOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: DcobiasOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCOBIAS_ON` reader - DCO bias status"]
pub type DcobiasOnR = crate::BitReader<DcobiasOnEnumRead>;
impl DcobiasOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcobiasOnEnumRead {
        match self.bits {
            false => DcobiasOnEnumRead::DcobiasOn0,
            true => DcobiasOnEnumRead::DcobiasOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_dcobias_on_0(&self) -> bool {
        *self == DcobiasOnEnumRead::DcobiasOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_dcobias_on_1(&self) -> bool {
        *self == DcobiasOnEnumRead::DcobiasOn1
    }
}
#[doc = "HFXT status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HfxtOnEnumRead {
    #[doc = "0: Inactive"]
    HfxtOn0 = 0,
    #[doc = "1: Active"]
    HfxtOn1 = 1,
}
impl From<HfxtOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: HfxtOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXT_ON` reader - HFXT status"]
pub type HfxtOnR = crate::BitReader<HfxtOnEnumRead>;
impl HfxtOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HfxtOnEnumRead {
        match self.bits {
            false => HfxtOnEnumRead::HfxtOn0,
            true => HfxtOnEnumRead::HfxtOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_hfxt_on_0(&self) -> bool {
        *self == HfxtOnEnumRead::HfxtOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_hfxt_on_1(&self) -> bool {
        *self == HfxtOnEnumRead::HfxtOn1
    }
}
#[doc = "HFXT2 status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfxt2OnEnumRead {
    #[doc = "0: Inactive"]
    Hfxt2On0 = 0,
    #[doc = "1: Active"]
    Hfxt2On1 = 1,
}
impl From<Hfxt2OnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: Hfxt2OnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXT2_ON` reader - HFXT2 status"]
pub type Hfxt2OnR = crate::BitReader<Hfxt2OnEnumRead>;
impl Hfxt2OnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxt2OnEnumRead {
        match self.bits {
            false => Hfxt2OnEnumRead::Hfxt2On0,
            true => Hfxt2OnEnumRead::Hfxt2On1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_hfxt2_on_0(&self) -> bool {
        *self == Hfxt2OnEnumRead::Hfxt2On0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_hfxt2_on_1(&self) -> bool {
        *self == Hfxt2OnEnumRead::Hfxt2On1
    }
}
#[doc = "MODOSC status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModoscOnEnumRead {
    #[doc = "0: Inactive"]
    ModoscOn0 = 0,
    #[doc = "1: Active"]
    ModoscOn1 = 1,
}
impl From<ModoscOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: ModoscOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODOSC_ON` reader - MODOSC status"]
pub type ModoscOnR = crate::BitReader<ModoscOnEnumRead>;
impl ModoscOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ModoscOnEnumRead {
        match self.bits {
            false => ModoscOnEnumRead::ModoscOn0,
            true => ModoscOnEnumRead::ModoscOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_modosc_on_0(&self) -> bool {
        *self == ModoscOnEnumRead::ModoscOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_modosc_on_1(&self) -> bool {
        *self == ModoscOnEnumRead::ModoscOn1
    }
}
#[doc = "VLO status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VloOnEnumRead {
    #[doc = "0: Inactive"]
    VloOn0 = 0,
    #[doc = "1: Active"]
    VloOn1 = 1,
}
impl From<VloOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: VloOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLO_ON` reader - VLO status"]
pub type VloOnR = crate::BitReader<VloOnEnumRead>;
impl VloOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VloOnEnumRead {
        match self.bits {
            false => VloOnEnumRead::VloOn0,
            true => VloOnEnumRead::VloOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_vlo_on_0(&self) -> bool {
        *self == VloOnEnumRead::VloOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_vlo_on_1(&self) -> bool {
        *self == VloOnEnumRead::VloOn1
    }
}
#[doc = "LFXT status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfxtOnEnumRead {
    #[doc = "0: Inactive"]
    LfxtOn0 = 0,
    #[doc = "1: Active"]
    LfxtOn1 = 1,
}
impl From<LfxtOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: LfxtOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXT_ON` reader - LFXT status"]
pub type LfxtOnR = crate::BitReader<LfxtOnEnumRead>;
impl LfxtOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LfxtOnEnumRead {
        match self.bits {
            false => LfxtOnEnumRead::LfxtOn0,
            true => LfxtOnEnumRead::LfxtOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_lfxt_on_0(&self) -> bool {
        *self == LfxtOnEnumRead::LfxtOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_lfxt_on_1(&self) -> bool {
        *self == LfxtOnEnumRead::LfxtOn1
    }
}
#[doc = "REFO status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefoOnEnumRead {
    #[doc = "0: Inactive"]
    RefoOn0 = 0,
    #[doc = "1: Active"]
    RefoOn1 = 1,
}
impl From<RefoOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: RefoOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFO_ON` reader - REFO status"]
pub type RefoOnR = crate::BitReader<RefoOnEnumRead>;
impl RefoOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefoOnEnumRead {
        match self.bits {
            false => RefoOnEnumRead::RefoOn0,
            true => RefoOnEnumRead::RefoOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_refo_on_0(&self) -> bool {
        *self == RefoOnEnumRead::RefoOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_refo_on_1(&self) -> bool {
        *self == RefoOnEnumRead::RefoOn1
    }
}
#[doc = "ACLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AclkOnEnumRead {
    #[doc = "0: Inactive"]
    AclkOn0 = 0,
    #[doc = "1: Active"]
    AclkOn1 = 1,
}
impl From<AclkOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: AclkOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLK_ON` reader - ACLK system clock status"]
pub type AclkOnR = crate::BitReader<AclkOnEnumRead>;
impl AclkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkOnEnumRead {
        match self.bits {
            false => AclkOnEnumRead::AclkOn0,
            true => AclkOnEnumRead::AclkOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_aclk_on_0(&self) -> bool {
        *self == AclkOnEnumRead::AclkOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_aclk_on_1(&self) -> bool {
        *self == AclkOnEnumRead::AclkOn1
    }
}
#[doc = "MCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MclkOnEnumRead {
    #[doc = "0: Inactive"]
    MclkOn0 = 0,
    #[doc = "1: Active"]
    MclkOn1 = 1,
}
impl From<MclkOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: MclkOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLK_ON` reader - MCLK system clock status"]
pub type MclkOnR = crate::BitReader<MclkOnEnumRead>;
impl MclkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MclkOnEnumRead {
        match self.bits {
            false => MclkOnEnumRead::MclkOn0,
            true => MclkOnEnumRead::MclkOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_mclk_on_0(&self) -> bool {
        *self == MclkOnEnumRead::MclkOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_mclk_on_1(&self) -> bool {
        *self == MclkOnEnumRead::MclkOn1
    }
}
#[doc = "HSMCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsmclkOnEnumRead {
    #[doc = "0: Inactive"]
    HsmclkOn0 = 0,
    #[doc = "1: Active"]
    HsmclkOn1 = 1,
}
impl From<HsmclkOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: HsmclkOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSMCLK_ON` reader - HSMCLK system clock status"]
pub type HsmclkOnR = crate::BitReader<HsmclkOnEnumRead>;
impl HsmclkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsmclkOnEnumRead {
        match self.bits {
            false => HsmclkOnEnumRead::HsmclkOn0,
            true => HsmclkOnEnumRead::HsmclkOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_hsmclk_on_0(&self) -> bool {
        *self == HsmclkOnEnumRead::HsmclkOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_hsmclk_on_1(&self) -> bool {
        *self == HsmclkOnEnumRead::HsmclkOn1
    }
}
#[doc = "SMCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmclkOnEnumRead {
    #[doc = "0: Inactive"]
    SmclkOn0 = 0,
    #[doc = "1: Active"]
    SmclkOn1 = 1,
}
impl From<SmclkOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: SmclkOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMCLK_ON` reader - SMCLK system clock status"]
pub type SmclkOnR = crate::BitReader<SmclkOnEnumRead>;
impl SmclkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmclkOnEnumRead {
        match self.bits {
            false => SmclkOnEnumRead::SmclkOn0,
            true => SmclkOnEnumRead::SmclkOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_smclk_on_0(&self) -> bool {
        *self == SmclkOnEnumRead::SmclkOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_smclk_on_1(&self) -> bool {
        *self == SmclkOnEnumRead::SmclkOn1
    }
}
#[doc = "MODCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModclkOnEnumRead {
    #[doc = "0: Inactive"]
    ModclkOn0 = 0,
    #[doc = "1: Active"]
    ModclkOn1 = 1,
}
impl From<ModclkOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: ModclkOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODCLK_ON` reader - MODCLK system clock status"]
pub type ModclkOnR = crate::BitReader<ModclkOnEnumRead>;
impl ModclkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ModclkOnEnumRead {
        match self.bits {
            false => ModclkOnEnumRead::ModclkOn0,
            true => ModclkOnEnumRead::ModclkOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_modclk_on_0(&self) -> bool {
        *self == ModclkOnEnumRead::ModclkOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_modclk_on_1(&self) -> bool {
        *self == ModclkOnEnumRead::ModclkOn1
    }
}
#[doc = "VLOCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VloclkOnEnumRead {
    #[doc = "0: Inactive"]
    VloclkOn0 = 0,
    #[doc = "1: Active"]
    VloclkOn1 = 1,
}
impl From<VloclkOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: VloclkOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLOCLK_ON` reader - VLOCLK system clock status"]
pub type VloclkOnR = crate::BitReader<VloclkOnEnumRead>;
impl VloclkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VloclkOnEnumRead {
        match self.bits {
            false => VloclkOnEnumRead::VloclkOn0,
            true => VloclkOnEnumRead::VloclkOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_vloclk_on_0(&self) -> bool {
        *self == VloclkOnEnumRead::VloclkOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_vloclk_on_1(&self) -> bool {
        *self == VloclkOnEnumRead::VloclkOn1
    }
}
#[doc = "LFXTCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfxtclkOnEnumRead {
    #[doc = "0: Inactive"]
    LfxtclkOn0 = 0,
    #[doc = "1: Active"]
    LfxtclkOn1 = 1,
}
impl From<LfxtclkOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: LfxtclkOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTCLK_ON` reader - LFXTCLK system clock status"]
pub type LfxtclkOnR = crate::BitReader<LfxtclkOnEnumRead>;
impl LfxtclkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LfxtclkOnEnumRead {
        match self.bits {
            false => LfxtclkOnEnumRead::LfxtclkOn0,
            true => LfxtclkOnEnumRead::LfxtclkOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_lfxtclk_on_0(&self) -> bool {
        *self == LfxtclkOnEnumRead::LfxtclkOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_lfxtclk_on_1(&self) -> bool {
        *self == LfxtclkOnEnumRead::LfxtclkOn1
    }
}
#[doc = "REFOCLK system clock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefoclkOnEnumRead {
    #[doc = "0: Inactive"]
    RefoclkOn0 = 0,
    #[doc = "1: Active"]
    RefoclkOn1 = 1,
}
impl From<RefoclkOnEnumRead> for bool {
    #[inline(always)]
    fn from(variant: RefoclkOnEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFOCLK_ON` reader - REFOCLK system clock status"]
pub type RefoclkOnR = crate::BitReader<RefoclkOnEnumRead>;
impl RefoclkOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefoclkOnEnumRead {
        match self.bits {
            false => RefoclkOnEnumRead::RefoclkOn0,
            true => RefoclkOnEnumRead::RefoclkOn1,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_refoclk_on_0(&self) -> bool {
        *self == RefoclkOnEnumRead::RefoclkOn0
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_refoclk_on_1(&self) -> bool {
        *self == RefoclkOnEnumRead::RefoclkOn1
    }
}
#[doc = "ACLK Ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AclkReadyEnumRead {
    #[doc = "0: Not ready"]
    AclkReady0 = 0,
    #[doc = "1: Ready"]
    AclkReady1 = 1,
}
impl From<AclkReadyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: AclkReadyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLK_READY` reader - ACLK Ready status"]
pub type AclkReadyR = crate::BitReader<AclkReadyEnumRead>;
impl AclkReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkReadyEnumRead {
        match self.bits {
            false => AclkReadyEnumRead::AclkReady0,
            true => AclkReadyEnumRead::AclkReady1,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_aclk_ready_0(&self) -> bool {
        *self == AclkReadyEnumRead::AclkReady0
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_aclk_ready_1(&self) -> bool {
        *self == AclkReadyEnumRead::AclkReady1
    }
}
#[doc = "MCLK Ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MclkReadyEnumRead {
    #[doc = "0: Not ready"]
    MclkReady0 = 0,
    #[doc = "1: Ready"]
    MclkReady1 = 1,
}
impl From<MclkReadyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: MclkReadyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLK_READY` reader - MCLK Ready status"]
pub type MclkReadyR = crate::BitReader<MclkReadyEnumRead>;
impl MclkReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MclkReadyEnumRead {
        match self.bits {
            false => MclkReadyEnumRead::MclkReady0,
            true => MclkReadyEnumRead::MclkReady1,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_mclk_ready_0(&self) -> bool {
        *self == MclkReadyEnumRead::MclkReady0
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_mclk_ready_1(&self) -> bool {
        *self == MclkReadyEnumRead::MclkReady1
    }
}
#[doc = "HSMCLK Ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsmclkReadyEnumRead {
    #[doc = "0: Not ready"]
    HsmclkReady0 = 0,
    #[doc = "1: Ready"]
    HsmclkReady1 = 1,
}
impl From<HsmclkReadyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: HsmclkReadyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSMCLK_READY` reader - HSMCLK Ready status"]
pub type HsmclkReadyR = crate::BitReader<HsmclkReadyEnumRead>;
impl HsmclkReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsmclkReadyEnumRead {
        match self.bits {
            false => HsmclkReadyEnumRead::HsmclkReady0,
            true => HsmclkReadyEnumRead::HsmclkReady1,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_hsmclk_ready_0(&self) -> bool {
        *self == HsmclkReadyEnumRead::HsmclkReady0
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_hsmclk_ready_1(&self) -> bool {
        *self == HsmclkReadyEnumRead::HsmclkReady1
    }
}
#[doc = "SMCLK Ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmclkReadyEnumRead {
    #[doc = "0: Not ready"]
    SmclkReady0 = 0,
    #[doc = "1: Ready"]
    SmclkReady1 = 1,
}
impl From<SmclkReadyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: SmclkReadyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMCLK_READY` reader - SMCLK Ready status"]
pub type SmclkReadyR = crate::BitReader<SmclkReadyEnumRead>;
impl SmclkReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmclkReadyEnumRead {
        match self.bits {
            false => SmclkReadyEnumRead::SmclkReady0,
            true => SmclkReadyEnumRead::SmclkReady1,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_smclk_ready_0(&self) -> bool {
        *self == SmclkReadyEnumRead::SmclkReady0
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_smclk_ready_1(&self) -> bool {
        *self == SmclkReadyEnumRead::SmclkReady1
    }
}
#[doc = "BCLK Ready status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BclkReadyEnumRead {
    #[doc = "0: Not ready"]
    BclkReady0 = 0,
    #[doc = "1: Ready"]
    BclkReady1 = 1,
}
impl From<BclkReadyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: BclkReadyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLK_READY` reader - BCLK Ready status"]
pub type BclkReadyR = crate::BitReader<BclkReadyEnumRead>;
impl BclkReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BclkReadyEnumRead {
        match self.bits {
            false => BclkReadyEnumRead::BclkReady0,
            true => BclkReadyEnumRead::BclkReady1,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_bclk_ready_0(&self) -> bool {
        *self == BclkReadyEnumRead::BclkReady0
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_bclk_ready_1(&self) -> bool {
        *self == BclkReadyEnumRead::BclkReady1
    }
}
impl R {
    #[doc = "Bit 0 - DCO status"]
    #[inline(always)]
    pub fn dco_on(&self) -> DcoOnR {
        DcoOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCO bias status"]
    #[inline(always)]
    pub fn dcobias_on(&self) -> DcobiasOnR {
        DcobiasOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HFXT status"]
    #[inline(always)]
    pub fn hfxt_on(&self) -> HfxtOnR {
        HfxtOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HFXT2 status"]
    #[inline(always)]
    pub fn hfxt2_on(&self) -> Hfxt2OnR {
        Hfxt2OnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MODOSC status"]
    #[inline(always)]
    pub fn modosc_on(&self) -> ModoscOnR {
        ModoscOnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VLO status"]
    #[inline(always)]
    pub fn vlo_on(&self) -> VloOnR {
        VloOnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LFXT status"]
    #[inline(always)]
    pub fn lfxt_on(&self) -> LfxtOnR {
        LfxtOnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REFO status"]
    #[inline(always)]
    pub fn refo_on(&self) -> RefoOnR {
        RefoOnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - ACLK system clock status"]
    #[inline(always)]
    pub fn aclk_on(&self) -> AclkOnR {
        AclkOnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MCLK system clock status"]
    #[inline(always)]
    pub fn mclk_on(&self) -> MclkOnR {
        MclkOnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSMCLK system clock status"]
    #[inline(always)]
    pub fn hsmclk_on(&self) -> HsmclkOnR {
        HsmclkOnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SMCLK system clock status"]
    #[inline(always)]
    pub fn smclk_on(&self) -> SmclkOnR {
        SmclkOnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MODCLK system clock status"]
    #[inline(always)]
    pub fn modclk_on(&self) -> ModclkOnR {
        ModclkOnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VLOCLK system clock status"]
    #[inline(always)]
    pub fn vloclk_on(&self) -> VloclkOnR {
        VloclkOnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LFXTCLK system clock status"]
    #[inline(always)]
    pub fn lfxtclk_on(&self) -> LfxtclkOnR {
        LfxtclkOnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - REFOCLK system clock status"]
    #[inline(always)]
    pub fn refoclk_on(&self) -> RefoclkOnR {
        RefoclkOnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ACLK Ready status"]
    #[inline(always)]
    pub fn aclk_ready(&self) -> AclkReadyR {
        AclkReadyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MCLK Ready status"]
    #[inline(always)]
    pub fn mclk_ready(&self) -> MclkReadyR {
        MclkReadyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - HSMCLK Ready status"]
    #[inline(always)]
    pub fn hsmclk_ready(&self) -> HsmclkReadyR {
        HsmclkReadyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SMCLK Ready status"]
    #[inline(always)]
    pub fn smclk_ready(&self) -> SmclkReadyR {
        SmclkReadyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - BCLK Ready status"]
    #[inline(always)]
    pub fn bclk_ready(&self) -> BclkReadyR {
        BclkReadyR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsstatSpec;
impl crate::RegisterSpec for CsstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csstat::R`](R) reader structure"]
impl crate::Readable for CsstatSpec {}
#[doc = "`reset()` method sets CSSTAT to value 0x03"]
impl crate::Resettable for CsstatSpec {
    const RESET_VALUE: u32 = 0x03;
}
