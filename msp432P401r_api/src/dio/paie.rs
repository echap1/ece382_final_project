#[doc = "Register `PAIE` reader"]
pub type R = crate::R<PaieSpec>;
#[doc = "Register `PAIE` writer"]
pub type W = crate::W<PaieSpec>;
#[doc = "Field `P1IE` reader - Port 1 Interrupt Enable"]
pub type P1ieR = crate::FieldReader;
#[doc = "Field `P1IE` writer - Port 1 Interrupt Enable"]
pub type P1ieW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P2IE` reader - Port 2 Interrupt Enable"]
pub type P2ieR = crate::FieldReader;
#[doc = "Field `P2IE` writer - Port 2 Interrupt Enable"]
pub type P2ieW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Interrupt Enable"]
    #[inline(always)]
    pub fn p1ie(&self) -> P1ieR {
        P1ieR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Enable"]
    #[inline(always)]
    pub fn p2ie(&self) -> P2ieR {
        P2ieR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Interrupt Enable"]
    #[inline(always)]
    pub fn p1ie(&mut self) -> P1ieW<PaieSpec> {
        P1ieW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Enable"]
    #[inline(always)]
    pub fn p2ie(&mut self) -> P2ieW<PaieSpec> {
        P2ieW::new(self, 8)
    }
}
#[doc = "Port A Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`paie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaieSpec;
impl crate::RegisterSpec for PaieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`paie::R`](R) reader structure"]
impl crate::Readable for PaieSpec {}
#[doc = "`write(|w| ..)` method takes [`paie::W`](W) writer structure"]
impl crate::Writable for PaieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PAIE to value 0"]
impl crate::Resettable for PaieSpec {
    const RESET_VALUE: u16 = 0;
}
