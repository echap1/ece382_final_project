#[doc = "Register `TEST_RESULTS` reader"]
pub type R = crate::R<TestResultsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Test Results\n\nYou can [`read`](crate::Reg::read) this register and get [`test_results::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestResultsSpec;
impl crate::RegisterSpec for TestResultsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test_results::R`](R) reader structure"]
impl crate::Readable for TestResultsSpec {}
#[doc = "`reset()` method sets TEST_RESULTS to value 0"]
impl crate::Resettable for TestResultsSpec {
    const RESET_VALUE: u32 = 0;
}
