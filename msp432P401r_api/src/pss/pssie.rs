#[doc = "Register `PSSIE` reader"]
pub type R = crate::R<PssieSpec>;
#[doc = "Register `PSSIE` writer"]
pub type W = crate::W<PssieSpec>;
#[doc = "High-side SVSM interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svsmhie {
    #[doc = "0: Interrupt disabled"]
    Svsmhie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Svsmhie1 = 1,
}
impl From<Svsmhie> for bool {
    #[inline(always)]
    fn from(variant: Svsmhie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSMHIE` reader - High-side SVSM interrupt enable"]
pub type SvsmhieR = crate::BitReader<Svsmhie>;
impl SvsmhieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svsmhie {
        match self.bits {
            false => Svsmhie::Svsmhie0,
            true => Svsmhie::Svsmhie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_svsmhie_0(&self) -> bool {
        *self == Svsmhie::Svsmhie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_svsmhie_1(&self) -> bool {
        *self == Svsmhie::Svsmhie1
    }
}
#[doc = "Field `SVSMHIE` writer - High-side SVSM interrupt enable"]
pub type SvsmhieW<'a, REG> = crate::BitWriter<'a, REG, Svsmhie>;
impl<'a, REG> SvsmhieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn svsmhie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Svsmhie::Svsmhie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn svsmhie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Svsmhie::Svsmhie1)
    }
}
impl R {
    #[doc = "Bit 1 - High-side SVSM interrupt enable"]
    #[inline(always)]
    pub fn svsmhie(&self) -> SvsmhieR {
        SvsmhieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - High-side SVSM interrupt enable"]
    #[inline(always)]
    pub fn svsmhie(&mut self) -> SvsmhieW<PssieSpec> {
        SvsmhieW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pssie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PssieSpec;
impl crate::RegisterSpec for PssieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pssie::R`](R) reader structure"]
impl crate::Readable for PssieSpec {}
#[doc = "`write(|w| ..)` method takes [`pssie::W`](W) writer structure"]
impl crate::Writable for PssieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSSIE to value 0"]
impl crate::Resettable for PssieSpec {
    const RESET_VALUE: u32 = 0;
}
