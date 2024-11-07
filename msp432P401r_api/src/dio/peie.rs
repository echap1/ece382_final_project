#[doc = "Register `PEIE` reader"]
pub type R = crate::R<PeieSpec>;
#[doc = "Register `PEIE` writer"]
pub type W = crate::W<PeieSpec>;
#[doc = "Field `P9IE` reader - Port 9 Interrupt Enable"]
pub type P9ieR = crate::FieldReader;
#[doc = "Field `P9IE` writer - Port 9 Interrupt Enable"]
pub type P9ieW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P10IE` reader - Port 10 Interrupt Enable"]
pub type P10ieR = crate::FieldReader;
#[doc = "Field `P10IE` writer - Port 10 Interrupt Enable"]
pub type P10ieW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Interrupt Enable"]
    #[inline(always)]
    pub fn p9ie(&self) -> P9ieR {
        P9ieR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Enable"]
    #[inline(always)]
    pub fn p10ie(&self) -> P10ieR {
        P10ieR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Interrupt Enable"]
    #[inline(always)]
    pub fn p9ie(&mut self) -> P9ieW<PeieSpec> {
        P9ieW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Enable"]
    #[inline(always)]
    pub fn p10ie(&mut self) -> P10ieW<PeieSpec> {
        P10ieW::new(self, 8)
    }
}
#[doc = "Port E Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`peie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeieSpec;
impl crate::RegisterSpec for PeieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`peie::R`](R) reader structure"]
impl crate::Readable for PeieSpec {}
#[doc = "`write(|w| ..)` method takes [`peie::W`](W) writer structure"]
impl crate::Writable for PeieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PEIE to value 0"]
impl crate::Resettable for PeieSpec {
    const RESET_VALUE: u16 = 0;
}
