#[doc = "Register `PDIE` reader"]
pub type R = crate::R<PdieSpec>;
#[doc = "Register `PDIE` writer"]
pub type W = crate::W<PdieSpec>;
#[doc = "Field `P7IE` reader - Port 7 Interrupt Enable"]
pub type P7ieR = crate::FieldReader;
#[doc = "Field `P7IE` writer - Port 7 Interrupt Enable"]
pub type P7ieW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P8IE` reader - Port 8 Interrupt Enable"]
pub type P8ieR = crate::FieldReader;
#[doc = "Field `P8IE` writer - Port 8 Interrupt Enable"]
pub type P8ieW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Interrupt Enable"]
    #[inline(always)]
    pub fn p7ie(&self) -> P7ieR {
        P7ieR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Enable"]
    #[inline(always)]
    pub fn p8ie(&self) -> P8ieR {
        P8ieR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Interrupt Enable"]
    #[inline(always)]
    pub fn p7ie(&mut self) -> P7ieW<PdieSpec> {
        P7ieW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Enable"]
    #[inline(always)]
    pub fn p8ie(&mut self) -> P8ieW<PdieSpec> {
        P8ieW::new(self, 8)
    }
}
#[doc = "Port D Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pdie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdieSpec;
impl crate::RegisterSpec for PdieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdie::R`](R) reader structure"]
impl crate::Readable for PdieSpec {}
#[doc = "`write(|w| ..)` method takes [`pdie::W`](W) writer structure"]
impl crate::Writable for PdieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDIE to value 0"]
impl crate::Resettable for PdieSpec {
    const RESET_VALUE: u16 = 0;
}
