#[doc = "Register `RESERVED7` reader"]
pub type R = crate::R<Reserved7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved7Spec;
impl crate::RegisterSpec for Reserved7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved7::R`](R) reader structure"]
impl crate::Readable for Reserved7Spec {}
#[doc = "`reset()` method sets RESERVED7 to value 0"]
impl crate::Resettable for Reserved7Spec {
    const RESET_VALUE: u32 = 0;
}
