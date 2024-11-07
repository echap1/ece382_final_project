#[doc = "Register `RESERVED25` reader"]
pub type R = crate::R<Reserved25Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved25::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved25Spec;
impl crate::RegisterSpec for Reserved25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved25::R`](R) reader structure"]
impl crate::Readable for Reserved25Spec {}
#[doc = "`reset()` method sets RESERVED25 to value 0"]
impl crate::Resettable for Reserved25Spec {
    const RESET_VALUE: u32 = 0;
}
