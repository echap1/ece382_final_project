#[doc = "Register `PCSELC` reader"]
pub type R = crate::R<PcselcSpec>;
#[doc = "Register `PCSELC` writer"]
pub type W = crate::W<PcselcSpec>;
#[doc = "Field `P5SELC` reader - Port 5 Complement Select"]
pub type P5selcR = crate::FieldReader;
#[doc = "Field `P5SELC` writer - Port 5 Complement Select"]
pub type P5selcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P6SELC` reader - Port 6 Complement Select"]
pub type P6selcR = crate::FieldReader;
#[doc = "Field `P6SELC` writer - Port 6 Complement Select"]
pub type P6selcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Complement Select"]
    #[inline(always)]
    pub fn p5selc(&self) -> P5selcR {
        P5selcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Complement Select"]
    #[inline(always)]
    pub fn p6selc(&self) -> P6selcR {
        P6selcR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Complement Select"]
    #[inline(always)]
    pub fn p5selc(&mut self) -> P5selcW<PcselcSpec> {
        P5selcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 6 Complement Select"]
    #[inline(always)]
    pub fn p6selc(&mut self) -> P6selcW<PcselcSpec> {
        P6selcW::new(self, 8)
    }
}
#[doc = "Port C Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pcselc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcselc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcselcSpec;
impl crate::RegisterSpec for PcselcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcselc::R`](R) reader structure"]
impl crate::Readable for PcselcSpec {}
#[doc = "`write(|w| ..)` method takes [`pcselc::W`](W) writer structure"]
impl crate::Writable for PcselcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PCSELC to value 0"]
impl crate::Resettable for PcselcSpec {
    const RESET_VALUE: u16 = 0;
}
