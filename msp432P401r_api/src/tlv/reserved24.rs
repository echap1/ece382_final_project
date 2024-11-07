#[doc = "Register `RESERVED24` reader"]
pub type R = crate::R<Reserved24Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved24::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved24Spec;
impl crate::RegisterSpec for Reserved24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved24::R`](R) reader structure"]
impl crate::Readable for Reserved24Spec {}
#[doc = "`reset()` method sets RESERVED24 to value 0"]
impl crate::Resettable for Reserved24Spec {
    const RESET_VALUE: u32 = 0;
}
