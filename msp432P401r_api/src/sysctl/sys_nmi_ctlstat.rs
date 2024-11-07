#[doc = "Register `SYS_NMI_CTLSTAT` reader"]
pub type R = crate::R<SysNmiCtlstatSpec>;
#[doc = "Register `SYS_NMI_CTLSTAT` writer"]
pub type W = crate::W<SysNmiCtlstatSpec>;
#[doc = "CS interrupt as a source of NMI\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsSrc {
    #[doc = "0: Disables CS interrupt as a source of NMI"]
    CsSrc0 = 0,
    #[doc = "1: Enables CS interrupt as a source of NMI"]
    CsSrc1 = 1,
}
impl From<CsSrc> for bool {
    #[inline(always)]
    fn from(variant: CsSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS_SRC` reader - CS interrupt as a source of NMI"]
pub type CsSrcR = crate::BitReader<CsSrc>;
impl CsSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CsSrc {
        match self.bits {
            false => CsSrc::CsSrc0,
            true => CsSrc::CsSrc1,
        }
    }
    #[doc = "Disables CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn is_cs_src_0(&self) -> bool {
        *self == CsSrc::CsSrc0
    }
    #[doc = "Enables CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn is_cs_src_1(&self) -> bool {
        *self == CsSrc::CsSrc1
    }
}
#[doc = "Field `CS_SRC` writer - CS interrupt as a source of NMI"]
pub type CsSrcW<'a, REG> = crate::BitWriter<'a, REG, CsSrc>;
impl<'a, REG> CsSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src_0(self) -> &'a mut crate::W<REG> {
        self.variant(CsSrc::CsSrc0)
    }
    #[doc = "Enables CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src_1(self) -> &'a mut crate::W<REG> {
        self.variant(CsSrc::CsSrc1)
    }
}
#[doc = "PSS interrupt as a source of NMI\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PssSrc {
    #[doc = "0: Disables the PSS interrupt as a source of NMI"]
    PssSrc0 = 0,
    #[doc = "1: Enables the PSS interrupt as a source of NMI"]
    PssSrc1 = 1,
}
impl From<PssSrc> for bool {
    #[inline(always)]
    fn from(variant: PssSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSS_SRC` reader - PSS interrupt as a source of NMI"]
pub type PssSrcR = crate::BitReader<PssSrc>;
impl PssSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PssSrc {
        match self.bits {
            false => PssSrc::PssSrc0,
            true => PssSrc::PssSrc1,
        }
    }
    #[doc = "Disables the PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn is_pss_src_0(&self) -> bool {
        *self == PssSrc::PssSrc0
    }
    #[doc = "Enables the PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn is_pss_src_1(&self) -> bool {
        *self == PssSrc::PssSrc1
    }
}
#[doc = "Field `PSS_SRC` writer - PSS interrupt as a source of NMI"]
pub type PssSrcW<'a, REG> = crate::BitWriter<'a, REG, PssSrc>;
impl<'a, REG> PssSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src_0(self) -> &'a mut crate::W<REG> {
        self.variant(PssSrc::PssSrc0)
    }
    #[doc = "Enables the PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src_1(self) -> &'a mut crate::W<REG> {
        self.variant(PssSrc::PssSrc1)
    }
}
#[doc = "PCM interrupt as a source of NMI\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcmSrc {
    #[doc = "0: Disbles the PCM interrupt as a source of NMI"]
    PcmSrc0 = 0,
    #[doc = "1: Enables the PCM interrupt as a source of NMI"]
    PcmSrc1 = 1,
}
impl From<PcmSrc> for bool {
    #[inline(always)]
    fn from(variant: PcmSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCM_SRC` reader - PCM interrupt as a source of NMI"]
pub type PcmSrcR = crate::BitReader<PcmSrc>;
impl PcmSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcmSrc {
        match self.bits {
            false => PcmSrc::PcmSrc0,
            true => PcmSrc::PcmSrc1,
        }
    }
    #[doc = "Disbles the PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn is_pcm_src_0(&self) -> bool {
        *self == PcmSrc::PcmSrc0
    }
    #[doc = "Enables the PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn is_pcm_src_1(&self) -> bool {
        *self == PcmSrc::PcmSrc1
    }
}
#[doc = "Field `PCM_SRC` writer - PCM interrupt as a source of NMI"]
pub type PcmSrcW<'a, REG> = crate::BitWriter<'a, REG, PcmSrc>;
impl<'a, REG> PcmSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disbles the PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src_0(self) -> &'a mut crate::W<REG> {
        self.variant(PcmSrc::PcmSrc0)
    }
    #[doc = "Enables the PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src_1(self) -> &'a mut crate::W<REG> {
        self.variant(PcmSrc::PcmSrc1)
    }
}
#[doc = "RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinSrc {
    #[doc = "0: Configures the RSTn_NMI pin as a source of POR Class Reset"]
    PinSrc0 = 0,
    #[doc = "1: Configures the RSTn_NMI pin as a source of NMI"]
    PinSrc1 = 1,
}
impl From<PinSrc> for bool {
    #[inline(always)]
    fn from(variant: PinSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN_SRC` reader - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
pub type PinSrcR = crate::BitReader<PinSrc>;
impl PinSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinSrc {
        match self.bits {
            false => PinSrc::PinSrc0,
            true => PinSrc::PinSrc1,
        }
    }
    #[doc = "Configures the RSTn_NMI pin as a source of POR Class Reset"]
    #[inline(always)]
    pub fn is_pin_src_0(&self) -> bool {
        *self == PinSrc::PinSrc0
    }
    #[doc = "Configures the RSTn_NMI pin as a source of NMI"]
    #[inline(always)]
    pub fn is_pin_src_1(&self) -> bool {
        *self == PinSrc::PinSrc1
    }
}
#[doc = "Field `PIN_SRC` writer - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
pub type PinSrcW<'a, REG> = crate::BitWriter<'a, REG, PinSrc>;
impl<'a, REG> PinSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configures the RSTn_NMI pin as a source of POR Class Reset"]
    #[inline(always)]
    pub fn pin_src_0(self) -> &'a mut crate::W<REG> {
        self.variant(PinSrc::PinSrc0)
    }
    #[doc = "Configures the RSTn_NMI pin as a source of NMI"]
    #[inline(always)]
    pub fn pin_src_1(self) -> &'a mut crate::W<REG> {
        self.variant(PinSrc::PinSrc1)
    }
}
#[doc = "CS interrupt was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsFlgEnumRead {
    #[doc = "0: indicates CS interrupt was not the source of NMI"]
    CsFlg0 = 0,
    #[doc = "1: indicates CS interrupt was the source of NMI"]
    CsFlg1 = 1,
}
impl From<CsFlgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: CsFlgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS_FLG` reader - CS interrupt was the source of NMI"]
pub type CsFlgR = crate::BitReader<CsFlgEnumRead>;
impl CsFlgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CsFlgEnumRead {
        match self.bits {
            false => CsFlgEnumRead::CsFlg0,
            true => CsFlgEnumRead::CsFlg1,
        }
    }
    #[doc = "indicates CS interrupt was not the source of NMI"]
    #[inline(always)]
    pub fn is_cs_flg_0(&self) -> bool {
        *self == CsFlgEnumRead::CsFlg0
    }
    #[doc = "indicates CS interrupt was the source of NMI"]
    #[inline(always)]
    pub fn is_cs_flg_1(&self) -> bool {
        *self == CsFlgEnumRead::CsFlg1
    }
}
#[doc = "PSS interrupt was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PssFlgEnumRead {
    #[doc = "0: indicates the PSS interrupt was not the source of NMI"]
    PssFlg0 = 0,
    #[doc = "1: indicates the PSS interrupt was the source of NMI"]
    PssFlg1 = 1,
}
impl From<PssFlgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: PssFlgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSS_FLG` reader - PSS interrupt was the source of NMI"]
pub type PssFlgR = crate::BitReader<PssFlgEnumRead>;
impl PssFlgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PssFlgEnumRead {
        match self.bits {
            false => PssFlgEnumRead::PssFlg0,
            true => PssFlgEnumRead::PssFlg1,
        }
    }
    #[doc = "indicates the PSS interrupt was not the source of NMI"]
    #[inline(always)]
    pub fn is_pss_flg_0(&self) -> bool {
        *self == PssFlgEnumRead::PssFlg0
    }
    #[doc = "indicates the PSS interrupt was the source of NMI"]
    #[inline(always)]
    pub fn is_pss_flg_1(&self) -> bool {
        *self == PssFlgEnumRead::PssFlg1
    }
}
#[doc = "PCM interrupt was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcmFlgEnumRead {
    #[doc = "0: indicates the PCM interrupt was not the source of NMI"]
    PcmFlg0 = 0,
    #[doc = "1: indicates the PCM interrupt was the source of NMI"]
    PcmFlg1 = 1,
}
impl From<PcmFlgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: PcmFlgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCM_FLG` reader - PCM interrupt was the source of NMI"]
pub type PcmFlgR = crate::BitReader<PcmFlgEnumRead>;
impl PcmFlgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcmFlgEnumRead {
        match self.bits {
            false => PcmFlgEnumRead::PcmFlg0,
            true => PcmFlgEnumRead::PcmFlg1,
        }
    }
    #[doc = "indicates the PCM interrupt was not the source of NMI"]
    #[inline(always)]
    pub fn is_pcm_flg_0(&self) -> bool {
        *self == PcmFlgEnumRead::PcmFlg0
    }
    #[doc = "indicates the PCM interrupt was the source of NMI"]
    #[inline(always)]
    pub fn is_pcm_flg_1(&self) -> bool {
        *self == PcmFlgEnumRead::PcmFlg1
    }
}
#[doc = "RSTn/NMI pin was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinFlg {
    #[doc = "0: Indicates the RSTn_NMI pin was not the source of NMI"]
    PinFlg0 = 0,
    #[doc = "1: Indicates the RSTn_NMI pin was the source of NMI"]
    PinFlg1 = 1,
}
impl From<PinFlg> for bool {
    #[inline(always)]
    fn from(variant: PinFlg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN_FLG` reader - RSTn/NMI pin was the source of NMI"]
