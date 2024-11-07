#[doc = "Register `RANDOM_NUM_3` reader"]
pub type R = crate::R<RandomNum3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "32-bit Random Number 3\n\nYou can [`read`](crate::Reg::read) this register and get [`random_num_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RandomNum3Spec;
impl crate::RegisterSpec for RandomNum3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`random_num_3::R`](R) reader structure"]
impl crate::Readable for RandomNum3Spec {}
#[doc = "`reset()` method sets RANDOM_NUM_3 to value 0"]
impl crate::Resettable for RandomNum3Spec {
    const RESET_VALUE: u32 = 0;
}
