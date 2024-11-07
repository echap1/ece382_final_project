#[doc = "Register `RESERVED18` reader"]
pub type R = crate::R<Reserved18Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved18::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved18Spec;
impl crate::RegisterSpec for Reserved18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved18::R`](R) reader structure"]
impl crate::Readable for Reserved18Spec {}
#[doc = "`reset()` method sets RESERVED18 to value 0"]
impl crate::Resettable for Reserved18Spec {
    const RESET_VALUE: u32 = 0;
}
