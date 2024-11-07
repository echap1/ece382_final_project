#[doc = "Register `ADC14CTL0` reader"]
pub type R = crate::R<Adc14ctl0Spec>;
#[doc = "Register `ADC14CTL0` writer"]
pub type W = crate::W<Adc14ctl0Spec>;
#[doc = "ADC14 start conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14sc {
    #[doc = "0: No sample-and-conversion-start"]
    Adc14sc0 = 0,
    #[doc = "1: Start sample-and-conversion"]
    Adc14sc1 = 1,
}
impl From<Adc14sc> for bool {
    #[inline(always)]
    fn from(variant: Adc14sc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14SC` reader - ADC14 start conversion"]
pub type Adc14scR = crate::BitReader<Adc14sc>;
impl Adc14scR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14sc {
        match self.bits {
            false => Adc14sc::Adc14sc0,
            true => Adc14sc::Adc14sc1,
        }
    }
    #[doc = "No sample-and-conversion-start"]
    #[inline(always)]
    pub fn is_adc14sc_0(&self) -> bool {
        *self == Adc14sc::Adc14sc0
    }
    #[doc = "Start sample-and-conversion"]
    #[inline(always)]
    pub fn is_adc14sc_1(&self) -> bool {
        *self == Adc14sc::Adc14sc1
    }
}
#[doc = "Field `ADC14SC` writer - ADC14 start conversion"]
pub type Adc14scW<'a, REG> = crate::BitWriter<'a, REG, Adc14sc>;
impl<'a, REG> Adc14scW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No sample-and-conversion-start"]
    #[inline(always)]
    pub fn adc14sc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sc::Adc14sc0)
    }
    #[doc = "Start sample-and-conversion"]
    #[inline(always)]
    pub fn adc14sc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sc::Adc14sc1)
    }
}
#[doc = "ADC14 enable conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14enc {
    #[doc = "0: ADC14 disabled"]
    Adc14enc0 = 0,
    #[doc = "1: ADC14 enabled"]
    Adc14enc1 = 1,
}
impl From<Adc14enc> for bool {
    #[inline(always)]
    fn from(variant: Adc14enc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14ENC` reader - ADC14 enable conversion"]
pub type Adc14encR = crate::BitReader<Adc14enc>;
impl Adc14encR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14enc {
        match self.bits {
            false => Adc14enc::Adc14enc0,
            true => Adc14enc::Adc14enc1,
        }
    }
    #[doc = "ADC14 disabled"]
    #[inline(always)]
    pub fn is_adc14enc_0(&self) -> bool {
        *self == Adc14enc::Adc14enc0
    }
    #[doc = "ADC14 enabled"]
    #[inline(always)]
    pub fn is_adc14enc_1(&self) -> bool {
        *self == Adc14enc::Adc14enc1
    }
}
#[doc = "Field `ADC14ENC` writer - ADC14 enable conversion"]
pub type Adc14encW<'a, REG> = crate::BitWriter<'a, REG, Adc14enc>;
impl<'a, REG> Adc14encW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC14 disabled"]
    #[inline(always)]
    pub fn adc14enc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14enc::Adc14enc0)
    }
    #[doc = "ADC14 enabled"]
    #[inline(always)]
    pub fn adc14enc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14enc::Adc14enc1)
    }
}
#[doc = "ADC14 on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14on {
    #[doc = "0: ADC14 off"]
    Adc14on0 = 0,
    #[doc = "1: ADC14 on. ADC core is ready to power up when a valid conversion is triggered."]
    Adc14on1 = 1,
}
impl From<Adc14on> for bool {
    #[inline(always)]
    fn from(variant: Adc14on) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14ON` reader - ADC14 on"]
pub type Adc14onR = crate::BitReader<Adc14on>;
impl Adc14onR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14on {
        match self.bits {
            false => Adc14on::Adc14on0,
            true => Adc14on::Adc14on1,
        }
    }
    #[doc = "ADC14 off"]
    #[inline(always)]
    pub fn is_adc14on_0(&self) -> bool {
        *self == Adc14on::Adc14on0
    }
    #[doc = "ADC14 on. ADC core is ready to power up when a valid conversion is triggered."]
    #[inline(always)]
    pub fn is_adc14on_1(&self) -> bool {
        *self == Adc14on::Adc14on1
    }
}
#[doc = "Field `ADC14ON` writer - ADC14 on"]
pub type Adc14onW<'a, REG> = crate::BitWriter<'a, REG, Adc14on>;
impl<'a, REG> Adc14onW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC14 off"]
    #[inline(always)]
    pub fn adc14on_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14on::Adc14on0)
    }
    #[doc = "ADC14 on. ADC core is ready to power up when a valid conversion is triggered."]
    #[inline(always)]
    pub fn adc14on_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14on::Adc14on1)
    }
}
#[doc = "ADC14 multiple sample and conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14msc {
    #[doc = "0: The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert"]
    Adc14msc0 = 0,
    #[doc = "1: The first rising edge of the SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed"]
    Adc14msc1 = 1,
}
impl From<Adc14msc> for bool {
    #[inline(always)]
    fn from(variant: Adc14msc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14MSC` reader - ADC14 multiple sample and conversion"]
pub type Adc14mscR = crate::BitReader<Adc14msc>;
impl Adc14mscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14msc {
        match self.bits {
            false => Adc14msc::Adc14msc0,
            true => Adc14msc::Adc14msc1,
        }
    }
    #[doc = "The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert"]
    #[inline(always)]
    pub fn is_adc14msc_0(&self) -> bool {
        *self == Adc14msc::Adc14msc0
    }
    #[doc = "The first rising edge of the SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed"]
    #[inline(always)]
    pub fn is_adc14msc_1(&self) -> bool {
        *self == Adc14msc::Adc14msc1
    }
}
#[doc = "Field `ADC14MSC` writer - ADC14 multiple sample and conversion"]
pub type Adc14mscW<'a, REG> = crate::BitWriter<'a, REG, Adc14msc>;
impl<'a, REG> Adc14mscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert"]
    #[inline(always)]
    pub fn adc14msc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14msc::Adc14msc0)
    }
    #[doc = "The first rising edge of the SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed"]
    #[inline(always)]
    pub fn adc14msc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14msc::Adc14msc1)
    }
}
#[doc = "ADC14 sample-and-hold time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc14sht0 {
    #[doc = "0: 4"]
    Adc14sht0_0 = 0,
    #[doc = "1: 8"]
    Adc14sht0_1 = 1,
    #[doc = "2: 16"]
    Adc14sht0_2 = 2,
    #[doc = "3: 32"]
    Adc14sht0_3 = 3,
    #[doc = "4: 64"]
    Adc14sht0_4 = 4,
    #[doc = "5: 96"]
    Adc14sht0_5 = 5,
    #[doc = "6: 128"]
    Adc14sht0_6 = 6,
    #[doc = "7: 192"]
    Adc14sht0_7 = 7,
}
impl From<Adc14sht0> for u8 {
    #[inline(always)]
    fn from(variant: Adc14sht0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14sht0 {
    type Ux = u8;
}
impl crate::IsEnum for Adc14sht0 {}
#[doc = "Field `ADC14SHT0` reader - ADC14 sample-and-hold time"]
pub type Adc14sht0R = crate::FieldReader<Adc14sht0>;
impl Adc14sht0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc14sht0> {
        match self.bits {
            0 => Some(Adc14sht0::Adc14sht0_0),
            1 => Some(Adc14sht0::Adc14sht0_1),
            2 => Some(Adc14sht0::Adc14sht0_2),
            3 => Some(Adc14sht0::Adc14sht0_3),
            4 => Some(Adc14sht0::Adc14sht0_4),
            5 => Some(Adc14sht0::Adc14sht0_5),
            6 => Some(Adc14sht0::Adc14sht0_6),
            7 => Some(Adc14sht0::Adc14sht0_7),
            _ => None,
        }
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_adc14sht0_0(&self) -> bool {
        *self == Adc14sht0::Adc14sht0_0
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_adc14sht0_1(&self) -> bool {
        *self == Adc14sht0::Adc14sht0_1
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_adc14sht0_2(&self) -> bool {
        *self == Adc14sht0::Adc14sht0_2
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn is_adc14sht0_3(&self) -> bool {
        *self == Adc14sht0::Adc14sht0_3
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn is_adc14sht0_4(&self) -> bool {
        *self == Adc14sht0::Adc14sht0_4
    }
    #[doc = "96"]
    #[inline(always)]
    pub fn is_adc14sht0_5(&self) -> bool {
        *self == Adc14sht0::Adc14sht0_5
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_adc14sht0_6(&self) -> bool {
        *self == Adc14sht0::Adc14sht0_6
    }
    #[doc = "192"]
    #[inline(always)]
    pub fn is_adc14sht0_7(&self) -> bool {
        *self == Adc14sht0::Adc14sht0_7
    }
}
#[doc = "Field `ADC14SHT0` writer - ADC14 sample-and-hold time"]
pub type Adc14sht0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Adc14sht0>;
impl<'a, REG> Adc14sht0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4"]
    #[inline(always)]
    pub fn adc14sht0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht0::Adc14sht0_0)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn adc14sht0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht0::Adc14sht0_1)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn adc14sht0_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht0::Adc14sht0_2)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn adc14sht0_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht0::Adc14sht0_3)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn adc14sht0_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht0::Adc14sht0_4)
    }
    #[doc = "96"]
    #[inline(always)]
    pub fn adc14sht0_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht0::Adc14sht0_5)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn adc14sht0_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht0::Adc14sht0_6)
    }
    #[doc = "192"]
    #[inline(always)]
    pub fn adc14sht0_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht0::Adc14sht0_7)
    }
}
#[doc = "ADC14 sample-and-hold time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc14sht1 {
    #[doc = "0: 4"]
    Adc14sht1_0 = 0,
    #[doc = "1: 8"]
    Adc14sht1_1 = 1,
    #[doc = "2: 16"]
    Adc14sht1_2 = 2,
    #[doc = "3: 32"]
    Adc14sht1_3 = 3,
    #[doc = "4: 64"]
    Adc14sht1_4 = 4,
    #[doc = "5: 96"]
    Adc14sht1_5 = 5,
    #[doc = "6: 128"]
    Adc14sht1_6 = 6,
    #[doc = "7: 192"]
    Adc14sht1_7 = 7,
}
impl From<Adc14sht1> for u8 {
    #[inline(always)]
    fn from(variant: Adc14sht1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14sht1 {
    type Ux = u8;
}
impl crate::IsEnum for Adc14sht1 {}
#[doc = "Field `ADC14SHT1` reader - ADC14 sample-and-hold time"]
pub type Adc14sht1R = crate::FieldReader<Adc14sht1>;
impl Adc14sht1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc14sht1> {
        match self.bits {
            0 => Some(Adc14sht1::Adc14sht1_0),
            1 => Some(Adc14sht1::Adc14sht1_1),
            2 => Some(Adc14sht1::Adc14sht1_2),
            3 => Some(Adc14sht1::Adc14sht1_3),
            4 => Some(Adc14sht1::Adc14sht1_4),
            5 => Some(Adc14sht1::Adc14sht1_5),
            6 => Some(Adc14sht1::Adc14sht1_6),
            7 => Some(Adc14sht1::Adc14sht1_7),
            _ => None,
        }
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_adc14sht1_0(&self) -> bool {
        *self == Adc14sht1::Adc14sht1_0
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_adc14sht1_1(&self) -> bool {
        *self == Adc14sht1::Adc14sht1_1
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_adc14sht1_2(&self) -> bool {
        *self == Adc14sht1::Adc14sht1_2
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn is_adc14sht1_3(&self) -> bool {
        *self == Adc14sht1::Adc14sht1_3
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn is_adc14sht1_4(&self) -> bool {
        *self == Adc14sht1::Adc14sht1_4
    }
    #[doc = "96"]
    #[inline(always)]
    pub fn is_adc14sht1_5(&self) -> bool {
        *self == Adc14sht1::Adc14sht1_5
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_adc14sht1_6(&self) -> bool {
        *self == Adc14sht1::Adc14sht1_6
    }
    #[doc = "192"]
    #[inline(always)]
    pub fn is_adc14sht1_7(&self) -> bool {
        *self == Adc14sht1::Adc14sht1_7
    }
}
#[doc = "Field `ADC14SHT1` writer - ADC14 sample-and-hold time"]
pub type Adc14sht1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Adc14sht1>;
impl<'a, REG> Adc14sht1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4"]
    #[inline(always)]
    pub fn adc14sht1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht1::Adc14sht1_0)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn adc14sht1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht1::Adc14sht1_1)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn adc14sht1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht1::Adc14sht1_2)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn adc14sht1_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht1::Adc14sht1_3)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn adc14sht1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht1::Adc14sht1_4)
    }
    #[doc = "96"]
    #[inline(always)]
    pub fn adc14sht1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht1::Adc14sht1_5)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn adc14sht1_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht1::Adc14sht1_6)
    }
    #[doc = "192"]
    #[inline(always)]
    pub fn adc14sht1_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14sht1::Adc14sht1_7)
    }
}
#[doc = "ADC14 busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14busyEnumRead {
    #[doc = "0: No operation is active"]
    Adc14busy0 = 0,
    #[doc = "1: A sequence, sample, or conversion is active"]
    Adc14busy1 = 1,
}
impl From<Adc14busyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: Adc14busyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14BUSY` reader - ADC14 busy"]
pub type Adc14busyR = crate::BitReader<Adc14busyEnumRead>;
impl Adc14busyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14busyEnumRead {
        match self.bits {
            false => Adc14busyEnumRead::Adc14busy0,
            true => Adc14busyEnumRead::Adc14busy1,
        }
    }
    #[doc = "No operation is active"]
    #[inline(always)]
    pub fn is_adc14busy_0(&self) -> bool {
        *self == Adc14busyEnumRead::Adc14busy0
    }
    #[doc = "A sequence, sample, or conversion is active"]
    #[inline(always)]
    pub fn is_adc14busy_1(&self) -> bool {
        *self == Adc14busyEnumRead::Adc14busy1
    }
}
#[doc = "ADC14 conversion sequence mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc14conseq {
    #[doc = "0: Single-channel, single-conversion"]
    Adc14conseq0 = 0,
    #[doc = "1: Sequence-of-channels"]
    Adc14conseq1 = 1,
    #[doc = "2: Repeat-single-channel"]
    Adc14conseq2 = 2,
    #[doc = "3: Repeat-sequence-of-channels"]
    Adc14conseq3 = 3,
}
impl From<Adc14conseq> for u8 {
    #[inline(always)]
    fn from(variant: Adc14conseq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14conseq {
    type Ux = u8;
}
impl crate::IsEnum for Adc14conseq {}
#[doc = "Field `ADC14CONSEQ` reader - ADC14 conversion sequence mode select"]
pub type Adc14conseqR = crate::FieldReader<Adc14conseq>;
impl Adc14conseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14conseq {
        match self.bits {
            0 => Adc14conseq::Adc14conseq0,
            1 => Adc14conseq::Adc14conseq1,
            2 => Adc14conseq::Adc14conseq2,
            3 => Adc14conseq::Adc14conseq3,
            _ => unreachable!(),
        }
    }
    #[doc = "Single-channel, single-conversion"]
    #[inline(always)]
    pub fn is_adc14conseq_0(&self) -> bool {
        *self == Adc14conseq::Adc14conseq0
    }
    #[doc = "Sequence-of-channels"]
    #[inline(always)]
    pub fn is_adc14conseq_1(&self) -> bool {
        *self == Adc14conseq::Adc14conseq1
    }
    #[doc = "Repeat-single-channel"]
    #[inline(always)]
    pub fn is_adc14conseq_2(&self) -> bool {
        *self == Adc14conseq::Adc14conseq2
    }
    #[doc = "Repeat-sequence-of-channels"]
    #[inline(always)]
    pub fn is_adc14conseq_3(&self) -> bool {
        *self == Adc14conseq::Adc14conseq3
    }
}
#[doc = "Field `ADC14CONSEQ` writer - ADC14 conversion sequence mode select"]
pub type Adc14conseqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc14conseq, crate::Safe>;
impl<'a, REG> Adc14conseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-channel, single-conversion"]
    #[inline(always)]
    pub fn adc14conseq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14conseq::Adc14conseq0)
    }
    #[doc = "Sequence-of-channels"]
    #[inline(always)]
    pub fn adc14conseq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14conseq::Adc14conseq1)
    }
    #[doc = "Repeat-single-channel"]
    #[inline(always)]
    pub fn adc14conseq_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14conseq::Adc14conseq2)
    }
    #[doc = "Repeat-sequence-of-channels"]
    #[inline(always)]
    pub fn adc14conseq_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14conseq::Adc14conseq3)
    }
}
#[doc = "ADC14 clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc14ssel {
    #[doc = "0: MODCLK"]
    Adc14ssel0 = 0,
    #[doc = "1: SYSCLK"]
    Adc14ssel1 = 1,
    #[doc = "2: ACLK"]
    Adc14ssel2 = 2,
    #[doc = "3: MCLK"]
    Adc14ssel3 = 3,
    #[doc = "4: SMCLK"]
    Adc14ssel4 = 4,
    #[doc = "5: HSMCLK"]
    Adc14ssel5 = 5,
}
impl From<Adc14ssel> for u8 {
    #[inline(always)]
    fn from(variant: Adc14ssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14ssel {
    type Ux = u8;
}
impl crate::IsEnum for Adc14ssel {}
#[doc = "Field `ADC14SSEL` reader - ADC14 clock source select"]
pub type Adc14sselR = crate::FieldReader<Adc14ssel>;
impl Adc14sselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc14ssel> {
        match self.bits {
            0 => Some(Adc14ssel::Adc14ssel0),
            1 => Some(Adc14ssel::Adc14ssel1),
            2 => Some(Adc14ssel::Adc14ssel2),
            3 => Some(Adc14ssel::Adc14ssel3),
            4 => Some(Adc14ssel::Adc14ssel4),
            5 => Some(Adc14ssel::Adc14ssel5),
            _ => None,
        }
    }
    #[doc = "MODCLK"]
    #[inline(always)]
    pub fn is_adc14ssel_0(&self) -> bool {
        *self == Adc14ssel::Adc14ssel0
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn is_adc14ssel_1(&self) -> bool {
        *self == Adc14ssel::Adc14ssel1
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_adc14ssel_2(&self) -> bool {
        *self == Adc14ssel::Adc14ssel2
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn is_adc14ssel_3(&self) -> bool {
        *self == Adc14ssel::Adc14ssel3
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_adc14ssel_4(&self) -> bool {
        *self == Adc14ssel::Adc14ssel4
    }
    #[doc = "HSMCLK"]
    #[inline(always)]
    pub fn is_adc14ssel_5(&self) -> bool {
        *self == Adc14ssel::Adc14ssel5
    }
}
#[doc = "Field `ADC14SSEL` writer - ADC14 clock source select"]
pub type Adc14sselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adc14ssel>;
impl<'a, REG> Adc14sselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MODCLK"]
    #[inline(always)]
    pub fn adc14ssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ssel::Adc14ssel0)
    }
    #[doc = "SYSCLK"]
    #[inline(always)]
    pub fn adc14ssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ssel::Adc14ssel1)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn adc14ssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ssel::Adc14ssel2)
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn adc14ssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ssel::Adc14ssel3)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn adc14ssel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ssel::Adc14ssel4)
    }
    #[doc = "HSMCLK"]
    #[inline(always)]
    pub fn adc14ssel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ssel::Adc14ssel5)
    }
}
#[doc = "ADC14 clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc14div {
    #[doc = "0: /1"]
    Adc14div0 = 0,
    #[doc = "1: /2"]
    Adc14div1 = 1,
    #[doc = "2: /3"]
    Adc14div2 = 2,
    #[doc = "3: /4"]
    Adc14div3 = 3,
    #[doc = "4: /5"]
    Adc14div4 = 4,
    #[doc = "5: /6"]
    Adc14div5 = 5,
    #[doc = "6: /7"]
    Adc14div6 = 6,
    #[doc = "7: /8"]
    Adc14div7 = 7,
}
impl From<Adc14div> for u8 {
    #[inline(always)]
    fn from(variant: Adc14div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14div {
    type Ux = u8;
}
impl crate::IsEnum for Adc14div {}
#[doc = "Field `ADC14DIV` reader - ADC14 clock divider"]
pub type Adc14divR = crate::FieldReader<Adc14div>;
impl Adc14divR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14div {
        match self.bits {
            0 => Adc14div::Adc14div0,
            1 => Adc14div::Adc14div1,
            2 => Adc14div::Adc14div2,
            3 => Adc14div::Adc14div3,
            4 => Adc14div::Adc14div4,
            5 => Adc14div::Adc14div5,
            6 => Adc14div::Adc14div6,
            7 => Adc14div::Adc14div7,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_adc14div_0(&self) -> bool {
        *self == Adc14div::Adc14div0
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_adc14div_1(&self) -> bool {
        *self == Adc14div::Adc14div1
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn is_adc14div_2(&self) -> bool {
        *self == Adc14div::Adc14div2
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_adc14div_3(&self) -> bool {
        *self == Adc14div::Adc14div3
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn is_adc14div_4(&self) -> bool {
        *self == Adc14div::Adc14div4
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn is_adc14div_5(&self) -> bool {
        *self == Adc14div::Adc14div5
    }
    #[doc = "/7"]
    #[inline(always)]
    pub fn is_adc14div_6(&self) -> bool {
        *self == Adc14div::Adc14div6
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_adc14div_7(&self) -> bool {
        *self == Adc14div::Adc14div7
    }
}
#[doc = "Field `ADC14DIV` writer - ADC14 clock divider"]
pub type Adc14divW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adc14div, crate::Safe>;
impl<'a, REG> Adc14divW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn adc14div_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14div::Adc14div0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn adc14div_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14div::Adc14div1)
    }
    #[doc = "/3"]
    #[inline(always)]
    pub fn adc14div_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14div::Adc14div2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn adc14div_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14div::Adc14div3)
    }
    #[doc = "/5"]
    #[inline(always)]
    pub fn adc14div_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14div::Adc14div4)
    }
    #[doc = "/6"]
    #[inline(always)]
    pub fn adc14div_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14div::Adc14div5)
    }
    #[doc = "/7"]
    #[inline(always)]
    pub fn adc14div_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14div::Adc14div6)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn adc14div_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14div::Adc14div7)
    }
}
#[doc = "ADC14 invert signal sample-and-hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14issh {
    #[doc = "0: The sample-input signal is not inverted"]
    Adc14issh0 = 0,
    #[doc = "1: The sample-input signal is inverted"]
    Adc14issh1 = 1,
}
impl From<Adc14issh> for bool {
    #[inline(always)]
    fn from(variant: Adc14issh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14ISSH` reader - ADC14 invert signal sample-and-hold"]
