#[doc = "Register `BSL_PORTIF_CFG_SPI` reader"]
pub type R = crate::R<BslPortifCfgSpiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "BSL Port Interface Configuration for SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_portif_cfg_spi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BslPortifCfgSpiSpec;
impl crate::RegisterSpec for BslPortifCfgSpiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsl_portif_cfg_spi::R`](R) reader structure"]
impl crate::Readable for BslPortifCfgSpiSpec {}
#[doc = "`reset()` method sets BSL_PORTIF_CFG_SPI to value 0"]
impl crate::Resettable for BslPortifCfgSpiSpec {
    const RESET_VALUE: u32 = 0;
}
