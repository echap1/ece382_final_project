#[doc = "Register `HWREV` reader"]
pub type R = crate::R<HwrevSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "HW Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`hwrev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwrevSpec;
impl crate::RegisterSpec for HwrevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwrev::R`](R) reader structure"]
impl crate::Readable for HwrevSpec {}
#[doc = "`reset()` method sets HWREV to value 0"]
impl crate::Resettable for HwrevSpec {
    const RESET_VALUE: u32 = 0;
}
