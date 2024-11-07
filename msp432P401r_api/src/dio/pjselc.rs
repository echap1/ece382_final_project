#[doc = "Register `PJSELC` reader"]
pub type R = crate::R<PjselcSpec>;
#[doc = "Register `PJSELC` writer"]
pub type W = crate::W<PjselcSpec>;
#[doc = "Field `PJSELC` reader - Port J Complement Select"]
pub type PjselcR = crate::FieldReader<u16>;
#[doc = "Field `PJSELC` writer - Port J Complement Select"]
pub type PjselcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port J Complement Select"]
    #[inline(always)]
    pub fn pjselc(&self) -> PjselcR {
        PjselcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Complement Select"]
    #[inline(always)]
    pub fn pjselc(&mut self) -> PjselcW<PjselcSpec> {
        PjselcW::new(self, 0)
    }
}
#[doc = "Port J Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pjselc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjselc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjselcSpec;
impl crate::RegisterSpec for PjselcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjselc::R`](R) reader structure"]
impl crate::Readable for PjselcSpec {}
#[doc = "`write(|w| ..)` method takes [`pjselc::W`](W) writer structure"]
impl crate::Writable for PjselcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PJSELC to value 0"]
impl crate::Resettable for PjselcSpec {
    const RESET_VALUE: u16 = 0;
}
