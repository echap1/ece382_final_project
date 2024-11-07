#[doc = "Register `PDSELC` reader"]
pub type R = crate::R<PdselcSpec>;
#[doc = "Register `PDSELC` writer"]
pub type W = crate::W<PdselcSpec>;
#[doc = "Field `P7SELC` reader - Port 7 Complement Select"]
pub type P7selcR = crate::FieldReader;
#[doc = "Field `P7SELC` writer - Port 7 Complement Select"]
pub type P7selcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P8SELC` reader - Port 8 Complement Select"]
pub type P8selcR = crate::FieldReader;
#[doc = "Field `P8SELC` writer - Port 8 Complement Select"]
pub type P8selcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Complement Select"]
    #[inline(always)]
    pub fn p7selc(&self) -> P7selcR {
        P7selcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Complement Select"]
    #[inline(always)]
    pub fn p8selc(&self) -> P8selcR {
        P8selcR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Complement Select"]
    #[inline(always)]
    pub fn p7selc(&mut self) -> P7selcW<PdselcSpec> {
        P7selcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 8 Complement Select"]
    #[inline(always)]
    pub fn p8selc(&mut self) -> P8selcW<PdselcSpec> {
        P8selcW::new(self, 8)
    }
}
#[doc = "Port D Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pdselc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdselc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdselcSpec;
impl crate::RegisterSpec for PdselcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdselc::R`](R) reader structure"]
impl crate::Readable for PdselcSpec {}
#[doc = "`write(|w| ..)` method takes [`pdselc::W`](W) writer structure"]
impl crate::Writable for PdselcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDSELC to value 0"]
impl crate::Resettable for PdselcSpec {
    const RESET_VALUE: u16 = 0;
}
