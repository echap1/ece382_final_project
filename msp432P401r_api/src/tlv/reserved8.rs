#[doc = "Register `RESERVED8` reader"]
pub type R = crate::R<Reserved8Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved8Spec;
impl crate::RegisterSpec for Reserved8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved8::R`](R) reader structure"]
impl crate::Readable for Reserved8Spec {}
#[doc = "`reset()` method sets RESERVED8 to value 0"]
impl crate::Resettable for Reserved8Spec {
    const RESET_VALUE: u32 = 0;
}
