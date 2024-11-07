#[doc = "Register `PBREN` reader"]
pub type R = crate::R<PbrenSpec>;
#[doc = "Register `PBREN` writer"]
pub type W = crate::W<PbrenSpec>;
#[doc = "Field `P3REN` reader - Port 3 Resistor Enable"]
pub type P3renR = crate::FieldReader;
#[doc = "Field `P3REN` writer - Port 3 Resistor Enable"]
pub type P3renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P4REN` reader - Port 4 Resistor Enable"]
pub type P4renR = crate::FieldReader;
#[doc = "Field `P4REN` writer - Port 4 Resistor Enable"]
pub type P4renW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub fn p3ren(&self) -> P3renR {
        P3renR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub fn p4ren(&self) -> P4renR {
        P4renR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub fn p3ren(&mut self) -> P3renW<PbrenSpec> {
        P3renW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub fn p4ren(&mut self) -> P4renW<PbrenSpec> {
        P4renW::new(self, 8)
    }
}
#[doc = "Port B Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pbren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbrenSpec;
impl crate::RegisterSpec for PbrenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbren::R`](R) reader structure"]
impl crate::Readable for PbrenSpec {}
#[doc = "`write(|w| ..)` method takes [`pbren::W`](W) writer structure"]
impl crate::Writable for PbrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBREN to value 0"]
impl crate::Resettable for PbrenSpec {
    const RESET_VALUE: u16 = 0;
}
