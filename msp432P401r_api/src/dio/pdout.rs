#[doc = "Register `PDOUT` reader"]
pub type R = crate::R<PdoutSpec>;
#[doc = "Register `PDOUT` writer"]
pub type W = crate::W<PdoutSpec>;
#[doc = "Field `P7OUT` reader - Port 7 Output"]
pub type P7outR = crate::FieldReader;
#[doc = "Field `P7OUT` writer - Port 7 Output"]
pub type P7outW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P8OUT` reader - Port 8 Output"]
pub type P8outR = crate::FieldReader;
#[doc = "Field `P8OUT` writer - Port 8 Output"]
pub type P8outW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Output"]
    #[inline(always)]
    pub fn p7out(&self) -> P7outR {
        P7outR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Output"]
    #[inline(always)]
    pub fn p8out(&self) -> P8outR {
        P8outR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Output"]
    #[inline(always)]
    pub fn p7out(&mut self) -> P7outW<PdoutSpec> {
        P7outW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 8 Output"]
    #[inline(always)]
    pub fn p8out(&mut self) -> P8outW<PdoutSpec> {
        P8outW::new(self, 8)
    }
}
#[doc = "Port D Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pdout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdoutSpec;
impl crate::RegisterSpec for PdoutSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdout::R`](R) reader structure"]
impl crate::Readable for PdoutSpec {}
#[doc = "`write(|w| ..)` method takes [`pdout::W`](W) writer structure"]
impl crate::Writable for PdoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDOUT to value 0"]
impl crate::Resettable for PdoutSpec {
    const RESET_VALUE: u16 = 0;
}
