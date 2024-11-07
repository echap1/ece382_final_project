#[doc = "Register `PEDIR` reader"]
pub type R = crate::R<PedirSpec>;
#[doc = "Register `PEDIR` writer"]
pub type W = crate::W<PedirSpec>;
#[doc = "Field `P9DIR` reader - Port 9 Direction"]
pub type P9dirR = crate::FieldReader;
#[doc = "Field `P9DIR` writer - Port 9 Direction"]
pub type P9dirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P10DIR` reader - Port 10 Direction"]
pub type P10dirR = crate::FieldReader;
#[doc = "Field `P10DIR` writer - Port 10 Direction"]
pub type P10dirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Direction"]
    #[inline(always)]
    pub fn p9dir(&self) -> P9dirR {
        P9dirR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Direction"]
    #[inline(always)]
    pub fn p10dir(&self) -> P10dirR {
        P10dirR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Direction"]
    #[inline(always)]
    pub fn p9dir(&mut self) -> P9dirW<PedirSpec> {
        P9dirW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 10 Direction"]
    #[inline(always)]
    pub fn p10dir(&mut self) -> P10dirW<PedirSpec> {
        P10dirW::new(self, 8)
    }
}
#[doc = "Port E Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pedir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pedir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PedirSpec;
impl crate::RegisterSpec for PedirSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pedir::R`](R) reader structure"]
impl crate::Readable for PedirSpec {}
#[doc = "`write(|w| ..)` method takes [`pedir::W`](W) writer structure"]
impl crate::Writable for PedirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PEDIR to value 0"]
impl crate::Resettable for PedirSpec {
    const RESET_VALUE: u16 = 0;
}
