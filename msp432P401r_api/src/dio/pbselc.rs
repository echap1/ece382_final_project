#[doc = "Register `PBSELC` reader"]
pub type R = crate::R<PbselcSpec>;
#[doc = "Register `PBSELC` writer"]
pub type W = crate::W<PbselcSpec>;
#[doc = "Field `P3SELC` reader - Port 3 Complement Select"]
pub type P3selcR = crate::FieldReader;
#[doc = "Field `P3SELC` writer - Port 3 Complement Select"]
pub type P3selcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P4SELC` reader - Port 4 Complement Select"]
pub type P4selcR = crate::FieldReader;
#[doc = "Field `P4SELC` writer - Port 4 Complement Select"]
pub type P4selcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Complement Select"]
    #[inline(always)]
    pub fn p3selc(&self) -> P3selcR {
        P3selcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Complement Select"]
    #[inline(always)]
    pub fn p4selc(&self) -> P4selcR {
        P4selcR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Complement Select"]
    #[inline(always)]
    pub fn p3selc(&mut self) -> P3selcW<PbselcSpec> {
        P3selcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 4 Complement Select"]
    #[inline(always)]
    pub fn p4selc(&mut self) -> P4selcW<PbselcSpec> {
        P4selcW::new(self, 8)
    }
}
#[doc = "Port B Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pbselc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbselc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbselcSpec;
impl crate::RegisterSpec for PbselcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbselc::R`](R) reader structure"]
impl crate::Readable for PbselcSpec {}
#[doc = "`write(|w| ..)` method takes [`pbselc::W`](W) writer structure"]
impl crate::Writable for PbselcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBSELC to value 0"]
impl crate::Resettable for PbselcSpec {
    const RESET_VALUE: u16 = 0;
}
