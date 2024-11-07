#[doc = "Register `DMA_PRIOSET` reader"]
pub type R = crate::R<DmaPriosetSpec>;
#[doc = "Register `DMA_PRIOSET` writer"]
pub type W = crate::W<DmaPriosetSpec>;
#[doc = "Returns the channel priority mask status, or sets the channel priority to high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SetEnumRead {
    #[doc = "0: DMA channel C is using the default priority level."]
    Set0Read = 0,
    #[doc = "1: DMA channel C is using a high priority level."]
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
#[doc = "Field `SET` reader - Returns the channel priority mask status, or sets the channel priority to high."]
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
    #[doc = "DMA channel C is using the default priority level."]
    #[inline(always)]
    pub fn is_set_0_read(&self) -> bool {
        *self == SetEnumRead::Set0Read
    }
    #[doc = "DMA channel C is using a high priority level."]
    #[inline(always)]
    pub fn is_set_1_read(&self) -> bool {
        *self == SetEnumRead::Set1Read
    }
}
#[doc = "Returns the channel priority mask status, or sets the channel priority to high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SetEnumWriteWO {
    #[doc = "0: No effect. Use the DMA_PRIOCLR Register to set channel C to the default priority level."]
    Set0Write = 0,
    #[doc = "1: Channel C uses the high priority level. Writing to a bit where a DMA channel is not implemented has no effect."]
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
#[doc = "Field `SET` writer - Returns the channel priority mask status, or sets the channel priority to high."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 32, SetEnumWriteWO>;
impl<'a, REG> SetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect. Use the DMA_PRIOCLR Register to set channel C to the default priority level."]
    #[inline(always)]
    pub fn set_0_write(self) -> &'a mut crate::W<REG> {
        self.variant(SetEnumWriteWO::Set0Write)
    }
    #[doc = "Channel C uses the high priority level. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn set_1_write(self) -> &'a mut crate::W<REG> {
        self.variant(SetEnumWriteWO::Set1Write)
    }
}
impl R {
    #[doc = "Bits 0:31 - Returns the channel priority mask status, or sets the channel priority to high."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Returns the channel priority mask status, or sets the channel priority to high."]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<DmaPriosetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "Channel Priority Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_prioset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_prioset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaPriosetSpec;
impl crate::RegisterSpec for DmaPriosetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_prioset::R`](R) reader structure"]
impl crate::Readable for DmaPriosetSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_prioset::W`](W) writer structure"]
impl crate::Writable for DmaPriosetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_PRIOSET to value 0"]
impl crate::Resettable for DmaPriosetSpec {
    const RESET_VALUE: u32 = 0;
}
