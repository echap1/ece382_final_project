#[doc = "Register `CSCLKEN` reader"]
pub type R = crate::R<CsclkenSpec>;
#[doc = "Register `CSCLKEN` writer"]
pub type W = crate::W<CsclkenSpec>;
#[doc = "ACLK system clock conditional request enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AclkEn {
    #[doc = "0: ACLK disabled regardless of conditional clock requests"]
    AclkEn0 = 0,
    #[doc = "1: ACLK enabled based on any conditional clock requests"]
    AclkEn1 = 1,
}
impl From<AclkEn> for bool {
    #[inline(always)]
    fn from(variant: AclkEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLK_EN` reader - ACLK system clock conditional request enable"]
pub type AclkEnR = crate::BitReader<AclkEn>;
impl AclkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkEn {
        match self.bits {
            false => AclkEn::AclkEn0,
            true => AclkEn::AclkEn1,
        }
    }
    #[doc = "ACLK disabled regardless of conditional clock requests"]
    #[inline(always)]
    pub fn is_aclk_en_0(&self) -> bool {
        *self == AclkEn::AclkEn0
    }
    #[doc = "ACLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn is_aclk_en_1(&self) -> bool {
        *self == AclkEn::AclkEn1
    }
}
#[doc = "Field `ACLK_EN` writer - ACLK system clock conditional request enable"]
pub type AclkEnW<'a, REG> = crate::BitWriter<'a, REG, AclkEn>;
impl<'a, REG> AclkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACLK disabled regardless of conditional clock requests"]
    #[inline(always)]
    pub fn aclk_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(AclkEn::AclkEn0)
    }
    #[doc = "ACLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn aclk_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(AclkEn::AclkEn1)
    }
}
#[doc = "MCLK system clock conditional request enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MclkEn {
    #[doc = "0: MCLK disabled regardless of conditional clock requests"]
    MclkEn0 = 0,
    #[doc = "1: MCLK enabled based on any conditional clock requests"]
    MclkEn1 = 1,
}
impl From<MclkEn> for bool {
    #[inline(always)]
    fn from(variant: MclkEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLK_EN` reader - MCLK system clock conditional request enable"]
pub type MclkEnR = crate::BitReader<MclkEn>;
impl MclkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MclkEn {
        match self.bits {
            false => MclkEn::MclkEn0,
            true => MclkEn::MclkEn1,
        }
    }
    #[doc = "MCLK disabled regardless of conditional clock requests"]
    #[inline(always)]
    pub fn is_mclk_en_0(&self) -> bool {
        *self == MclkEn::MclkEn0
    }
    #[doc = "MCLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn is_mclk_en_1(&self) -> bool {
        *self == MclkEn::MclkEn1
    }
}
#[doc = "Field `MCLK_EN` writer - MCLK system clock conditional request enable"]
pub type MclkEnW<'a, REG> = crate::BitWriter<'a, REG, MclkEn>;
impl<'a, REG> MclkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCLK disabled regardless of conditional clock requests"]
    #[inline(always)]
    pub fn mclk_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(MclkEn::MclkEn0)
    }
    #[doc = "MCLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn mclk_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(MclkEn::MclkEn1)
    }
}
#[doc = "HSMCLK system clock conditional request enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsmclkEn {
    #[doc = "0: HSMCLK disabled regardless of conditional clock requests"]
    HsmclkEn0 = 0,
    #[doc = "1: HSMCLK enabled based on any conditional clock requests"]
    HsmclkEn1 = 1,
}
impl From<HsmclkEn> for bool {
    #[inline(always)]
    fn from(variant: HsmclkEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSMCLK_EN` reader - HSMCLK system clock conditional request enable"]
pub type HsmclkEnR = crate::BitReader<HsmclkEn>;
impl HsmclkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsmclkEn {
        match self.bits {
            false => HsmclkEn::HsmclkEn0,
            true => HsmclkEn::HsmclkEn1,
        }
    }
    #[doc = "HSMCLK disabled regardless of conditional clock requests"]
    #[inline(always)]
    pub fn is_hsmclk_en_0(&self) -> bool {
        *self == HsmclkEn::HsmclkEn0
    }
    #[doc = "HSMCLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn is_hsmclk_en_1(&self) -> bool {
        *self == HsmclkEn::HsmclkEn1
    }
}
#[doc = "Field `HSMCLK_EN` writer - HSMCLK system clock conditional request enable"]
pub type HsmclkEnW<'a, REG> = crate::BitWriter<'a, REG, HsmclkEn>;
impl<'a, REG> HsmclkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSMCLK disabled regardless of conditional clock requests"]
    #[inline(always)]
    pub fn hsmclk_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(HsmclkEn::HsmclkEn0)
    }
    #[doc = "HSMCLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn hsmclk_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(HsmclkEn::HsmclkEn1)
    }
}
#[doc = "SMCLK system clock conditional request enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmclkEn {
    #[doc = "0: SMCLK disabled regardless of conditional clock requests."]
    SmclkEn0 = 0,
    #[doc = "1: SMCLK enabled based on any conditional clock requests"]
    SmclkEn1 = 1,
}
impl From<SmclkEn> for bool {
    #[inline(always)]
    fn from(variant: SmclkEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMCLK_EN` reader - SMCLK system clock conditional request enable"]
pub type SmclkEnR = crate::BitReader<SmclkEn>;
impl SmclkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmclkEn {
        match self.bits {
            false => SmclkEn::SmclkEn0,
            true => SmclkEn::SmclkEn1,
        }
    }
    #[doc = "SMCLK disabled regardless of conditional clock requests."]
    #[inline(always)]
    pub fn is_smclk_en_0(&self) -> bool {
        *self == SmclkEn::SmclkEn0
    }
    #[doc = "SMCLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn is_smclk_en_1(&self) -> bool {
        *self == SmclkEn::SmclkEn1
    }
}
#[doc = "Field `SMCLK_EN` writer - SMCLK system clock conditional request enable"]
pub type SmclkEnW<'a, REG> = crate::BitWriter<'a, REG, SmclkEn>;
impl<'a, REG> SmclkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMCLK disabled regardless of conditional clock requests."]
    #[inline(always)]
    pub fn smclk_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(SmclkEn::SmclkEn0)
    }
    #[doc = "SMCLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn smclk_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(SmclkEn::SmclkEn1)
    }
}
#[doc = "Turns on the VLO oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VloEn {
    #[doc = "0: VLO is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK."]
    VloEn0 = 0,
    #[doc = "1: VLO is on"]
    VloEn1 = 1,
}
impl From<VloEn> for bool {
    #[inline(always)]
    fn from(variant: VloEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLO_EN` reader - Turns on the VLO oscillator"]
pub type VloEnR = crate::BitReader<VloEn>;
impl VloEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VloEn {
        match self.bits {
            false => VloEn::VloEn0,
            true => VloEn::VloEn1,
        }
    }
    #[doc = "VLO is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK."]
    #[inline(always)]
    pub fn is_vlo_en_0(&self) -> bool {
        *self == VloEn::VloEn0
    }
    #[doc = "VLO is on"]
    #[inline(always)]
    pub fn is_vlo_en_1(&self) -> bool {
        *self == VloEn::VloEn1
    }
}
#[doc = "Field `VLO_EN` writer - Turns on the VLO oscillator"]
pub type VloEnW<'a, REG> = crate::BitWriter<'a, REG, VloEn>;
impl<'a, REG> VloEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VLO is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK."]
    #[inline(always)]
    pub fn vlo_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(VloEn::VloEn0)
    }
    #[doc = "VLO is on"]
    #[inline(always)]
    pub fn vlo_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(VloEn::VloEn1)
    }
}
#[doc = "Turns on the REFO oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefoEn {
    #[doc = "0: REFO is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK"]
    RefoEn0 = 0,
    #[doc = "1: REFO is on"]
    RefoEn1 = 1,
}
impl From<RefoEn> for bool {
    #[inline(always)]
    fn from(variant: RefoEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFO_EN` reader - Turns on the REFO oscillator"]
pub type RefoEnR = crate::BitReader<RefoEn>;
impl RefoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefoEn {
        match self.bits {
            false => RefoEn::RefoEn0,
            true => RefoEn::RefoEn1,
        }
    }
    #[doc = "REFO is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK"]
    #[inline(always)]
    pub fn is_refo_en_0(&self) -> bool {
        *self == RefoEn::RefoEn0
    }
    #[doc = "REFO is on"]
    #[inline(always)]
    pub fn is_refo_en_1(&self) -> bool {
        *self == RefoEn::RefoEn1
    }
}
#[doc = "Field `REFO_EN` writer - Turns on the REFO oscillator"]
pub type RefoEnW<'a, REG> = crate::BitWriter<'a, REG, RefoEn>;
impl<'a, REG> RefoEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "REFO is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK"]
    #[inline(always)]
    pub fn refo_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(RefoEn::RefoEn0)
    }
    #[doc = "REFO is on"]
    #[inline(always)]
    pub fn refo_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(RefoEn::RefoEn1)
    }
}
#[doc = "Turns on the MODOSC oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModoscEn {
    #[doc = "0: MODOSC is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK"]
    ModoscEn0 = 0,
    #[doc = "1: MODOSC is on"]
    ModoscEn1 = 1,
}
impl From<ModoscEn> for bool {
    #[inline(always)]
    fn from(variant: ModoscEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODOSC_EN` reader - Turns on the MODOSC oscillator"]
pub type ModoscEnR = crate::BitReader<ModoscEn>;
impl ModoscEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ModoscEn {
        match self.bits {
            false => ModoscEn::ModoscEn0,
            true => ModoscEn::ModoscEn1,
        }
    }
    #[doc = "MODOSC is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK"]
    #[inline(always)]
    pub fn is_modosc_en_0(&self) -> bool {
        *self == ModoscEn::ModoscEn0
    }
    #[doc = "MODOSC is on"]
    #[inline(always)]
    pub fn is_modosc_en_1(&self) -> bool {
        *self == ModoscEn::ModoscEn1
    }
}
#[doc = "Field `MODOSC_EN` writer - Turns on the MODOSC oscillator"]
pub type ModoscEnW<'a, REG> = crate::BitWriter<'a, REG, ModoscEn>;
impl<'a, REG> ModoscEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MODOSC is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK"]
    #[inline(always)]
    pub fn modosc_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(ModoscEn::ModoscEn0)
    }
    #[doc = "MODOSC is on"]
    #[inline(always)]
    pub fn modosc_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(ModoscEn::ModoscEn1)
    }
}
#[doc = "Selects REFO nominal frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refofsel {
    #[doc = "0: 32 kHz"]
    Refofsel0 = 0,
    #[doc = "1: 128 kHz"]
    Refofsel1 = 1,
}
impl From<Refofsel> for bool {
    #[inline(always)]
    fn from(variant: Refofsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFOFSEL` reader - Selects REFO nominal frequency"]
pub type RefofselR = crate::BitReader<Refofsel>;
impl RefofselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refofsel {
        match self.bits {
            false => Refofsel::Refofsel0,
            true => Refofsel::Refofsel1,
        }
    }
    #[doc = "32 kHz"]
    #[inline(always)]
    pub fn is_refofsel_0(&self) -> bool {
        *self == Refofsel::Refofsel0
    }
    #[doc = "128 kHz"]
    #[inline(always)]
    pub fn is_refofsel_1(&self) -> bool {
        *self == Refofsel::Refofsel1
    }
}
#[doc = "Field `REFOFSEL` writer - Selects REFO nominal frequency"]
pub type RefofselW<'a, REG> = crate::BitWriter<'a, REG, Refofsel>;
impl<'a, REG> RefofselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "32 kHz"]
    #[inline(always)]
    pub fn refofsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refofsel::Refofsel0)
    }
    #[doc = "128 kHz"]
    #[inline(always)]
    pub fn refofsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refofsel::Refofsel1)
    }
}
impl R {
    #[doc = "Bit 0 - ACLK system clock conditional request enable"]
    #[inline(always)]
    pub fn aclk_en(&self) -> AclkEnR {
        AclkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn mclk_en(&self) -> MclkEnR {
        MclkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSMCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn hsmclk_en(&self) -> HsmclkEnR {
        HsmclkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn smclk_en(&self) -> SmclkEnR {
        SmclkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Turns on the VLO oscillator"]
    #[inline(always)]
    pub fn vlo_en(&self) -> VloEnR {
        VloEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Turns on the REFO oscillator"]
    #[inline(always)]
    pub fn refo_en(&self) -> RefoEnR {
        RefoEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Turns on the MODOSC oscillator"]
    #[inline(always)]
    pub fn modosc_en(&self) -> ModoscEnR {
        ModoscEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects REFO nominal frequency"]
    #[inline(always)]
    pub fn refofsel(&self) -> RefofselR {
        RefofselR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACLK system clock conditional request enable"]
    #[inline(always)]
    pub fn aclk_en(&mut self) -> AclkEnW<CsclkenSpec> {
        AclkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - MCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn mclk_en(&mut self) -> MclkEnW<CsclkenSpec> {
        MclkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - HSMCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn hsmclk_en(&mut self) -> HsmclkEnW<CsclkenSpec> {
        HsmclkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - SMCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn smclk_en(&mut self) -> SmclkEnW<CsclkenSpec> {
        SmclkEnW::new(self, 3)
    }
    #[doc = "Bit 8 - Turns on the VLO oscillator"]
    #[inline(always)]
    pub fn vlo_en(&mut self) -> VloEnW<CsclkenSpec> {
        VloEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Turns on the REFO oscillator"]
    #[inline(always)]
    pub fn refo_en(&mut self) -> RefoEnW<CsclkenSpec> {
        RefoEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Turns on the MODOSC oscillator"]
    #[inline(always)]
    pub fn modosc_en(&mut self) -> ModoscEnW<CsclkenSpec> {
        ModoscEnW::new(self, 10)
    }
    #[doc = "Bit 15 - Selects REFO nominal frequency"]
    #[inline(always)]
    pub fn refofsel(&mut self) -> RefofselW<CsclkenSpec> {
        RefofselW::new(self, 15)
    }
}
#[doc = "Clock Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csclken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csclken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsclkenSpec;
impl crate::RegisterSpec for CsclkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csclken::R`](R) reader structure"]
impl crate::Readable for CsclkenSpec {}
#[doc = "`write(|w| ..)` method takes [`csclken::W`](W) writer structure"]
impl crate::Writable for CsclkenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSCLKEN to value 0x0f"]
impl crate::Resettable for CsclkenSpec {
    const RESET_VALUE: u32 = 0x0f;
}
