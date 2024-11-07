#[doc = "Register `UCBxADDRX` reader"]
pub type R = crate::R<UcbxAddrxSpec>;
#[doc = "Field `ADDRX` reader - Received Address Register"]
pub type AddrxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Received Address Register"]
    #[inline(always)]
    pub fn addrx(&self) -> AddrxR {
        AddrxR::new(self.bits & 0x03ff)
    }
}
#[doc = "eUSCI_Bx I2C Received Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_addrx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcbxAddrxSpec;
impl crate::RegisterSpec for UcbxAddrxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucbx_addrx::R`](R) reader structure"]
impl crate::Readable for UcbxAddrxSpec {}
#[doc = "`reset()` method sets UCBxADDRX to value 0"]
impl crate::Resettable for UcbxAddrxSpec {
    const RESET_VALUE: u16 = 0;
}
