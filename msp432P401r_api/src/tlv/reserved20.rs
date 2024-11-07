#[doc = "Register `RESERVED20` reader"]
pub type R = crate::R<Reserved20Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved20::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved20Spec;
impl crate::RegisterSpec for Reserved20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved20::R`](R) reader structure"]
impl crate::Readable for Reserved20Spec {}
#[doc = "`reset()` method sets RESERVED20 to value 0"]
impl crate::Resettable for Reserved20Spec {
    const RESET_VALUE: u32 = 0;
}
