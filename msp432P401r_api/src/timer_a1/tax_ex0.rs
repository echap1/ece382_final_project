#[doc = "Register `TAxEX0` reader"]
pub type R = crate::R<TaxEx0Spec>;
#[doc = "Register `TAxEX0` writer"]
pub type W = crate::W<TaxEx0Spec>;
#[doc = "Input divider expansion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Taidex {
    #[doc = "0: Divide by 1"]
    Taidex0 = 0,
    #[doc = "1: Divide by 2"]
    Taidex1 = 1,
    #[doc = "2: Divide by 3"]
    Taidex2 = 2,
    #[doc = "3: Divide by 4"]
    Taidex3 = 3,
    #[doc = "4: Divide by 5"]
    Taidex4 = 4,
    #[doc = "5: Divide by 6"]
    Taidex5 = 5,
    #[doc = "6: Divide by 7"]
    Taidex6 = 6,
    #[doc = "7: Divide by 8"]
    Taidex7 = 7,
}
impl From<Taidex> for u8 {
    #[inline(always)]
    fn from(variant: Taidex) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Taidex {
    type Ux = u8;
}
impl crate::IsEnum for Taidex {}
#[doc = "Field `TAIDEX` reader - Input divider expansion"]
pub type TaidexR = crate::FieldReader<Taidex>;
impl TaidexR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taidex {
        match self.bits {
            0 => Taidex::Taidex0,
            1 => Taidex::Taidex1,
            2 => Taidex::Taidex2,
            3 => Taidex::Taidex3,
            4 => Taidex::Taidex4,
            5 => Taidex::Taidex5,
            6 => Taidex::Taidex6,
            7 => Taidex::Taidex7,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_taidex_0(&self) -> bool {
        *self == Taidex::Taidex0
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_taidex_1(&self) -> bool {
        *self == Taidex::Taidex1
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn is_taidex_2(&self) -> bool {
        *self == Taidex::Taidex2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_taidex_3(&self) -> bool {
        *self == Taidex::Taidex3
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn is_taidex_4(&self) -> bool {
        *self == Taidex::Taidex4
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_taidex_5(&self) -> bool {
        *self == Taidex::Taidex5
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn is_taidex_6(&self) -> bool {
        *self == Taidex::Taidex6
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_taidex_7(&self) -> bool {
        *self == Taidex::Taidex7
    }
}
#[doc = "Field `TAIDEX` writer - Input divider expansion"]
pub type TaidexW<'a, REG> = crate::FieldWriter<'a, REG, 3, Taidex, crate::Safe>;
impl<'a, REG> TaidexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn taidex_0(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn taidex_1(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex1)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn taidex_2(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn taidex_3(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex3)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn taidex_4(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex4)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn taidex_5(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex5)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn taidex_6(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex6)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn taidex_7(self) -> &'a mut crate::W<REG> {
        self.variant(Taidex::Taidex7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    pub fn taidex(&self) -> TaidexR {
        TaidexR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    pub fn taidex(&mut self) -> TaidexW<TaxEx0Spec> {
        TaidexW::new(self, 0)
    }
}
#[doc = "TimerAx Expansion 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_ex0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tax_ex0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaxEx0Spec;
impl crate::RegisterSpec for TaxEx0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tax_ex0::R`](R) reader structure"]
impl crate::Readable for TaxEx0Spec {}
#[doc = "`write(|w| ..)` method takes [`tax_ex0::W`](W) writer structure"]
impl crate::Writable for TaxEx0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TAxEX0 to value 0"]
impl crate::Resettable for TaxEx0Spec {
    const RESET_VALUE: u16 = 0;
}
