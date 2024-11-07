#[doc = "Register `REFCTL0` reader"]
pub type R = crate::R<Refctl0Spec>;
#[doc = "Register `REFCTL0` writer"]
pub type W = crate::W<Refctl0Spec>;
#[doc = "Reference enable"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refon {
    #[doc = "0: Disables reference if no other reference requests are pending"]
    Refon0 = 0,
    #[doc = "1: Enables reference in static mode"]
    Refon1 = 1,
}
impl From<Refon> for bool {
    #[inline(always)]
    fn from(variant: Refon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFON` reader - Reference enable"]
pub type RefonR = crate::BitReader<Refon>;
impl RefonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refon {
        match self.bits {
            false => Refon::Refon0,
            true => Refon::Refon1,
        }
    }
    #[doc = "Disables reference if no other reference requests are pending"]
    #[inline(always)]
    pub fn is_refon_0(&self) -> bool {
        *self == Refon::Refon0
    }
    #[doc = "Enables reference in static mode"]
    #[inline(always)]
    pub fn is_refon_1(&self) -> bool {
        *self == Refon::Refon1
    }
}
#[doc = "Field `REFON` writer - Reference enable"]
pub type RefonW<'a, REG> = crate::BitWriter<'a, REG, Refon>;
impl<'a, REG> RefonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables reference if no other reference requests are pending"]
    #[inline(always)]
    pub fn refon_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refon::Refon0)
    }
    #[doc = "Enables reference in static mode"]
    #[inline(always)]
    pub fn refon_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refon::Refon1)
    }
}
#[doc = "Reference output buffer"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refout {
    #[doc = "0: Reference output not available externally"]
    Refout0 = 0,
    #[doc = "1: Reference output available externally. If ADC14REFBURST = 0, output is available continuously. If ADC14REFBURST = 1, output is available only during an ADC14 conversion."]
    Refout1 = 1,
}
impl From<Refout> for bool {
    #[inline(always)]
    fn from(variant: Refout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFOUT` reader - Reference output buffer"]
pub type RefoutR = crate::BitReader<Refout>;
impl RefoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refout {
        match self.bits {
            false => Refout::Refout0,
            true => Refout::Refout1,
        }
    }
    #[doc = "Reference output not available externally"]
    #[inline(always)]
    pub fn is_refout_0(&self) -> bool {
        *self == Refout::Refout0
    }
    #[doc = "Reference output available externally. If ADC14REFBURST = 0, output is available continuously. If ADC14REFBURST = 1, output is available only during an ADC14 conversion."]
    #[inline(always)]
    pub fn is_refout_1(&self) -> bool {
        *self == Refout::Refout1
    }
}
#[doc = "Field `REFOUT` writer - Reference output buffer"]
pub type RefoutW<'a, REG> = crate::BitWriter<'a, REG, Refout>;
impl<'a, REG> RefoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reference output not available externally"]
    #[inline(always)]
    pub fn refout_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refout::Refout0)
    }
    #[doc = "Reference output available externally. If ADC14REFBURST = 0, output is available continuously. If ADC14REFBURST = 1, output is available only during an ADC14 conversion."]
    #[inline(always)]
    pub fn refout_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refout::Refout1)
    }
}
#[doc = "Temperature sensor disabled"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reftcoff {
    #[doc = "0: Temperature sensor enabled"]
    Reftcoff0 = 0,
    #[doc = "1: Temperature sensor disabled to save power"]
    Reftcoff1 = 1,
}
impl From<Reftcoff> for bool {
    #[inline(always)]
    fn from(variant: Reftcoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFTCOFF` reader - Temperature sensor disabled"]
pub type ReftcoffR = crate::BitReader<Reftcoff>;
impl ReftcoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reftcoff {
        match self.bits {
            false => Reftcoff::Reftcoff0,
            true => Reftcoff::Reftcoff1,
        }
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn is_reftcoff_0(&self) -> bool {
        *self == Reftcoff::Reftcoff0
    }
    #[doc = "Temperature sensor disabled to save power"]
    #[inline(always)]
    pub fn is_reftcoff_1(&self) -> bool {
        *self == Reftcoff::Reftcoff1
    }
}
#[doc = "Field `REFTCOFF` writer - Temperature sensor disabled"]
pub type ReftcoffW<'a, REG> = crate::BitWriter<'a, REG, Reftcoff>;
impl<'a, REG> ReftcoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn reftcoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Reftcoff::Reftcoff0)
    }
    #[doc = "Temperature sensor disabled to save power"]
    #[inline(always)]
    pub fn reftcoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Reftcoff::Reftcoff1)
    }
}
#[doc = "Reference voltage level select"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refvsel {
    #[doc = "0: 1.2 V available when reference requested or REFON = 1"]
    Refvsel0 = 0,
    #[doc = "1: 1.45 V available when reference requested or REFON = 1"]
    Refvsel1 = 1,
    #[doc = "3: 2.5 V available when reference requested or REFON = 1"]
    Refvsel3 = 3,
}
impl From<Refvsel> for u8 {
    #[inline(always)]
    fn from(variant: Refvsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refvsel {
    type Ux = u8;
}
impl crate::IsEnum for Refvsel {}
#[doc = "Field `REFVSEL` reader - Reference voltage level select"]
pub type RefvselR = crate::FieldReader<Refvsel>;
impl RefvselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refvsel> {
        match self.bits {
            0 => Some(Refvsel::Refvsel0),
            1 => Some(Refvsel::Refvsel1),
            3 => Some(Refvsel::Refvsel3),
            _ => None,
        }
    }
    #[doc = "1.2 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn is_refvsel_0(&self) -> bool {
        *self == Refvsel::Refvsel0
    }
    #[doc = "1.45 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        *self == Refvsel::Refvsel1
    }
    #[doc = "2.5 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        *self == Refvsel::Refvsel3
    }
}
#[doc = "Field `REFVSEL` writer - Reference voltage level select"]
pub type RefvselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refvsel>;
impl<'a, REG> RefvselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.2 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn refvsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::Refvsel0)
    }
    #[doc = "1.45 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn refvsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::Refvsel1)
    }
    #[doc = "2.5 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn refvsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Refvsel::Refvsel3)
    }
}
#[doc = "Reference generator one-time trigger"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refgenot {
    #[doc = "0: No trigger"]
    Refgenot0 = 0,
    #[doc = "1: Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    Refgenot1 = 1,
}
impl From<Refgenot> for bool {
    #[inline(always)]
    fn from(variant: Refgenot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFGENOT` reader - Reference generator one-time trigger"]
pub type RefgenotR = crate::BitReader<Refgenot>;
impl RefgenotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refgenot {
        match self.bits {
            false => Refgenot::Refgenot0,
            true => Refgenot::Refgenot1,
        }
    }
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn is_refgenot_0(&self) -> bool {
        *self == Refgenot::Refgenot0
    }
    #[doc = "Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn is_refgenot_1(&self) -> bool {
        *self == Refgenot::Refgenot1
    }
}
#[doc = "Field `REFGENOT` writer - Reference generator one-time trigger"]
pub type RefgenotW<'a, REG> = crate::BitWriter<'a, REG, Refgenot>;
impl<'a, REG> RefgenotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn refgenot_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refgenot::Refgenot0)
    }
    #[doc = "Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn refgenot_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refgenot::Refgenot1)
    }
}
#[doc = "Bandgap and bandgap buffer one-time trigger"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refbgot {
    #[doc = "0: No trigger"]
    Refbgot0 = 0,
    #[doc = "1: Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    Refbgot1 = 1,
}
impl From<Refbgot> for bool {
    #[inline(always)]
    fn from(variant: Refbgot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFBGOT` reader - Bandgap and bandgap buffer one-time trigger"]
pub type RefbgotR = crate::BitReader<Refbgot>;
impl RefbgotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refbgot {
        match self.bits {
            false => Refbgot::Refbgot0,
            true => Refbgot::Refbgot1,
        }
    }
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn is_refbgot_0(&self) -> bool {
        *self == Refbgot::Refbgot0
    }
    #[doc = "Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn is_refbgot_1(&self) -> bool {
        *self == Refbgot::Refbgot1
    }
}
#[doc = "Field `REFBGOT` writer - Bandgap and bandgap buffer one-time trigger"]
pub type RefbgotW<'a, REG> = crate::BitWriter<'a, REG, Refbgot>;
impl<'a, REG> RefbgotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn refbgot_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refbgot::Refbgot0)
    }
    #[doc = "Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn refbgot_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refbgot::Refbgot1)
    }
}
#[doc = "Reference generator active"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefgenactEnumRead {
    #[doc = "0: Reference generator not active"]
    Refgenact0 = 0,
    #[doc = "1: Reference generator active"]
    Refgenact1 = 1,
}
impl From<RefgenactEnumRead> for bool {
    #[inline(always)]
    fn from(variant: RefgenactEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFGENACT` reader - Reference generator active"]
pub type RefgenactR = crate::BitReader<RefgenactEnumRead>;
impl RefgenactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefgenactEnumRead {
        match self.bits {
            false => RefgenactEnumRead::Refgenact0,
            true => RefgenactEnumRead::Refgenact1,
        }
    }
    #[doc = "Reference generator not active"]
    #[inline(always)]
    pub fn is_refgenact_0(&self) -> bool {
        *self == RefgenactEnumRead::Refgenact0
    }
    #[doc = "Reference generator active"]
    #[inline(always)]
    pub fn is_refgenact_1(&self) -> bool {
        *self == RefgenactEnumRead::Refgenact1
    }
}
#[doc = "Reference bandgap active"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefbgactEnumRead {
    #[doc = "0: Reference bandgap buffer not active"]
    Refbgact0 = 0,
    #[doc = "1: Reference bandgap buffer active"]
    Refbgact1 = 1,
}
impl From<RefbgactEnumRead> for bool {
    #[inline(always)]
    fn from(variant: RefbgactEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFBGACT` reader - Reference bandgap active"]
pub type RefbgactR = crate::BitReader<RefbgactEnumRead>;
impl RefbgactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefbgactEnumRead {
        match self.bits {
            false => RefbgactEnumRead::Refbgact0,
            true => RefbgactEnumRead::Refbgact1,
        }
    }
    #[doc = "Reference bandgap buffer not active"]
    #[inline(always)]
    pub fn is_refbgact_0(&self) -> bool {
        *self == RefbgactEnumRead::Refbgact0
    }
    #[doc = "Reference bandgap buffer active"]
    #[inline(always)]
    pub fn is_refbgact_1(&self) -> bool {
        *self == RefbgactEnumRead::Refbgact1
    }
}
#[doc = "Reference generator busy"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefgenbusyEnumRead {
    #[doc = "0: Reference generator not busy"]
    Refgenbusy0 = 0,
    #[doc = "1: Reference generator busy"]
    Refgenbusy1 = 1,
}
impl From<RefgenbusyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: RefgenbusyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFGENBUSY` reader - Reference generator busy"]
pub type RefgenbusyR = crate::BitReader<RefgenbusyEnumRead>;
impl RefgenbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefgenbusyEnumRead {
        match self.bits {
            false => RefgenbusyEnumRead::Refgenbusy0,
            true => RefgenbusyEnumRead::Refgenbusy1,
        }
    }
    #[doc = "Reference generator not busy"]
    #[inline(always)]
    pub fn is_refgenbusy_0(&self) -> bool {
        *self == RefgenbusyEnumRead::Refgenbusy0
    }
    #[doc = "Reference generator busy"]
    #[inline(always)]
    pub fn is_refgenbusy_1(&self) -> bool {
        *self == RefgenbusyEnumRead::Refgenbusy1
    }
}
#[doc = "Bandgap mode"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BgmodeEnumRead {
    #[doc = "0: Static mode"]
    Bgmode0 = 0,
    #[doc = "1: Sampled mode"]
    Bgmode1 = 1,
}
impl From<BgmodeEnumRead> for bool {
    #[inline(always)]
    fn from(variant: BgmodeEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGMODE` reader - Bandgap mode"]
pub type BgmodeR = crate::BitReader<BgmodeEnumRead>;
impl BgmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BgmodeEnumRead {
        match self.bits {
            false => BgmodeEnumRead::Bgmode0,
            true => BgmodeEnumRead::Bgmode1,
        }
    }
    #[doc = "Static mode"]
    #[inline(always)]
    pub fn is_bgmode_0(&self) -> bool {
        *self == BgmodeEnumRead::Bgmode0
    }
    #[doc = "Sampled mode"]
    #[inline(always)]
    pub fn is_bgmode_1(&self) -> bool {
        *self == BgmodeEnumRead::Bgmode1
    }
}
#[doc = "Variable reference voltage ready status"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefgenrdyEnumRead {
    #[doc = "0: Reference voltage output is not ready to be used"]
    Refgenrdy0 = 0,
    #[doc = "1: Reference voltage output is ready to be used"]
    Refgenrdy1 = 1,
}
impl From<RefgenrdyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: RefgenrdyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFGENRDY` reader - Variable reference voltage ready status"]
pub type RefgenrdyR = crate::BitReader<RefgenrdyEnumRead>;
impl RefgenrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefgenrdyEnumRead {
        match self.bits {
            false => RefgenrdyEnumRead::Refgenrdy0,
            true => RefgenrdyEnumRead::Refgenrdy1,
        }
    }
    #[doc = "Reference voltage output is not ready to be used"]
    #[inline(always)]
    pub fn is_refgenrdy_0(&self) -> bool {
        *self == RefgenrdyEnumRead::Refgenrdy0
    }
    #[doc = "Reference voltage output is ready to be used"]
    #[inline(always)]
    pub fn is_refgenrdy_1(&self) -> bool {
        *self == RefgenrdyEnumRead::Refgenrdy1
    }
}
#[doc = "Buffered bandgap voltage ready status"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefbgrdyEnumRead {
    #[doc = "0: Buffered bandgap voltage is not ready to be used"]
    Refbgrdy0 = 0,
    #[doc = "1: Buffered bandgap voltage is ready to be used"]
    Refbgrdy1 = 1,
}
impl From<RefbgrdyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: RefbgrdyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFBGRDY` reader - Buffered bandgap voltage ready status"]
pub type RefbgrdyR = crate::BitReader<RefbgrdyEnumRead>;
impl RefbgrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefbgrdyEnumRead {
        match self.bits {
            false => RefbgrdyEnumRead::Refbgrdy0,
            true => RefbgrdyEnumRead::Refbgrdy1,
        }
    }
    #[doc = "Buffered bandgap voltage is not ready to be used"]
    #[inline(always)]
    pub fn is_refbgrdy_0(&self) -> bool {
        *self == RefbgrdyEnumRead::Refbgrdy0
    }
    #[doc = "Buffered bandgap voltage is ready to be used"]
    #[inline(always)]
    pub fn is_refbgrdy_1(&self) -> bool {
        *self == RefbgrdyEnumRead::Refbgrdy1
    }
}
impl R {
    #[doc = "Bit 0 - Reference enable"]
    #[inline(always)]
    pub fn refon(&self) -> RefonR {
        RefonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reference output buffer"]
    #[inline(always)]
    pub fn refout(&self) -> RefoutR {
        RefoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Temperature sensor disabled"]
    #[inline(always)]
    pub fn reftcoff(&self) -> ReftcoffR {
        ReftcoffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reference voltage level select"]
    #[inline(always)]
    pub fn refvsel(&self) -> RefvselR {
        RefvselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Reference generator one-time trigger"]
    #[inline(always)]
    pub fn refgenot(&self) -> RefgenotR {
        RefgenotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer one-time trigger"]
    #[inline(always)]
    pub fn refbgot(&self) -> RefbgotR {
        RefbgotR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&self) -> RefgenactR {
        RefgenactR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&self) -> RefbgactR {
        RefbgactR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reference generator busy"]
    #[inline(always)]
    pub fn refgenbusy(&self) -> RefgenbusyR {
        RefgenbusyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&self) -> BgmodeR {
        BgmodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Variable reference voltage ready status"]
    #[inline(always)]
    pub fn refgenrdy(&self) -> RefgenrdyR {
        RefgenrdyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Buffered bandgap voltage ready status"]
    #[inline(always)]
    pub fn refbgrdy(&self) -> RefbgrdyR {
        RefbgrdyR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reference enable"]
    #[inline(always)]
    pub fn refon(&mut self) -> RefonW<Refctl0Spec> {
        RefonW::new(self, 0)
    }
    #[doc = "Bit 1 - Reference output buffer"]
    #[inline(always)]
    pub fn refout(&mut self) -> RefoutW<Refctl0Spec> {
        RefoutW::new(self, 1)
    }
    #[doc = "Bit 3 - Temperature sensor disabled"]
    #[inline(always)]
    pub fn reftcoff(&mut self) -> ReftcoffW<Refctl0Spec> {
        ReftcoffW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Reference voltage level select"]
    #[inline(always)]
    pub fn refvsel(&mut self) -> RefvselW<Refctl0Spec> {
        RefvselW::new(self, 4)
    }
    #[doc = "Bit 6 - Reference generator one-time trigger"]
    #[inline(always)]
    pub fn refgenot(&mut self) -> RefgenotW<Refctl0Spec> {
        RefgenotW::new(self, 6)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer one-time trigger"]
    #[inline(always)]
    pub fn refbgot(&mut self) -> RefbgotW<Refctl0Spec> {
        RefbgotW::new(self, 7)
    }
}
#[doc = "REF Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`refctl0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Refctl0Spec;
impl crate::RegisterSpec for Refctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`refctl0::R`](R) reader structure"]
impl crate::Readable for Refctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`refctl0::W`](W) writer structure"]
impl crate::Writable for Refctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
