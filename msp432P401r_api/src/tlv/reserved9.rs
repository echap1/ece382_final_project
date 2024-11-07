#[doc = "Register `RESERVED9` reader"]
pub type R = crate::R<Reserved9Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved9Spec;
impl crate::RegisterSpec for Reserved9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved9::R`](R) reader structure"]
impl crate::Readable for Reserved9Spec {}
#[doc = "`reset()` method sets RESERVED9 to value 0"]
impl crate::Resettable for Reserved9Spec {
    const RESET_VALUE: u32 = 0;
}
