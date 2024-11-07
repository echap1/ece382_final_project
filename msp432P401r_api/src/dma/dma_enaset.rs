#[doc = "Register `DMA_ENASET` reader"]
pub type R = crate::R<DmaEnasetSpec>;
#[doc = "Register `DMA_ENASET` writer"]
pub type W = crate::W<DmaEnasetSpec>;
#[doc = "Returns the enable status of the channels, or enables the corresponding channels.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SetEnumRead {
    #[doc = "0: Channel C is disabled."]
    Set0Read = 0,
    #[doc = "1: Channel C is enabled."]
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
#[doc = "Field `SET` reader - Returns the enable status of the channels, or enables the corresponding channels."]
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
    #[doc = "Channel C is disabled."]
    #[inline(always)]
    pub fn is_set_0_read(&self) -> bool {
        *self == SetEnumRead::Set0Read
    }
    #[doc = "Channel C is enabled."]
    #[inline(always)]
    pub fn is_set_1_read(&self) -> bool {
        *self == SetEnumRead::Set1Read
    }
}
#[doc = "Returns the enable status of the channels, or enables the corresponding channels.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SetEnumWriteWO {
    #[doc = "0: No effect. Use the DMA_ENACLR Register to disable a channel."]
    Set0Write = 0,
    #[doc = "1: Enables channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
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
#[doc = "Field `SET` writer - Returns the enable status of the channels, or enables the corresponding channels."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 32, SetEnumWriteWO>;
impl<'a, REG> SetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect. Use the DMA_ENACLR Register to disable a channel."]
    #[inline(always)]
    pub fn set_0_write(self) -> &'a mut crate::W<REG> {
        self.variant(SetEnumWriteWO::Set0Write)
    }
    #[doc = "Enables channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn set_1_write(self) -> &'a mut crate::W<REG> {
        self.variant(SetEnumWriteWO::Set1Write)
    }
}
impl R {
    #[doc = "Bits 0:31 - Returns the enable status of the channels, or enables the corresponding channels."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Returns the enable status of the channels, or enables the corresponding channels."]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<DmaEnasetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "Channel Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_enaset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_enaset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaEnasetSpec;
impl crate::RegisterSpec for DmaEnasetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_enaset::R`](R) reader structure"]
impl crate::Readable for DmaEnasetSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_enaset::W`](W) writer structure"]
impl crate::Writable for DmaEnasetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_ENASET to value 0"]
impl crate::Resettable for DmaEnasetSpec {
    const RESET_VALUE: u32 = 0;
}
