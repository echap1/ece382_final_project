#[doc = "Register `UCAxRXBUF` reader"]
pub type R = crate::R<UcaxRxbufSpec>;
#[doc = "Field `UCRXBUF` reader - Receive data buffer"]
pub type UcrxbufR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UcrxbufR {
        UcrxbufR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_rxbuf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcaxRxbufSpec;
impl crate::RegisterSpec for UcaxRxbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucax_rxbuf::R`](R) reader structure"]
impl crate::Readable for UcaxRxbufSpec {}
#[doc = "`reset()` method sets UCAxRXBUF to value 0"]
impl crate::Resettable for UcaxRxbufSpec {
    const RESET_VALUE: u16 = 0;
}
