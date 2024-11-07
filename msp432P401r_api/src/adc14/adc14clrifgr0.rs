#[doc = "Register `ADC14CLRIFGR0` writer"]
pub type W = crate::W<Adc14clrifgr0Spec>;
#[doc = "clear ADC14IFG0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg0EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg0_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg0_1 = 1,
}
impl From<Clradc14ifg0EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg0EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG0` writer - clear ADC14IFG0"]
pub type Clradc14ifg0W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg0EnumWrite>;
impl<'a, REG> Clradc14ifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg0EnumWrite::Clradc14ifg0_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg0EnumWrite::Clradc14ifg0_1)
    }
}
#[doc = "clear ADC14IFG1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg1EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg1_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg1_1 = 1,
}
impl From<Clradc14ifg1EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg1EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG1` writer - clear ADC14IFG1"]
pub type Clradc14ifg1W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg1EnumWrite>;
impl<'a, REG> Clradc14ifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg1EnumWrite::Clradc14ifg1_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg1EnumWrite::Clradc14ifg1_1)
    }
}
#[doc = "clear ADC14IFG2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg2EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg2_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg2_1 = 1,
}
impl From<Clradc14ifg2EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg2EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG2` writer - clear ADC14IFG2"]
pub type Clradc14ifg2W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg2EnumWrite>;
impl<'a, REG> Clradc14ifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg2EnumWrite::Clradc14ifg2_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg2EnumWrite::Clradc14ifg2_1)
    }
}
#[doc = "clear ADC14IFG3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg3EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg3_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg3_1 = 1,
}
impl From<Clradc14ifg3EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg3EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG3` writer - clear ADC14IFG3"]
pub type Clradc14ifg3W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg3EnumWrite>;
impl<'a, REG> Clradc14ifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg3EnumWrite::Clradc14ifg3_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg3EnumWrite::Clradc14ifg3_1)
    }
}
#[doc = "clear ADC14IFG4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg4EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg4_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg4_1 = 1,
}
impl From<Clradc14ifg4EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg4EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG4` writer - clear ADC14IFG4"]
pub type Clradc14ifg4W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg4EnumWrite>;
impl<'a, REG> Clradc14ifg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg4_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg4EnumWrite::Clradc14ifg4_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg4_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg4EnumWrite::Clradc14ifg4_1)
    }
}
#[doc = "clear ADC14IFG5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg5EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg5_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg5_1 = 1,
}
impl From<Clradc14ifg5EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg5EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG5` writer - clear ADC14IFG5"]
pub type Clradc14ifg5W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg5EnumWrite>;
impl<'a, REG> Clradc14ifg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg5EnumWrite::Clradc14ifg5_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg5EnumWrite::Clradc14ifg5_1)
    }
}
#[doc = "clear ADC14IFG6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg6EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg6_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg6_1 = 1,
}
impl From<Clradc14ifg6EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg6EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG6` writer - clear ADC14IFG6"]
pub type Clradc14ifg6W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg6EnumWrite>;
impl<'a, REG> Clradc14ifg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg6_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg6EnumWrite::Clradc14ifg6_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg6_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg6EnumWrite::Clradc14ifg6_1)
    }
}
#[doc = "clear ADC14IFG7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg7EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg7_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg7_1 = 1,
}
impl From<Clradc14ifg7EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg7EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG7` writer - clear ADC14IFG7"]
pub type Clradc14ifg7W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg7EnumWrite>;
impl<'a, REG> Clradc14ifg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg7_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg7EnumWrite::Clradc14ifg7_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg7_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg7EnumWrite::Clradc14ifg7_1)
    }
}
#[doc = "clear ADC14IFG8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg8EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg8_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg8_1 = 1,
}
impl From<Clradc14ifg8EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg8EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG8` writer - clear ADC14IFG8"]
pub type Clradc14ifg8W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg8EnumWrite>;
impl<'a, REG> Clradc14ifg8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg8_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg8EnumWrite::Clradc14ifg8_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg8_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg8EnumWrite::Clradc14ifg8_1)
    }
}
#[doc = "clear ADC14IFG9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg9EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg9_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg9_1 = 1,
}
impl From<Clradc14ifg9EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg9EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG9` writer - clear ADC14IFG9"]
pub type Clradc14ifg9W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg9EnumWrite>;
impl<'a, REG> Clradc14ifg9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg9_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg9EnumWrite::Clradc14ifg9_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg9_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg9EnumWrite::Clradc14ifg9_1)
    }
}
#[doc = "clear ADC14IFG10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg10EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg10_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg10_1 = 1,
}
impl From<Clradc14ifg10EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg10EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG10` writer - clear ADC14IFG10"]
pub type Clradc14ifg10W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg10EnumWrite>;
impl<'a, REG> Clradc14ifg10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg10_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg10EnumWrite::Clradc14ifg10_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg10_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg10EnumWrite::Clradc14ifg10_1)
    }
}
#[doc = "clear ADC14IFG11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg11EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg11_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg11_1 = 1,
}
impl From<Clradc14ifg11EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg11EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG11` writer - clear ADC14IFG11"]
pub type Clradc14ifg11W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg11EnumWrite>;
impl<'a, REG> Clradc14ifg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg11_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg11EnumWrite::Clradc14ifg11_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg11_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg11EnumWrite::Clradc14ifg11_1)
    }
}
#[doc = "clear ADC14IFG12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg12EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg12_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg12_1 = 1,
}
impl From<Clradc14ifg12EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg12EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG12` writer - clear ADC14IFG12"]
pub type Clradc14ifg12W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg12EnumWrite>;
impl<'a, REG> Clradc14ifg12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg12_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg12EnumWrite::Clradc14ifg12_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg12_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg12EnumWrite::Clradc14ifg12_1)
    }
}
#[doc = "clear ADC14IFG13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg13EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg13_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg13_1 = 1,
}
impl From<Clradc14ifg13EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg13EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG13` writer - clear ADC14IFG13"]
pub type Clradc14ifg13W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg13EnumWrite>;
impl<'a, REG> Clradc14ifg13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg13_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg13EnumWrite::Clradc14ifg13_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg13_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg13EnumWrite::Clradc14ifg13_1)
    }
}
#[doc = "clear ADC14IFG14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg14EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg14_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg14_1 = 1,
}
impl From<Clradc14ifg14EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg14EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG14` writer - clear ADC14IFG14"]
pub type Clradc14ifg14W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg14EnumWrite>;
impl<'a, REG> Clradc14ifg14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg14_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg14EnumWrite::Clradc14ifg14_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg14_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg14EnumWrite::Clradc14ifg14_1)
    }
}
#[doc = "clear ADC14IFG15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg15EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg15_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg15_1 = 1,
}
impl From<Clradc14ifg15EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg15EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG15` writer - clear ADC14IFG15"]
pub type Clradc14ifg15W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg15EnumWrite>;
impl<'a, REG> Clradc14ifg15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg15_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg15EnumWrite::Clradc14ifg15_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg15_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg15EnumWrite::Clradc14ifg15_1)
    }
}
#[doc = "clear ADC14IFG16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg16EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg16_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg16_1 = 1,
}
impl From<Clradc14ifg16EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg16EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG16` writer - clear ADC14IFG16"]
pub type Clradc14ifg16W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg16EnumWrite>;
impl<'a, REG> Clradc14ifg16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg16_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg16EnumWrite::Clradc14ifg16_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg16_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg16EnumWrite::Clradc14ifg16_1)
    }
}
#[doc = "clear ADC14IFG17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg17EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg17_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg17_1 = 1,
}
impl From<Clradc14ifg17EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg17EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG17` writer - clear ADC14IFG17"]
pub type Clradc14ifg17W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg17EnumWrite>;
impl<'a, REG> Clradc14ifg17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg17_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg17EnumWrite::Clradc14ifg17_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg17_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg17EnumWrite::Clradc14ifg17_1)
    }
}
#[doc = "clear ADC14IFG18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg18EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg18_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg18_1 = 1,
}
impl From<Clradc14ifg18EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg18EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG18` writer - clear ADC14IFG18"]
pub type Clradc14ifg18W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg18EnumWrite>;
impl<'a, REG> Clradc14ifg18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg18_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg18EnumWrite::Clradc14ifg18_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg18_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg18EnumWrite::Clradc14ifg18_1)
    }
}
#[doc = "clear ADC14IFG19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg19EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg19_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg19_1 = 1,
}
impl From<Clradc14ifg19EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg19EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG19` writer - clear ADC14IFG19"]
pub type Clradc14ifg19W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg19EnumWrite>;
impl<'a, REG> Clradc14ifg19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg19_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg19EnumWrite::Clradc14ifg19_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg19_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg19EnumWrite::Clradc14ifg19_1)
    }
}
#[doc = "clear ADC14IFG20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg20EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg20_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg20_1 = 1,
}
impl From<Clradc14ifg20EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg20EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG20` writer - clear ADC14IFG20"]
pub type Clradc14ifg20W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg20EnumWrite>;
impl<'a, REG> Clradc14ifg20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg20_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg20EnumWrite::Clradc14ifg20_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg20_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg20EnumWrite::Clradc14ifg20_1)
    }
}
#[doc = "clear ADC14IFG21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg21EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg21_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg21_1 = 1,
}
impl From<Clradc14ifg21EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg21EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG21` writer - clear ADC14IFG21"]
pub type Clradc14ifg21W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg21EnumWrite>;
impl<'a, REG> Clradc14ifg21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg21_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg21EnumWrite::Clradc14ifg21_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg21_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg21EnumWrite::Clradc14ifg21_1)
    }
}
#[doc = "clear ADC14IFG22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg22EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg22_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg22_1 = 1,
}
impl From<Clradc14ifg22EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg22EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG22` writer - clear ADC14IFG22"]
pub type Clradc14ifg22W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg22EnumWrite>;
impl<'a, REG> Clradc14ifg22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg22_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg22EnumWrite::Clradc14ifg22_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg22_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg22EnumWrite::Clradc14ifg22_1)
    }
}
#[doc = "clear ADC14IFG23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg23EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg23_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg23_1 = 1,
}
impl From<Clradc14ifg23EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg23EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG23` writer - clear ADC14IFG23"]
pub type Clradc14ifg23W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg23EnumWrite>;
impl<'a, REG> Clradc14ifg23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg23_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg23EnumWrite::Clradc14ifg23_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg23_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg23EnumWrite::Clradc14ifg23_1)
    }
}
#[doc = "clear ADC14IFG24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg24EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg24_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg24_1 = 1,
}
impl From<Clradc14ifg24EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg24EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG24` writer - clear ADC14IFG24"]
pub type Clradc14ifg24W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg24EnumWrite>;
impl<'a, REG> Clradc14ifg24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg24_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg24EnumWrite::Clradc14ifg24_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg24_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg24EnumWrite::Clradc14ifg24_1)
    }
}
#[doc = "clear ADC14IFG25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg25EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg25_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg25_1 = 1,
}
impl From<Clradc14ifg25EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg25EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG25` writer - clear ADC14IFG25"]
pub type Clradc14ifg25W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg25EnumWrite>;
impl<'a, REG> Clradc14ifg25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg25_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg25EnumWrite::Clradc14ifg25_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg25_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg25EnumWrite::Clradc14ifg25_1)
    }
}
#[doc = "clear ADC14IFG26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg26EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg26_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg26_1 = 1,
}
impl From<Clradc14ifg26EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg26EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG26` writer - clear ADC14IFG26"]
pub type Clradc14ifg26W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg26EnumWrite>;
impl<'a, REG> Clradc14ifg26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg26_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg26EnumWrite::Clradc14ifg26_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg26_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg26EnumWrite::Clradc14ifg26_1)
    }
}
#[doc = "clear ADC14IFG27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg27EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg27_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg27_1 = 1,
}
impl From<Clradc14ifg27EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg27EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG27` writer - clear ADC14IFG27"]
pub type Clradc14ifg27W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg27EnumWrite>;
impl<'a, REG> Clradc14ifg27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg27_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg27EnumWrite::Clradc14ifg27_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg27_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg27EnumWrite::Clradc14ifg27_1)
    }
}
#[doc = "clear ADC14IFG28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg28EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg28_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg28_1 = 1,
}
impl From<Clradc14ifg28EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg28EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG28` writer - clear ADC14IFG28"]
pub type Clradc14ifg28W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg28EnumWrite>;
impl<'a, REG> Clradc14ifg28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg28_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg28EnumWrite::Clradc14ifg28_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg28_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg28EnumWrite::Clradc14ifg28_1)
    }
}
#[doc = "clear ADC14IFG29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg29EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg29_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg29_1 = 1,
}
impl From<Clradc14ifg29EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg29EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG29` writer - clear ADC14IFG29"]
pub type Clradc14ifg29W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg29EnumWrite>;
impl<'a, REG> Clradc14ifg29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg29_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg29EnumWrite::Clradc14ifg29_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg29_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg29EnumWrite::Clradc14ifg29_1)
    }
}
#[doc = "clear ADC14IFG30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg30EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg30_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg30_1 = 1,
}
impl From<Clradc14ifg30EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg30EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG30` writer - clear ADC14IFG30"]
pub type Clradc14ifg30W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg30EnumWrite>;
impl<'a, REG> Clradc14ifg30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg30_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg30EnumWrite::Clradc14ifg30_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg30_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg30EnumWrite::Clradc14ifg30_1)
    }
}
#[doc = "clear ADC14IFG31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ifg31EnumWrite {
    #[doc = "0: no effect"]
    Clradc14ifg31_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ifg31_1 = 1,
}
impl From<Clradc14ifg31EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ifg31EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14IFG31` writer - clear ADC14IFG31"]
pub type Clradc14ifg31W<'a, REG> = crate::BitWriter<'a, REG, Clradc14ifg31EnumWrite>;
impl<'a, REG> Clradc14ifg31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ifg31_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg31EnumWrite::Clradc14ifg31_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ifg31_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ifg31EnumWrite::Clradc14ifg31_1)
    }
}
impl W {
    #[doc = "Bit 0 - clear ADC14IFG0"]
    #[inline(always)]
    pub fn clradc14ifg0(&mut self) -> Clradc14ifg0W<Adc14clrifgr0Spec> {
        Clradc14ifg0W::new(self, 0)
    }
    #[doc = "Bit 1 - clear ADC14IFG1"]
    #[inline(always)]
    pub fn clradc14ifg1(&mut self) -> Clradc14ifg1W<Adc14clrifgr0Spec> {
        Clradc14ifg1W::new(self, 1)
    }
    #[doc = "Bit 2 - clear ADC14IFG2"]
    #[inline(always)]
    pub fn clradc14ifg2(&mut self) -> Clradc14ifg2W<Adc14clrifgr0Spec> {
        Clradc14ifg2W::new(self, 2)
    }
    #[doc = "Bit 3 - clear ADC14IFG3"]
    #[inline(always)]
    pub fn clradc14ifg3(&mut self) -> Clradc14ifg3W<Adc14clrifgr0Spec> {
        Clradc14ifg3W::new(self, 3)
    }
    #[doc = "Bit 4 - clear ADC14IFG4"]
    #[inline(always)]
    pub fn clradc14ifg4(&mut self) -> Clradc14ifg4W<Adc14clrifgr0Spec> {
        Clradc14ifg4W::new(self, 4)
    }
    #[doc = "Bit 5 - clear ADC14IFG5"]
    #[inline(always)]
    pub fn clradc14ifg5(&mut self) -> Clradc14ifg5W<Adc14clrifgr0Spec> {
        Clradc14ifg5W::new(self, 5)
    }
    #[doc = "Bit 6 - clear ADC14IFG6"]
    #[inline(always)]
    pub fn clradc14ifg6(&mut self) -> Clradc14ifg6W<Adc14clrifgr0Spec> {
        Clradc14ifg6W::new(self, 6)
    }
    #[doc = "Bit 7 - clear ADC14IFG7"]
    #[inline(always)]
    pub fn clradc14ifg7(&mut self) -> Clradc14ifg7W<Adc14clrifgr0Spec> {
        Clradc14ifg7W::new(self, 7)
    }
    #[doc = "Bit 8 - clear ADC14IFG8"]
    #[inline(always)]
    pub fn clradc14ifg8(&mut self) -> Clradc14ifg8W<Adc14clrifgr0Spec> {
        Clradc14ifg8W::new(self, 8)
    }
    #[doc = "Bit 9 - clear ADC14IFG9"]
    #[inline(always)]
    pub fn clradc14ifg9(&mut self) -> Clradc14ifg9W<Adc14clrifgr0Spec> {
        Clradc14ifg9W::new(self, 9)
    }
    #[doc = "Bit 10 - clear ADC14IFG10"]
    #[inline(always)]
    pub fn clradc14ifg10(&mut self) -> Clradc14ifg10W<Adc14clrifgr0Spec> {
        Clradc14ifg10W::new(self, 10)
    }
    #[doc = "Bit 11 - clear ADC14IFG11"]
    #[inline(always)]
    pub fn clradc14ifg11(&mut self) -> Clradc14ifg11W<Adc14clrifgr0Spec> {
        Clradc14ifg11W::new(self, 11)
    }
    #[doc = "Bit 12 - clear ADC14IFG12"]
    #[inline(always)]
    pub fn clradc14ifg12(&mut self) -> Clradc14ifg12W<Adc14clrifgr0Spec> {
        Clradc14ifg12W::new(self, 12)
    }
    #[doc = "Bit 13 - clear ADC14IFG13"]
    #[inline(always)]
    pub fn clradc14ifg13(&mut self) -> Clradc14ifg13W<Adc14clrifgr0Spec> {
        Clradc14ifg13W::new(self, 13)
    }
    #[doc = "Bit 14 - clear ADC14IFG14"]
    #[inline(always)]
    pub fn clradc14ifg14(&mut self) -> Clradc14ifg14W<Adc14clrifgr0Spec> {
        Clradc14ifg14W::new(self, 14)
    }
    #[doc = "Bit 15 - clear ADC14IFG15"]
    #[inline(always)]
    pub fn clradc14ifg15(&mut self) -> Clradc14ifg15W<Adc14clrifgr0Spec> {
        Clradc14ifg15W::new(self, 15)
    }
    #[doc = "Bit 16 - clear ADC14IFG16"]
    #[inline(always)]
    pub fn clradc14ifg16(&mut self) -> Clradc14ifg16W<Adc14clrifgr0Spec> {
        Clradc14ifg16W::new(self, 16)
    }
    #[doc = "Bit 17 - clear ADC14IFG17"]
    #[inline(always)]
    pub fn clradc14ifg17(&mut self) -> Clradc14ifg17W<Adc14clrifgr0Spec> {
        Clradc14ifg17W::new(self, 17)
    }
    #[doc = "Bit 18 - clear ADC14IFG18"]
    #[inline(always)]
    pub fn clradc14ifg18(&mut self) -> Clradc14ifg18W<Adc14clrifgr0Spec> {
        Clradc14ifg18W::new(self, 18)
    }
    #[doc = "Bit 19 - clear ADC14IFG19"]
    #[inline(always)]
    pub fn clradc14ifg19(&mut self) -> Clradc14ifg19W<Adc14clrifgr0Spec> {
        Clradc14ifg19W::new(self, 19)
    }
    #[doc = "Bit 20 - clear ADC14IFG20"]
    #[inline(always)]
    pub fn clradc14ifg20(&mut self) -> Clradc14ifg20W<Adc14clrifgr0Spec> {
        Clradc14ifg20W::new(self, 20)
    }
    #[doc = "Bit 21 - clear ADC14IFG21"]
    #[inline(always)]
    pub fn clradc14ifg21(&mut self) -> Clradc14ifg21W<Adc14clrifgr0Spec> {
        Clradc14ifg21W::new(self, 21)
    }
    #[doc = "Bit 22 - clear ADC14IFG22"]
    #[inline(always)]
    pub fn clradc14ifg22(&mut self) -> Clradc14ifg22W<Adc14clrifgr0Spec> {
        Clradc14ifg22W::new(self, 22)
    }
    #[doc = "Bit 23 - clear ADC14IFG23"]
    #[inline(always)]
    pub fn clradc14ifg23(&mut self) -> Clradc14ifg23W<Adc14clrifgr0Spec> {
        Clradc14ifg23W::new(self, 23)
    }
    #[doc = "Bit 24 - clear ADC14IFG24"]
    #[inline(always)]
    pub fn clradc14ifg24(&mut self) -> Clradc14ifg24W<Adc14clrifgr0Spec> {
        Clradc14ifg24W::new(self, 24)
    }
    #[doc = "Bit 25 - clear ADC14IFG25"]
    #[inline(always)]
    pub fn clradc14ifg25(&mut self) -> Clradc14ifg25W<Adc14clrifgr0Spec> {
        Clradc14ifg25W::new(self, 25)
    }
    #[doc = "Bit 26 - clear ADC14IFG26"]
    #[inline(always)]
    pub fn clradc14ifg26(&mut self) -> Clradc14ifg26W<Adc14clrifgr0Spec> {
        Clradc14ifg26W::new(self, 26)
    }
    #[doc = "Bit 27 - clear ADC14IFG27"]
    #[inline(always)]
    pub fn clradc14ifg27(&mut self) -> Clradc14ifg27W<Adc14clrifgr0Spec> {
        Clradc14ifg27W::new(self, 27)
    }
    #[doc = "Bit 28 - clear ADC14IFG28"]
    #[inline(always)]
    pub fn clradc14ifg28(&mut self) -> Clradc14ifg28W<Adc14clrifgr0Spec> {
        Clradc14ifg28W::new(self, 28)
    }
    #[doc = "Bit 29 - clear ADC14IFG29"]
    #[inline(always)]
    pub fn clradc14ifg29(&mut self) -> Clradc14ifg29W<Adc14clrifgr0Spec> {
        Clradc14ifg29W::new(self, 29)
    }
    #[doc = "Bit 30 - clear ADC14IFG30"]
    #[inline(always)]
    pub fn clradc14ifg30(&mut self) -> Clradc14ifg30W<Adc14clrifgr0Spec> {
        Clradc14ifg30W::new(self, 30)
    }
    #[doc = "Bit 31 - clear ADC14IFG31"]
    #[inline(always)]
    pub fn clradc14ifg31(&mut self) -> Clradc14ifg31W<Adc14clrifgr0Spec> {
        Clradc14ifg31W::new(self, 31)
    }
}
#[doc = "Clear Interrupt Flag 0 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14clrifgr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14clrifgr0Spec;
impl crate::RegisterSpec for Adc14clrifgr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`adc14clrifgr0::W`](W) writer structure"]
impl crate::Writable for Adc14clrifgr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14CLRIFGR0 to value 0"]
impl crate::Resettable for Adc14clrifgr0Spec {
    const RESET_VALUE: u32 = 0;
}
