#[doc = "Register `ADC14IER1` reader"]
pub type R = crate::R<Adc14ier1Spec>;
#[doc = "Register `ADC14IER1` writer"]
pub type W = crate::W<Adc14ier1Spec>;
#[doc = "Interrupt enable for ADC14MEMx within comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14inie {
    #[doc = "0: Interrupt disabled"]
    Adc14inie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14inie1 = 1,
}
impl From<Adc14inie> for bool {
    #[inline(always)]
    fn from(variant: Adc14inie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14INIE` reader - Interrupt enable for ADC14MEMx within comparator window"]
pub type Adc14inieR = crate::BitReader<Adc14inie>;
impl Adc14inieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14inie {
        match self.bits {
            false => Adc14inie::Adc14inie0,
            true => Adc14inie::Adc14inie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14inie_0(&self) -> bool {
        *self == Adc14inie::Adc14inie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14inie_1(&self) -> bool {
        *self == Adc14inie::Adc14inie1
    }
}
#[doc = "Field `ADC14INIE` writer - Interrupt enable for ADC14MEMx within comparator window"]
pub type Adc14inieW<'a, REG> = crate::BitWriter<'a, REG, Adc14inie>;
impl<'a, REG> Adc14inieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14inie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inie::Adc14inie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14inie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inie::Adc14inie1)
    }
}
#[doc = "Interrupt enable for ADC14MEMx below comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14loie {
    #[doc = "0: Interrupt disabled"]
    Adc14loie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14loie1 = 1,
}
impl From<Adc14loie> for bool {
    #[inline(always)]
    fn from(variant: Adc14loie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14LOIE` reader - Interrupt enable for ADC14MEMx below comparator window"]
pub type Adc14loieR = crate::BitReader<Adc14loie>;
impl Adc14loieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14loie {
        match self.bits {
            false => Adc14loie::Adc14loie0,
            true => Adc14loie::Adc14loie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14loie_0(&self) -> bool {
        *self == Adc14loie::Adc14loie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14loie_1(&self) -> bool {
        *self == Adc14loie::Adc14loie1
    }
}
#[doc = "Field `ADC14LOIE` writer - Interrupt enable for ADC14MEMx below comparator window"]
pub type Adc14loieW<'a, REG> = crate::BitWriter<'a, REG, Adc14loie>;
impl<'a, REG> Adc14loieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14loie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14loie::Adc14loie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14loie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14loie::Adc14loie1)
    }
}
#[doc = "Interrupt enable for ADC14MEMx above comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14hiie {
    #[doc = "0: Interrupt disabled"]
    Adc14hiie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14hiie1 = 1,
}
impl From<Adc14hiie> for bool {
    #[inline(always)]
    fn from(variant: Adc14hiie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14HIIE` reader - Interrupt enable for ADC14MEMx above comparator window"]
pub type Adc14hiieR = crate::BitReader<Adc14hiie>;
impl Adc14hiieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14hiie {
        match self.bits {
            false => Adc14hiie::Adc14hiie0,
            true => Adc14hiie::Adc14hiie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14hiie_0(&self) -> bool {
        *self == Adc14hiie::Adc14hiie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14hiie_1(&self) -> bool {
        *self == Adc14hiie::Adc14hiie1
    }
}
#[doc = "Field `ADC14HIIE` writer - Interrupt enable for ADC14MEMx above comparator window"]
pub type Adc14hiieW<'a, REG> = crate::BitWriter<'a, REG, Adc14hiie>;
impl<'a, REG> Adc14hiieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14hiie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14hiie::Adc14hiie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14hiie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14hiie::Adc14hiie1)
    }
}
#[doc = "ADC14MEMx overflow-interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ovie {
    #[doc = "0: Interrupt disabled"]
    Adc14ovie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14ovie1 = 1,
}
impl From<Adc14ovie> for bool {
    #[inline(always)]
    fn from(variant: Adc14ovie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14OVIE` reader - ADC14MEMx overflow-interrupt enable"]
pub type Adc14ovieR = crate::BitReader<Adc14ovie>;
impl Adc14ovieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ovie {
        match self.bits {
            false => Adc14ovie::Adc14ovie0,
            true => Adc14ovie::Adc14ovie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14ovie_0(&self) -> bool {
        *self == Adc14ovie::Adc14ovie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14ovie_1(&self) -> bool {
        *self == Adc14ovie::Adc14ovie1
    }
}
#[doc = "Field `ADC14OVIE` writer - ADC14MEMx overflow-interrupt enable"]
pub type Adc14ovieW<'a, REG> = crate::BitWriter<'a, REG, Adc14ovie>;
impl<'a, REG> Adc14ovieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ovie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ovie::Adc14ovie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ovie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ovie::Adc14ovie1)
    }
}
#[doc = "ADC14 conversion-time-overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14tovie {
    #[doc = "0: Interrupt disabled"]
    Adc14tovie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14tovie1 = 1,
}
impl From<Adc14tovie> for bool {
    #[inline(always)]
    fn from(variant: Adc14tovie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14TOVIE` reader - ADC14 conversion-time-overflow interrupt enable"]
pub type Adc14tovieR = crate::BitReader<Adc14tovie>;
impl Adc14tovieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14tovie {
        match self.bits {
            false => Adc14tovie::Adc14tovie0,
            true => Adc14tovie::Adc14tovie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14tovie_0(&self) -> bool {
        *self == Adc14tovie::Adc14tovie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14tovie_1(&self) -> bool {
        *self == Adc14tovie::Adc14tovie1
    }
}
#[doc = "Field `ADC14TOVIE` writer - ADC14 conversion-time-overflow interrupt enable"]
pub type Adc14tovieW<'a, REG> = crate::BitWriter<'a, REG, Adc14tovie>;
impl<'a, REG> Adc14tovieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14tovie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14tovie::Adc14tovie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14tovie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14tovie::Adc14tovie1)
    }
}
#[doc = "ADC14 local buffered reference ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14rdyie {
    #[doc = "0: Interrupt disabled"]
    Adc14rdyie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Adc14rdyie1 = 1,
}
impl From<Adc14rdyie> for bool {
    #[inline(always)]
    fn from(variant: Adc14rdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14RDYIE` reader - ADC14 local buffered reference ready interrupt enable"]
pub type Adc14rdyieR = crate::BitReader<Adc14rdyie>;
impl Adc14rdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14rdyie {
        match self.bits {
            false => Adc14rdyie::Adc14rdyie0,
            true => Adc14rdyie::Adc14rdyie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_adc14rdyie_0(&self) -> bool {
        *self == Adc14rdyie::Adc14rdyie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_adc14rdyie_1(&self) -> bool {
        *self == Adc14rdyie::Adc14rdyie1
    }
}
#[doc = "Field `ADC14RDYIE` writer - ADC14 local buffered reference ready interrupt enable"]
pub type Adc14rdyieW<'a, REG> = crate::BitWriter<'a, REG, Adc14rdyie>;
impl<'a, REG> Adc14rdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14rdyie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14rdyie::Adc14rdyie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14rdyie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14rdyie::Adc14rdyie1)
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt enable for ADC14MEMx within comparator window"]
    #[inline(always)]
    pub fn adc14inie(&self) -> Adc14inieR {
        Adc14inieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for ADC14MEMx below comparator window"]
    #[inline(always)]
    pub fn adc14loie(&self) -> Adc14loieR {
        Adc14loieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for ADC14MEMx above comparator window"]
    #[inline(always)]
    pub fn adc14hiie(&self) -> Adc14hiieR {
        Adc14hiieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC14MEMx overflow-interrupt enable"]
    #[inline(always)]
    pub fn adc14ovie(&self) -> Adc14ovieR {
        Adc14ovieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC14 conversion-time-overflow interrupt enable"]
    #[inline(always)]
    pub fn adc14tovie(&self) -> Adc14tovieR {
        Adc14tovieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC14 local buffered reference ready interrupt enable"]
    #[inline(always)]
    pub fn adc14rdyie(&self) -> Adc14rdyieR {
        Adc14rdyieR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt enable for ADC14MEMx within comparator window"]
    #[inline(always)]
    pub fn adc14inie(&mut self) -> Adc14inieW<Adc14ier1Spec> {
        Adc14inieW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable for ADC14MEMx below comparator window"]
    #[inline(always)]
    pub fn adc14loie(&mut self) -> Adc14loieW<Adc14ier1Spec> {
        Adc14loieW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable for ADC14MEMx above comparator window"]
    #[inline(always)]
    pub fn adc14hiie(&mut self) -> Adc14hiieW<Adc14ier1Spec> {
        Adc14hiieW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC14MEMx overflow-interrupt enable"]
    #[inline(always)]
    pub fn adc14ovie(&mut self) -> Adc14ovieW<Adc14ier1Spec> {
        Adc14ovieW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC14 conversion-time-overflow interrupt enable"]
    #[inline(always)]
    pub fn adc14tovie(&mut self) -> Adc14tovieW<Adc14ier1Spec> {
        Adc14tovieW::new(self, 5)
    }
    #[doc = "Bit 6 - ADC14 local buffered reference ready interrupt enable"]
    #[inline(always)]
    pub fn adc14rdyie(&mut self) -> Adc14rdyieW<Adc14ier1Spec> {
        Adc14rdyieW::new(self, 6)
    }
}
#[doc = "Interrupt Enable 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14ier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14ier1Spec;
impl crate::RegisterSpec for Adc14ier1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14ier1::R`](R) reader structure"]
impl crate::Readable for Adc14ier1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc14ier1::W`](W) writer structure"]
impl crate::Writable for Adc14ier1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14IER1 to value 0"]
impl crate::Resettable for Adc14ier1Spec {
    const RESET_VALUE: u32 = 0;
}
