#[doc = "Register `REF_2P5V` reader"]
pub type R = crate::R<Ref2p5vSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "REF 2.5V Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_2p5v::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ref2p5vSpec;
impl crate::RegisterSpec for Ref2p5vSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_2p5v::R`](R) reader structure"]
impl crate::Readable for Ref2p5vSpec {}
#[doc = "`reset()` method sets REF_2P5V to value 0"]
impl crate::Resettable for Ref2p5vSpec {
    const RESET_VALUE: u32 = 0;
}
