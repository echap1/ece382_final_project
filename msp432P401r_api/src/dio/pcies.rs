#[doc = "Register `PCIES` reader"]
pub type R = crate::R<PciesSpec>;
#[doc = "Register `PCIES` writer"]
pub type W = crate::W<PciesSpec>;
#[doc = "Field `P5IES` reader - Port 5 Interrupt Edge Select"]
pub type P5iesR = crate::FieldReader;
#[doc = "Field `P5IES` writer - Port 5 Interrupt Edge Select"]
pub type P5iesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P6IES` reader - Port 6 Interrupt Edge Select"]
pub type P6iesR = crate::FieldReader;
#[doc = "Field `P6IES` writer - Port 6 Interrupt Edge Select"]
pub type P6iesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p5ies(&self) -> P5iesR {
        P5iesR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p6ies(&self) -> P6iesR {
        P6iesR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p5ies(&mut self) -> P5iesW<PciesSpec> {
        P5iesW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p6ies(&mut self) -> P6iesW<PciesSpec> {
        P6iesW::new(self, 8)
    }
}
#[doc = "Port C Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pcies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciesSpec;
impl crate::RegisterSpec for PciesSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcies::R`](R) reader structure"]
impl crate::Readable for PciesSpec {}
#[doc = "`write(|w| ..)` method takes [`pcies::W`](W) writer structure"]
impl crate::Writable for PciesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PCIES to value 0"]
impl crate::Resettable for PciesSpec {
    const RESET_VALUE: u16 = 0;
}
