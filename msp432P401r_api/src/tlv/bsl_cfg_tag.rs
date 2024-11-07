#[doc = "Register `BSL_CFG_TAG` reader"]
pub type R = crate::R<BslCfgTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "BSL Configuration Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_cfg_tag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BslCfgTagSpec;
impl crate::RegisterSpec for BslCfgTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsl_cfg_tag::R`](R) reader structure"]
impl crate::Readable for BslCfgTagSpec {}
#[doc = "`reset()` method sets BSL_CFG_TAG to value 0x0f"]
impl crate::Resettable for BslCfgTagSpec {
    const RESET_VALUE: u32 = 0x0f;
}
