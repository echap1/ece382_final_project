#[doc = "Register `RESERVED17` reader"]
pub type R = crate::R<Reserved17Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved17::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved17Spec;
impl crate::RegisterSpec for Reserved17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved17::R`](R) reader structure"]
impl crate::Readable for Reserved17Spec {}
#[doc = "`reset()` method sets RESERVED17 to value 0"]
impl crate::Resettable for Reserved17Spec {
    const RESET_VALUE: u32 = 0;
}
