#[doc = "Register `DIE_YPOS` reader"]
pub type R = crate::R<DieYposSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Die Y-Position\n\nYou can [`read`](crate::Reg::read) this register and get [`die_ypos::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DieYposSpec;
impl crate::RegisterSpec for DieYposSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`die_ypos::R`](R) reader structure"]
impl crate::Readable for DieYposSpec {}
#[doc = "`reset()` method sets DIE_YPOS to value 0"]
impl crate::Resettable for DieYposSpec {
    const RESET_VALUE: u32 = 0;
}
