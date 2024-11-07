#[doc = "Register `DMA_WAITSTAT` reader"]
pub type R = crate::R<DmaWaitstatSpec>;
#[doc = "Channel wait on request status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WaitreqEnumRead {
    #[doc = "0: dma_waitonreq\\[C\\]
is LOW."]
    Waitreq0 = 0,
    #[doc = "1: dma_waitonreq\\[C\\]
is HIGH."]
    Waitreq1 = 1,
}
impl From<WaitreqEnumRead> for u32 {
    #[inline(always)]
    fn from(variant: WaitreqEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WaitreqEnumRead {
    type Ux = u32;
}
impl crate::IsEnum for WaitreqEnumRead {}
#[doc = "Field `WAITREQ` reader - Channel wait on request status."]
pub type WaitreqR = crate::FieldReader<WaitreqEnumRead>;
impl WaitreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WaitreqEnumRead> {
        match self.bits {
            0 => Some(WaitreqEnumRead::Waitreq0),
            1 => Some(WaitreqEnumRead::Waitreq1),
            _ => None,
        }
    }
    #[doc = "dma_waitonreq\\[C\\]
is LOW."]
    #[inline(always)]
    pub fn is_waitreq_0(&self) -> bool {
        *self == WaitreqEnumRead::Waitreq0
    }
    #[doc = "dma_waitonreq\\[C\\]
is HIGH."]
    #[inline(always)]
    pub fn is_waitreq_1(&self) -> bool {
        *self == WaitreqEnumRead::Waitreq1
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel wait on request status."]
    #[inline(always)]
    pub fn waitreq(&self) -> WaitreqR {
        WaitreqR::new(self.bits)
    }
}
#[doc = "Channel Wait on Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_waitstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaWaitstatSpec;
impl crate::RegisterSpec for DmaWaitstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_waitstat::R`](R) reader structure"]
impl crate::Readable for DmaWaitstatSpec {}
#[doc = "`reset()` method sets DMA_WAITSTAT to value 0"]
impl crate::Resettable for DmaWaitstatSpec {
    const RESET_VALUE: u32 = 0;
}
