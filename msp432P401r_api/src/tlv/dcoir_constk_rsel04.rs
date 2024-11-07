#[doc = "Register `DCOIR_CONSTK_RSEL04` reader"]
pub type R = crate::R<DcoirConstkRsel04Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DCO IR mode: DCO Constant (K) for DCORSEL 0 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoir_constk_rsel04::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcoirConstkRsel04Spec;
impl crate::RegisterSpec for DcoirConstkRsel04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcoir_constk_rsel04::R`](R) reader structure"]
impl crate::Readable for DcoirConstkRsel04Spec {}
#[doc = "`reset()` method sets DCOIR_CONSTK_RSEL04 to value 0"]
impl crate::Resettable for DcoirConstkRsel04Spec {
    const RESET_VALUE: u32 = 0;
}
