#[doc = "Register `PSSCTL0` reader"]
pub type R = crate::R<Pssctl0Spec>;
#[doc = "Register `PSSCTL0` writer"]
pub type W = crate::W<Pssctl0Spec>;
#[doc = "SVSM high-side off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svsmhoff {
    #[doc = "0: The SVSMH is on"]
    Svsmhoff0 = 0,
    #[doc = "1: The SVSMH is off"]
    Svsmhoff1 = 1,
}
impl From<Svsmhoff> for bool {
    #[inline(always)]
    fn from(variant: Svsmhoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSMHOFF` reader - SVSM high-side off"]
pub type SvsmhoffR = crate::BitReader<Svsmhoff>;
impl SvsmhoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svsmhoff {
        match self.bits {
            false => Svsmhoff::Svsmhoff0,
            true => Svsmhoff::Svsmhoff1,
        }
    }
    #[doc = "The SVSMH is on"]
    #[inline(always)]
    pub fn is_svsmhoff_0(&self) -> bool {
        *self == Svsmhoff::Svsmhoff0
    }
    #[doc = "The SVSMH is off"]
    #[inline(always)]
    pub fn is_svsmhoff_1(&self) -> bool {
        *self == Svsmhoff::Svsmhoff1
    }
}
#[doc = "Field `SVSMHOFF` writer - SVSM high-side off"]
pub type SvsmhoffW<'a, REG> = crate::BitWriter<'a, REG, Svsmhoff>;
impl<'a, REG> SvsmhoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SVSMH is on"]
    #[inline(always)]
    pub fn svsmhoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Svsmhoff::Svsmhoff0)
    }
    #[doc = "The SVSMH is off"]
    #[inline(always)]
    pub fn svsmhoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Svsmhoff::Svsmhoff1)
    }
}
#[doc = "SVSM high-side low power normal performance mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svsmhlp {
    #[doc = "0: Full performance mode. See the device-specific data sheet for response times."]
    Svsmhlp0 = 0,
    #[doc = "1: Low power normal performance mode in LPM3, LPM4, and LPMx.5, full performance in all other modes. See the device-specific data sheet for response times."]
    Svsmhlp1 = 1,
}
impl From<Svsmhlp> for bool {
    #[inline(always)]
    fn from(variant: Svsmhlp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSMHLP` reader - SVSM high-side low power normal performance mode"]
pub type SvsmhlpR = crate::BitReader<Svsmhlp>;
impl SvsmhlpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svsmhlp {
        match self.bits {
            false => Svsmhlp::Svsmhlp0,
            true => Svsmhlp::Svsmhlp1,
        }
    }
    #[doc = "Full performance mode. See the device-specific data sheet for response times."]
    #[inline(always)]
    pub fn is_svsmhlp_0(&self) -> bool {
        *self == Svsmhlp::Svsmhlp0
    }
    #[doc = "Low power normal performance mode in LPM3, LPM4, and LPMx.5, full performance in all other modes. See the device-specific data sheet for response times."]
    #[inline(always)]
    pub fn is_svsmhlp_1(&self) -> bool {
        *self == Svsmhlp::Svsmhlp1
    }
}
#[doc = "Field `SVSMHLP` writer - SVSM high-side low power normal performance mode"]
pub type SvsmhlpW<'a, REG> = crate::BitWriter<'a, REG, Svsmhlp>;
impl<'a, REG> SvsmhlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full performance mode. See the device-specific data sheet for response times."]
    #[inline(always)]
    pub fn svsmhlp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Svsmhlp::Svsmhlp0)
    }
    #[doc = "Low power normal performance mode in LPM3, LPM4, and LPMx.5, full performance in all other modes. See the device-specific data sheet for response times."]
    #[inline(always)]
    pub fn svsmhlp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Svsmhlp::Svsmhlp1)
    }
}
#[doc = "Supply supervisor or monitor selection for the high-side\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svsmhs {
    #[doc = "0: Configure as SVSH"]
    Svsmhs0 = 0,
    #[doc = "1: Configure as SVMH"]
    Svsmhs1 = 1,
}
impl From<Svsmhs> for bool {
    #[inline(always)]
    fn from(variant: Svsmhs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSMHS` reader - Supply supervisor or monitor selection for the high-side"]
pub type SvsmhsR = crate::BitReader<Svsmhs>;
impl SvsmhsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svsmhs {
        match self.bits {
            false => Svsmhs::Svsmhs0,
            true => Svsmhs::Svsmhs1,
        }
    }
    #[doc = "Configure as SVSH"]
    #[inline(always)]
    pub fn is_svsmhs_0(&self) -> bool {
        *self == Svsmhs::Svsmhs0
    }
    #[doc = "Configure as SVMH"]
    #[inline(always)]
    pub fn is_svsmhs_1(&self) -> bool {
        *self == Svsmhs::Svsmhs1
    }
}
#[doc = "Field `SVSMHS` writer - Supply supervisor or monitor selection for the high-side"]
pub type SvsmhsW<'a, REG> = crate::BitWriter<'a, REG, Svsmhs>;
impl<'a, REG> SvsmhsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configure as SVSH"]
    #[inline(always)]
    pub fn svsmhs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Svsmhs::Svsmhs0)
    }
    #[doc = "Configure as SVMH"]
    #[inline(always)]
    pub fn svsmhs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Svsmhs::Svsmhs1)
    }
}
#[doc = "Field `SVSMHTH` reader - SVSM high-side reset voltage level"]
pub type SvsmhthR = crate::FieldReader;
#[doc = "Field `SVSMHTH` writer - SVSM high-side reset voltage level"]
pub type SvsmhthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "SVSM high-side output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svmhoe {
    #[doc = "0: SVSMHIFG bit is not output"]
    Svmhoe0 = 0,
    #[doc = "1: SVSMHIFG bit is output to the device SVMHOUT pin. The device-specific port logic must be configured accordingly"]
    Svmhoe1 = 1,
}
impl From<Svmhoe> for bool {
    #[inline(always)]
    fn from(variant: Svmhoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVMHOE` reader - SVSM high-side output enable"]
pub type SvmhoeR = crate::BitReader<Svmhoe>;
impl SvmhoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svmhoe {
        match self.bits {
            false => Svmhoe::Svmhoe0,
            true => Svmhoe::Svmhoe1,
        }
    }
    #[doc = "SVSMHIFG bit is not output"]
    #[inline(always)]
    pub fn is_svmhoe_0(&self) -> bool {
        *self == Svmhoe::Svmhoe0
    }
    #[doc = "SVSMHIFG bit is output to the device SVMHOUT pin. The device-specific port logic must be configured accordingly"]
    #[inline(always)]
    pub fn is_svmhoe_1(&self) -> bool {
        *self == Svmhoe::Svmhoe1
    }
}
#[doc = "Field `SVMHOE` writer - SVSM high-side output enable"]
pub type SvmhoeW<'a, REG> = crate::BitWriter<'a, REG, Svmhoe>;
impl<'a, REG> SvmhoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SVSMHIFG bit is not output"]
    #[inline(always)]
    pub fn svmhoe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Svmhoe::Svmhoe0)
    }
    #[doc = "SVSMHIFG bit is output to the device SVMHOUT pin. The device-specific port logic must be configured accordingly"]
    #[inline(always)]
    pub fn svmhoe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Svmhoe::Svmhoe1)
    }
}
#[doc = "SVMHOUT pin polarity active low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svmhoutpolal {
    #[doc = "0: SVMHOUT is active high. An error condition is signaled by a 1 at the SVMHOUT pin"]
    Svmhoutpolal0 = 0,
    #[doc = "1: SVMHOUT is active low. An error condition is signaled by a 0 at the SVMHOUT pin"]
    Svmhoutpolal1 = 1,
}
impl From<Svmhoutpolal> for bool {
    #[inline(always)]
    fn from(variant: Svmhoutpolal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVMHOUTPOLAL` reader - SVMHOUT pin polarity active low"]
pub type SvmhoutpolalR = crate::BitReader<Svmhoutpolal>;
impl SvmhoutpolalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svmhoutpolal {
        match self.bits {
            false => Svmhoutpolal::Svmhoutpolal0,
            true => Svmhoutpolal::Svmhoutpolal1,
        }
    }
    #[doc = "SVMHOUT is active high. An error condition is signaled by a 1 at the SVMHOUT pin"]
    #[inline(always)]
    pub fn is_svmhoutpolal_0(&self) -> bool {
        *self == Svmhoutpolal::Svmhoutpolal0
    }
    #[doc = "SVMHOUT is active low. An error condition is signaled by a 0 at the SVMHOUT pin"]
    #[inline(always)]
    pub fn is_svmhoutpolal_1(&self) -> bool {
        *self == Svmhoutpolal::Svmhoutpolal1
    }
}
#[doc = "Field `SVMHOUTPOLAL` writer - SVMHOUT pin polarity active low"]
pub type SvmhoutpolalW<'a, REG> = crate::BitWriter<'a, REG, Svmhoutpolal>;
impl<'a, REG> SvmhoutpolalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SVMHOUT is active high. An error condition is signaled by a 1 at the SVMHOUT pin"]
    #[inline(always)]
    pub fn svmhoutpolal_0(self) -> &'a mut crate::W<REG> {
        self.variant(Svmhoutpolal::Svmhoutpolal0)
    }
    #[doc = "SVMHOUT is active low. An error condition is signaled by a 0 at the SVMHOUT pin"]
    #[inline(always)]
    pub fn svmhoutpolal_1(self) -> &'a mut crate::W<REG> {
        self.variant(Svmhoutpolal::Svmhoutpolal1)
    }
}
#[doc = "Force DC-DC regulator operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcdcForce {
    #[doc = "0: DC-DC regulator operation not forced. Automatic fail-safe mechanism switches the core voltage regulator from DC-DC to LDO when the supply voltage falls below the minimum supply voltage necessary for DC-DC operation."]
    DcdcForce0 = 0,
    #[doc = "1: DC-DC regulator operation forced. Automatic fail-safe mechanism is disabled and device continues to operate out of DC-DC regulator."]
    DcdcForce1 = 1,
}
impl From<DcdcForce> for bool {
    #[inline(always)]
    fn from(variant: DcdcForce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDC_FORCE` reader - Force DC-DC regulator operation"]
pub type DcdcForceR = crate::BitReader<DcdcForce>;
impl DcdcForceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcdcForce {
        match self.bits {
            false => DcdcForce::DcdcForce0,
            true => DcdcForce::DcdcForce1,
        }
    }
    #[doc = "DC-DC regulator operation not forced. Automatic fail-safe mechanism switches the core voltage regulator from DC-DC to LDO when the supply voltage falls below the minimum supply voltage necessary for DC-DC operation."]
    #[inline(always)]
    pub fn is_dcdc_force_0(&self) -> bool {
        *self == DcdcForce::DcdcForce0
    }
    #[doc = "DC-DC regulator operation forced. Automatic fail-safe mechanism is disabled and device continues to operate out of DC-DC regulator."]
    #[inline(always)]
    pub fn is_dcdc_force_1(&self) -> bool {
        *self == DcdcForce::DcdcForce1
    }
}
#[doc = "Field `DCDC_FORCE` writer - Force DC-DC regulator operation"]
pub type DcdcForceW<'a, REG> = crate::BitWriter<'a, REG, DcdcForce>;
impl<'a, REG> DcdcForceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DC-DC regulator operation not forced. Automatic fail-safe mechanism switches the core voltage regulator from DC-DC to LDO when the supply voltage falls below the minimum supply voltage necessary for DC-DC operation."]
    #[inline(always)]
    pub fn dcdc_force_0(self) -> &'a mut crate::W<REG> {
        self.variant(DcdcForce::DcdcForce0)
    }
    #[doc = "DC-DC regulator operation forced. Automatic fail-safe mechanism is disabled and device continues to operate out of DC-DC regulator."]
    #[inline(always)]
    pub fn dcdc_force_1(self) -> &'a mut crate::W<REG> {
        self.variant(DcdcForce::DcdcForce1)
    }
}
#[doc = "Controls core voltage level transition time\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vcoretran {
    #[doc = "0: 32 s / 100 mV"]
    Vcoretran0 = 0,
    #[doc = "1: 64 s / 100 mV"]
    Vcoretran1 = 1,
    #[doc = "2: 128 s / 100 mV (default)"]
    Vcoretran2 = 2,
    #[doc = "3: 256 s / 100 mV"]
    Vcoretran3 = 3,
}
impl From<Vcoretran> for u8 {
    #[inline(always)]
    fn from(variant: Vcoretran) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vcoretran {
    type Ux = u8;
}
impl crate::IsEnum for Vcoretran {}
#[doc = "Field `VCORETRAN` reader - Controls core voltage level transition time"]
pub type VcoretranR = crate::FieldReader<Vcoretran>;
impl VcoretranR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcoretran {
        match self.bits {
            0 => Vcoretran::Vcoretran0,
            1 => Vcoretran::Vcoretran1,
            2 => Vcoretran::Vcoretran2,
            3 => Vcoretran::Vcoretran3,
            _ => unreachable!(),
        }
    }
    #[doc = "32 s / 100 mV"]
    #[inline(always)]
    pub fn is_vcoretran_0(&self) -> bool {
        *self == Vcoretran::Vcoretran0
    }
    #[doc = "64 s / 100 mV"]
    #[inline(always)]
    pub fn is_vcoretran_1(&self) -> bool {
        *self == Vcoretran::Vcoretran1
    }
    #[doc = "128 s / 100 mV (default)"]
    #[inline(always)]
    pub fn is_vcoretran_2(&self) -> bool {
        *self == Vcoretran::Vcoretran2
    }
    #[doc = "256 s / 100 mV"]
    #[inline(always)]
    pub fn is_vcoretran_3(&self) -> bool {
        *self == Vcoretran::Vcoretran3
    }
}
#[doc = "Field `VCORETRAN` writer - Controls core voltage level transition time"]
pub type VcoretranW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vcoretran, crate::Safe>;
impl<'a, REG> VcoretranW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 s / 100 mV"]
    #[inline(always)]
    pub fn vcoretran_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vcoretran::Vcoretran0)
    }
    #[doc = "64 s / 100 mV"]
    #[inline(always)]
    pub fn vcoretran_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vcoretran::Vcoretran1)
    }
    #[doc = "128 s / 100 mV (default)"]
    #[inline(always)]
    pub fn vcoretran_2(self) -> &'a mut crate::W<REG> {
        self.variant(Vcoretran::Vcoretran2)
    }
    #[doc = "256 s / 100 mV"]
    #[inline(always)]
    pub fn vcoretran_3(self) -> &'a mut crate::W<REG> {
        self.variant(Vcoretran::Vcoretran3)
    }
}
impl R {
    #[doc = "Bit 0 - SVSM high-side off"]
    #[inline(always)]
    pub fn svsmhoff(&self) -> SvsmhoffR {
        SvsmhoffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SVSM high-side low power normal performance mode"]
    #[inline(always)]
    pub fn svsmhlp(&self) -> SvsmhlpR {
        SvsmhlpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Supply supervisor or monitor selection for the high-side"]
    #[inline(always)]
    pub fn svsmhs(&self) -> SvsmhsR {
        SvsmhsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - SVSM high-side reset voltage level"]
    #[inline(always)]
    pub fn svsmhth(&self) -> SvsmhthR {
        SvsmhthR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SVSM high-side output enable"]
    #[inline(always)]
    pub fn svmhoe(&self) -> SvmhoeR {
        SvmhoeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SVMHOUT pin polarity active low"]
    #[inline(always)]
    pub fn svmhoutpolal(&self) -> SvmhoutpolalR {
        SvmhoutpolalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Force DC-DC regulator operation"]
    #[inline(always)]
    pub fn dcdc_force(&self) -> DcdcForceR {
        DcdcForceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Controls core voltage level transition time"]
    #[inline(always)]
    pub fn vcoretran(&self) -> VcoretranR {
        VcoretranR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SVSM high-side off"]
    #[inline(always)]
    pub fn svsmhoff(&mut self) -> SvsmhoffW<Pssctl0Spec> {
        SvsmhoffW::new(self, 0)
    }
    #[doc = "Bit 1 - SVSM high-side low power normal performance mode"]
    #[inline(always)]
    pub fn svsmhlp(&mut self) -> SvsmhlpW<Pssctl0Spec> {
        SvsmhlpW::new(self, 1)
    }
    #[doc = "Bit 2 - Supply supervisor or monitor selection for the high-side"]
    #[inline(always)]
    pub fn svsmhs(&mut self) -> SvsmhsW<Pssctl0Spec> {
        SvsmhsW::new(self, 2)
    }
    #[doc = "Bits 3:5 - SVSM high-side reset voltage level"]
    #[inline(always)]
    pub fn svsmhth(&mut self) -> SvsmhthW<Pssctl0Spec> {
        SvsmhthW::new(self, 3)
    }
    #[doc = "Bit 6 - SVSM high-side output enable"]
    #[inline(always)]
    pub fn svmhoe(&mut self) -> SvmhoeW<Pssctl0Spec> {
        SvmhoeW::new(self, 6)
    }
    #[doc = "Bit 7 - SVMHOUT pin polarity active low"]
    #[inline(always)]
    pub fn svmhoutpolal(&mut self) -> SvmhoutpolalW<Pssctl0Spec> {
        SvmhoutpolalW::new(self, 7)
    }
    #[doc = "Bit 10 - Force DC-DC regulator operation"]
    #[inline(always)]
    pub fn dcdc_force(&mut self) -> DcdcForceW<Pssctl0Spec> {
        DcdcForceW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Controls core voltage level transition time"]
    #[inline(always)]
    pub fn vcoretran(&mut self) -> VcoretranW<Pssctl0Spec> {
        VcoretranW::new(self, 12)
    }
}
#[doc = "Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pssctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pssctl0Spec;
impl crate::RegisterSpec for Pssctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pssctl0::R`](R) reader structure"]
impl crate::Readable for Pssctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pssctl0::W`](W) writer structure"]
impl crate::Writable for Pssctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSSCTL0 to value 0x2000"]
impl crate::Resettable for Pssctl0Spec {
    const RESET_VALUE: u32 = 0x2000;
}
