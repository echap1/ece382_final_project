#[doc = "Register `RANDOM_NUM_TAG` reader"]
pub type R = crate::R<RandomNumTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "128-bit Random Number Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`random_num_tag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RandomNumTagSpec;
impl crate::RegisterSpec for RandomNumTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`random_num_tag::R`](R) reader structure"]
impl crate::Readable for RandomNumTagSpec {}
#[doc = "`reset()` method sets RANDOM_NUM_TAG to value 0x0d"]
impl crate::Resettable for RandomNumTagSpec {
    const RESET_VALUE: u32 = 0x0d;
}
