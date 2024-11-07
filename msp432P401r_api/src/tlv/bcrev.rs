#[doc = "Register `BCREV` reader"]
pub type R = crate::R<BcrevSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Boot Code Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`bcrev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcrevSpec;
impl crate::RegisterSpec for BcrevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcrev::R`](R) reader structure"]
impl crate::Readable for BcrevSpec {}
#[doc = "`reset()` method sets BCREV to value 0"]
impl crate::Resettable for BcrevSpec {
    const RESET_VALUE: u32 = 0;
}
