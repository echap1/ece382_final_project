#[doc = "Register `FLCTL_ERSVER_TIMCTL` reader"]
pub type R = crate::R<FlctlErsverTimctlSpec>;
#[doc = "Field `SETUP` reader - Length of the Setup phase for this operation"]
pub type SetupR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Erase Verify Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_ersver_timctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlErsverTimctlSpec;
impl crate::RegisterSpec for FlctlErsverTimctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_ersver_timctl::R`](R) reader structure"]
impl crate::Readable for FlctlErsverTimctlSpec {}
