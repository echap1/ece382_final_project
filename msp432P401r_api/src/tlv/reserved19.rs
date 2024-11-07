#[doc = "Register `RESERVED19` reader"]
pub type R = crate::R<Reserved19Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved19::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved19Spec;
impl crate::RegisterSpec for Reserved19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved19::R`](R) reader structure"]
impl crate::Readable for Reserved19Spec {}
#[doc = "`reset()` method sets RESERVED19 to value 0"]
impl crate::Resettable for Reserved19Spec {
    const RESET_VALUE: u32 = 0;
}
