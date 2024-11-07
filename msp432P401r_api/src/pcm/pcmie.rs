#[doc = "Register `PCMIE` reader"]
pub type R = crate::R<PcmieSpec>;
#[doc = "Register `PCMIE` writer"]
pub type W = crate::W<PcmieSpec>;
#[doc = "LPM invalid transition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpmInvalidTrIe {
    #[doc = "0: Disabled"]
    LpmInvalidTrIe0 = 0,
    #[doc = "1: Enabled"]
    LpmInvalidTrIe1 = 1,
}
impl From<LpmInvalidTrIe> for bool {
    #[inline(always)]
    fn from(variant: LpmInvalidTrIe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPM_INVALID_TR_IE` reader - LPM invalid transition interrupt enable"]
pub type LpmInvalidTrIeR = crate::BitReader<LpmInvalidTrIe>;
impl LpmInvalidTrIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpmInvalidTrIe {
        match self.bits {
            false => LpmInvalidTrIe::LpmInvalidTrIe0,
            true => LpmInvalidTrIe::LpmInvalidTrIe1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_lpm_invalid_tr_ie_0(&self) -> bool {
        *self == LpmInvalidTrIe::LpmInvalidTrIe0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_lpm_invalid_tr_ie_1(&self) -> bool {
        *self == LpmInvalidTrIe::LpmInvalidTrIe1
    }
}
#[doc = "Field `LPM_INVALID_TR_IE` writer - LPM invalid transition interrupt enable"]
pub type LpmInvalidTrIeW<'a, REG> = crate::BitWriter<'a, REG, LpmInvalidTrIe>;
impl<'a, REG> LpmInvalidTrIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie_0(self) -> &'a mut crate::W<REG> {
        self.variant(LpmInvalidTrIe::LpmInvalidTrIe0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie_1(self) -> &'a mut crate::W<REG> {
        self.variant(LpmInvalidTrIe::LpmInvalidTrIe1)
    }
}
#[doc = "LPM invalid clock interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpmInvalidClkIe {
    #[doc = "0: Disabled"]
    LpmInvalidClkIe0 = 0,
    #[doc = "1: Enabled"]
    LpmInvalidClkIe1 = 1,
}
impl From<LpmInvalidClkIe> for bool {
    #[inline(always)]
    fn from(variant: LpmInvalidClkIe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPM_INVALID_CLK_IE` reader - LPM invalid clock interrupt enable"]
pub type LpmInvalidClkIeR = crate::BitReader<LpmInvalidClkIe>;
impl LpmInvalidClkIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LpmInvalidClkIe {
        match self.bits {
            false => LpmInvalidClkIe::LpmInvalidClkIe0,
            true => LpmInvalidClkIe::LpmInvalidClkIe1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_lpm_invalid_clk_ie_0(&self) -> bool {
        *self == LpmInvalidClkIe::LpmInvalidClkIe0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_lpm_invalid_clk_ie_1(&self) -> bool {
        *self == LpmInvalidClkIe::LpmInvalidClkIe1
    }
}
#[doc = "Field `LPM_INVALID_CLK_IE` writer - LPM invalid clock interrupt enable"]
pub type LpmInvalidClkIeW<'a, REG> = crate::BitWriter<'a, REG, LpmInvalidClkIe>;
impl<'a, REG> LpmInvalidClkIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie_0(self) -> &'a mut crate::W<REG> {
        self.variant(LpmInvalidClkIe::LpmInvalidClkIe0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie_1(self) -> &'a mut crate::W<REG> {
        self.variant(LpmInvalidClkIe::LpmInvalidClkIe1)
    }
}
#[doc = "Active mode invalid transition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AmInvalidTrIe {
    #[doc = "0: Disabled"]
    AmInvalidTrIe0 = 0,
    #[doc = "1: Enabled"]
    AmInvalidTrIe1 = 1,
}
impl From<AmInvalidTrIe> for bool {
    #[inline(always)]
    fn from(variant: AmInvalidTrIe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AM_INVALID_TR_IE` reader - Active mode invalid transition interrupt enable"]
pub type AmInvalidTrIeR = crate::BitReader<AmInvalidTrIe>;
impl AmInvalidTrIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AmInvalidTrIe {
        match self.bits {
            false => AmInvalidTrIe::AmInvalidTrIe0,
            true => AmInvalidTrIe::AmInvalidTrIe1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_am_invalid_tr_ie_0(&self) -> bool {
        *self == AmInvalidTrIe::AmInvalidTrIe0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_am_invalid_tr_ie_1(&self) -> bool {
        *self == AmInvalidTrIe::AmInvalidTrIe1
    }
}
#[doc = "Field `AM_INVALID_TR_IE` writer - Active mode invalid transition interrupt enable"]
pub type AmInvalidTrIeW<'a, REG> = crate::BitWriter<'a, REG, AmInvalidTrIe>;
impl<'a, REG> AmInvalidTrIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn am_invalid_tr_ie_0(self) -> &'a mut crate::W<REG> {
        self.variant(AmInvalidTrIe::AmInvalidTrIe0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn am_invalid_tr_ie_1(self) -> &'a mut crate::W<REG> {
        self.variant(AmInvalidTrIe::AmInvalidTrIe1)
    }
}
#[doc = "DC-DC error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcdcErrorIe {
    #[doc = "0: Disabled"]
    DcdcErrorIe0 = 0,
    #[doc = "1: Enabled"]
    DcdcErrorIe1 = 1,
}
impl From<DcdcErrorIe> for bool {
    #[inline(always)]
    fn from(variant: DcdcErrorIe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDC_ERROR_IE` reader - DC-DC error interrupt enable"]
pub type DcdcErrorIeR = crate::BitReader<DcdcErrorIe>;
impl DcdcErrorIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcdcErrorIe {
        match self.bits {
            false => DcdcErrorIe::DcdcErrorIe0,
            true => DcdcErrorIe::DcdcErrorIe1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_dcdc_error_ie_0(&self) -> bool {
        *self == DcdcErrorIe::DcdcErrorIe0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_dcdc_error_ie_1(&self) -> bool {
        *self == DcdcErrorIe::DcdcErrorIe1
    }
}
#[doc = "Field `DCDC_ERROR_IE` writer - DC-DC error interrupt enable"]
pub type DcdcErrorIeW<'a, REG> = crate::BitWriter<'a, REG, DcdcErrorIe>;
impl<'a, REG> DcdcErrorIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dcdc_error_ie_0(self) -> &'a mut crate::W<REG> {
        self.variant(DcdcErrorIe::DcdcErrorIe0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dcdc_error_ie_1(self) -> &'a mut crate::W<REG> {
        self.variant(DcdcErrorIe::DcdcErrorIe1)
    }
}
impl R {
    #[doc = "Bit 0 - LPM invalid transition interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie(&self) -> LpmInvalidTrIeR {
        LpmInvalidTrIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM invalid clock interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie(&self) -> LpmInvalidClkIeR {
        LpmInvalidClkIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active mode invalid transition interrupt enable"]
    #[inline(always)]
    pub fn am_invalid_tr_ie(&self) -> AmInvalidTrIeR {
        AmInvalidTrIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - DC-DC error interrupt enable"]
    #[inline(always)]
    pub fn dcdc_error_ie(&self) -> DcdcErrorIeR {
        DcdcErrorIeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM invalid transition interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie(&mut self) -> LpmInvalidTrIeW<PcmieSpec> {
        LpmInvalidTrIeW::new(self, 0)
    }
    #[doc = "Bit 1 - LPM invalid clock interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie(&mut self) -> LpmInvalidClkIeW<PcmieSpec> {
        LpmInvalidClkIeW::new(self, 1)
    }
    #[doc = "Bit 2 - Active mode invalid transition interrupt enable"]
    #[inline(always)]
    pub fn am_invalid_tr_ie(&mut self) -> AmInvalidTrIeW<PcmieSpec> {
        AmInvalidTrIeW::new(self, 2)
    }
    #[doc = "Bit 6 - DC-DC error interrupt enable"]
    #[inline(always)]
    pub fn dcdc_error_ie(&mut self) -> DcdcErrorIeW<PcmieSpec> {
        DcdcErrorIeW::new(self, 6)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcmieSpec;
impl crate::RegisterSpec for PcmieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcmie::R`](R) reader structure"]
impl crate::Readable for PcmieSpec {}
#[doc = "`write(|w| ..)` method takes [`pcmie::W`](W) writer structure"]
impl crate::Writable for PcmieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCMIE to value 0"]
impl crate::Resettable for PcmieSpec {
    const RESET_VALUE: u32 = 0;
}
