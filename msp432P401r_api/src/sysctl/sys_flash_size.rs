#[doc = "Register `SYS_FLASH_SIZE` reader"]
pub type R = crate::R<SysFlashSizeSpec>;
#[doc = "Field `SIZE` reader - Flash User Region size"]
pub type SizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Flash User Region size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits)
    }
}
#[doc = "Flash Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_flash_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysFlashSizeSpec;
impl crate::RegisterSpec for SysFlashSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_flash_size::R`](R) reader structure"]
impl crate::Readable for SysFlashSizeSpec {}
