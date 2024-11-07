#[doc = "Register `PDDIR` reader"]
pub type R = crate::R<PddirSpec>;
#[doc = "Register `PDDIR` writer"]
pub type W = crate::W<PddirSpec>;
#[doc = "Field `P7DIR` reader - Port 7 Direction"]
pub type P7dirR = crate::FieldReader;
#[doc = "Field `P7DIR` writer - Port 7 Direction"]
pub type P7dirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P8DIR` reader - Port 8 Direction"]
pub type P8dirR = crate::FieldReader;
#[doc = "Field `P8DIR` writer - Port 8 Direction"]
pub type P8dirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Direction"]
    #[inline(always)]
    pub fn p7dir(&self) -> P7dirR {
        P7dirR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Direction"]
    #[inline(always)]
    pub fn p8dir(&self) -> P8dirR {
        P8dirR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Direction"]
    #[inline(always)]
    pub fn p7dir(&mut self) -> P7dirW<PddirSpec> {
        P7dirW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 8 Direction"]
    #[inline(always)]
    pub fn p8dir(&mut self) -> P8dirW<PddirSpec> {
        P8dirW::new(self, 8)
    }
}
#[doc = "Port D Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pddir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pddir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PddirSpec;
impl crate::RegisterSpec for PddirSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pddir::R`](R) reader structure"]
impl crate::Readable for PddirSpec {}
#[doc = "`write(|w| ..)` method takes [`pddir::W`](W) writer structure"]
impl crate::Writable for PddirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDDIR to value 0"]
impl crate::Resettable for PddirSpec {
    const RESET_VALUE: u16 = 0;
}
