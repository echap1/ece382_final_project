#[doc = "Register `PJIN` reader"]
pub type R = crate::R<PjinSpec>;
#[doc = "Field `PJIN` reader - Port J Input"]
pub type PjinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Port J Input"]
    #[inline(always)]
    pub fn pjin(&self) -> PjinR {
        PjinR::new(self.bits)
    }
}
#[doc = "Port J Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pjin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjinSpec;
impl crate::RegisterSpec for PjinSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjin::R`](R) reader structure"]
impl crate::Readable for PjinSpec {}
#[doc = "`reset()` method sets PJIN to value 0"]
impl crate::Resettable for PjinSpec {
    const RESET_VALUE: u16 = 0;
}
