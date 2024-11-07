#[doc = "Register `ADC14IER0` reader"]
pub type R = crate::R<Adc14ier0Spec>;
#[doc = "Register `ADC14IER0` writer"]
pub type W = crate::W<Adc14ier0Spec>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie0 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie0_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie0_1 = 1,
}
impl From<Adc14ie0> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE0` reader - Interrupt enable"]
pub type Adc14ie0R = crate::BitReader<Adc14ie0>;
impl Adc14ie0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie0 {
        match self.bits {
            false => Adc14ie0::Adc14ie0_0,
            true => Adc14ie0::Adc14ie0_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie0_0(&self) -> bool {
        *self == Adc14ie0::Adc14ie0_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie0_1(&self) -> bool {
        *self == Adc14ie0::Adc14ie0_1
    }
}
#[doc = "Field `ADC14IE0` writer - Interrupt enable"]
pub type Adc14ie0W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie0>;
impl<'a, REG> Adc14ie0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie0::Adc14ie0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie0::Adc14ie0_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie1 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie1_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie1_1 = 1,
}
impl From<Adc14ie1> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE1` reader - Interrupt enable"]
pub type Adc14ie1R = crate::BitReader<Adc14ie1>;
impl Adc14ie1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie1 {
        match self.bits {
            false => Adc14ie1::Adc14ie1_0,
            true => Adc14ie1::Adc14ie1_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie1_0(&self) -> bool {
        *self == Adc14ie1::Adc14ie1_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie1_1(&self) -> bool {
        *self == Adc14ie1::Adc14ie1_1
    }
}
#[doc = "Field `ADC14IE1` writer - Interrupt enable"]
pub type Adc14ie1W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie1>;
impl<'a, REG> Adc14ie1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie1::Adc14ie1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie1::Adc14ie1_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie2 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie2_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie2_1 = 1,
}
impl From<Adc14ie2> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE2` reader - Interrupt enable"]
pub type Adc14ie2R = crate::BitReader<Adc14ie2>;
impl Adc14ie2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie2 {
        match self.bits {
            false => Adc14ie2::Adc14ie2_0,
            true => Adc14ie2::Adc14ie2_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie2_0(&self) -> bool {
        *self == Adc14ie2::Adc14ie2_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie2_1(&self) -> bool {
        *self == Adc14ie2::Adc14ie2_1
    }
}
#[doc = "Field `ADC14IE2` writer - Interrupt enable"]
pub type Adc14ie2W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie2>;
impl<'a, REG> Adc14ie2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie2::Adc14ie2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie2::Adc14ie2_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie3 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie3_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie3_1 = 1,
}
impl From<Adc14ie3> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE3` reader - Interrupt enable"]
pub type Adc14ie3R = crate::BitReader<Adc14ie3>;
impl Adc14ie3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie3 {
        match self.bits {
            false => Adc14ie3::Adc14ie3_0,
            true => Adc14ie3::Adc14ie3_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie3_0(&self) -> bool {
        *self == Adc14ie3::Adc14ie3_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie3_1(&self) -> bool {
        *self == Adc14ie3::Adc14ie3_1
    }
}
#[doc = "Field `ADC14IE3` writer - Interrupt enable"]
pub type Adc14ie3W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie3>;
impl<'a, REG> Adc14ie3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie3_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie3::Adc14ie3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie3_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie3::Adc14ie3_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie4 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie4_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie4_1 = 1,
}
impl From<Adc14ie4> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE4` reader - Interrupt enable"]
pub type Adc14ie4R = crate::BitReader<Adc14ie4>;
impl Adc14ie4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie4 {
        match self.bits {
            false => Adc14ie4::Adc14ie4_0,
            true => Adc14ie4::Adc14ie4_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie4_0(&self) -> bool {
        *self == Adc14ie4::Adc14ie4_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie4_1(&self) -> bool {
        *self == Adc14ie4::Adc14ie4_1
    }
}
#[doc = "Field `ADC14IE4` writer - Interrupt enable"]
pub type Adc14ie4W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie4>;
impl<'a, REG> Adc14ie4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie4_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie4::Adc14ie4_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie4_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie4::Adc14ie4_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie5 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie5_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie5_1 = 1,
}
impl From<Adc14ie5> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE5` reader - Interrupt enable"]
pub type Adc14ie5R = crate::BitReader<Adc14ie5>;
impl Adc14ie5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie5 {
        match self.bits {
            false => Adc14ie5::Adc14ie5_0,
            true => Adc14ie5::Adc14ie5_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie5_0(&self) -> bool {
        *self == Adc14ie5::Adc14ie5_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie5_1(&self) -> bool {
        *self == Adc14ie5::Adc14ie5_1
    }
}
#[doc = "Field `ADC14IE5` writer - Interrupt enable"]
pub type Adc14ie5W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie5>;
impl<'a, REG> Adc14ie5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie5::Adc14ie5_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie5::Adc14ie5_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie6 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie6_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie6_1 = 1,
}
impl From<Adc14ie6> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE6` reader - Interrupt enable"]
pub type Adc14ie6R = crate::BitReader<Adc14ie6>;
impl Adc14ie6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie6 {
        match self.bits {
            false => Adc14ie6::Adc14ie6_0,
            true => Adc14ie6::Adc14ie6_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie6_0(&self) -> bool {
        *self == Adc14ie6::Adc14ie6_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie6_1(&self) -> bool {
        *self == Adc14ie6::Adc14ie6_1
    }
}
#[doc = "Field `ADC14IE6` writer - Interrupt enable"]
pub type Adc14ie6W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie6>;
impl<'a, REG> Adc14ie6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie6_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie6::Adc14ie6_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie6_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie6::Adc14ie6_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie7 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie7_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie7_1 = 1,
}
impl From<Adc14ie7> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE7` reader - Interrupt enable"]
pub type Adc14ie7R = crate::BitReader<Adc14ie7>;
impl Adc14ie7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie7 {
        match self.bits {
            false => Adc14ie7::Adc14ie7_0,
            true => Adc14ie7::Adc14ie7_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie7_0(&self) -> bool {
        *self == Adc14ie7::Adc14ie7_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie7_1(&self) -> bool {
        *self == Adc14ie7::Adc14ie7_1
    }
}
#[doc = "Field `ADC14IE7` writer - Interrupt enable"]
pub type Adc14ie7W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie7>;
impl<'a, REG> Adc14ie7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie7_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie7::Adc14ie7_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie7_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie7::Adc14ie7_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie8 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie8_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie8_1 = 1,
}
impl From<Adc14ie8> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE8` reader - Interrupt enable"]
pub type Adc14ie8R = crate::BitReader<Adc14ie8>;
impl Adc14ie8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie8 {
        match self.bits {
            false => Adc14ie8::Adc14ie8_0,
            true => Adc14ie8::Adc14ie8_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie8_0(&self) -> bool {
        *self == Adc14ie8::Adc14ie8_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie8_1(&self) -> bool {
        *self == Adc14ie8::Adc14ie8_1
    }
}
#[doc = "Field `ADC14IE8` writer - Interrupt enable"]
pub type Adc14ie8W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie8>;
impl<'a, REG> Adc14ie8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie8_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie8::Adc14ie8_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie8_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie8::Adc14ie8_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie9 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie9_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie9_1 = 1,
}
impl From<Adc14ie9> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE9` reader - Interrupt enable"]
pub type Adc14ie9R = crate::BitReader<Adc14ie9>;
impl Adc14ie9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie9 {
        match self.bits {
            false => Adc14ie9::Adc14ie9_0,
            true => Adc14ie9::Adc14ie9_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie9_0(&self) -> bool {
        *self == Adc14ie9::Adc14ie9_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie9_1(&self) -> bool {
        *self == Adc14ie9::Adc14ie9_1
    }
}
#[doc = "Field `ADC14IE9` writer - Interrupt enable"]
pub type Adc14ie9W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie9>;
impl<'a, REG> Adc14ie9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie9_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie9::Adc14ie9_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie9_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie9::Adc14ie9_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie10 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie10_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie10_1 = 1,
}
impl From<Adc14ie10> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE10` reader - Interrupt enable"]
pub type Adc14ie10R = crate::BitReader<Adc14ie10>;
impl Adc14ie10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie10 {
        match self.bits {
            false => Adc14ie10::Adc14ie10_0,
            true => Adc14ie10::Adc14ie10_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie10_0(&self) -> bool {
        *self == Adc14ie10::Adc14ie10_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie10_1(&self) -> bool {
        *self == Adc14ie10::Adc14ie10_1
    }
}
#[doc = "Field `ADC14IE10` writer - Interrupt enable"]
pub type Adc14ie10W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie10>;
impl<'a, REG> Adc14ie10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie10_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie10::Adc14ie10_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie10_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie10::Adc14ie10_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie11 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie11_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie11_1 = 1,
}
impl From<Adc14ie11> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE11` reader - Interrupt enable"]
pub type Adc14ie11R = crate::BitReader<Adc14ie11>;
impl Adc14ie11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie11 {
        match self.bits {
            false => Adc14ie11::Adc14ie11_0,
            true => Adc14ie11::Adc14ie11_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie11_0(&self) -> bool {
        *self == Adc14ie11::Adc14ie11_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie11_1(&self) -> bool {
        *self == Adc14ie11::Adc14ie11_1
    }
}
#[doc = "Field `ADC14IE11` writer - Interrupt enable"]
pub type Adc14ie11W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie11>;
impl<'a, REG> Adc14ie11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie11_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie11::Adc14ie11_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie11_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie11::Adc14ie11_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie12 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie12_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie12_1 = 1,
}
impl From<Adc14ie12> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE12` reader - Interrupt enable"]
pub type Adc14ie12R = crate::BitReader<Adc14ie12>;
impl Adc14ie12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie12 {
        match self.bits {
            false => Adc14ie12::Adc14ie12_0,
            true => Adc14ie12::Adc14ie12_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie12_0(&self) -> bool {
        *self == Adc14ie12::Adc14ie12_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie12_1(&self) -> bool {
        *self == Adc14ie12::Adc14ie12_1
    }
}
#[doc = "Field `ADC14IE12` writer - Interrupt enable"]
pub type Adc14ie12W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie12>;
impl<'a, REG> Adc14ie12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie12_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie12::Adc14ie12_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie12_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie12::Adc14ie12_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie13 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie13_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie13_1 = 1,
}
impl From<Adc14ie13> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE13` reader - Interrupt enable"]
pub type Adc14ie13R = crate::BitReader<Adc14ie13>;
impl Adc14ie13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie13 {
        match self.bits {
            false => Adc14ie13::Adc14ie13_0,
            true => Adc14ie13::Adc14ie13_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie13_0(&self) -> bool {
        *self == Adc14ie13::Adc14ie13_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie13_1(&self) -> bool {
        *self == Adc14ie13::Adc14ie13_1
    }
}
#[doc = "Field `ADC14IE13` writer - Interrupt enable"]
pub type Adc14ie13W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie13>;
impl<'a, REG> Adc14ie13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie13_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie13::Adc14ie13_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie13_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie13::Adc14ie13_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie14 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie14_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie14_1 = 1,
}
impl From<Adc14ie14> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE14` reader - Interrupt enable"]
pub type Adc14ie14R = crate::BitReader<Adc14ie14>;
impl Adc14ie14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie14 {
        match self.bits {
            false => Adc14ie14::Adc14ie14_0,
            true => Adc14ie14::Adc14ie14_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie14_0(&self) -> bool {
        *self == Adc14ie14::Adc14ie14_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie14_1(&self) -> bool {
        *self == Adc14ie14::Adc14ie14_1
    }
}
#[doc = "Field `ADC14IE14` writer - Interrupt enable"]
pub type Adc14ie14W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie14>;
impl<'a, REG> Adc14ie14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie14_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie14::Adc14ie14_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie14_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie14::Adc14ie14_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie15 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie15_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie15_1 = 1,
}
impl From<Adc14ie15> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE15` reader - Interrupt enable"]
pub type Adc14ie15R = crate::BitReader<Adc14ie15>;
impl Adc14ie15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie15 {
        match self.bits {
            false => Adc14ie15::Adc14ie15_0,
            true => Adc14ie15::Adc14ie15_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie15_0(&self) -> bool {
        *self == Adc14ie15::Adc14ie15_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie15_1(&self) -> bool {
        *self == Adc14ie15::Adc14ie15_1
    }
}
#[doc = "Field `ADC14IE15` writer - Interrupt enable"]
pub type Adc14ie15W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie15>;
impl<'a, REG> Adc14ie15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie15_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie15::Adc14ie15_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie15_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie15::Adc14ie15_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie16 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie16_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie16_1 = 1,
}
impl From<Adc14ie16> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE16` reader - Interrupt enable"]
pub type Adc14ie16R = crate::BitReader<Adc14ie16>;
impl Adc14ie16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie16 {
        match self.bits {
            false => Adc14ie16::Adc14ie16_0,
            true => Adc14ie16::Adc14ie16_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie16_0(&self) -> bool {
        *self == Adc14ie16::Adc14ie16_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie16_1(&self) -> bool {
        *self == Adc14ie16::Adc14ie16_1
    }
}
#[doc = "Field `ADC14IE16` writer - Interrupt enable"]
pub type Adc14ie16W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie16>;
impl<'a, REG> Adc14ie16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie16_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie16::Adc14ie16_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie16_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie16::Adc14ie16_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie17 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie17_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie17_1 = 1,
}
impl From<Adc14ie17> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE17` reader - Interrupt enable"]
pub type Adc14ie17R = crate::BitReader<Adc14ie17>;
impl Adc14ie17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie17 {
        match self.bits {
            false => Adc14ie17::Adc14ie17_0,
            true => Adc14ie17::Adc14ie17_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie17_0(&self) -> bool {
        *self == Adc14ie17::Adc14ie17_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie17_1(&self) -> bool {
        *self == Adc14ie17::Adc14ie17_1
    }
}
#[doc = "Field `ADC14IE17` writer - Interrupt enable"]
pub type Adc14ie17W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie17>;
impl<'a, REG> Adc14ie17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie17_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie17::Adc14ie17_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie17_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie17::Adc14ie17_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie18 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie18_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie18_1 = 1,
}
impl From<Adc14ie18> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE18` reader - Interrupt enable"]
pub type Adc14ie18R = crate::BitReader<Adc14ie18>;
impl Adc14ie18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie18 {
        match self.bits {
            false => Adc14ie18::Adc14ie18_0,
            true => Adc14ie18::Adc14ie18_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie18_0(&self) -> bool {
        *self == Adc14ie18::Adc14ie18_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie18_1(&self) -> bool {
        *self == Adc14ie18::Adc14ie18_1
    }
}
#[doc = "Field `ADC14IE18` writer - Interrupt enable"]
pub type Adc14ie18W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie18>;
impl<'a, REG> Adc14ie18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie18_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie18::Adc14ie18_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie18_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie18::Adc14ie18_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie19 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie19_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie19_1 = 1,
}
impl From<Adc14ie19> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE19` reader - Interrupt enable"]
pub type Adc14ie19R = crate::BitReader<Adc14ie19>;
impl Adc14ie19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie19 {
        match self.bits {
            false => Adc14ie19::Adc14ie19_0,
            true => Adc14ie19::Adc14ie19_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie19_0(&self) -> bool {
        *self == Adc14ie19::Adc14ie19_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie19_1(&self) -> bool {
        *self == Adc14ie19::Adc14ie19_1
    }
}
#[doc = "Field `ADC14IE19` writer - Interrupt enable"]
pub type Adc14ie19W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie19>;
impl<'a, REG> Adc14ie19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie19_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie19::Adc14ie19_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie19_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie19::Adc14ie19_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie20 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie20_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie20_1 = 1,
}
impl From<Adc14ie20> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE20` reader - Interrupt enable"]
pub type Adc14ie20R = crate::BitReader<Adc14ie20>;
impl Adc14ie20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie20 {
        match self.bits {
            false => Adc14ie20::Adc14ie20_0,
            true => Adc14ie20::Adc14ie20_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie20_0(&self) -> bool {
        *self == Adc14ie20::Adc14ie20_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie20_1(&self) -> bool {
        *self == Adc14ie20::Adc14ie20_1
    }
}
#[doc = "Field `ADC14IE20` writer - Interrupt enable"]
pub type Adc14ie20W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie20>;
impl<'a, REG> Adc14ie20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie20_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie20::Adc14ie20_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie20_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie20::Adc14ie20_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie21 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie21_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie21_1 = 1,
}
impl From<Adc14ie21> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE21` reader - Interrupt enable"]
pub type Adc14ie21R = crate::BitReader<Adc14ie21>;
impl Adc14ie21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie21 {
        match self.bits {
            false => Adc14ie21::Adc14ie21_0,
            true => Adc14ie21::Adc14ie21_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie21_0(&self) -> bool {
        *self == Adc14ie21::Adc14ie21_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie21_1(&self) -> bool {
        *self == Adc14ie21::Adc14ie21_1
    }
}
#[doc = "Field `ADC14IE21` writer - Interrupt enable"]
pub type Adc14ie21W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie21>;
impl<'a, REG> Adc14ie21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie21_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie21::Adc14ie21_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie21_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie21::Adc14ie21_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie22 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie22_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie22_1 = 1,
}
impl From<Adc14ie22> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE22` reader - Interrupt enable"]
pub type Adc14ie22R = crate::BitReader<Adc14ie22>;
impl Adc14ie22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie22 {
        match self.bits {
            false => Adc14ie22::Adc14ie22_0,
            true => Adc14ie22::Adc14ie22_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie22_0(&self) -> bool {
        *self == Adc14ie22::Adc14ie22_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie22_1(&self) -> bool {
        *self == Adc14ie22::Adc14ie22_1
    }
}
#[doc = "Field `ADC14IE22` writer - Interrupt enable"]
pub type Adc14ie22W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie22>;
impl<'a, REG> Adc14ie22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie22_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie22::Adc14ie22_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie22_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie22::Adc14ie22_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie23 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie23_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie23_1 = 1,
}
impl From<Adc14ie23> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE23` reader - Interrupt enable"]
pub type Adc14ie23R = crate::BitReader<Adc14ie23>;
impl Adc14ie23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie23 {
        match self.bits {
            false => Adc14ie23::Adc14ie23_0,
            true => Adc14ie23::Adc14ie23_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie23_0(&self) -> bool {
        *self == Adc14ie23::Adc14ie23_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie23_1(&self) -> bool {
        *self == Adc14ie23::Adc14ie23_1
    }
}
#[doc = "Field `ADC14IE23` writer - Interrupt enable"]
pub type Adc14ie23W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie23>;
impl<'a, REG> Adc14ie23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie23_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie23::Adc14ie23_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie23_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie23::Adc14ie23_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie24 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie24_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie24_1 = 1,
}
impl From<Adc14ie24> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE24` reader - Interrupt enable"]
pub type Adc14ie24R = crate::BitReader<Adc14ie24>;
impl Adc14ie24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie24 {
        match self.bits {
            false => Adc14ie24::Adc14ie24_0,
            true => Adc14ie24::Adc14ie24_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie24_0(&self) -> bool {
        *self == Adc14ie24::Adc14ie24_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie24_1(&self) -> bool {
        *self == Adc14ie24::Adc14ie24_1
    }
}
#[doc = "Field `ADC14IE24` writer - Interrupt enable"]
pub type Adc14ie24W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie24>;
impl<'a, REG> Adc14ie24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie24_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie24::Adc14ie24_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie24_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie24::Adc14ie24_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie25 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie25_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie25_1 = 1,
}
impl From<Adc14ie25> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE25` reader - Interrupt enable"]
pub type Adc14ie25R = crate::BitReader<Adc14ie25>;
impl Adc14ie25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie25 {
        match self.bits {
            false => Adc14ie25::Adc14ie25_0,
            true => Adc14ie25::Adc14ie25_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie25_0(&self) -> bool {
        *self == Adc14ie25::Adc14ie25_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie25_1(&self) -> bool {
        *self == Adc14ie25::Adc14ie25_1
    }
}
#[doc = "Field `ADC14IE25` writer - Interrupt enable"]
pub type Adc14ie25W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie25>;
impl<'a, REG> Adc14ie25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie25_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie25::Adc14ie25_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie25_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie25::Adc14ie25_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie26 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie26_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie26_1 = 1,
}
impl From<Adc14ie26> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE26` reader - Interrupt enable"]
pub type Adc14ie26R = crate::BitReader<Adc14ie26>;
impl Adc14ie26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie26 {
        match self.bits {
            false => Adc14ie26::Adc14ie26_0,
            true => Adc14ie26::Adc14ie26_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie26_0(&self) -> bool {
        *self == Adc14ie26::Adc14ie26_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie26_1(&self) -> bool {
        *self == Adc14ie26::Adc14ie26_1
    }
}
#[doc = "Field `ADC14IE26` writer - Interrupt enable"]
pub type Adc14ie26W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie26>;
impl<'a, REG> Adc14ie26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie26_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie26::Adc14ie26_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie26_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie26::Adc14ie26_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie27 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie27_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie27_1 = 1,
}
impl From<Adc14ie27> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE27` reader - Interrupt enable"]
pub type Adc14ie27R = crate::BitReader<Adc14ie27>;
impl Adc14ie27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie27 {
        match self.bits {
            false => Adc14ie27::Adc14ie27_0,
            true => Adc14ie27::Adc14ie27_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie27_0(&self) -> bool {
        *self == Adc14ie27::Adc14ie27_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie27_1(&self) -> bool {
        *self == Adc14ie27::Adc14ie27_1
    }
}
#[doc = "Field `ADC14IE27` writer - Interrupt enable"]
pub type Adc14ie27W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie27>;
impl<'a, REG> Adc14ie27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie27_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie27::Adc14ie27_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie27_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie27::Adc14ie27_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie28 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie28_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie28_1 = 1,
}
impl From<Adc14ie28> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE28` reader - Interrupt enable"]
pub type Adc14ie28R = crate::BitReader<Adc14ie28>;
impl Adc14ie28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie28 {
        match self.bits {
            false => Adc14ie28::Adc14ie28_0,
            true => Adc14ie28::Adc14ie28_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie28_0(&self) -> bool {
        *self == Adc14ie28::Adc14ie28_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie28_1(&self) -> bool {
        *self == Adc14ie28::Adc14ie28_1
    }
}
#[doc = "Field `ADC14IE28` writer - Interrupt enable"]
pub type Adc14ie28W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie28>;
impl<'a, REG> Adc14ie28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie28_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie28::Adc14ie28_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie28_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie28::Adc14ie28_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie29 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie29_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie29_1 = 1,
}
impl From<Adc14ie29> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE29` reader - Interrupt enable"]
pub type Adc14ie29R = crate::BitReader<Adc14ie29>;
impl Adc14ie29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie29 {
        match self.bits {
            false => Adc14ie29::Adc14ie29_0,
            true => Adc14ie29::Adc14ie29_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie29_0(&self) -> bool {
        *self == Adc14ie29::Adc14ie29_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie29_1(&self) -> bool {
        *self == Adc14ie29::Adc14ie29_1
    }
}
#[doc = "Field `ADC14IE29` writer - Interrupt enable"]
pub type Adc14ie29W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie29>;
impl<'a, REG> Adc14ie29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie29_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie29::Adc14ie29_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie29_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie29::Adc14ie29_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie30 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie30_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie30_1 = 1,
}
impl From<Adc14ie30> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE30` reader - Interrupt enable"]
pub type Adc14ie30R = crate::BitReader<Adc14ie30>;
impl Adc14ie30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie30 {
        match self.bits {
            false => Adc14ie30::Adc14ie30_0,
            true => Adc14ie30::Adc14ie30_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie30_0(&self) -> bool {
        *self == Adc14ie30::Adc14ie30_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie30_1(&self) -> bool {
        *self == Adc14ie30::Adc14ie30_1
    }
}
#[doc = "Field `ADC14IE30` writer - Interrupt enable"]
pub type Adc14ie30W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie30>;
impl<'a, REG> Adc14ie30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie30_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie30::Adc14ie30_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie30_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie30::Adc14ie30_1)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ie31 {
    #[doc = "0: Interrupt disabled"]
    Adc14ie31_0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ie31_1 = 1,
}
impl From<Adc14ie31> for bool {
    #[inline(always)]
    fn from(variant: Adc14ie31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14IE31` reader - Interrupt enable"]
pub type Adc14ie31R = crate::BitReader<Adc14ie31>;
impl Adc14ie31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ie31 {
        match self.bits {
            false => Adc14ie31::Adc14ie31_0,
            true => Adc14ie31::Adc14ie31_1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ie31_0(&self) -> bool {
        *self == Adc14ie31::Adc14ie31_0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ie31_1(&self) -> bool {
        *self == Adc14ie31::Adc14ie31_1
    }
}
#[doc = "Field `ADC14IE31` writer - Interrupt enable"]
pub type Adc14ie31W<'a, REG> = crate::BitWriter<'a, REG, Adc14ie31>;
impl<'a, REG> Adc14ie31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie31_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie31::Adc14ie31_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie31_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ie31::Adc14ie31_1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie0(&self) -> Adc14ie0R {
        Adc14ie0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie1(&self) -> Adc14ie1R {
        Adc14ie1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie2(&self) -> Adc14ie2R {
        Adc14ie2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie3(&self) -> Adc14ie3R {
        Adc14ie3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie4(&self) -> Adc14ie4R {
        Adc14ie4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie5(&self) -> Adc14ie5R {
        Adc14ie5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie6(&self) -> Adc14ie6R {
        Adc14ie6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie7(&self) -> Adc14ie7R {
        Adc14ie7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie8(&self) -> Adc14ie8R {
        Adc14ie8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie9(&self) -> Adc14ie9R {
        Adc14ie9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie10(&self) -> Adc14ie10R {
        Adc14ie10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie11(&self) -> Adc14ie11R {
        Adc14ie11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie12(&self) -> Adc14ie12R {
        Adc14ie12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie13(&self) -> Adc14ie13R {
        Adc14ie13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie14(&self) -> Adc14ie14R {
        Adc14ie14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie15(&self) -> Adc14ie15R {
        Adc14ie15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie16(&self) -> Adc14ie16R {
        Adc14ie16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie17(&self) -> Adc14ie17R {
        Adc14ie17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie18(&self) -> Adc14ie18R {
        Adc14ie18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie19(&self) -> Adc14ie19R {
        Adc14ie19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie20(&self) -> Adc14ie20R {
        Adc14ie20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie21(&self) -> Adc14ie21R {
        Adc14ie21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie22(&self) -> Adc14ie22R {
        Adc14ie22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie23(&self) -> Adc14ie23R {
        Adc14ie23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie24(&self) -> Adc14ie24R {
        Adc14ie24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie25(&self) -> Adc14ie25R {
        Adc14ie25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie26(&self) -> Adc14ie26R {
        Adc14ie26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie27(&self) -> Adc14ie27R {
        Adc14ie27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie28(&self) -> Adc14ie28R {
        Adc14ie28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie29(&self) -> Adc14ie29R {
        Adc14ie29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie30(&self) -> Adc14ie30R {
        Adc14ie30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie31(&self) -> Adc14ie31R {
        Adc14ie31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie0(&mut self) -> Adc14ie0W<Adc14ier0Spec> {
        Adc14ie0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie1(&mut self) -> Adc14ie1W<Adc14ier0Spec> {
        Adc14ie1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie2(&mut self) -> Adc14ie2W<Adc14ier0Spec> {
        Adc14ie2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie3(&mut self) -> Adc14ie3W<Adc14ier0Spec> {
        Adc14ie3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie4(&mut self) -> Adc14ie4W<Adc14ier0Spec> {
        Adc14ie4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie5(&mut self) -> Adc14ie5W<Adc14ier0Spec> {
        Adc14ie5W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie6(&mut self) -> Adc14ie6W<Adc14ier0Spec> {
        Adc14ie6W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie7(&mut self) -> Adc14ie7W<Adc14ier0Spec> {
        Adc14ie7W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie8(&mut self) -> Adc14ie8W<Adc14ier0Spec> {
        Adc14ie8W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie9(&mut self) -> Adc14ie9W<Adc14ier0Spec> {
        Adc14ie9W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie10(&mut self) -> Adc14ie10W<Adc14ier0Spec> {
        Adc14ie10W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie11(&mut self) -> Adc14ie11W<Adc14ier0Spec> {
        Adc14ie11W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie12(&mut self) -> Adc14ie12W<Adc14ier0Spec> {
        Adc14ie12W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie13(&mut self) -> Adc14ie13W<Adc14ier0Spec> {
        Adc14ie13W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie14(&mut self) -> Adc14ie14W<Adc14ier0Spec> {
        Adc14ie14W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie15(&mut self) -> Adc14ie15W<Adc14ier0Spec> {
        Adc14ie15W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie16(&mut self) -> Adc14ie16W<Adc14ier0Spec> {
        Adc14ie16W::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie17(&mut self) -> Adc14ie17W<Adc14ier0Spec> {
        Adc14ie17W::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie18(&mut self) -> Adc14ie18W<Adc14ier0Spec> {
        Adc14ie18W::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie19(&mut self) -> Adc14ie19W<Adc14ier0Spec> {
        Adc14ie19W::new(self, 19)
    }
    #[doc = "Bit 20 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie20(&mut self) -> Adc14ie20W<Adc14ier0Spec> {
        Adc14ie20W::new(self, 20)
    }
    #[doc = "Bit 21 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie21(&mut self) -> Adc14ie21W<Adc14ier0Spec> {
        Adc14ie21W::new(self, 21)
    }
    #[doc = "Bit 22 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie22(&mut self) -> Adc14ie22W<Adc14ier0Spec> {
        Adc14ie22W::new(self, 22)
    }
    #[doc = "Bit 23 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie23(&mut self) -> Adc14ie23W<Adc14ier0Spec> {
        Adc14ie23W::new(self, 23)
    }
    #[doc = "Bit 24 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie24(&mut self) -> Adc14ie24W<Adc14ier0Spec> {
        Adc14ie24W::new(self, 24)
    }
    #[doc = "Bit 25 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie25(&mut self) -> Adc14ie25W<Adc14ier0Spec> {
        Adc14ie25W::new(self, 25)
    }
    #[doc = "Bit 26 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie26(&mut self) -> Adc14ie26W<Adc14ier0Spec> {
        Adc14ie26W::new(self, 26)
    }
    #[doc = "Bit 27 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie27(&mut self) -> Adc14ie27W<Adc14ier0Spec> {
        Adc14ie27W::new(self, 27)
    }
    #[doc = "Bit 28 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie28(&mut self) -> Adc14ie28W<Adc14ier0Spec> {
        Adc14ie28W::new(self, 28)
    }
    #[doc = "Bit 29 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie29(&mut self) -> Adc14ie29W<Adc14ier0Spec> {
        Adc14ie29W::new(self, 29)
    }
    #[doc = "Bit 30 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie30(&mut self) -> Adc14ie30W<Adc14ier0Spec> {
        Adc14ie30W::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie31(&mut self) -> Adc14ie31W<Adc14ier0Spec> {
        Adc14ie31W::new(self, 31)
    }
}
#[doc = "Interrupt Enable 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ier0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14ier0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14ier0Spec;
impl crate::RegisterSpec for Adc14ier0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14ier0::R`](R) reader structure"]
impl crate::Readable for Adc14ier0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc14ier0::W`](W) writer structure"]
impl crate::Writable for Adc14ier0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14IER0 to value 0"]
impl crate::Resettable for Adc14ier0Spec {
    const RESET_VALUE: u32 = 0;
}
