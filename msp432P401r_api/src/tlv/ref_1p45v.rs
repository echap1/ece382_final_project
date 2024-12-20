#[doc = "Register `REF_1P45V` reader"]
pub type R = crate::R<Ref1p45vSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "REF 1.45V Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_1p45v::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ref1p45vSpec;
impl crate::RegisterSpec for Ref1p45vSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_1p45v::R`](R) reader structure"]
impl crate::Readable for Ref1p45vSpec {}
#[doc = "`reset()` method sets REF_1P45V to value 0"]
impl crate::Resettable for Ref1p45vSpec {
    const RESET_VALUE: u32 = 0;
}
