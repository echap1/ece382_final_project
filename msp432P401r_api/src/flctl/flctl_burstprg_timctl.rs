#[doc = "Register `FLCTL_BURSTPRG_TIMCTL` reader"]
pub type R = crate::R<FlctlBurstprgTimctlSpec>;
#[doc = "Field `ACTIVE` reader - Length of the Active phase for this operation"]
pub type ActiveR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 8:27 - Length of the Active phase for this operation"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits >> 8) & 0x000f_ffff)
    }
}
#[doc = "Burst Program Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_burstprg_timctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlBurstprgTimctlSpec;
impl crate::RegisterSpec for FlctlBurstprgTimctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_burstprg_timctl::R`](R) reader structure"]
impl crate::Readable for FlctlBurstprgTimctlSpec {}
