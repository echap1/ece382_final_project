#[doc = "Register `UCBxTXBUF` reader"]
pub type R = crate::R<UcbxTxbufSpec>;
#[doc = "Register `UCBxTXBUF` writer"]
pub type W = crate::W<UcbxTxbufSpec>;
#[doc = "Field `UCTXBUF` reader - Transmit data buffer"]
pub type UctxbufR = crate::FieldReader;
#[doc = "Field `UCTXBUF` writer - Transmit data buffer"]
pub type UctxbufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&self) -> UctxbufR {
        UctxbufR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&mut self) -> UctxbufW<UcbxTxbufSpec> {
        UctxbufW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcbxTxbufSpec;
impl crate::RegisterSpec for UcbxTxbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucbx_txbuf::R`](R) reader structure"]
impl crate::Readable for UcbxTxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`ucbx_txbuf::W`](W) writer structure"]
impl crate::Writable for UcbxTxbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCBxTXBUF to value 0"]
impl crate::Resettable for UcbxTxbufSpec {
    const RESET_VALUE: u16 = 0;
}
