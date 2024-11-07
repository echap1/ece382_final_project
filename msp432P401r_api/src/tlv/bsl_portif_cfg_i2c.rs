#[doc = "Register `BSL_PORTIF_CFG_I2C` reader"]
pub type R = crate::R<BslPortifCfgI2cSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "BSL Port Interface Configuration for I2C\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_portif_cfg_i2c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BslPortifCfgI2cSpec;
impl crate::RegisterSpec for BslPortifCfgI2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsl_portif_cfg_i2c::R`](R) reader structure"]
impl crate::Readable for BslPortifCfgI2cSpec {}
#[doc = "`reset()` method sets BSL_PORTIF_CFG_I2C to value 0"]
impl crate::Resettable for BslPortifCfgI2cSpec {
    const RESET_VALUE: u32 = 0;
}
