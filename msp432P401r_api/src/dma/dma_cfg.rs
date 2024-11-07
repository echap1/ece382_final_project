#[doc = "Register `DMA_CFG` writer"]
pub type W = crate::W<DmaCfgSpec>;
#[doc = "Enable status of the controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MastenEnumWrite {
    #[doc = "0: Controller disabled"]
    Masten0 = 0,
    #[doc = "1: Controller enabled"]
    Masten1 = 1,
}
impl From<MastenEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: MastenEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTEN` writer - Enable status of the controller"]
pub type MastenW<'a, REG> = crate::BitWriter<'a, REG, MastenEnumWrite>;
impl<'a, REG> MastenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Controller disabled"]
    #[inline(always)]
    pub fn masten_0(self) -> &'a mut crate::W<REG> {
        self.variant(MastenEnumWrite::Masten0)
    }
    #[doc = "Controller enabled"]
    #[inline(always)]
    pub fn masten_1(self) -> &'a mut crate::W<REG> {
        self.variant(MastenEnumWrite::Masten1)
    }
}
#[doc = "Field `CHPROTCTRL` writer - Sets the AHB-Lite protection by controlling the HPROT\\[3:1\\]
signal levels as follows: Bit \\[7\\]
Controls HPROT\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HPROT\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HPROT\\[1\\]
to indicate if a privileged access is occurring. Note: When bit \\[n\\]
= 1 then the corresponding HPROT is HIGH. When bit \\[n\\]
= 0 then the corresponding HPROT is LOW."]
pub type ChprotctrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl W {
    #[doc = "Bit 0 - Enable status of the controller"]
    #[inline(always)]
    pub fn masten(&mut self) -> MastenW<DmaCfgSpec> {
        MastenW::new(self, 0)
    }
    #[doc = "Bits 5:7 - Sets the AHB-Lite protection by controlling the HPROT\\[3:1\\]
signal levels as follows: Bit \\[7\\]
Controls HPROT\\[3\\]
to indicate if a cacheable access is occurring. Bit \\[6\\]
Controls HPROT\\[2\\]
to indicate if a bufferable access is occurring. Bit \\[5\\]
Controls HPROT\\[1\\]
to indicate if a privileged access is occurring. Note: When bit \\[n\\]
= 1 then the corresponding HPROT is HIGH. When bit \\[n\\]
= 0 then the corresponding HPROT is LOW."]
    #[inline(always)]
    pub fn chprotctrl(&mut self) -> ChprotctrlW<DmaCfgSpec> {
        ChprotctrlW::new(self, 5)
    }
}
#[doc = "Configuration Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cfg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCfgSpec;
impl crate::RegisterSpec for DmaCfgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_cfg::W`](W) writer structure"]
impl crate::Writable for DmaCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CFG to value 0"]
impl crate::Resettable for DmaCfgSpec {
    const RESET_VALUE: u32 = 0;
}
