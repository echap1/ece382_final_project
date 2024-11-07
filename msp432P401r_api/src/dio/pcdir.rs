#[doc = "Register `PCDIR` reader"]
pub type R = crate::R<PcdirSpec>;
#[doc = "Register `PCDIR` writer"]
pub type W = crate::W<PcdirSpec>;
#[doc = "Field `P5DIR` reader - Port 5 Direction"]
pub type P5dirR = crate::FieldReader;
#[doc = "Field `P5DIR` writer - Port 5 Direction"]
pub type P5dirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P6DIR` reader - Port 6 Direction"]
pub type P6dirR = crate::FieldReader;
#[doc = "Field `P6DIR` writer - Port 6 Direction"]
pub type P6dirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Direction"]
    #[inline(always)]
    pub fn p5dir(&self) -> P5dirR {
        P5dirR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Direction"]
    #[inline(always)]
    pub fn p6dir(&self) -> P6dirR {
        P6dirR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Direction"]
    #[inline(always)]
    pub fn p5dir(&mut self) -> P5dirW<PcdirSpec> {
        P5dirW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 6 Direction"]
    #[inline(always)]
    pub fn p6dir(&mut self) -> P6dirW<PcdirSpec> {
        P6dirW::new(self, 8)
    }
}
#[doc = "Port C Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pcdir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcdir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcdirSpec;
impl crate::RegisterSpec for PcdirSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcdir::R`](R) reader structure"]
impl crate::Readable for PcdirSpec {}
#[doc = "`write(|w| ..)` method takes [`pcdir::W`](W) writer structure"]
impl crate::Writable for PcdirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PCDIR to value 0"]
impl crate::Resettable for PcdirSpec {
    const RESET_VALUE: u16 = 0;
}
