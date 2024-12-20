#[doc = "Register `RESERVED15` reader"]
pub type R = crate::R<Reserved15Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved15Spec;
impl crate::RegisterSpec for Reserved15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved15::R`](R) reader structure"]
impl crate::Readable for Reserved15Spec {}
#[doc = "`reset()` method sets RESERVED15 to value 0"]
impl crate::Resettable for Reserved15Spec {
    const RESET_VALUE: u32 = 0;
}
