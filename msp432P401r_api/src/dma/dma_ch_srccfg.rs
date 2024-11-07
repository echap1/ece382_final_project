#[doc = "Register `DMA_CH_SRCCFG[%s]` reader"]
pub type R = crate::R<DmaChSrccfgSpec>;
#[doc = "Register `DMA_CH_SRCCFG[%s]` writer"]
pub type W = crate::W<DmaChSrccfgSpec>;
#[doc = "Field `DMA_SRC` reader - Device level DMA source mapping to channel input"]
pub type DmaSrcR = crate::FieldReader;
#[doc = "Field `DMA_SRC` writer - Device level DMA source mapping to channel input"]
pub type DmaSrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Device level DMA source mapping to channel input"]
    #[inline(always)]
    pub fn dma_src(&self) -> DmaSrcR {
        DmaSrcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Device level DMA source mapping to channel input"]
    #[inline(always)]
    pub fn dma_src(&mut self) -> DmaSrcW<DmaChSrccfgSpec> {
        DmaSrcW::new(self, 0)
    }
}
#[doc = "Channel n Source Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ch_srccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ch_srccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaChSrccfgSpec;
impl crate::RegisterSpec for DmaChSrccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ch_srccfg::R`](R) reader structure"]
impl crate::Readable for DmaChSrccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_ch_srccfg::W`](W) writer structure"]
impl crate::Writable for DmaChSrccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CH_SRCCFG[%s]
to value 0"]
impl crate::Resettable for DmaChSrccfgSpec {
    const RESET_VALUE: u32 = 0;
}
