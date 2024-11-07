#[doc = "Register `DMA_ALTBASE` reader"]
pub type R = crate::R<DmaAltbaseSpec>;
#[doc = "Field `ADDR` reader - Base address of the alternate data structure"]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Base address of the alternate data structure"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "Channel Alternate Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_altbase::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaAltbaseSpec;
impl crate::RegisterSpec for DmaAltbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_altbase::R`](R) reader structure"]
impl crate::Readable for DmaAltbaseSpec {}
#[doc = "`reset()` method sets DMA_ALTBASE to value 0"]
impl crate::Resettable for DmaAltbaseSpec {
    const RESET_VALUE: u32 = 0;
}
