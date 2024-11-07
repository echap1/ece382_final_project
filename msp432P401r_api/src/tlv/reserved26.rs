#[doc = "Register `RESERVED26` reader"]
pub type R = crate::R<Reserved26Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved26::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved26Spec;
impl crate::RegisterSpec for Reserved26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved26::R`](R) reader structure"]
impl crate::Readable for Reserved26Spec {}
#[doc = "`reset()` method sets RESERVED26 to value 0"]
impl crate::Resettable for Reserved26Spec {
    const RESET_VALUE: u32 = 0;
}
