#[doc = "Register `PEREN` reader"]
pub type R = crate::R<PerenSpec>;
#[doc = "Register `PEREN` writer"]
pub type W = crate::W<PerenSpec>;
#[doc = "Field `P9REN` reader - Port 9 Resistor Enable"]
pub type P9renR = crate::FieldReader;
#[doc = "Field `P9REN` writer - Port 9 Resistor Enable"]
pub type P9renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P10REN` reader - Port 10 Resistor Enable"]
pub type P10renR = crate::FieldReader;
#[doc = "Field `P10REN` writer - Port 10 Resistor Enable"]
pub type P10renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Resistor Enable"]
    #[inline(always)]
    pub fn p9ren(&self) -> P9renR {
        P9renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Resistor Enable"]
    #[inline(always)]
    pub fn p10ren(&self) -> P10renR {
        P10renR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Resistor Enable"]
    #[inline(always)]
    pub fn p9ren(&mut self) -> P9renW<PerenSpec> {
        P9renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 10 Resistor Enable"]
    #[inline(always)]
    pub fn p10ren(&mut self) -> P10renW<PerenSpec> {
        P10renW::new(self, 8)
    }
}
#[doc = "Port E Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`peren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerenSpec;
impl crate::RegisterSpec for PerenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`peren::R`](R) reader structure"]
impl crate::Readable for PerenSpec {}
#[doc = "`write(|w| ..)` method takes [`peren::W`](W) writer structure"]
impl crate::Writable for PerenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PEREN to value 0"]
impl crate::Resettable for PerenSpec {
    const RESET_VALUE: u16 = 0;
}
