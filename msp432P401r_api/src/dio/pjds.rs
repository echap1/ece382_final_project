#[doc = "Register `PJDS` reader"]
pub type R = crate::R<PjdsSpec>;
#[doc = "Register `PJDS` writer"]
pub type W = crate::W<PjdsSpec>;
#[doc = "Field `PJDS` reader - Port J Drive Strength"]
pub type PjdsR = crate::FieldReader<u16>;
#[doc = "Field `PJDS` writer - Port J Drive Strength"]
pub type PjdsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port J Drive Strength"]
    #[inline(always)]
    pub fn pjds(&self) -> PjdsR {
        PjdsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Drive Strength"]
    #[inline(always)]
    pub fn pjds(&mut self) -> PjdsW<PjdsSpec> {
        PjdsW::new(self, 0)
    }
}
#[doc = "Port J Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`pjds::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjdsSpec;
impl crate::RegisterSpec for PjdsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjds::R`](R) reader structure"]
impl crate::Readable for PjdsSpec {}
#[doc = "`write(|w| ..)` method takes [`pjds::W`](W) writer structure"]
impl crate::Writable for PjdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
