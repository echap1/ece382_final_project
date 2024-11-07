#[doc = "Register `CExINT` reader"]
pub type R = crate::R<CexIntSpec>;
#[doc = "Register `CExINT` writer"]
pub type W = crate::W<CexIntSpec>;
#[doc = "Comparator output interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceifg {
    #[doc = "0: No interrupt pending"]
    Ceifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ceifg1 = 1,
}
impl From<Ceifg> for bool {
    #[inline(always)]
    fn from(variant: Ceifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIFG` reader - Comparator output interrupt flag"]
pub type CeifgR = crate::BitReader<Ceifg>;
impl CeifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceifg {
        match self.bits {
            false => Ceifg::Ceifg0,
            true => Ceifg::Ceifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ceifg_0(&self) -> bool {
        *self == Ceifg::Ceifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ceifg_1(&self) -> bool {
        *self == Ceifg::Ceifg1
    }
}
#[doc = "Field `CEIFG` writer - Comparator output interrupt flag"]
pub type CeifgW<'a, REG> = crate::BitWriter<'a, REG, Ceifg>;
impl<'a, REG> CeifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ceifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceifg::Ceifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ceifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceifg::Ceifg1)
    }
}
#[doc = "Comparator output inverted interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceiifg {
    #[doc = "0: No interrupt pending"]
    Ceiifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ceiifg1 = 1,
}
impl From<Ceiifg> for bool {
    #[inline(always)]
    fn from(variant: Ceiifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIIFG` reader - Comparator output inverted interrupt flag"]
pub type CeiifgR = crate::BitReader<Ceiifg>;
impl CeiifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceiifg {
        match self.bits {
            false => Ceiifg::Ceiifg0,
            true => Ceiifg::Ceiifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ceiifg_0(&self) -> bool {
        *self == Ceiifg::Ceiifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ceiifg_1(&self) -> bool {
        *self == Ceiifg::Ceiifg1
    }
}
#[doc = "Field `CEIIFG` writer - Comparator output inverted interrupt flag"]
pub type CeiifgW<'a, REG> = crate::BitWriter<'a, REG, Ceiifg>;
impl<'a, REG> CeiifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ceiifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceiifg::Ceiifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ceiifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceiifg::Ceiifg1)
    }
}
#[doc = "Comparator ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cerdyifg {
    #[doc = "0: No interrupt pending"]
    Cerdyifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Cerdyifg1 = 1,
}
impl From<Cerdyifg> for bool {
    #[inline(always)]
    fn from(variant: Cerdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERDYIFG` reader - Comparator ready interrupt flag"]
pub type CerdyifgR = crate::BitReader<Cerdyifg>;
impl CerdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cerdyifg {
        match self.bits {
            false => Cerdyifg::Cerdyifg0,
            true => Cerdyifg::Cerdyifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_cerdyifg_0(&self) -> bool {
        *self == Cerdyifg::Cerdyifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_cerdyifg_1(&self) -> bool {
        *self == Cerdyifg::Cerdyifg1
    }
}
#[doc = "Field `CERDYIFG` writer - Comparator ready interrupt flag"]
pub type CerdyifgW<'a, REG> = crate::BitWriter<'a, REG, Cerdyifg>;
impl<'a, REG> CerdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn cerdyifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cerdyifg::Cerdyifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn cerdyifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cerdyifg::Cerdyifg1)
    }
}
#[doc = "Comparator output interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceie {
    #[doc = "0: Interrupt disabled"]
    Ceie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ceie1 = 1,
}
impl From<Ceie> for bool {
    #[inline(always)]
    fn from(variant: Ceie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIE` reader - Comparator output interrupt enable"]
pub type CeieR = crate::BitReader<Ceie>;
impl CeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceie {
        match self.bits {
            false => Ceie::Ceie0,
            true => Ceie::Ceie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ceie_0(&self) -> bool {
        *self == Ceie::Ceie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ceie_1(&self) -> bool {
        *self == Ceie::Ceie1
    }
}
#[doc = "Field `CEIE` writer - Comparator output interrupt enable"]
pub type CeieW<'a, REG> = crate::BitWriter<'a, REG, Ceie>;
impl<'a, REG> CeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ceie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceie::Ceie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ceie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceie::Ceie1)
    }
}
#[doc = "Comparator output interrupt enable inverted polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceiie {
    #[doc = "0: Interrupt disabled"]
    Ceiie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ceiie1 = 1,
}
impl From<Ceiie> for bool {
    #[inline(always)]
    fn from(variant: Ceiie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIIE` reader - Comparator output interrupt enable inverted polarity"]
pub type CeiieR = crate::BitReader<Ceiie>;
impl CeiieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceiie {
        match self.bits {
            false => Ceiie::Ceiie0,
            true => Ceiie::Ceiie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ceiie_0(&self) -> bool {
        *self == Ceiie::Ceiie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ceiie_1(&self) -> bool {
        *self == Ceiie::Ceiie1
    }
}
#[doc = "Field `CEIIE` writer - Comparator output interrupt enable inverted polarity"]
pub type CeiieW<'a, REG> = crate::BitWriter<'a, REG, Ceiie>;
impl<'a, REG> CeiieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ceiie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceiie::Ceiie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ceiie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceiie::Ceiie1)
    }
}
#[doc = "Comparator ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cerdyie {
    #[doc = "0: Interrupt disabled"]
    Cerdyie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Cerdyie1 = 1,
}
impl From<Cerdyie> for bool {
    #[inline(always)]
    fn from(variant: Cerdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERDYIE` reader - Comparator ready interrupt enable"]
pub type CerdyieR = crate::BitReader<Cerdyie>;
impl CerdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cerdyie {
        match self.bits {
            false => Cerdyie::Cerdyie0,
            true => Cerdyie::Cerdyie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_cerdyie_0(&self) -> bool {
        *self == Cerdyie::Cerdyie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_cerdyie_1(&self) -> bool {
        *self == Cerdyie::Cerdyie1
    }
}
#[doc = "Field `CERDYIE` writer - Comparator ready interrupt enable"]
pub type CerdyieW<'a, REG> = crate::BitWriter<'a, REG, Cerdyie>;
impl<'a, REG> CerdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn cerdyie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cerdyie::Cerdyie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn cerdyie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cerdyie::Cerdyie1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn ceifg(&self) -> CeifgR {
        CeifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn ceiifg(&self) -> CeiifgR {
        CeiifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator ready interrupt flag"]
    #[inline(always)]
    pub fn cerdyifg(&self) -> CerdyifgR {
        CerdyifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparator output interrupt enable"]
    #[inline(always)]
    pub fn ceie(&self) -> CeieR {
        CeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator output interrupt enable inverted polarity"]
    #[inline(always)]
    pub fn ceiie(&self) -> CeiieR {
        CeiieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparator ready interrupt enable"]
    #[inline(always)]
    pub fn cerdyie(&self) -> CerdyieR {
        CerdyieR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator output interrupt flag"]
    #[inline(always)]
    pub fn ceifg(&mut self) -> CeifgW<CexIntSpec> {
        CeifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator output inverted interrupt flag"]
    #[inline(always)]
    pub fn ceiifg(&mut self) -> CeiifgW<CexIntSpec> {
        CeiifgW::new(self, 1)
    }
    #[doc = "Bit 4 - Comparator ready interrupt flag"]
    #[inline(always)]
    pub fn cerdyifg(&mut self) -> CerdyifgW<CexIntSpec> {
        CerdyifgW::new(self, 4)
    }
    #[doc = "Bit 8 - Comparator output interrupt enable"]
    #[inline(always)]
    pub fn ceie(&mut self) -> CeieW<CexIntSpec> {
        CeieW::new(self, 8)
    }
    #[doc = "Bit 9 - Comparator output interrupt enable inverted polarity"]
    #[inline(always)]
    pub fn ceiie(&mut self) -> CeiieW<CexIntSpec> {
        CeiieW::new(self, 9)
    }
    #[doc = "Bit 12 - Comparator ready interrupt enable"]
    #[inline(always)]
    pub fn cerdyie(&mut self) -> CerdyieW<CexIntSpec> {
        CerdyieW::new(self, 12)
    }
}
#[doc = "Comparator Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cex_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cex_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CexIntSpec;
impl crate::RegisterSpec for CexIntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cex_int::R`](R) reader structure"]
impl crate::Readable for CexIntSpec {}
#[doc = "`write(|w| ..)` method takes [`cex_int::W`](W) writer structure"]
impl crate::Writable for CexIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CExINT to value 0"]
impl crate::Resettable for CexIntSpec {
    const RESET_VALUE: u16 = 0;
}
