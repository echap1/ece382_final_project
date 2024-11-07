#[doc = "Register `FLCTL_LKGVER_TIMCTL` reader"]
pub type R = crate::R<FlctlLkgverTimctlSpec>;
#[doc = "Field `SETUP` reader - Length of the Setup phase for this operation"]
pub type SetupR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Leakage Verify Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_lkgver_timctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlLkgverTimctlSpec;
impl crate::RegisterSpec for FlctlLkgverTimctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_lkgver_timctl::R`](R) reader structure"]
impl crate::Readable for FlctlLkgverTimctlSpec {}
