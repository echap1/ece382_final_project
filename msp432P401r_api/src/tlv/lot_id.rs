#[doc = "Register `LOT_ID` reader"]
pub type R = crate::R<LotIdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Lot ID\n\nYou can [`read`](crate::Reg::read) this register and get [`lot_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LotIdSpec;
impl crate::RegisterSpec for LotIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lot_id::R`](R) reader structure"]
impl crate::Readable for LotIdSpec {}
#[doc = "`reset()` method sets LOT_ID to value 0"]
impl crate::Resettable for LotIdSpec {
    const RESET_VALUE: u32 = 0;
}