#[doc = "Register `DMA_INT2_SRCCFG` reader"]
pub type R = crate::R<DmaInt2SrccfgSpec>;
#[doc = "Register `DMA_INT2_SRCCFG` writer"]
pub type W = crate::W<DmaInt2SrccfgSpec>;
#[doc = "Field `INT_SRC` reader - Controls which channel's completion event is mapped as a source of this Interrupt"]
pub type IntSrcR = crate::FieldReader;
#[doc = "Field `INT_SRC` writer - Controls which channel's completion event is mapped as a source of this Interrupt"]
pub type IntSrcW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EN` reader - Enables DMA_INT2 mapping"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enables DMA_INT2 mapping"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Controls which channel's completion event is mapped as a source of this Interrupt"]
    #[inline(always)]
    pub fn int_src(&self) -> IntSrcR {
        IntSrcR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Enables DMA_INT2 mapping"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Controls which channel's completion event is mapped as a source of this Interrupt"]
    #[inline(always)]
    pub fn int_src(&mut self) -> IntSrcW<DmaInt2SrccfgSpec> {
        IntSrcW::new(self, 0)
    }
    #[doc = "Bit 5 - Enables DMA_INT2 mapping"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<DmaInt2SrccfgSpec> {
        EnW::new(self, 5)
    }
}
#[doc = "Interrupt 2 Source Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int2_srccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int2_srccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaInt2SrccfgSpec;
impl crate::RegisterSpec for DmaInt2SrccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int2_srccfg::R`](R) reader structure"]
impl crate::Readable for DmaInt2SrccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_int2_srccfg::W`](W) writer structure"]
impl crate::Writable for DmaInt2SrccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_INT2_SRCCFG to value 0"]
impl crate::Resettable for DmaInt2SrccfgSpec {
    const RESET_VALUE: u32 = 0;
}
