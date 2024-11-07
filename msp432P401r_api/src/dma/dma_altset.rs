#[doc = "Register `DMA_ALTSET` reader"]
pub type R = crate::R<DmaAltsetSpec>;
#[doc = "Register `DMA_ALTSET` writer"]
pub type W = crate::W<DmaAltsetSpec>;
#[doc = "Channel Primary-Alternate Set Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SetEnumRead {
    #[doc = "0: DMA channel C is using the primary data structure."]
    Set0Read = 0,
    #[doc = "1: DMA channel C is using the alternate data structure."]
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
#[doc = "Field `SET` reader - Channel Primary-Alternate Set Register"]
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
    #[doc = "DMA channel C is using the primary data structure."]
    #[inline(always)]
    pub fn is_set_0_read(&self) -> bool {
        *self == SetEnumRead::Set0Read
    }
    #[doc = "DMA channel C is using the alternate data structure."]
    #[inline(always)]
    pub fn is_set_1_read(&self) -> bool {
        *self == SetEnumRead::Set1Read
    }
}
#[doc = "Channel Primary-Alternate Set Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SetEnumWriteWO {
    #[doc = "0: No effect. Use the DMA_ALTCLR Register to set bit \\[C\\]
to 0."]
    Sel0Write = 0,
    #[doc = "1: Selects the alternate data structure for channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    Sel1Write = 1,
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
#[doc = "Field `SET` writer - Channel Primary-Alternate Set Register"]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 32, SetEnumWriteWO>;
impl<'a, REG> SetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect. Use the DMA_ALTCLR Register to set bit \\[C\\]
to 0."]
    #[inline(always)]
    pub fn sel_0_write(self) -> &'a mut crate::W<REG> {
        self.variant(SetEnumWriteWO::Sel0Write)
    }
    #[doc = "Selects the alternate data structure for channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn sel_1_write(self) -> &'a mut crate::W<REG> {
        self.variant(SetEnumWriteWO::Sel1Write)
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel Primary-Alternate Set Register"]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Primary-Alternate Set Register"]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<DmaAltsetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "Channel Primary-Alternate Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_altset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_altset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaAltsetSpec;
impl crate::RegisterSpec for DmaAltsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_altset::R`](R) reader structure"]
impl crate::Readable for DmaAltsetSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_altset::W`](W) writer structure"]
impl crate::Writable for DmaAltsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_ALTSET to value 0"]
impl crate::Resettable for DmaAltsetSpec {
    const RESET_VALUE: u32 = 0;
}
