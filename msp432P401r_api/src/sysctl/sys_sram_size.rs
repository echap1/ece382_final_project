#[doc = "Register `SYS_SRAM_SIZE` reader"]
pub type R = crate::R<SysSramSizeSpec>;
#[doc = "Field `SIZE` reader - Indicates the size of SRAM on the device"]
pub type SizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the size of SRAM on the device"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(self.bits)
    }
}
#[doc = "SRAM Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sram_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSramSizeSpec;
impl crate::RegisterSpec for SysSramSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sram_size::R`](R) reader structure"]
impl crate::Readable for SysSramSizeSpec {}
