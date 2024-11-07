#[doc = "Register `RESERVED6` reader"]
pub type R = crate::R<Reserved6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved6Spec;
impl crate::RegisterSpec for Reserved6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved6::R`](R) reader structure"]
impl crate::Readable for Reserved6Spec {}
#[doc = "`reset()` method sets RESERVED6 to value 0"]
impl crate::Resettable for Reserved6Spec {
    const RESET_VALUE: u32 = 0;
}
