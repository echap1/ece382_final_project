#[doc = "Register `BSL_PERIPHIF_SEL` reader"]
pub type R = crate::R<BslPeriphifSelSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "BSL Peripheral Interface Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_periphif_sel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BslPeriphifSelSpec;
impl crate::RegisterSpec for BslPeriphifSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsl_periphif_sel::R`](R) reader structure"]
impl crate::Readable for BslPeriphifSelSpec {}
#[doc = "`reset()` method sets BSL_PERIPHIF_SEL to value 0"]
impl crate::Resettable for BslPeriphifSelSpec {
    const RESET_VALUE: u32 = 0;
}
