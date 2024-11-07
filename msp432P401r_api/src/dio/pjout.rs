#[doc = "Register `PJOUT` reader"]
pub type R = crate::R<PjoutSpec>;
#[doc = "Register `PJOUT` writer"]
pub type W = crate::W<PjoutSpec>;
#[doc = "Field `PJOUT` reader - Port J Output"]
pub type PjoutR = crate::FieldReader<u16>;
#[doc = "Field `PJOUT` writer - Port J Output"]
pub type PjoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port J Output"]
    #[inline(always)]
    pub fn pjout(&self) -> PjoutR {
        PjoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Output"]
    #[inline(always)]
    pub fn pjout(&mut self) -> PjoutW<PjoutSpec> {
        PjoutW::new(self, 0)
    }
}
#[doc = "Port J Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pjout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjoutSpec;
impl crate::RegisterSpec for PjoutSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjout::R`](R) reader structure"]
impl crate::Readable for PjoutSpec {}
#[doc = "`write(|w| ..)` method takes [`pjout::W`](W) writer structure"]
impl crate::Writable for PjoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PJOUT to value 0"]
impl crate::Resettable for PjoutSpec {
    const RESET_VALUE: u16 = 0;
}
