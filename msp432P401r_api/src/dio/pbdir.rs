#[doc = "Register `PBDIR` reader"]
pub type R = crate::R<PbdirSpec>;
#[doc = "Register `PBDIR` writer"]
pub type W = crate::W<PbdirSpec>;
#[doc = "Field `P3DIR` reader - Port 3 Direction"]
pub type P3dirR = crate::FieldReader;
#[doc = "Field `P3DIR` writer - Port 3 Direction"]
pub type P3dirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P4DIR` reader - Port 4 Direction"]
pub type P4dirR = crate::FieldReader;
#[doc = "Field `P4DIR` writer - Port 4 Direction"]
pub type P4dirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Direction"]
    #[inline(always)]
    pub fn p3dir(&self) -> P3dirR {
        P3dirR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Direction"]
    #[inline(always)]
    pub fn p4dir(&self) -> P4dirR {
        P4dirR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Direction"]
    #[inline(always)]
    pub fn p3dir(&mut self) -> P3dirW<PbdirSpec> {
        P3dirW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 4 Direction"]
    #[inline(always)]
    pub fn p4dir(&mut self) -> P4dirW<PbdirSpec> {
        P4dirW::new(self, 8)
    }
}
#[doc = "Port B Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pbdir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbdir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbdirSpec;
impl crate::RegisterSpec for PbdirSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbdir::R`](R) reader structure"]
impl crate::Readable for PbdirSpec {}
#[doc = "`write(|w| ..)` method takes [`pbdir::W`](W) writer structure"]
impl crate::Writable for PbdirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBDIR to value 0"]
impl crate::Resettable for PbdirSpec {
    const RESET_VALUE: u16 = 0;
}
