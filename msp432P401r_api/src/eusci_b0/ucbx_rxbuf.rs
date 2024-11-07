#[doc = "Register `UCBxRXBUF` reader"]
pub type R = crate::R<UcbxRxbufSpec>;
#[doc = "Field `UCRXBUF` reader - Receive data buffer"]
pub type UcrxbufR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UcrxbufR {
        UcrxbufR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_rxbuf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcbxRxbufSpec;
impl crate::RegisterSpec for UcbxRxbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucbx_rxbuf::R`](R) reader structure"]
impl crate::Readable for UcbxRxbufSpec {}
#[doc = "`reset()` method sets UCBxRXBUF to value 0"]
impl crate::Resettable for UcbxRxbufSpec {
    const RESET_VALUE: u16 = 0;
}
