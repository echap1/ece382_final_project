#[doc = "Register `DMA_USEBURSTCLR` writer"]
pub type W = crate::W<DmaUseburstclrSpec>;
#[doc = "Set the appropriate bit to enable dma_sreq to generate requests.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ClrEnumWrite {
    #[doc = "0: No effect. Use the DMA_USEBURST_SET Register to disable dma_sreq from generating requests."]
    Clr0 = 0,
    #[doc = "1: Enables dma_sreq\\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    Clr1 = 1,
}
impl From<ClrEnumWrite> for u32 {
    #[inline(always)]
    fn from(variant: ClrEnumWrite) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClrEnumWrite {
    type Ux = u32;
}
impl crate::IsEnum for ClrEnumWrite {}
#[doc = "Field `CLR` writer - Set the appropriate bit to enable dma_sreq to generate requests."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, ClrEnumWrite>;
impl<'a, REG> ClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect. Use the DMA_USEBURST_SET Register to disable dma_sreq from generating requests."]
    #[inline(always)]
    pub fn clr_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrEnumWrite::Clr0)
    }
    #[doc = "Enables dma_sreq\\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn clr_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrEnumWrite::Clr1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the appropriate bit to enable dma_sreq to generate requests."]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<DmaUseburstclrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Channel Useburst Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_useburstclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaUseburstclrSpec;
impl crate::RegisterSpec for DmaUseburstclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_useburstclr::W`](W) writer structure"]
impl crate::Writable for DmaUseburstclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_USEBURSTCLR to value 0"]
impl crate::Resettable for DmaUseburstclrSpec {
    const RESET_VALUE: u32 = 0;
}
