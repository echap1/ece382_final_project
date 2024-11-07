#[doc = "Register `PESELC` reader"]
pub type R = crate::R<PeselcSpec>;
#[doc = "Register `PESELC` writer"]
pub type W = crate::W<PeselcSpec>;
#[doc = "Field `P9SELC` reader - Port 9 Complement Select"]
pub type P9selcR = crate::FieldReader;
#[doc = "Field `P9SELC` writer - Port 9 Complement Select"]
pub type P9selcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P10SELC` reader - Port 10 Complement Select"]
pub type P10selcR = crate::FieldReader;
#[doc = "Field `P10SELC` writer - Port 10 Complement Select"]
pub type P10selcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Complement Select"]
    #[inline(always)]
    pub fn p9selc(&self) -> P9selcR {
        P9selcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Complement Select"]
    #[inline(always)]
    pub fn p10selc(&self) -> P10selcR {
        P10selcR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Complement Select"]
    #[inline(always)]
    pub fn p9selc(&mut self) -> P9selcW<PeselcSpec> {
        P9selcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 10 Complement Select"]
    #[inline(always)]
    pub fn p10selc(&mut self) -> P10selcW<PeselcSpec> {
        P10selcW::new(self, 8)
    }
}
#[doc = "Port E Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`peselc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peselc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeselcSpec;
impl crate::RegisterSpec for PeselcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`peselc::R`](R) reader structure"]
impl crate::Readable for PeselcSpec {}
#[doc = "`write(|w| ..)` method takes [`peselc::W`](W) writer structure"]
impl crate::Writable for PeselcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PESELC to value 0"]
impl crate::Resettable for PeselcSpec {
    const RESET_VALUE: u16 = 0;
}
