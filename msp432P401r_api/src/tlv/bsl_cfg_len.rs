#[doc = "Register `BSL_CFG_LEN` reader"]
pub type R = crate::R<BslCfgLenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "BSL Configuration Length\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_cfg_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BslCfgLenSpec;
impl crate::RegisterSpec for BslCfgLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsl_cfg_len::R`](R) reader structure"]
impl crate::Readable for BslCfgLenSpec {}
#[doc = "`reset()` method sets BSL_CFG_LEN to value 0"]
impl crate::Resettable for BslCfgLenSpec {
    const RESET_VALUE: u32 = 0;
}
