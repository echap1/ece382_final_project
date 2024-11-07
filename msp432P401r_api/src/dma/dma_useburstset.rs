#[doc = "Register `DMA_USEBURSTSET` reader"]
pub type R = crate::R<DmaUseburstsetSpec>;
#[doc = "Register `DMA_USEBURSTSET` writer"]
pub type W = crate::W<DmaUseburstsetSpec>;
#[doc = "Returns the useburst status, or disables dma_sreq from generating DMA requests.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SetEnumRead {
    #[doc = "0: DMA channel C responds to requests that it receives on dma_req\\[C\\]
or dma_sreq\\[C\\]. The controller performs 2R, or single, bus transfers."]
    Set0Read = 0,
    #[doc = "1: DMA channel C does not respond to requests that it receives on dma_sreq\\[C\\]. The controller only responds to dma_req\\[C\\]
requests and performs 2R transfers."]
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
#[doc = "Field `SET` reader - Returns the useburst status, or disables dma_sreq from generating DMA requests."]
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
    #[doc = "DMA channel C responds to requests that it receives on dma_req\\[C\\]
or dma_sreq\\[C\\]. The controller performs 2R, or single, bus transfers."]
    #[inline(always)]
    pub fn is_set_0_read(&self) -> bool {
        *self == SetEnumRead::Set0Read
    }
    #[doc = "DMA channel C does not respond to requests that it receives on dma_sreq\\[C\\]. The controller only responds to dma_req\\[C\\]
requests and performs 2R transfers."]
    #[inline(always)]
    pub fn is_set_1_read(&self) -> bool {
        *self == SetEnumRead::Set1Read
    }
}
#[doc = "Returns the useburst status, or disables dma_sreq from generating DMA requests.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SetEnumWriteWO {
    #[doc = "0: No effect. Use the DMA_USEBURST_CLR Register to set bit \\[C\\]
to 0."]
    Set0Write = 0,
    #[doc = "1: Disables dma_sreq\\[C\\]
from generating DMA requests. The controller performs 2R transfers. Writing to a bit where a DMA channel is not implemented has no effect."]
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
#[doc = "Field `SET` writer - Returns the useburst status, or disables dma_sreq from generating DMA requests."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 32, SetEnumWriteWO>;
impl<'a, REG> SetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect. Use the DMA_USEBURST_CLR Register to set bit \\[C\\]
to 0."]
    #[inline(always)]
    pub fn set_0_write(self) -> &'a mut crate::W<REG> {
        self.variant(SetEnumWriteWO::Set0Write)
    }
    #[doc = "Disables dma_sreq\\[C\\]
from generating DMA requests. The controller performs 2R transfers. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn set_1_write(self) -> &'a mut crate::W<REG> {
        self.variant(SetEnumWriteWO::Set1Write)
    }
}
impl R {
    #[doc = "Bits 0:31 - Returns the useburst status, or disables dma_sreq from generating DMA requests."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Returns the useburst status, or disables dma_sreq from generating DMA requests."]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<DmaUseburstsetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "Channel Useburst Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_useburstset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_useburstset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaUseburstsetSpec;
impl crate::RegisterSpec for DmaUseburstsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_useburstset::R`](R) reader structure"]
impl crate::Readable for DmaUseburstsetSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_useburstset::W`](W) writer structure"]
impl crate::Writable for DmaUseburstsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_USEBURSTSET to value 0"]
impl crate::Resettable for DmaUseburstsetSpec {
    const RESET_VALUE: u32 = 0;
}
