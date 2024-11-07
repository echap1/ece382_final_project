#[doc = "Register `RESERVED21` reader"]
pub type R = crate::R<Reserved21Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved21::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved21Spec;
impl crate::RegisterSpec for Reserved21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved21::R`](R) reader structure"]
impl crate::Readable for Reserved21Spec {}
#[doc = "`reset()` method sets RESERVED21 to value 0"]
impl crate::Resettable for Reserved21Spec {
    const RESET_VALUE: u32 = 0;
}
