#[doc = "Register `PJDIR` reader"]
pub type R = crate::R<PjdirSpec>;
#[doc = "Register `PJDIR` writer"]
pub type W = crate::W<PjdirSpec>;
#[doc = "Field `PJDIR` reader - Port J Direction"]
pub type PjdirR = crate::FieldReader<u16>;
#[doc = "Field `PJDIR` writer - Port J Direction"]
pub type PjdirW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port J Direction"]
    #[inline(always)]
    pub fn pjdir(&self) -> PjdirR {
        PjdirR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Direction"]
    #[inline(always)]
    pub fn pjdir(&mut self) -> PjdirW<PjdirSpec> {
        PjdirW::new(self, 0)
    }
}
#[doc = "Port J Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pjdir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjdir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjdirSpec;
impl crate::RegisterSpec for PjdirSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjdir::R`](R) reader structure"]
impl crate::Readable for PjdirSpec {}
#[doc = "`write(|w| ..)` method takes [`pjdir::W`](W) writer structure"]
impl crate::Writable for PjdirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PJDIR to value 0"]
impl crate::Resettable for PjdirSpec {
    const RESET_VALUE: u16 = 0;
}
