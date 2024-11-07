#[doc = "Register `BSL_PORTIF_CFG_UART` reader"]
pub type R = crate::R<BslPortifCfgUartSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "BSL Port Interface Configuration for UART\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_portif_cfg_uart::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BslPortifCfgUartSpec;
impl crate::RegisterSpec for BslPortifCfgUartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsl_portif_cfg_uart::R`](R) reader structure"]
impl crate::Readable for BslPortifCfgUartSpec {}
#[doc = "`reset()` method sets BSL_PORTIF_CFG_UART to value 0"]
impl crate::Resettable for BslPortifCfgUartSpec {
    const RESET_VALUE: u32 = 0;
}
