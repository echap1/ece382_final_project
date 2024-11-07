#[doc = "Register `RANDOM_NUM_4` reader"]
pub type R = crate::R<RandomNum4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "32-bit Random Number 4\n\nYou can [`read`](crate::Reg::read) this register and get [`random_num_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RandomNum4Spec;
impl crate::RegisterSpec for RandomNum4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`random_num_4::R`](R) reader structure"]
impl crate::Readable for RandomNum4Spec {}
#[doc = "`reset()` method sets RANDOM_NUM_4 to value 0"]
impl crate::Resettable for RandomNum4Spec {
    const RESET_VALUE: u32 = 0;
}
