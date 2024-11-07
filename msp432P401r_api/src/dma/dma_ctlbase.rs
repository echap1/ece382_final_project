#[doc = "Register `DMA_CTLBASE` reader"]
pub type R = crate::R<DmaCtlbaseSpec>;
#[doc = "Register `DMA_CTLBASE` writer"]
pub type W = crate::W<DmaCtlbaseSpec>;
#[doc = "Field `ADDR` reader - Pointer to the base address of the primary data structure."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Pointer to the base address of the primary data structure."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Pointer to the base address of the primary data structure."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - Pointer to the base address of the primary data structure."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<DmaCtlbaseSpec> {
        AddrW::new(self, 5)
    }
}
#[doc = "Channel Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ctlbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ctlbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCtlbaseSpec;
impl crate::RegisterSpec for DmaCtlbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ctlbase::R`](R) reader structure"]
impl crate::Readable for DmaCtlbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_ctlbase::W`](W) writer structure"]
impl crate::Writable for DmaCtlbaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CTLBASE to value 0"]
impl crate::Resettable for DmaCtlbaseSpec {
    const RESET_VALUE: u32 = 0;
}
