#[doc = "Register `RESERVED11` reader"]
pub type R = crate::R<Reserved11Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved11Spec;
impl crate::RegisterSpec for Reserved11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved11::R`](R) reader structure"]
impl crate::Readable for Reserved11Spec {}
#[doc = "`reset()` method sets RESERVED11 to value 0"]
impl crate::Resettable for Reserved11Spec {
    const RESET_VALUE: u32 = 0;
}
