#[doc = "Register `PJREN` reader"]
pub type R = crate::R<PjrenSpec>;
#[doc = "Register `PJREN` writer"]
pub type W = crate::W<PjrenSpec>;
#[doc = "Field `PJREN` reader - Port J Resistor Enable"]
pub type PjrenR = crate::FieldReader<u16>;
#[doc = "Field `PJREN` writer - Port J Resistor Enable"]
pub type PjrenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port J Resistor Enable"]
    #[inline(always)]
    pub fn pjren(&self) -> PjrenR {
        PjrenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Resistor Enable"]
    #[inline(always)]
    pub fn pjren(&mut self) -> PjrenW<PjrenSpec> {
        PjrenW::new(self, 0)
    }
}
#[doc = "Port J Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pjren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjrenSpec;
impl crate::RegisterSpec for PjrenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjren::R`](R) reader structure"]
impl crate::Readable for PjrenSpec {}
#[doc = "`write(|w| ..)` method takes [`pjren::W`](W) writer structure"]
impl crate::Writable for PjrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PJREN to value 0"]
impl crate::Resettable for PjrenSpec {
    const RESET_VALUE: u16 = 0;
}
