#[doc = "Register `RESERVED16` reader"]
pub type R = crate::R<Reserved16Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved16Spec;
impl crate::RegisterSpec for Reserved16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved16::R`](R) reader structure"]
impl crate::Readable for Reserved16Spec {}
#[doc = "`reset()` method sets RESERVED16 to value 0"]
impl crate::Resettable for Reserved16Spec {
    const RESET_VALUE: u32 = 0;
}
