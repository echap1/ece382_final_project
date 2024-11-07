#[doc = "Register `DIE_REC_TAG` reader"]
pub type R = crate::R<DieRecTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Die Record Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`die_rec_tag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DieRecTagSpec;
impl crate::RegisterSpec for DieRecTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`die_rec_tag::R`](R) reader structure"]
impl crate::Readable for DieRecTagSpec {}
#[doc = "`reset()` method sets DIE_REC_TAG to value 0x0c"]
impl crate::Resettable for DieRecTagSpec {
    const RESET_VALUE: u32 = 0x0c;
}
