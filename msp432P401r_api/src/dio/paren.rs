#[doc = "Register `PAREN` reader"]
pub type R = crate::R<ParenSpec>;
#[doc = "Register `PAREN` writer"]
pub type W = crate::W<ParenSpec>;
#[doc = "Field `P1REN` reader - Port 1 Resistor Enable"]
pub type P1renR = crate::FieldReader;
#[doc = "Field `P1REN` writer - Port 1 Resistor Enable"]
pub type P1renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P2REN` reader - Port 2 Resistor Enable"]
pub type P2renR = crate::FieldReader;
#[doc = "Field `P2REN` writer - Port 2 Resistor Enable"]
pub type P2renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Resistor Enable"]
    #[inline(always)]
    pub fn p1ren(&self) -> P1renR {
        P1renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Resistor Enable"]
    #[inline(always)]
    pub fn p2ren(&self) -> P2renR {
        P2renR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Resistor Enable"]
    #[inline(always)]
    pub fn p1ren(&mut self) -> P1renW<ParenSpec> {
        P1renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 2 Resistor Enable"]
    #[inline(always)]
    pub fn p2ren(&mut self) -> P2renW<ParenSpec> {
        P2renW::new(self, 8)
    }
}
#[doc = "Port A Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`paren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParenSpec;
impl crate::RegisterSpec for ParenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`paren::R`](R) reader structure"]
impl crate::Readable for ParenSpec {}
#[doc = "`write(|w| ..)` method takes [`paren::W`](W) writer structure"]
impl crate::Writable for ParenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PAREN to value 0"]
impl crate::Resettable for ParenSpec {
    const RESET_VALUE: u16 = 0;
}
