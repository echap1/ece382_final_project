#[doc = "Register `TLV_CHECKSUM` reader"]
pub type R = crate::R<TlvChecksumSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "TLV Checksum\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_checksum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TlvChecksumSpec;
impl crate::RegisterSpec for TlvChecksumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tlv_checksum::R`](R) reader structure"]
impl crate::Readable for TlvChecksumSpec {}
#[doc = "`reset()` method sets TLV_CHECKSUM to value 0"]
impl crate::Resettable for TlvChecksumSpec {
    const RESET_VALUE: u32 = 0;
}
