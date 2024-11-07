#[doc = "Register `PASELC` reader"]
pub type R = crate::R<PaselcSpec>;
#[doc = "Register `PASELC` writer"]
pub type W = crate::W<PaselcSpec>;
#[doc = "Field `P1SELC` reader - Port 1 Complement Select"]
pub type P1selcR = crate::FieldReader;
#[doc = "Field `P1SELC` writer - Port 1 Complement Select"]
pub type P1selcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P2SELC` reader - Port 2 Complement Select"]
pub type P2selcR = crate::FieldReader;
#[doc = "Field `P2SELC` writer - Port 2 Complement Select"]
pub type P2selcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Complement Select"]
    #[inline(always)]
    pub fn p1selc(&self) -> P1selcR {
        P1selcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Complement Select"]
    #[inline(always)]
    pub fn p2selc(&self) -> P2selcR {
        P2selcR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Complement Select"]
    #[inline(always)]
    pub fn p1selc(&mut self) -> P1selcW<PaselcSpec> {
        P1selcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 2 Complement Select"]
    #[inline(always)]
    pub fn p2selc(&mut self) -> P2selcW<PaselcSpec> {
        P2selcW::new(self, 8)
    }
}
#[doc = "Port A Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`paselc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paselc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaselcSpec;
impl crate::RegisterSpec for PaselcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`paselc::R`](R) reader structure"]
impl crate::Readable for PaselcSpec {}
#[doc = "`write(|w| ..)` method takes [`paselc::W`](W) writer structure"]
impl crate::Writable for PaselcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PASELC to value 0"]
impl crate::Resettable for PaselcSpec {
    const RESET_VALUE: u16 = 0;
}
