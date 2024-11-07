#[doc = "Register `FLCTL_PRGVER_TIMCTL` reader"]
pub type R = crate::R<FlctlPrgverTimctlSpec>;
#[doc = "Field `SETUP` reader - Length of the Setup phase for this operation"]
pub type SetupR = crate::FieldReader;
#[doc = "Field `ACTIVE` reader - Length of the Active phase for this operation"]
pub type ActiveR = crate::FieldReader;
#[doc = "Field `HOLD` reader - Length of the Hold phase for this operation"]
pub type HoldR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Length of the Active phase for this operation"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Length of the Hold phase for this operation"]
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Program Verify Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgver_timctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlPrgverTimctlSpec;
impl crate::RegisterSpec for FlctlPrgverTimctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_prgver_timctl::R`](R) reader structure"]
impl crate::Readable for FlctlPrgverTimctlSpec {}
