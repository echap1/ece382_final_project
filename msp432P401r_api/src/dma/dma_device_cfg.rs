#[doc = "Register `DMA_DEVICE_CFG` reader"]
pub type R = crate::R<DmaDeviceCfgSpec>;
#[doc = "Field `NUM_DMA_CHANNELS` reader - Number of DMA channels available"]
pub type NumDmaChannelsR = crate::FieldReader;
#[doc = "Field `NUM_SRC_PER_CHANNEL` reader - Number of DMA sources per channel"]
pub type NumSrcPerChannelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of DMA channels available"]
    #[inline(always)]
    pub fn num_dma_channels(&self) -> NumDmaChannelsR {
        NumDmaChannelsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of DMA sources per channel"]
    #[inline(always)]
    pub fn num_src_per_channel(&self) -> NumSrcPerChannelR {
        NumSrcPerChannelR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Device Configuration Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_device_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaDeviceCfgSpec;
impl crate::RegisterSpec for DmaDeviceCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_device_cfg::R`](R) reader structure"]
impl crate::Readable for DmaDeviceCfgSpec {}
#[doc = "`reset()` method sets DMA_DEVICE_CFG to value 0"]
impl crate::Resettable for DmaDeviceCfgSpec {
    const RESET_VALUE: u32 = 0;
}
