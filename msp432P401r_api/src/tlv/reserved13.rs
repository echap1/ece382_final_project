#[doc = "Register `RESERVED13` reader"]
pub type R = crate::R<Reserved13Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved13Spec;
impl crate::RegisterSpec for Reserved13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved13::R`](R) reader structure"]
impl crate::Readable for Reserved13Spec {}
#[doc = "`reset()` method sets RESERVED13 to value 0"]
impl crate::Resettable for Reserved13Spec {
    const RESET_VALUE: u32 = 0;
}
