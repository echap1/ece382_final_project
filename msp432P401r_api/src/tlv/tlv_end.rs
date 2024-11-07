#[doc = "Register `TLV_END` reader"]
pub type R = crate::R<TlvEndSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "TLV End Word\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_end::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TlvEndSpec;
impl crate::RegisterSpec for TlvEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tlv_end::R`](R) reader structure"]
impl crate::Readable for TlvEndSpec {}
#[doc = "`reset()` method sets TLV_END to value 0x0bd0_e11d"]
impl crate::Resettable for TlvEndSpec {
    const RESET_VALUE: u32 = 0x0bd0_e11d;
}
