#[doc = "Register `DMA_ALTCLR` writer"]
pub type W = crate::W<DmaAltclrSpec>;
#[doc = "Channel Primary-Alternate Clear Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ClrEnumWrite {
    #[doc = "0: No effect. Use the DMA_ALTSET Register to select the alternate data structure."]
    Clr0 = 0,
    #[doc = "1: Selects the primary data structure for channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
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
#[doc = "Field `CLR` writer - Channel Primary-Alternate Clear Register"]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, ClrEnumWrite>;
impl<'a, REG> ClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect. Use the DMA_ALTSET Register to select the alternate data structure."]
    #[inline(always)]
    pub fn clr_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrEnumWrite::Clr0)
    }
    #[doc = "Selects the primary data structure for channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn clr_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrEnumWrite::Clr1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Primary-Alternate Clear Register"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<DmaAltclrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Channel Primary-Alternate Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_altclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaAltclrSpec;
impl crate::RegisterSpec for DmaAltclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_altclr::W`](W) writer structure"]
impl crate::Writable for DmaAltclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_ALTCLR to value 0"]
impl crate::Resettable for DmaAltclrSpec {
    const RESET_VALUE: u32 = 0;
}
