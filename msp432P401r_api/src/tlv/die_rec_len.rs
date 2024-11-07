#[doc = "Register `DIE_REC_LEN` reader"]
pub type R = crate::R<DieRecLenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Die Record Length\n\nYou can [`read`](crate::Reg::read) this register and get [`die_rec_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DieRecLenSpec;
impl crate::RegisterSpec for DieRecLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`die_rec_len::R`](R) reader structure"]
impl crate::Readable for DieRecLenSpec {}
#[doc = "`reset()` method sets DIE_REC_LEN to value 0"]
impl crate::Resettable for DieRecLenSpec {
    const RESET_VALUE: u32 = 0;
}
