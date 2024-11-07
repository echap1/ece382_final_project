#[doc = "Register `PEOUT` reader"]
pub type R = crate::R<PeoutSpec>;
#[doc = "Register `PEOUT` writer"]
pub type W = crate::W<PeoutSpec>;
#[doc = "Field `P9OUT` reader - Port 9 Output"]
pub type P9outR = crate::FieldReader;
#[doc = "Field `P9OUT` writer - Port 9 Output"]
pub type P9outW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P10OUT` reader - Port 10 Output"]
pub type P10outR = crate::FieldReader;
#[doc = "Field `P10OUT` writer - Port 10 Output"]
pub type P10outW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Output"]
    #[inline(always)]
    pub fn p9out(&self) -> P9outR {
        P9outR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Output"]
    #[inline(always)]
    pub fn p10out(&self) -> P10outR {
        P10outR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Output"]
    #[inline(always)]
    pub fn p9out(&mut self) -> P9outW<PeoutSpec> {
        P9outW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 10 Output"]
    #[inline(always)]
    pub fn p10out(&mut self) -> P10outW<PeoutSpec> {
        P10outW::new(self, 8)
    }
}
#[doc = "Port E Output\n\nYou can [`read`](crate::Reg::read) this register and get [`peout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeoutSpec;
impl crate::RegisterSpec for PeoutSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`peout::R`](R) reader structure"]
impl crate::Readable for PeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`peout::W`](W) writer structure"]
impl crate::Writable for PeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PEOUT to value 0"]
impl crate::Resettable for PeoutSpec {
    const RESET_VALUE: u16 = 0;
}