pub type Adc14isshR = crate::BitReader<Adc14issh>;
impl Adc14isshR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14issh {
        match self.bits {
            false => Adc14issh::Adc14issh0,
            true => Adc14issh::Adc14issh1,
        }
    }
    #[doc = "The sample-input signal is not inverted"]
    #[inline(always)]
    pub fn is_adc14issh_0(&self) -> bool {
        *self == Adc14issh::Adc14issh0
    }
    #[doc = "The sample-input signal is inverted"]
    #[inline(always)]
    pub fn is_adc14issh_1(&self) -> bool {
        *self == Adc14issh::Adc14issh1
    }
}
#[doc = "Field `ADC14ISSH` writer - ADC14 invert signal sample-and-hold"]
pub type Adc14isshW<'a, REG> = crate::BitWriter<'a, REG, Adc14issh>;
impl<'a, REG> Adc14isshW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The sample-input signal is not inverted"]
    #[inline(always)]
    pub fn adc14issh_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14issh::Adc14issh0)
    }
    #[doc = "The sample-input signal is inverted"]
    #[inline(always)]
    pub fn adc14issh_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14issh::Adc14issh1)
    }
}
#[doc = "ADC14 sample-and-hold pulse-mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14shp {
    #[doc = "0: SAMPCON signal is sourced from the sample-input signal"]
    Adc14shp0 = 0,
    #[doc = "1: SAMPCON signal is sourced from the sampling timer"]
    Adc14shp1 = 1,
}
impl From<Adc14shp> for bool {
    #[inline(always)]
    fn from(variant: Adc14shp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14SHP` reader - ADC14 sample-and-hold pulse-mode select"]
pub type Adc14shpR = crate::BitReader<Adc14shp>;
impl Adc14shpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14shp {
        match self.bits {
            false => Adc14shp::Adc14shp0,
            true => Adc14shp::Adc14shp1,
        }
    }
    #[doc = "SAMPCON signal is sourced from the sample-input signal"]
    #[inline(always)]
    pub fn is_adc14shp_0(&self) -> bool {
        *self == Adc14shp::Adc14shp0
    }
    #[doc = "SAMPCON signal is sourced from the sampling timer"]
    #[inline(always)]
    pub fn is_adc14shp_1(&self) -> bool {
        *self == Adc14shp::Adc14shp1
    }
}
#[doc = "Field `ADC14SHP` writer - ADC14 sample-and-hold pulse-mode select"]
pub type Adc14shpW<'a, REG> = crate::BitWriter<'a, REG, Adc14shp>;
impl<'a, REG> Adc14shpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAMPCON signal is sourced from the sample-input signal"]
    #[inline(always)]
    pub fn adc14shp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14shp::Adc14shp0)
    }
    #[doc = "SAMPCON signal is sourced from the sampling timer"]
    #[inline(always)]
    pub fn adc14shp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14shp::Adc14shp1)
    }
}
#[doc = "ADC14 sample-and-hold source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc14shs {
    #[doc = "0: ADC14SC bit"]
    Adc14shs0 = 0,
    #[doc = "1: See device-specific data sheet for source"]
    Adc14shs1 = 1,
    #[doc = "2: See device-specific data sheet for source"]
    Adc14shs2 = 2,
    #[doc = "3: See device-specific data sheet for source"]
    Adc14shs3 = 3,
    #[doc = "4: See device-specific data sheet for source"]
    Adc14shs4 = 4,
    #[doc = "5: See device-specific data sheet for source"]
    Adc14shs5 = 5,
    #[doc = "6: See device-specific data sheet for source"]
    Adc14shs6 = 6,
    #[doc = "7: See device-specific data sheet for source"]
    Adc14shs7 = 7,
}
impl From<Adc14shs> for u8 {
    #[inline(always)]
    fn from(variant: Adc14shs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14shs {
    type Ux = u8;
}
impl crate::IsEnum for Adc14shs {}
#[doc = "Field `ADC14SHS` reader - ADC14 sample-and-hold source select"]
pub type Adc14shsR = crate::FieldReader<Adc14shs>;
impl Adc14shsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14shs {
        match self.bits {
            0 => Adc14shs::Adc14shs0,
            1 => Adc14shs::Adc14shs1,
            2 => Adc14shs::Adc14shs2,
            3 => Adc14shs::Adc14shs3,
            4 => Adc14shs::Adc14shs4,
            5 => Adc14shs::Adc14shs5,
            6 => Adc14shs::Adc14shs6,
            7 => Adc14shs::Adc14shs7,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC14SC bit"]
    #[inline(always)]
    pub fn is_adc14shs_0(&self) -> bool {
        *self == Adc14shs::Adc14shs0
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc14shs_1(&self) -> bool {
        *self == Adc14shs::Adc14shs1
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc14shs_2(&self) -> bool {
        *self == Adc14shs::Adc14shs2
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc14shs_3(&self) -> bool {
        *self == Adc14shs::Adc14shs3
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc14shs_4(&self) -> bool {
        *self == Adc14shs::Adc14shs4
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc14shs_5(&self) -> bool {
        *self == Adc14shs::Adc14shs5
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc14shs_6(&self) -> bool {
        *self == Adc14shs::Adc14shs6
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn is_adc14shs_7(&self) -> bool {
        *self == Adc14shs::Adc14shs7
    }
}
#[doc = "Field `ADC14SHS` writer - ADC14 sample-and-hold source select"]
pub type Adc14shsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adc14shs, crate::Safe>;
impl<'a, REG> Adc14shsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC14SC bit"]
    #[inline(always)]
    pub fn adc14shs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14shs::Adc14shs0)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14shs::Adc14shs1)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14shs::Adc14shs2)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14shs::Adc14shs3)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14shs::Adc14shs4)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14shs::Adc14shs5)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14shs::Adc14shs6)
    }
    #[doc = "See device-specific data sheet for source"]
    #[inline(always)]
    pub fn adc14shs_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14shs::Adc14shs7)
    }
}
#[doc = "ADC14 predivider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc14pdiv {
    #[doc = "0: Predivide by 1"]
    Adc14pdiv0 = 0,
    #[doc = "1: Predivide by 4"]
    Adc14pdiv1 = 1,
    #[doc = "2: Predivide by 32"]
    Adc14pdiv2 = 2,
    #[doc = "3: Predivide by 64"]
    Adc14pdiv3 = 3,
}
impl From<Adc14pdiv> for u8 {
    #[inline(always)]
    fn from(variant: Adc14pdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14pdiv {
    type Ux = u8;
}
impl crate::IsEnum for Adc14pdiv {}
#[doc = "Field `ADC14PDIV` reader - ADC14 predivider"]
pub type Adc14pdivR = crate::FieldReader<Adc14pdiv>;
impl Adc14pdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14pdiv {
        match self.bits {
            0 => Adc14pdiv::Adc14pdiv0,
            1 => Adc14pdiv::Adc14pdiv1,
            2 => Adc14pdiv::Adc14pdiv2,
            3 => Adc14pdiv::Adc14pdiv3,
            _ => unreachable!(),
        }
    }
    #[doc = "Predivide by 1"]
    #[inline(always)]
    pub fn is_adc14pdiv_0(&self) -> bool {
        *self == Adc14pdiv::Adc14pdiv0
    }
    #[doc = "Predivide by 4"]
    #[inline(always)]
    pub fn is_adc14pdiv_1(&self) -> bool {
        *self == Adc14pdiv::Adc14pdiv1
    }
    #[doc = "Predivide by 32"]
    #[inline(always)]
    pub fn is_adc14pdiv_2(&self) -> bool {
        *self == Adc14pdiv::Adc14pdiv2
    }
    #[doc = "Predivide by 64"]
    #[inline(always)]
    pub fn is_adc14pdiv_3(&self) -> bool {
        *self == Adc14pdiv::Adc14pdiv3
    }
}
#[doc = "Field `ADC14PDIV` writer - ADC14 predivider"]
pub type Adc14pdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc14pdiv, crate::Safe>;
impl<'a, REG> Adc14pdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Predivide by 1"]
    #[inline(always)]
    pub fn adc14pdiv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14pdiv::Adc14pdiv0)
    }
    #[doc = "Predivide by 4"]
    #[inline(always)]
    pub fn adc14pdiv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14pdiv::Adc14pdiv1)
    }
    #[doc = "Predivide by 32"]
    #[inline(always)]
    pub fn adc14pdiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14pdiv::Adc14pdiv2)
    }
    #[doc = "Predivide by 64"]
    #[inline(always)]
    pub fn adc14pdiv_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14pdiv::Adc14pdiv3)
    }
}
impl R {
    #[doc = "Bit 0 - ADC14 start conversion"]
    #[inline(always)]
    pub fn adc14sc(&self) -> Adc14scR {
        Adc14scR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC14 enable conversion"]
    #[inline(always)]
    pub fn adc14enc(&self) -> Adc14encR {
        Adc14encR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC14 on"]
    #[inline(always)]
    pub fn adc14on(&self) -> Adc14onR {
        Adc14onR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC14 multiple sample and conversion"]
    #[inline(always)]
    pub fn adc14msc(&self) -> Adc14mscR {
        Adc14mscR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADC14 sample-and-hold time"]
    #[inline(always)]
    pub fn adc14sht0(&self) -> Adc14sht0R {
        Adc14sht0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ADC14 sample-and-hold time"]
    #[inline(always)]
    pub fn adc14sht1(&self) -> Adc14sht1R {
        Adc14sht1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - ADC14 busy"]
    #[inline(always)]
    pub fn adc14busy(&self) -> Adc14busyR {
        Adc14busyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - ADC14 conversion sequence mode select"]
    #[inline(always)]
    pub fn adc14conseq(&self) -> Adc14conseqR {
        Adc14conseqR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - ADC14 clock source select"]
    #[inline(always)]
    pub fn adc14ssel(&self) -> Adc14sselR {
        Adc14sselR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - ADC14 clock divider"]
    #[inline(always)]
    pub fn adc14div(&self) -> Adc14divR {
        Adc14divR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - ADC14 invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adc14issh(&self) -> Adc14isshR {
        Adc14isshR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ADC14 sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adc14shp(&self) -> Adc14shpR {
        Adc14shpR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - ADC14 sample-and-hold source select"]
    #[inline(always)]
    pub fn adc14shs(&self) -> Adc14shsR {
        Adc14shsR::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - ADC14 predivider"]
    #[inline(always)]
    pub fn adc14pdiv(&self) -> Adc14pdivR {
        Adc14pdivR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC14 start conversion"]
    #[inline(always)]
    pub fn adc14sc(&mut self) -> Adc14scW<Adc14ctl0Spec> {
        Adc14scW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC14 enable conversion"]
    #[inline(always)]
    pub fn adc14enc(&mut self) -> Adc14encW<Adc14ctl0Spec> {
        Adc14encW::new(self, 1)
    }
    #[doc = "Bit 4 - ADC14 on"]
    #[inline(always)]
    pub fn adc14on(&mut self) -> Adc14onW<Adc14ctl0Spec> {
        Adc14onW::new(self, 4)
    }
    #[doc = "Bit 7 - ADC14 multiple sample and conversion"]
    #[inline(always)]
    pub fn adc14msc(&mut self) -> Adc14mscW<Adc14ctl0Spec> {
        Adc14mscW::new(self, 7)
    }
    #[doc = "Bits 8:11 - ADC14 sample-and-hold time"]
    #[inline(always)]
    pub fn adc14sht0(&mut self) -> Adc14sht0W<Adc14ctl0Spec> {
        Adc14sht0W::new(self, 8)
    }
    #[doc = "Bits 12:15 - ADC14 sample-and-hold time"]
    #[inline(always)]
    pub fn adc14sht1(&mut self) -> Adc14sht1W<Adc14ctl0Spec> {
        Adc14sht1W::new(self, 12)
    }
    #[doc = "Bits 17:18 - ADC14 conversion sequence mode select"]
    #[inline(always)]
    pub fn adc14conseq(&mut self) -> Adc14conseqW<Adc14ctl0Spec> {
        Adc14conseqW::new(self, 17)
    }
    #[doc = "Bits 19:21 - ADC14 clock source select"]
    #[inline(always)]
    pub fn adc14ssel(&mut self) -> Adc14sselW<Adc14ctl0Spec> {
        Adc14sselW::new(self, 19)
    }
    #[doc = "Bits 22:24 - ADC14 clock divider"]
    #[inline(always)]
    pub fn adc14div(&mut self) -> Adc14divW<Adc14ctl0Spec> {
        Adc14divW::new(self, 22)
    }
    #[doc = "Bit 25 - ADC14 invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adc14issh(&mut self) -> Adc14isshW<Adc14ctl0Spec> {
        Adc14isshW::new(self, 25)
    }
    #[doc = "Bit 26 - ADC14 sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adc14shp(&mut self) -> Adc14shpW<Adc14ctl0Spec> {
        Adc14shpW::new(self, 26)
    }
    #[doc = "Bits 27:29 - ADC14 sample-and-hold source select"]
    #[inline(always)]
    pub fn adc14shs(&mut self) -> Adc14shsW<Adc14ctl0Spec> {
        Adc14shsW::new(self, 27)
    }
    #[doc = "Bits 30:31 - ADC14 predivider"]
    #[inline(always)]
    pub fn adc14pdiv(&mut self) -> Adc14pdivW<Adc14ctl0Spec> {
        Adc14pdivW::new(self, 30)
    }
}
#[doc = "Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14ctl0Spec;
impl crate::RegisterSpec for Adc14ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14ctl0::R`](R) reader structure"]
impl crate::Readable for Adc14ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc14ctl0::W`](W) writer structure"]
impl crate::Writable for Adc14ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14CTL0 to value 0"]
impl crate::Resettable for Adc14ctl0Spec {
    const RESET_VALUE: u32 = 0;
}
