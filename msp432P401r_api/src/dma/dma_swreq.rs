#[doc = "Register `DMA_SWREQ` writer"]
pub type W = crate::W<DmaSwreqSpec>;
#[doc = "Set the appropriate bit to generate a software DMA request on the corresponding DMA channel. Writing to a bit where a DMA channel is not implemented does not create a DMA request for that channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ChnlSwReqEnumWrite {
    #[doc = "0: Does not create a DMA request for the channel"]
    ChnlSwReq0 = 0,
    #[doc = "1: Creates a DMA request for the channel"]
    ChnlSwReq1 = 1,
}
impl From<ChnlSwReqEnumWrite> for u32 {
    #[inline(always)]
    fn from(variant: ChnlSwReqEnumWrite) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ChnlSwReqEnumWrite {
    type Ux = u32;
}
impl crate::IsEnum for ChnlSwReqEnumWrite {}
#[doc = "Field `CHNL_SW_REQ` writer - Set the appropriate bit to generate a software DMA request on the corresponding DMA channel. Writing to a bit where a DMA channel is not implemented does not create a DMA request for that channel."]
pub type ChnlSwReqW<'a, REG> = crate::FieldWriter<'a, REG, 32, ChnlSwReqEnumWrite>;
impl<'a, REG> ChnlSwReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Does not create a DMA request for the channel"]
    #[inline(always)]
    pub fn chnl_sw_req_0(self) -> &'a mut crate::W<REG> {
        self.variant(ChnlSwReqEnumWrite::ChnlSwReq0)
    }
    #[doc = "Creates a DMA request for the channel"]
    #[inline(always)]
    pub fn chnl_sw_req_1(self) -> &'a mut crate::W<REG> {
        self.variant(ChnlSwReqEnumWrite::ChnlSwReq1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the appropriate bit to generate a software DMA request on the corresponding DMA channel. Writing to a bit where a DMA channel is not implemented does not create a DMA request for that channel."]
    #[inline(always)]
    pub fn chnl_sw_req(&mut self) -> ChnlSwReqW<DmaSwreqSpec> {
        ChnlSwReqW::new(self, 0)
    }
}
#[doc = "Channel Software Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_swreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSwreqSpec;
impl crate::RegisterSpec for DmaSwreqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_swreq::W`](W) writer structure"]
impl crate::Writable for DmaSwreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_SWREQ to value 0"]
impl crate::Resettable for DmaSwreqSpec {
    const RESET_VALUE: u32 = 0;
}
