#[doc = "Register `DIE_XPOS` reader"]
pub type R = crate::R<DieXposSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Die X-Position\n\nYou can [`read`](crate::Reg::read) this register and get [`die_xpos::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DieXposSpec;
impl crate::RegisterSpec for DieXposSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`die_xpos::R`](R) reader structure"]
impl crate::Readable for DieXposSpec {}
#[doc = "`reset()` method sets DIE_XPOS to value 0"]
impl crate::Resettable for DieXposSpec {
    const RESET_VALUE: u32 = 0;
}
