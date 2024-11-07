#[doc = "Register `RESERVED12` reader"]
pub type R = crate::R<Reserved12Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved12Spec;
impl crate::RegisterSpec for Reserved12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved12::R`](R) reader structure"]
impl crate::Readable for Reserved12Spec {}
#[doc = "`reset()` method sets RESERVED12 to value 0"]
impl crate::Resettable for Reserved12Spec {
    const RESET_VALUE: u32 = 0;
}
