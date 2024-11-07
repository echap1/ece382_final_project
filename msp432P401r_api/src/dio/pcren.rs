#[doc = "Register `PCREN` reader"]
pub type R = crate::R<PcrenSpec>;
#[doc = "Register `PCREN` writer"]
pub type W = crate::W<PcrenSpec>;
#[doc = "Field `P5REN` reader - Port 5 Resistor Enable"]
pub type P5renR = crate::FieldReader;
#[doc = "Field `P5REN` writer - Port 5 Resistor Enable"]
pub type P5renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P6REN` reader - Port 6 Resistor Enable"]
pub type P6renR = crate::FieldReader;
#[doc = "Field `P6REN` writer - Port 6 Resistor Enable"]
pub type P6renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Resistor Enable"]
    #[inline(always)]
    pub fn p5ren(&self) -> P5renR {
        P5renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Resistor Enable"]
    #[inline(always)]
    pub fn p6ren(&self) -> P6renR {
        P6renR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Resistor Enable"]
    #[inline(always)]
    pub fn p5ren(&mut self) -> P5renW<PcrenSpec> {
        P5renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 6 Resistor Enable"]
    #[inline(always)]
    pub fn p6ren(&mut self) -> P6renW<PcrenSpec> {
        P6renW::new(self, 8)
    }
}
#[doc = "Port C Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pcren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrenSpec;
impl crate::RegisterSpec for PcrenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcren::R`](R) reader structure"]
impl crate::Readable for PcrenSpec {}
#[doc = "`write(|w| ..)` method takes [`pcren::W`](W) writer structure"]
impl crate::Writable for PcrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PCREN to value 0"]
impl crate::Resettable for PcrenSpec {
    const RESET_VALUE: u16 = 0;
}
