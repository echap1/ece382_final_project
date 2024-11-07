#[doc = "Register `RESERVED2` reader"]
pub type R = crate::R<Reserved2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved2Spec;
impl crate::RegisterSpec for Reserved2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved2::R`](R) reader structure"]
impl crate::Readable for Reserved2Spec {}
#[doc = "`reset()` method sets RESERVED2 to value 0"]
impl crate::Resettable for Reserved2Spec {
    const RESET_VALUE: u32 = 0;
}
