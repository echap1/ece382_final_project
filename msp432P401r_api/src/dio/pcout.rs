#[doc = "Register `PCOUT` reader"]
pub type R = crate::R<PcoutSpec>;
#[doc = "Register `PCOUT` writer"]
pub type W = crate::W<PcoutSpec>;
#[doc = "Field `P5OUT` reader - Port 5 Output"]
pub type P5outR = crate::FieldReader;
#[doc = "Field `P5OUT` writer - Port 5 Output"]
pub type P5outW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P6OUT` reader - Port 6 Output"]
pub type P6outR = crate::FieldReader;
#[doc = "Field `P6OUT` writer - Port 6 Output"]
pub type P6outW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Output"]
    #[inline(always)]
    pub fn p5out(&self) -> P5outR {
        P5outR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Output"]
    #[inline(always)]
    pub fn p6out(&self) -> P6outR {
        P6outR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Output"]
    #[inline(always)]
    pub fn p5out(&mut self) -> P5outW<PcoutSpec> {
        P5outW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 6 Output"]
    #[inline(always)]
    pub fn p6out(&mut self) -> P6outW<PcoutSpec> {
        P6outW::new(self, 8)
    }
}
#[doc = "Port C Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pcout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcoutSpec;
impl crate::RegisterSpec for PcoutSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcout::R`](R) reader structure"]
impl crate::Readable for PcoutSpec {}
#[doc = "`write(|w| ..)` method takes [`pcout::W`](W) writer structure"]
impl crate::Writable for PcoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PCOUT to value 0"]
impl crate::Resettable for PcoutSpec {
    const RESET_VALUE: u16 = 0;
}
