#[doc = "Register `RESERVED5` reader"]
pub type R = crate::R<Reserved5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved5Spec;
impl crate::RegisterSpec for Reserved5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved5::R`](R) reader structure"]
impl crate::Readable for Reserved5Spec {}
#[doc = "`reset()` method sets RESERVED5 to value 0"]
impl crate::Resettable for Reserved5Spec {
    const RESET_VALUE: u32 = 0;
}
