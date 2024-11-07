#[doc = "Register `PEIES` reader"]
pub type R = crate::R<PeiesSpec>;
#[doc = "Register `PEIES` writer"]
pub type W = crate::W<PeiesSpec>;
#[doc = "Field `P9IES` reader - Port 9 Interrupt Edge Select"]
pub type P9iesR = crate::FieldReader;
#[doc = "Field `P9IES` writer - Port 9 Interrupt Edge Select"]
pub type P9iesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P10IES` reader - Port 10 Interrupt Edge Select"]
pub type P10iesR = crate::FieldReader;
#[doc = "Field `P10IES` writer - Port 10 Interrupt Edge Select"]
pub type P10iesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p9ies(&self) -> P9iesR {
        P9iesR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p10ies(&self) -> P10iesR {
        P10iesR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p9ies(&mut self) -> P9iesW<PeiesSpec> {
        P9iesW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p10ies(&mut self) -> P10iesW<PeiesSpec> {
        P10iesW::new(self, 8)
    }
}
#[doc = "Port E Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`peies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeiesSpec;
impl crate::RegisterSpec for PeiesSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`peies::R`](R) reader structure"]
impl crate::Readable for PeiesSpec {}
#[doc = "`write(|w| ..)` method takes [`peies::W`](W) writer structure"]
impl crate::Writable for PeiesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PEIES to value 0"]
impl crate::Resettable for PeiesSpec {
    const RESET_VALUE: u16 = 0;
}
