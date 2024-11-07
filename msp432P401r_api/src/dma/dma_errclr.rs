#[doc = "Register `DMA_ERRCLR` reader"]
pub type R = crate::R<DmaErrclrSpec>;
#[doc = "Register `DMA_ERRCLR` writer"]
pub type W = crate::W<DmaErrclrSpec>;
#[doc = "Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrclrEnumRead {
    #[doc = "0: dma_err is LOW"]
    Errclr0Read = 0,
    #[doc = "1: dma_err is HIGH."]
    Errclr1Read = 1,
}
impl From<ErrclrEnumRead> for bool {
    #[inline(always)]
    fn from(variant: ErrclrEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCLR` reader - Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted."]
pub type ErrclrR = crate::BitReader<ErrclrEnumRead>;
impl ErrclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErrclrEnumRead {
        match self.bits {
            false => ErrclrEnumRead::Errclr0Read,
            true => ErrclrEnumRead::Errclr1Read,
        }
    }
    #[doc = "dma_err is LOW"]
    #[inline(always)]
    pub fn is_errclr_0_read(&self) -> bool {
        *self == ErrclrEnumRead::Errclr0Read
    }
    #[doc = "dma_err is HIGH."]
    #[inline(always)]
    pub fn is_errclr_1_read(&self) -> bool {
        *self == ErrclrEnumRead::Errclr1Read
    }
}
#[doc = "Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrclrEnumWriteWO {
    #[doc = "0: No effect, status of dma_err is unchanged."]
    Errclr0Write = 0,
    #[doc = "1: Sets dma_err LOW."]
    Errclr1Write = 1,
}
impl From<ErrclrEnumWriteWO> for bool {
    #[inline(always)]
    fn from(variant: ErrclrEnumWriteWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCLR` writer - Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted."]
pub type ErrclrW<'a, REG> = crate::BitWriter<'a, REG, ErrclrEnumWriteWO>;
impl<'a, REG> ErrclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect, status of dma_err is unchanged."]
    #[inline(always)]
    pub fn errclr_0_write(self) -> &'a mut crate::W<REG> {
        self.variant(ErrclrEnumWriteWO::Errclr0Write)
    }
    #[doc = "Sets dma_err LOW."]
    #[inline(always)]
    pub fn errclr_1_write(self) -> &'a mut crate::W<REG> {
        self.variant(ErrclrEnumWriteWO::Errclr1Write)
    }
}
impl R {
    #[doc = "Bit 0 - Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted."]
    #[inline(always)]
    pub fn errclr(&self) -> ErrclrR {
        ErrclrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted."]
    #[inline(always)]
    pub fn errclr(&mut self) -> ErrclrW<DmaErrclrSpec> {
        ErrclrW::new(self, 0)
    }
}
#[doc = "Bus Error Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_errclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_errclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaErrclrSpec;
impl crate::RegisterSpec for DmaErrclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_errclr::R`](R) reader structure"]
impl crate::Readable for DmaErrclrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_errclr::W`](W) writer structure"]
impl crate::Writable for DmaErrclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_ERRCLR to value 0"]
impl crate::Resettable for DmaErrclrSpec {
    const RESET_VALUE: u32 = 0;
}
