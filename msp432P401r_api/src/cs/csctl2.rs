#[doc = "Register `CSCTL2` reader"]
pub type R = crate::R<Csctl2Spec>;
#[doc = "Register `CSCTL2` writer"]
pub type W = crate::W<Csctl2Spec>;
#[doc = "LFXT oscillator current can be adjusted to its drive needs\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfxtdrive {
    #[doc = "0: Lowest drive strength and current consumption LFXT oscillator."]
    Lfxtdrive0 = 0,
    #[doc = "1: Increased drive strength LFXT oscillator."]
    Lfxtdrive1 = 1,
    #[doc = "2: Increased drive strength LFXT oscillator."]
    Lfxtdrive2 = 2,
    #[doc = "3: Maximum drive strength and maximum current consumption LFXT oscillator."]
    Lfxtdrive3 = 3,
}
impl From<Lfxtdrive> for u8 {
    #[inline(always)]
    fn from(variant: Lfxtdrive) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfxtdrive {
    type Ux = u8;
}
impl crate::IsEnum for Lfxtdrive {}
#[doc = "Field `LFXTDRIVE` reader - LFXT oscillator current can be adjusted to its drive needs"]
pub type LfxtdriveR = crate::FieldReader<Lfxtdrive>;
impl LfxtdriveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxtdrive {
        match self.bits {
            0 => Lfxtdrive::Lfxtdrive0,
            1 => Lfxtdrive::Lfxtdrive1,
            2 => Lfxtdrive::Lfxtdrive2,
            3 => Lfxtdrive::Lfxtdrive3,
            _ => unreachable!(),
        }
    }
    #[doc = "Lowest drive strength and current consumption LFXT oscillator."]
    #[inline(always)]
    pub fn is_lfxtdrive_0(&self) -> bool {
        *self == Lfxtdrive::Lfxtdrive0
    }
    #[doc = "Increased drive strength LFXT oscillator."]
    #[inline(always)]
    pub fn is_lfxtdrive_1(&self) -> bool {
        *self == Lfxtdrive::Lfxtdrive1
    }
    #[doc = "Increased drive strength LFXT oscillator."]
    #[inline(always)]
    pub fn is_lfxtdrive_2(&self) -> bool {
        *self == Lfxtdrive::Lfxtdrive2
    }
    #[doc = "Maximum drive strength and maximum current consumption LFXT oscillator."]
    #[inline(always)]
    pub fn is_lfxtdrive_3(&self) -> bool {
        *self == Lfxtdrive::Lfxtdrive3
    }
}
#[doc = "Field `LFXTDRIVE` writer - LFXT oscillator current can be adjusted to its drive needs"]
pub type LfxtdriveW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lfxtdrive, crate::Safe>;
impl<'a, REG> LfxtdriveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest drive strength and current consumption LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtdrive::Lfxtdrive0)
    }
    #[doc = "Increased drive strength LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtdrive::Lfxtdrive1)
    }
    #[doc = "Increased drive strength LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_2(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtdrive::Lfxtdrive2)
    }
    #[doc = "Maximum drive strength and maximum current consumption LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_3(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtdrive::Lfxtdrive3)
    }
}
#[doc = "Disables the automatic gain control of the LFXT crystal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfxtagcoff {
    #[doc = "0: AGC enabled."]
    Lfxtagcoff0 = 0,
    #[doc = "1: AGC disabled."]
    Lfxtagcoff1 = 1,
}
impl From<Lfxtagcoff> for bool {
    #[inline(always)]
    fn from(variant: Lfxtagcoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTAGCOFF` reader - Disables the automatic gain control of the LFXT crystal"]
pub type LfxtagcoffR = crate::BitReader<Lfxtagcoff>;
impl LfxtagcoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxtagcoff {
        match self.bits {
            false => Lfxtagcoff::Lfxtagcoff0,
            true => Lfxtagcoff::Lfxtagcoff1,
        }
    }
    #[doc = "AGC enabled."]
    #[inline(always)]
    pub fn is_lfxtagcoff_0(&self) -> bool {
        *self == Lfxtagcoff::Lfxtagcoff0
    }
    #[doc = "AGC disabled."]
    #[inline(always)]
    pub fn is_lfxtagcoff_1(&self) -> bool {
        *self == Lfxtagcoff::Lfxtagcoff1
    }
}
#[doc = "Field `LFXTAGCOFF` writer - Disables the automatic gain control of the LFXT crystal"]
pub type LfxtagcoffW<'a, REG> = crate::BitWriter<'a, REG, Lfxtagcoff>;
impl<'a, REG> LfxtagcoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AGC enabled."]
    #[inline(always)]
    pub fn lfxtagcoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtagcoff::Lfxtagcoff0)
    }
    #[doc = "AGC disabled."]
    #[inline(always)]
    pub fn lfxtagcoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtagcoff::Lfxtagcoff1)
    }
}
#[doc = "Turns on the LFXT oscillator regardless if used as a clock resource\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfxtEn {
    #[doc = "0: LFXT is on if it is used as a source for ACLK, MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    LfxtEn0 = 0,
    #[doc = "1: LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation."]
    LfxtEn1 = 1,
}
impl From<LfxtEn> for bool {
    #[inline(always)]
    fn from(variant: LfxtEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXT_EN` reader - Turns on the LFXT oscillator regardless if used as a clock resource"]
pub type LfxtEnR = crate::BitReader<LfxtEn>;
impl LfxtEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LfxtEn {
        match self.bits {
            false => LfxtEn::LfxtEn0,
            true => LfxtEn::LfxtEn1,
        }
    }
    #[doc = "LFXT is on if it is used as a source for ACLK, MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    #[inline(always)]
    pub fn is_lfxt_en_0(&self) -> bool {
        *self == LfxtEn::LfxtEn0
    }
    #[doc = "LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation."]
    #[inline(always)]
    pub fn is_lfxt_en_1(&self) -> bool {
        *self == LfxtEn::LfxtEn1
    }
}
#[doc = "Field `LFXT_EN` writer - Turns on the LFXT oscillator regardless if used as a clock resource"]
pub type LfxtEnW<'a, REG> = crate::BitWriter<'a, REG, LfxtEn>;
impl<'a, REG> LfxtEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LFXT is on if it is used as a source for ACLK, MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    #[inline(always)]
    pub fn lfxt_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(LfxtEn::LfxtEn0)
    }
    #[doc = "LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation."]
    #[inline(always)]
    pub fn lfxt_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(LfxtEn::LfxtEn1)
    }
}
#[doc = "LFXT bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfxtbypass {
    #[doc = "0: LFXT sourced by external crystal."]
    Lfxtbypass0 = 0,
    #[doc = "1: LFXT sourced by external square wave."]
    Lfxtbypass1 = 1,
}
impl From<Lfxtbypass> for bool {
    #[inline(always)]
    fn from(variant: Lfxtbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTBYPASS` reader - LFXT bypass select"]
pub type LfxtbypassR = crate::BitReader<Lfxtbypass>;
impl LfxtbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxtbypass {
        match self.bits {
            false => Lfxtbypass::Lfxtbypass0,
            true => Lfxtbypass::Lfxtbypass1,
        }
    }
    #[doc = "LFXT sourced by external crystal."]
    #[inline(always)]
    pub fn is_lfxtbypass_0(&self) -> bool {
        *self == Lfxtbypass::Lfxtbypass0
    }
    #[doc = "LFXT sourced by external square wave."]
    #[inline(always)]
    pub fn is_lfxtbypass_1(&self) -> bool {
        *self == Lfxtbypass::Lfxtbypass1
    }
}
#[doc = "Field `LFXTBYPASS` writer - LFXT bypass select"]
pub type LfxtbypassW<'a, REG> = crate::BitWriter<'a, REG, Lfxtbypass>;
impl<'a, REG> LfxtbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LFXT sourced by external crystal."]
    #[inline(always)]
    pub fn lfxtbypass_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtbypass::Lfxtbypass0)
    }
    #[doc = "LFXT sourced by external square wave."]
    #[inline(always)]
    pub fn lfxtbypass_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtbypass::Lfxtbypass1)
    }
}
#[doc = "HFXT oscillator drive selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfxtdrive {
    #[doc = "0: To be used for HFXTFREQ setting 000b"]
    Hfxtdrive0 = 0,
    #[doc = "1: To be used for HFXTFREQ settings 001b to 110b"]
    Hfxtdrive1 = 1,
}
impl From<Hfxtdrive> for bool {
    #[inline(always)]
    fn from(variant: Hfxtdrive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXTDRIVE` reader - HFXT oscillator drive selection"]
pub type HfxtdriveR = crate::BitReader<Hfxtdrive>;
impl HfxtdriveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxtdrive {
        match self.bits {
            false => Hfxtdrive::Hfxtdrive0,
            true => Hfxtdrive::Hfxtdrive1,
        }
    }
    #[doc = "To be used for HFXTFREQ setting 000b"]
    #[inline(always)]
    pub fn is_hfxtdrive_0(&self) -> bool {
        *self == Hfxtdrive::Hfxtdrive0
    }
    #[doc = "To be used for HFXTFREQ settings 001b to 110b"]
    #[inline(always)]
    pub fn is_hfxtdrive_1(&self) -> bool {
        *self == Hfxtdrive::Hfxtdrive1
    }
}
#[doc = "Field `HFXTDRIVE` writer - HFXT oscillator drive selection"]
pub type HfxtdriveW<'a, REG> = crate::BitWriter<'a, REG, Hfxtdrive>;
impl<'a, REG> HfxtdriveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "To be used for HFXTFREQ setting 000b"]
    #[inline(always)]
    pub fn hfxtdrive_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtdrive::Hfxtdrive0)
    }
    #[doc = "To be used for HFXTFREQ settings 001b to 110b"]
    #[inline(always)]
    pub fn hfxtdrive_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtdrive::Hfxtdrive1)
    }
}
#[doc = "HFXT frequency selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfxtfreq {
    #[doc = "0: 1 MHz to 4 MHz"]
    Hfxtfreq0 = 0,
    #[doc = "1: >4 MHz to 8 MHz"]
    Hfxtfreq1 = 1,
    #[doc = "2: >8 MHz to 16 MHz"]
    Hfxtfreq2 = 2,
    #[doc = "3: >16 MHz to 24 MHz"]
    Hfxtfreq3 = 3,
    #[doc = "4: >24 MHz to 32 MHz"]
    Hfxtfreq4 = 4,
    #[doc = "5: >32 MHz to 40 MHz"]
    Hfxtfreq5 = 5,
    #[doc = "6: >40 MHz to 48 MHz"]
    Hfxtfreq6 = 6,
}
impl From<Hfxtfreq> for u8 {
    #[inline(always)]
    fn from(variant: Hfxtfreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfxtfreq {
    type Ux = u8;
}
impl crate::IsEnum for Hfxtfreq {}
#[doc = "Field `HFXTFREQ` reader - HFXT frequency selection"]
pub type HfxtfreqR = crate::FieldReader<Hfxtfreq>;
impl HfxtfreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hfxtfreq> {
        match self.bits {
            0 => Some(Hfxtfreq::Hfxtfreq0),
            1 => Some(Hfxtfreq::Hfxtfreq1),
            2 => Some(Hfxtfreq::Hfxtfreq2),
            3 => Some(Hfxtfreq::Hfxtfreq3),
            4 => Some(Hfxtfreq::Hfxtfreq4),
            5 => Some(Hfxtfreq::Hfxtfreq5),
            6 => Some(Hfxtfreq::Hfxtfreq6),
            _ => None,
        }
    }
    #[doc = "1 MHz to 4 MHz"]
    #[inline(always)]
    pub fn is_hfxtfreq_0(&self) -> bool {
        *self == Hfxtfreq::Hfxtfreq0
    }
    #[doc = ">4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn is_hfxtfreq_1(&self) -> bool {
        *self == Hfxtfreq::Hfxtfreq1
    }
    #[doc = ">8 MHz to 16 MHz"]
    #[inline(always)]
    pub fn is_hfxtfreq_2(&self) -> bool {
        *self == Hfxtfreq::Hfxtfreq2
    }
    #[doc = ">16 MHz to 24 MHz"]
    #[inline(always)]
    pub fn is_hfxtfreq_3(&self) -> bool {
        *self == Hfxtfreq::Hfxtfreq3
    }
    #[doc = ">24 MHz to 32 MHz"]
    #[inline(always)]
    pub fn is_hfxtfreq_4(&self) -> bool {
        *self == Hfxtfreq::Hfxtfreq4
    }
    #[doc = ">32 MHz to 40 MHz"]
    #[inline(always)]
    pub fn is_hfxtfreq_5(&self) -> bool {
        *self == Hfxtfreq::Hfxtfreq5
    }
    #[doc = ">40 MHz to 48 MHz"]
    #[inline(always)]
    pub fn is_hfxtfreq_6(&self) -> bool {
        *self == Hfxtfreq::Hfxtfreq6
    }
}
#[doc = "Field `HFXTFREQ` writer - HFXT frequency selection"]
pub type HfxtfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hfxtfreq>;
impl<'a, REG> HfxtfreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 MHz to 4 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtfreq::Hfxtfreq0)
    }
    #[doc = ">4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtfreq::Hfxtfreq1)
    }
    #[doc = ">8 MHz to 16 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_2(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtfreq::Hfxtfreq2)
    }
    #[doc = ">16 MHz to 24 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_3(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtfreq::Hfxtfreq3)
    }
    #[doc = ">24 MHz to 32 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_4(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtfreq::Hfxtfreq4)
    }
    #[doc = ">32 MHz to 40 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_5(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtfreq::Hfxtfreq5)
    }
    #[doc = ">40 MHz to 48 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_6(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtfreq::Hfxtfreq6)
    }
}
#[doc = "Turns on the HFXT oscillator regardless if used as a clock resource\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HfxtEn {
    #[doc = "0: HFXT is on if it is used as a source for MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    HfxtEn0 = 0,
    #[doc = "1: HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation."]
    HfxtEn1 = 1,
}
impl From<HfxtEn> for bool {
    #[inline(always)]
    fn from(variant: HfxtEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXT_EN` reader - Turns on the HFXT oscillator regardless if used as a clock resource"]
pub type HfxtEnR = crate::BitReader<HfxtEn>;
impl HfxtEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HfxtEn {
        match self.bits {
            false => HfxtEn::HfxtEn0,
            true => HfxtEn::HfxtEn1,
        }
    }
    #[doc = "HFXT is on if it is used as a source for MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    #[inline(always)]
    pub fn is_hfxt_en_0(&self) -> bool {
        *self == HfxtEn::HfxtEn0
    }
    #[doc = "HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation."]
    #[inline(always)]
    pub fn is_hfxt_en_1(&self) -> bool {
        *self == HfxtEn::HfxtEn1
    }
}
#[doc = "Field `HFXT_EN` writer - Turns on the HFXT oscillator regardless if used as a clock resource"]
pub type HfxtEnW<'a, REG> = crate::BitWriter<'a, REG, HfxtEn>;
impl<'a, REG> HfxtEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFXT is on if it is used as a source for MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    #[inline(always)]
    pub fn hfxt_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(HfxtEn::HfxtEn0)
    }
    #[doc = "HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation."]
    #[inline(always)]
    pub fn hfxt_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(HfxtEn::HfxtEn1)
    }
}
#[doc = "HFXT bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfxtbypass {
    #[doc = "0: HFXT sourced by external crystal."]
    Hfxtbypass0 = 0,
    #[doc = "1: HFXT sourced by external square wave."]
    Hfxtbypass1 = 1,
}
impl From<Hfxtbypass> for bool {
    #[inline(always)]
    fn from(variant: Hfxtbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXTBYPASS` reader - HFXT bypass select"]
pub type HfxtbypassR = crate::BitReader<Hfxtbypass>;
impl HfxtbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxtbypass {
        match self.bits {
            false => Hfxtbypass::Hfxtbypass0,
            true => Hfxtbypass::Hfxtbypass1,
        }
    }
    #[doc = "HFXT sourced by external crystal."]
    #[inline(always)]
    pub fn is_hfxtbypass_0(&self) -> bool {
        *self == Hfxtbypass::Hfxtbypass0
    }
    #[doc = "HFXT sourced by external square wave."]
    #[inline(always)]
    pub fn is_hfxtbypass_1(&self) -> bool {
        *self == Hfxtbypass::Hfxtbypass1
    }
}
#[doc = "Field `HFXTBYPASS` writer - HFXT bypass select"]
pub type HfxtbypassW<'a, REG> = crate::BitWriter<'a, REG, Hfxtbypass>;
impl<'a, REG> HfxtbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFXT sourced by external crystal."]
    #[inline(always)]
    pub fn hfxtbypass_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtbypass::Hfxtbypass0)
    }
    #[doc = "HFXT sourced by external square wave."]
    #[inline(always)]
    pub fn hfxtbypass_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtbypass::Hfxtbypass1)
    }
}
impl R {
    #[doc = "Bits 0:1 - LFXT oscillator current can be adjusted to its drive needs"]
    #[inline(always)]
    pub fn lfxtdrive(&self) -> LfxtdriveR {
        LfxtdriveR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - Disables the automatic gain control of the LFXT crystal"]
    #[inline(always)]
    pub fn lfxtagcoff(&self) -> LfxtagcoffR {
        LfxtagcoffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Turns on the LFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn lfxt_en(&self) -> LfxtEnR {
        LfxtEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LFXT bypass select"]
    #[inline(always)]
    pub fn lfxtbypass(&self) -> LfxtbypassR {
        LfxtbypassR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - HFXT oscillator drive selection"]
    #[inline(always)]
    pub fn hfxtdrive(&self) -> HfxtdriveR {
        HfxtdriveR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22 - HFXT frequency selection"]
    #[inline(always)]
    pub fn hfxtfreq(&self) -> HfxtfreqR {
        HfxtfreqR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Turns on the HFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn hfxt_en(&self) -> HfxtEnR {
        HfxtEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HFXT bypass select"]
    #[inline(always)]
    pub fn hfxtbypass(&self) -> HfxtbypassR {
        HfxtbypassR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LFXT oscillator current can be adjusted to its drive needs"]
    #[inline(always)]
    pub fn lfxtdrive(&mut self) -> LfxtdriveW<Csctl2Spec> {
        LfxtdriveW::new(self, 0)
    }
    #[doc = "Bit 7 - Disables the automatic gain control of the LFXT crystal"]
    #[inline(always)]
    pub fn lfxtagcoff(&mut self) -> LfxtagcoffW<Csctl2Spec> {
        LfxtagcoffW::new(self, 7)
    }
    #[doc = "Bit 8 - Turns on the LFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn lfxt_en(&mut self) -> LfxtEnW<Csctl2Spec> {
        LfxtEnW::new(self, 8)
    }
    #[doc = "Bit 9 - LFXT bypass select"]
    #[inline(always)]
    pub fn lfxtbypass(&mut self) -> LfxtbypassW<Csctl2Spec> {
        LfxtbypassW::new(self, 9)
    }
    #[doc = "Bit 16 - HFXT oscillator drive selection"]
    #[inline(always)]
    pub fn hfxtdrive(&mut self) -> HfxtdriveW<Csctl2Spec> {
        HfxtdriveW::new(self, 16)
    }
    #[doc = "Bits 20:22 - HFXT frequency selection"]
    #[inline(always)]
    pub fn hfxtfreq(&mut self) -> HfxtfreqW<Csctl2Spec> {
        HfxtfreqW::new(self, 20)
    }
    #[doc = "Bit 24 - Turns on the HFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn hfxt_en(&mut self) -> HfxtEnW<Csctl2Spec> {
        HfxtEnW::new(self, 24)
    }
    #[doc = "Bit 25 - HFXT bypass select"]
    #[inline(always)]
    pub fn hfxtbypass(&mut self) -> HfxtbypassW<Csctl2Spec> {
        HfxtbypassW::new(self, 25)
    }
}
#[doc = "Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl2Spec;
impl crate::RegisterSpec for Csctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csctl2::R`](R) reader structure"]
impl crate::Readable for Csctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl2::W`](W) writer structure"]
impl crate::Writable for Csctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSCTL2 to value 0x0001_0003"]
impl crate::Resettable for Csctl2Spec {
    const RESET_VALUE: u32 = 0x0001_0003;
}
