#[doc = "Register `TAxCCTL[%s]` reader"]
pub type R = crate::R<TaxCctlSpec>;
#[doc = "Register `TAxCCTL[%s]` writer"]
pub type W = crate::W<TaxCctlSpec>;
#[doc = "Capture/compare interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccifg {
    #[doc = "0: No interrupt pending"]
    Ccifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ccifg1 = 1,
}
impl From<Ccifg> for bool {
    #[inline(always)]
    fn from(variant: Ccifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIFG` reader - Capture/compare interrupt flag"]
pub type CcifgR = crate::BitReader<Ccifg>;
impl CcifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccifg {
        match self.bits {
            false => Ccifg::Ccifg0,
            true => Ccifg::Ccifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ccifg_0(&self) -> bool {
        *self == Ccifg::Ccifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ccifg_1(&self) -> bool {
        *self == Ccifg::Ccifg1
    }
}
#[doc = "Field `CCIFG` writer - Capture/compare interrupt flag"]
pub type CcifgW<'a, REG> = crate::BitWriter<'a, REG, Ccifg>;
impl<'a, REG> CcifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ccifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccifg::Ccifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ccifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccifg::Ccifg1)
    }
}
#[doc = "Capture overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cov {
    #[doc = "0: No capture overflow occurred"]
    Cov0 = 0,
    #[doc = "1: Capture overflow occurred"]
    Cov1 = 1,
}
impl From<Cov> for bool {
    #[inline(always)]
    fn from(variant: Cov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COV` reader - Capture overflow"]
pub type CovR = crate::BitReader<Cov>;
impl CovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cov {
        match self.bits {
            false => Cov::Cov0,
            true => Cov::Cov1,
        }
    }
    #[doc = "No capture overflow occurred"]
    #[inline(always)]
    pub fn is_cov_0(&self) -> bool {
        *self == Cov::Cov0
    }
    #[doc = "Capture overflow occurred"]
    #[inline(always)]
    pub fn is_cov_1(&self) -> bool {
        *self == Cov::Cov1
    }
}
#[doc = "Field `COV` writer - Capture overflow"]
pub type CovW<'a, REG> = crate::BitWriter<'a, REG, Cov>;
impl<'a, REG> CovW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No capture overflow occurred"]
    #[inline(always)]
    pub fn cov_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cov::Cov0)
    }
    #[doc = "Capture overflow occurred"]
    #[inline(always)]
    pub fn cov_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cov::Cov1)
    }
}
#[doc = "Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out {
    #[doc = "0: Output low"]
    Out0 = 0,
    #[doc = "1: Output high"]
    Out1 = 1,
}
impl From<Out> for bool {
    #[inline(always)]
    fn from(variant: Out) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT` reader - Output"]
pub type OutR = crate::BitReader<Out>;
impl OutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out {
        match self.bits {
            false => Out::Out0,
            true => Out::Out1,
        }
    }
    #[doc = "Output low"]
    #[inline(always)]
    pub fn is_out_0(&self) -> bool {
        *self == Out::Out0
    }
    #[doc = "Output high"]
    #[inline(always)]
    pub fn is_out_1(&self) -> bool {
        *self == Out::Out1
    }
}
#[doc = "Field `OUT` writer - Output"]
pub type OutW<'a, REG> = crate::BitWriter<'a, REG, Out>;
impl<'a, REG> OutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output low"]
    #[inline(always)]
    pub fn out_0(self) -> &'a mut crate::W<REG> {
        self.variant(Out::Out0)
    }
    #[doc = "Output high"]
    #[inline(always)]
    pub fn out_1(self) -> &'a mut crate::W<REG> {
        self.variant(Out::Out1)
    }
}
#[doc = "Field `CCI` reader - Capture/compare input"]
pub type CciR = crate::BitReader;
#[doc = "Capture/compare interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccie {
    #[doc = "0: Interrupt disabled"]
    Ccie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ccie1 = 1,
}
impl From<Ccie> for bool {
    #[inline(always)]
    fn from(variant: Ccie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIE` reader - Capture/compare interrupt enable"]
pub type CcieR = crate::BitReader<Ccie>;
impl CcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccie {
        match self.bits {
            false => Ccie::Ccie0,
            true => Ccie::Ccie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ccie_0(&self) -> bool {
        *self == Ccie::Ccie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ccie_1(&self) -> bool {
        *self == Ccie::Ccie1
    }
}
#[doc = "Field `CCIE` writer - Capture/compare interrupt enable"]
pub type CcieW<'a, REG> = crate::BitWriter<'a, REG, Ccie>;
impl<'a, REG> CcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ccie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccie::Ccie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ccie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccie::Ccie1)
    }
}
#[doc = "Output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outmod {
    #[doc = "0: OUT bit value"]
    Outmod0 = 0,
    #[doc = "1: Set"]
    Outmod1 = 1,
    #[doc = "2: Toggle/reset"]
    Outmod2 = 2,
    #[doc = "3: Set/reset"]
    Outmod3 = 3,
    #[doc = "4: Toggle"]
    Outmod4 = 4,
    #[doc = "5: Reset"]
    Outmod5 = 5,
    #[doc = "6: Toggle/set"]
    Outmod6 = 6,
    #[doc = "7: Reset/set"]
    Outmod7 = 7,
}
impl From<Outmod> for u8 {
    #[inline(always)]
    fn from(variant: Outmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outmod {
    type Ux = u8;
}
impl crate::IsEnum for Outmod {}
#[doc = "Field `OUTMOD` reader - Output mode"]
pub type OutmodR = crate::FieldReader<Outmod>;
impl OutmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outmod {
        match self.bits {
            0 => Outmod::Outmod0,
            1 => Outmod::Outmod1,
            2 => Outmod::Outmod2,
            3 => Outmod::Outmod3,
            4 => Outmod::Outmod4,
            5 => Outmod::Outmod5,
            6 => Outmod::Outmod6,
            7 => Outmod::Outmod7,
            _ => unreachable!(),
        }
    }
    #[doc = "OUT bit value"]
    #[inline(always)]
    pub fn is_outmod_0(&self) -> bool {
        *self == Outmod::Outmod0
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_outmod_1(&self) -> bool {
        *self == Outmod::Outmod1
    }
    #[doc = "Toggle/reset"]
    #[inline(always)]
    pub fn is_outmod_2(&self) -> bool {
        *self == Outmod::Outmod2
    }
    #[doc = "Set/reset"]
    #[inline(always)]
    pub fn is_outmod_3(&self) -> bool {
        *self == Outmod::Outmod3
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn is_outmod_4(&self) -> bool {
        *self == Outmod::Outmod4
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_outmod_5(&self) -> bool {
        *self == Outmod::Outmod5
    }
    #[doc = "Toggle/set"]
    #[inline(always)]
    pub fn is_outmod_6(&self) -> bool {
        *self == Outmod::Outmod6
    }
    #[doc = "Reset/set"]
    #[inline(always)]
    pub fn is_outmod_7(&self) -> bool {
        *self == Outmod::Outmod7
    }
}
#[doc = "Field `OUTMOD` writer - Output mode"]
pub type OutmodW<'a, REG> = crate::FieldWriter<'a, REG, 3, Outmod, crate::Safe>;
impl<'a, REG> OutmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OUT bit value"]
    #[inline(always)]
    pub fn outmod_0(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod0)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn outmod_1(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod1)
    }
    #[doc = "Toggle/reset"]
    #[inline(always)]
    pub fn outmod_2(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod2)
    }
    #[doc = "Set/reset"]
    #[inline(always)]
    pub fn outmod_3(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod3)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn outmod_4(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod4)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn outmod_5(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod5)
    }
    #[doc = "Toggle/set"]
    #[inline(always)]
    pub fn outmod_6(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod6)
    }
    #[doc = "Reset/set"]
    #[inline(always)]
    pub fn outmod_7(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod7)
    }
}
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap {
    #[doc = "0: Compare mode"]
    Cap0 = 0,
    #[doc = "1: Capture mode"]
    Cap1 = 1,
}
impl From<Cap> for bool {
    #[inline(always)]
    fn from(variant: Cap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP` reader - Capture mode"]
pub type CapR = crate::BitReader<Cap>;
impl CapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap {
        match self.bits {
            false => Cap::Cap0,
            true => Cap::Cap1,
        }
    }
    #[doc = "Compare mode"]
    #[inline(always)]
    pub fn is_cap_0(&self) -> bool {
        *self == Cap::Cap0
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn is_cap_1(&self) -> bool {
        *self == Cap::Cap1
    }
}
#[doc = "Field `CAP` writer - Capture mode"]
pub type CapW<'a, REG> = crate::BitWriter<'a, REG, Cap>;
impl<'a, REG> CapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare mode"]
    #[inline(always)]
    pub fn cap_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cap::Cap0)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn cap_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cap::Cap1)
    }
}
#[doc = "Field `SCCI` reader - Synchronized capture/compare input"]
pub type ScciR = crate::BitReader;
#[doc = "Field `SCCI` writer - Synchronized capture/compare input"]
pub type ScciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Synchronize capture source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scs {
    #[doc = "0: Asynchronous capture"]
    Scs0 = 0,
    #[doc = "1: Synchronous capture"]
    Scs1 = 1,
}
impl From<Scs> for bool {
    #[inline(always)]
    fn from(variant: Scs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCS` reader - Synchronize capture source"]
pub type ScsR = crate::BitReader<Scs>;
impl ScsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scs {
        match self.bits {
            false => Scs::Scs0,
            true => Scs::Scs1,
        }
    }
    #[doc = "Asynchronous capture"]
    #[inline(always)]
    pub fn is_scs_0(&self) -> bool {
        *self == Scs::Scs0
    }
    #[doc = "Synchronous capture"]
    #[inline(always)]
    pub fn is_scs_1(&self) -> bool {
        *self == Scs::Scs1
    }
}
#[doc = "Field `SCS` writer - Synchronize capture source"]
pub type ScsW<'a, REG> = crate::BitWriter<'a, REG, Scs>;
impl<'a, REG> ScsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous capture"]
    #[inline(always)]
    pub fn scs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Scs0)
    }
    #[doc = "Synchronous capture"]
    #[inline(always)]
    pub fn scs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Scs1)
    }
}
#[doc = "Capture/compare input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccis {
    #[doc = "0: CCIxA"]
    Ccis0 = 0,
    #[doc = "1: CCIxB"]
    Ccis1 = 1,
    #[doc = "2: GND"]
    Ccis2 = 2,
    #[doc = "3: VCC"]
    Ccis3 = 3,
}
impl From<Ccis> for u8 {
    #[inline(always)]
    fn from(variant: Ccis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccis {
    type Ux = u8;
}
impl crate::IsEnum for Ccis {}
#[doc = "Field `CCIS` reader - Capture/compare input select"]
pub type CcisR = crate::FieldReader<Ccis>;
impl CcisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccis {
        match self.bits {
            0 => Ccis::Ccis0,
            1 => Ccis::Ccis1,
            2 => Ccis::Ccis2,
            3 => Ccis::Ccis3,
            _ => unreachable!(),
        }
    }
    #[doc = "CCIxA"]
    #[inline(always)]
    pub fn is_ccis_0(&self) -> bool {
        *self == Ccis::Ccis0
    }
    #[doc = "CCIxB"]
    #[inline(always)]
    pub fn is_ccis_1(&self) -> bool {
        *self == Ccis::Ccis1
    }
    #[doc = "GND"]
    #[inline(always)]
    pub fn is_ccis_2(&self) -> bool {
        *self == Ccis::Ccis2
    }
    #[doc = "VCC"]
    #[inline(always)]
    pub fn is_ccis_3(&self) -> bool {
        *self == Ccis::Ccis3
    }
}
#[doc = "Field `CCIS` writer - Capture/compare input select"]
pub type CcisW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccis, crate::Safe>;
impl<'a, REG> CcisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCIxA"]
    #[inline(always)]
    pub fn ccis_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Ccis0)
    }
    #[doc = "CCIxB"]
    #[inline(always)]
    pub fn ccis_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Ccis1)
    }
    #[doc = "GND"]
    #[inline(always)]
    pub fn ccis_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Ccis2)
    }
    #[doc = "VCC"]
    #[inline(always)]
    pub fn ccis_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Ccis3)
    }
}
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cm {
    #[doc = "0: No capture"]
    Cm0 = 0,
    #[doc = "1: Capture on rising edge"]
    Cm1 = 1,
    #[doc = "2: Capture on falling edge"]
    Cm2 = 2,
    #[doc = "3: Capture on both rising and falling edges"]
    Cm3 = 3,
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cm {
    type Ux = u8;
}
impl crate::IsEnum for Cm {}
#[doc = "Field `CM` reader - Capture mode"]
pub type CmR = crate::FieldReader<Cm>;
impl CmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm {
        match self.bits {
            0 => Cm::Cm0,
            1 => Cm::Cm1,
            2 => Cm::Cm2,
            3 => Cm::Cm3,
            _ => unreachable!(),
        }
    }
    #[doc = "No capture"]
    #[inline(always)]
    pub fn is_cm_0(&self) -> bool {
        *self == Cm::Cm0
    }
    #[doc = "Capture on rising edge"]
    #[inline(always)]
    pub fn is_cm_1(&self) -> bool {
        *self == Cm::Cm1
    }
    #[doc = "Capture on falling edge"]
    #[inline(always)]
    pub fn is_cm_2(&self) -> bool {
        *self == Cm::Cm2
    }
    #[doc = "Capture on both rising and falling edges"]
    #[inline(always)]
    pub fn is_cm_3(&self) -> bool {
        *self == Cm::Cm3
    }
}
#[doc = "Field `CM` writer - Capture mode"]
pub type CmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cm, crate::Safe>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No capture"]
    #[inline(always)]
    pub fn cm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Cm0)
    }
    #[doc = "Capture on rising edge"]
    #[inline(always)]
    pub fn cm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Cm1)
    }
    #[doc = "Capture on falling edge"]
    #[inline(always)]
    pub fn cm_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Cm2)
    }
    #[doc = "Capture on both rising and falling edges"]
    #[inline(always)]
    pub fn cm_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Cm3)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&self) -> CcifgR {
        CcifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture overflow"]
    #[inline(always)]
    pub fn cov(&self) -> CovR {
        CovR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare input"]
    #[inline(always)]
    pub fn cci(&self) -> CciR {
        CciR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CcieR {
        CcieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Output mode"]
    #[inline(always)]
    pub fn outmod(&self) -> OutmodR {
        OutmodR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Capture mode"]
    #[inline(always)]
    pub fn cap(&self) -> CapR {
        CapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronized capture/compare input"]
    #[inline(always)]
    pub fn scci(&self) -> ScciR {
        ScciR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronize capture source"]
    #[inline(always)]
    pub fn scs(&self) -> ScsR {
        ScsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Capture/compare input select"]
    #[inline(always)]
    pub fn ccis(&self) -> CcisR {
        CcisR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Capture mode"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&mut self) -> CcifgW<TaxCctlSpec> {
        CcifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture overflow"]
    #[inline(always)]
    pub fn cov(&mut self) -> CovW<TaxCctlSpec> {
        CovW::new(self, 1)
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<TaxCctlSpec> {
        OutW::new(self, 2)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CcieW<TaxCctlSpec> {
        CcieW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Output mode"]
    #[inline(always)]
    pub fn outmod(&mut self) -> OutmodW<TaxCctlSpec> {
        OutmodW::new(self, 5)
    }
    #[doc = "Bit 8 - Capture mode"]
    #[inline(always)]
    pub fn cap(&mut self) -> CapW<TaxCctlSpec> {
        CapW::new(self, 8)
    }
    #[doc = "Bit 10 - Synchronized capture/compare input"]
    #[inline(always)]
    pub fn scci(&mut self) -> ScciW<TaxCctlSpec> {
        ScciW::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronize capture source"]
    #[inline(always)]
    pub fn scs(&mut self) -> ScsW<TaxCctlSpec> {
        ScsW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Capture/compare input select"]
    #[inline(always)]
    pub fn ccis(&mut self) -> CcisW<TaxCctlSpec> {
        CcisW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Capture mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<TaxCctlSpec> {
        CmW::new(self, 14)
    }
}
#[doc = "Timer_A Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_cctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tax_cctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaxCctlSpec;
impl crate::RegisterSpec for TaxCctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tax_cctl::R`](R) reader structure"]
impl crate::Readable for TaxCctlSpec {}
#[doc = "`write(|w| ..)` method takes [`tax_cctl::W`](W) writer structure"]
impl crate::Writable for TaxCctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TAxCCTL[%s]
to value 0"]
impl crate::Resettable for TaxCctlSpec {
    const RESET_VALUE: u16 = 0;
}
