#[doc = "Register `RESERVED4` reader"]
pub type R = crate::R<Reserved4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved4Spec;
impl crate::RegisterSpec for Reserved4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved4::R`](R) reader structure"]
impl crate::Readable for Reserved4Spec {}
#[doc = "`reset()` method sets RESERVED4 to value 0"]
impl crate::Resettable for Reserved4Spec {
    const RESET_VALUE: u32 = 0;
}
