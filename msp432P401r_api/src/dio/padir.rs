#[doc = "Register `PADIR` reader"]
pub type R = crate::R<PadirSpec>;
#[doc = "Register `PADIR` writer"]
pub type W = crate::W<PadirSpec>;
#[doc = "Field `P1DIR` reader - Port 1 Direction"]
pub type P1dirR = crate::FieldReader;
#[doc = "Field `P1DIR` writer - Port 1 Direction"]
pub type P1dirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P2DIR` reader - Port 2 Direction"]
pub type P2dirR = crate::FieldReader;
#[doc = "Field `P2DIR` writer - Port 2 Direction"]
pub type P2dirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Direction"]
    #[inline(always)]
    pub fn p1dir(&self) -> P1dirR {
        P1dirR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Direction"]
    #[inline(always)]
    pub fn p2dir(&self) -> P2dirR {
        P2dirR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Direction"]
    #[inline(always)]
    pub fn p1dir(&mut self) -> P1dirW<PadirSpec> {
        P1dirW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 2 Direction"]
    #[inline(always)]
    pub fn p2dir(&mut self) -> P2dirW<PadirSpec> {
        P2dirW::new(self, 8)
    }
}
#[doc = "Port A Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`padir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadirSpec;
impl crate::RegisterSpec for PadirSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`padir::R`](R) reader structure"]
impl crate::Readable for PadirSpec {}
#[doc = "`write(|w| ..)` method takes [`padir::W`](W) writer structure"]
impl crate::Writable for PadirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PADIR to value 0"]
impl crate::Resettable for PadirSpec {
    const RESET_VALUE: u16 = 0;
}
