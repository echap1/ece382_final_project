#[doc = "Register `RANDOM_NUM_LEN` reader"]
pub type R = crate::R<RandomNumLenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "128-bit Random Number Length\n\nYou can [`read`](crate::Reg::read) this register and get [`random_num_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RandomNumLenSpec;
impl crate::RegisterSpec for RandomNumLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`random_num_len::R`](R) reader structure"]
impl crate::Readable for RandomNumLenSpec {}
#[doc = "`reset()` method sets RANDOM_NUM_LEN to value 0"]
impl crate::Resettable for RandomNumLenSpec {
    const RESET_VALUE: u32 = 0;
}