pub type PinFlgR = crate::BitReader<PinFlg>;
impl PinFlgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PinFlg {
        match self.bits {
            false => PinFlg::PinFlg0,
            true => PinFlg::PinFlg1,
        }
    }
    #[doc = "Indicates the RSTn_NMI pin was not the source of NMI"]
    #[inline(always)]
    pub fn is_pin_flg_0(&self) -> bool {
        *self == PinFlg::PinFlg0
    }
    #[doc = "Indicates the RSTn_NMI pin was the source of NMI"]
    #[inline(always)]
    pub fn is_pin_flg_1(&self) -> bool {
        *self == PinFlg::PinFlg1
    }
}
#[doc = "Field `PIN_FLG` writer - RSTn/NMI pin was the source of NMI"]
pub type PinFlgW<'a, REG> = crate::BitWriter<'a, REG, PinFlg>;
impl<'a, REG> PinFlgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Indicates the RSTn_NMI pin was not the source of NMI"]
    #[inline(always)]
    pub fn pin_flg_0(self) -> &'a mut crate::W<REG> {
        self.variant(PinFlg::PinFlg0)
    }
    #[doc = "Indicates the RSTn_NMI pin was the source of NMI"]
    #[inline(always)]
    pub fn pin_flg_1(self) -> &'a mut crate::W<REG> {
        self.variant(PinFlg::PinFlg1)
    }
}
impl R {
    #[doc = "Bit 0 - CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src(&self) -> CsSrcR {
        CsSrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src(&self) -> PssSrcR {
        PssSrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src(&self) -> PcmSrcR {
        PcmSrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
    #[inline(always)]
    pub fn pin_src(&self) -> PinSrcR {
        PinSrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - CS interrupt was the source of NMI"]
    #[inline(always)]
    pub fn cs_flg(&self) -> CsFlgR {
        CsFlgR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PSS interrupt was the source of NMI"]
    #[inline(always)]
    pub fn pss_flg(&self) -> PssFlgR {
        PssFlgR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PCM interrupt was the source of NMI"]
    #[inline(always)]
    pub fn pcm_flg(&self) -> PcmFlgR {
        PcmFlgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RSTn/NMI pin was the source of NMI"]
    #[inline(always)]
    pub fn pin_flg(&self) -> PinFlgR {
        PinFlgR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src(&mut self) -> CsSrcW<SysNmiCtlstatSpec> {
        CsSrcW::new(self, 0)
    }
    #[doc = "Bit 1 - PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src(&mut self) -> PssSrcW<SysNmiCtlstatSpec> {
        PssSrcW::new(self, 1)
    }
    #[doc = "Bit 2 - PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src(&mut self) -> PcmSrcW<SysNmiCtlstatSpec> {
        PcmSrcW::new(self, 2)
    }
    #[doc = "Bit 3 - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
    #[inline(always)]
    pub fn pin_src(&mut self) -> PinSrcW<SysNmiCtlstatSpec> {
        PinSrcW::new(self, 3)
    }
    #[doc = "Bit 19 - RSTn/NMI pin was the source of NMI"]
    #[inline(always)]
    pub fn pin_flg(&mut self) -> PinFlgW<SysNmiCtlstatSpec> {
        PinFlgW::new(self, 19)
    }
}
#[doc = "NMI Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_nmi_ctlstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_nmi_ctlstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysNmiCtlstatSpec;
impl crate::RegisterSpec for SysNmiCtlstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_nmi_ctlstat::R`](R) reader structure"]
impl crate::Readable for SysNmiCtlstatSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_nmi_ctlstat::W`](W) writer structure"]
impl crate::Writable for SysNmiCtlstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_NMI_CTLSTAT to value 0x07"]
impl crate::Resettable for SysNmiCtlstatSpec {
    const RESET_VALUE: u32 = 0x07;
}
