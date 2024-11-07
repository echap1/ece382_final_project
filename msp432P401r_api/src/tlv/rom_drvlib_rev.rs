#[doc = "Register `ROM_DRVLIB_REV` reader"]
pub type R = crate::R<RomDrvlibRevSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ROM Driver Library Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_drvlib_rev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomDrvlibRevSpec;
impl crate::RegisterSpec for RomDrvlibRevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_drvlib_rev::R`](R) reader structure"]
impl crate::Readable for RomDrvlibRevSpec {}
#[doc = "`reset()` method sets ROM_DRVLIB_REV to value 0"]
impl crate::Resettable for RomDrvlibRevSpec {
    const RESET_VALUE: u32 = 0;
}
