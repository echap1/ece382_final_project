#[doc = "Register `PAOUT` reader"]
pub type R = crate::R<PaoutSpec>;
#[doc = "Register `PAOUT` writer"]
pub type W = crate::W<PaoutSpec>;
#[doc = "Field `P1OUT` reader - Port 1 Output"]
pub type P1outR = crate::FieldReader;
#[doc = "Field `P1OUT` writer - Port 1 Output"]
pub type P1outW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P2OUT` reader - Port 2 Output"]
pub type P2outR = crate::FieldReader;
#[doc = "Field `P2OUT` writer - Port 2 Output"]
pub type P2outW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Output"]
    #[inline(always)]
    pub fn p1out(&self) -> P1outR {
        P1outR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Output"]
    #[inline(always)]
    pub fn p2out(&self) -> P2outR {
        P2outR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Output"]
    #[inline(always)]
    pub fn p1out(&mut self) -> P1outW<PaoutSpec> {
        P1outW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 2 Output"]
    #[inline(always)]
    pub fn p2out(&mut self) -> P2outW<PaoutSpec> {
        P2outW::new(self, 8)
    }
}
#[doc = "Port A Output\n\nYou can [`read`](crate::Reg::read) this register and get [`paout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaoutSpec;
impl crate::RegisterSpec for PaoutSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`paout::R`](R) reader structure"]
impl crate::Readable for PaoutSpec {}
#[doc = "`write(|w| ..)` method takes [`paout::W`](W) writer structure"]
impl crate::Writable for PaoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PAOUT to value 0"]
impl crate::Resettable for PaoutSpec {
    const RESET_VALUE: u16 = 0;
}
