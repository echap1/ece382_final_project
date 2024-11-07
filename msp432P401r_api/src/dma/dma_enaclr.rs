#[doc = "Register `DMA_ENACLR` writer"]
pub type W = crate::W<DmaEnaclrSpec>;
#[doc = "Set the appropriate bit to disable the corresponding DMA channel. Note: The controller disables a channel, by setting the appropriate bit, when: a) it completes the DMA cycle b) it reads a channel_cfg memory location which has cycle_ctrl = b000 c) an ERROR occurs on the AHB-Lite bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ClrEnumWrite {
    #[doc = "0: No effect. Use the DMA_ENASET Register to enable DMA channels."]
    Clr0 = 0,
    #[doc = "1: Disables channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
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
#[doc = "Field `CLR` writer - Set the appropriate bit to disable the corresponding DMA channel. Note: The controller disables a channel, by setting the appropriate bit, when: a) it completes the DMA cycle b) it reads a channel_cfg memory location which has cycle_ctrl = b000 c) an ERROR occurs on the AHB-Lite bus."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, ClrEnumWrite>;
impl<'a, REG> ClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect. Use the DMA_ENASET Register to enable DMA channels."]
    #[inline(always)]
    pub fn clr_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrEnumWrite::Clr0)
    }
    #[doc = "Disables channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn clr_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrEnumWrite::Clr1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the appropriate bit to disable the corresponding DMA channel. Note: The controller disables a channel, by setting the appropriate bit, when: a) it completes the DMA cycle b) it reads a channel_cfg memory location which has cycle_ctrl = b000 c) an ERROR occurs on the AHB-Lite bus."]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<DmaEnaclrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Channel Enable Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_enaclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaEnaclrSpec;
impl crate::RegisterSpec for DmaEnaclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_enaclr::W`](W) writer structure"]
impl crate::Writable for DmaEnaclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_ENACLR to value 0"]
impl crate::Resettable for DmaEnaclrSpec {
    const RESET_VALUE: u32 = 0;
}
