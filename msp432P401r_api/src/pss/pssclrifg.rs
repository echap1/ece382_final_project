#[doc = "Register `PSSCLRIFG` reader"]
pub type R = crate::R<PssclrifgSpec>;
#[doc = "Register `PSSCLRIFG` writer"]
pub type W = crate::W<PssclrifgSpec>;
#[doc = "SVSMH clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrsvsmhifgEnumWrite {
    #[doc = "0: No effect"]
    Clrsvsmhifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    Clrsvsmhifg1 = 1,
}
impl From<ClrsvsmhifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrsvsmhifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRSVSMHIFG` writer - SVSMH clear interrupt flag"]
pub type ClrsvsmhifgW<'a, REG> = crate::BitWriter<'a, REG, ClrsvsmhifgEnumWrite>;
impl<'a, REG> ClrsvsmhifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clrsvsmhifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrsvsmhifgEnumWrite::Clrsvsmhifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clrsvsmhifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrsvsmhifgEnumWrite::Clrsvsmhifg1)
    }
}
impl W {
    #[doc = "Bit 1 - SVSMH clear interrupt flag"]
    #[inline(always)]
    pub fn clrsvsmhifg(&mut self) -> ClrsvsmhifgW<PssclrifgSpec> {
        ClrsvsmhifgW::new(self, 1)
    }
}
#[doc = "Clear Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pssclrifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssclrifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PssclrifgSpec;
impl crate::RegisterSpec for PssclrifgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pssclrifg::R`](R) reader structure"]
impl crate::Readable for PssclrifgSpec {}
#[doc = "`write(|w| ..)` method takes [`pssclrifg::W`](W) writer structure"]
impl crate::Writable for PssclrifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSSCLRIFG to value 0"]
impl crate::Resettable for PssclrifgSpec {
    const RESET_VALUE: u32 = 0;
}
