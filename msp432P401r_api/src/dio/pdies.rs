#[doc = "Register `PDIES` reader"]
pub type R = crate::R<PdiesSpec>;
#[doc = "Register `PDIES` writer"]
pub type W = crate::W<PdiesSpec>;
#[doc = "Field `P7IES` reader - Port 7 Interrupt Edge Select"]
pub type P7iesR = crate::FieldReader;
#[doc = "Field `P7IES` writer - Port 7 Interrupt Edge Select"]
pub type P7iesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P8IES` reader - Port 8 Interrupt Edge Select"]
pub type P8iesR = crate::FieldReader;
#[doc = "Field `P8IES` writer - Port 8 Interrupt Edge Select"]
pub type P8iesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p7ies(&self) -> P7iesR {
        P7iesR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p8ies(&self) -> P8iesR {
        P8iesR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p7ies(&mut self) -> P7iesW<PdiesSpec> {
        P7iesW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p8ies(&mut self) -> P8iesW<PdiesSpec> {
        P8iesW::new(self, 8)
    }
}
#[doc = "Port D Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pdies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdiesSpec;
impl crate::RegisterSpec for PdiesSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdies::R`](R) reader structure"]
impl crate::Readable for PdiesSpec {}
#[doc = "`write(|w| ..)` method takes [`pdies::W`](W) writer structure"]
impl crate::Writable for PdiesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDIES to value 0"]
impl crate::Resettable for PdiesSpec {
    const RESET_VALUE: u16 = 0;
}
