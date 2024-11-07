#[doc = "Register `PDREN` reader"]
pub type R = crate::R<PdrenSpec>;
#[doc = "Register `PDREN` writer"]
pub type W = crate::W<PdrenSpec>;
#[doc = "Field `P7REN` reader - Port 7 Resistor Enable"]
pub type P7renR = crate::FieldReader;
#[doc = "Field `P7REN` writer - Port 7 Resistor Enable"]
pub type P7renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P8REN` reader - Port 8 Resistor Enable"]
pub type P8renR = crate::FieldReader;
#[doc = "Field `P8REN` writer - Port 8 Resistor Enable"]
pub type P8renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Resistor Enable"]
    #[inline(always)]
    pub fn p7ren(&self) -> P7renR {
        P7renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Resistor Enable"]
    #[inline(always)]
    pub fn p8ren(&self) -> P8renR {
        P8renR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Resistor Enable"]
    #[inline(always)]
    pub fn p7ren(&mut self) -> P7renW<PdrenSpec> {
        P7renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 8 Resistor Enable"]
    #[inline(always)]
    pub fn p8ren(&mut self) -> P8renW<PdrenSpec> {
        P8renW::new(self, 8)
    }
}
#[doc = "Port D Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pdren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdrenSpec;
impl crate::RegisterSpec for PdrenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdren::R`](R) reader structure"]
impl crate::Readable for PdrenSpec {}
#[doc = "`write(|w| ..)` method takes [`pdren::W`](W) writer structure"]
impl crate::Writable for PdrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDREN to value 0"]
impl crate::Resettable for PdrenSpec {
    const RESET_VALUE: u16 = 0;
}
