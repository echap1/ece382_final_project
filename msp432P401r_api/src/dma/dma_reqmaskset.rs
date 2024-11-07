#[doc = "Register `DMA_REQMASKSET` reader"]
pub type R = crate::R<DmaReqmasksetSpec>;
#[doc = "Register `DMA_REQMASKSET` writer"]
pub type W = crate::W<DmaReqmasksetSpec>;
#[doc = "Returns the request mask status of dma_req and dma_sreq, or disables the corresponding channel from generating DMA requests.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SetEnumRead {
    #[doc = "0: External requests are enabled for channel C."]
    Set0Read = 0,
    #[doc = "1: External requests are disabled for channel C."]
    Set1Read = 1,
}
impl From<SetEnumRead> for u32 {
    #[inline(always)]
    fn from(variant: SetEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SetEnumRead {
    type Ux = u32;
}
impl crate::IsEnum for SetEnumRead {}
#[doc = "Field `SET` reader - Returns the request mask status of dma_req and dma_sreq, or disables the corresponding channel from generating DMA requests."]
pub type SetR = crate::FieldReader<SetEnumRead>;
impl SetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SetEnumRead> {
        match self.bits {
            0 => Some(SetEnumRead::Set0Read),
            1 => Some(SetEnumRead::Set1Read),
            _ => None,
        }
    }
    #[doc = "External requests are enabled for channel C."]
    #[inline(always)]
    pub fn is_set_0_read(&self) -> bool {
        *self == SetEnumRead::Set0Read
    }
    #[doc = "External requests are disabled for channel C."]
    #[inline(always)]
    pub fn is_set_1_read(&self) -> bool {
        *self == SetEnumRead::Set1Read
    }
}
#[doc = "Returns the request mask status of dma_req and dma_sreq, or disables the corresponding channel from generating DMA requests.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SetEnumWriteWO {
    #[doc = "0: No effect. Use the DMA_REQMASKCLR Register to enable DMA requests."]
    Set0Write = 0,
    #[doc = "1: Disables dma_req\\[C\\]
and dma_sreq\\[C\\]
from generating DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    Set1Write = 1,
}
impl From<SetEnumWriteWO> for u32 {
    #[inline(always)]
    fn from(variant: SetEnumWriteWO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SetEnumWriteWO {
    type Ux = u32;
}
impl crate::IsEnum for SetEnumWriteWO {}
#[doc = "Field `SET` writer - Returns the request mask status of dma_req and dma_sreq, or disables the corresponding channel from generating DMA requests."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 32, SetEnumWriteWO>;
impl<'a, REG> SetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect. Use the DMA_REQMASKCLR Register to enable DMA requests."]
    #[inline(always)]
    pub fn set_0_write(self) -> &'a mut crate::W<REG> {
        self.variant(SetEnumWriteWO::Set0Write)
    }
    #[doc = "Disables dma_req\\[C\\]
and dma_sreq\\[C\\]
from generating DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn set_1_write(self) -> &'a mut crate::W<REG> {
        self.variant(SetEnumWriteWO::Set1Write)
    }
}
impl R {
    #[doc = "Bits 0:31 - Returns the request mask status of dma_req and dma_sreq, or disables the corresponding channel from generating DMA requests."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Returns the request mask status of dma_req and dma_sreq, or disables the corresponding channel from generating DMA requests."]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<DmaReqmasksetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "Channel Request Mask Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_reqmaskset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_reqmaskset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaReqmasksetSpec;
impl crate::RegisterSpec for DmaReqmasksetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_reqmaskset::R`](R) reader structure"]
impl crate::Readable for DmaReqmasksetSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_reqmaskset::W`](W) writer structure"]
impl crate::Writable for DmaReqmasksetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_REQMASKSET to value 0"]
impl crate::Resettable for DmaReqmasksetSpec {
    const RESET_VALUE: u32 = 0;
}
