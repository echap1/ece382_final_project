#[doc = "Register `RESERVED10` reader"]
pub type R = crate::R<Reserved10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved10Spec;
impl crate::RegisterSpec for Reserved10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved10::R`](R) reader structure"]
impl crate::Readable for Reserved10Spec {}
#[doc = "`reset()` method sets RESERVED10 to value 0"]
impl crate::Resettable for Reserved10Spec {
    const RESET_VALUE: u32 = 0;
}
