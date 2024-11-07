#[doc = "Register `RESERVED22` reader"]
pub type R = crate::R<Reserved22Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved22::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved22Spec;
impl crate::RegisterSpec for Reserved22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved22::R`](R) reader structure"]
impl crate::Readable for Reserved22Spec {}
#[doc = "`reset()` method sets RESERVED22 to value 0"]
impl crate::Resettable for Reserved22Spec {
    const RESET_VALUE: u32 = 0;
}
