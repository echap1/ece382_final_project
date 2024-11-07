#[doc = "Register `RESERVED23` reader"]
pub type R = crate::R<Reserved23Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved23::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved23Spec;
impl crate::RegisterSpec for Reserved23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved23::R`](R) reader structure"]
impl crate::Readable for Reserved23Spec {}
#[doc = "`reset()` method sets RESERVED23 to value 0"]
impl crate::Resettable for Reserved23Spec {
    const RESET_VALUE: u32 = 0;
}
