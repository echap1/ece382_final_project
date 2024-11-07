#[doc = "Register `RESERVED3` reader"]
pub type R = crate::R<Reserved3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved3Spec;
impl crate::RegisterSpec for Reserved3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved3::R`](R) reader structure"]
impl crate::Readable for Reserved3Spec {}
#[doc = "`reset()` method sets RESERVED3 to value 0"]
impl crate::Resettable for Reserved3Spec {
    const RESET_VALUE: u32 = 0;
}
