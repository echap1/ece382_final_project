#[doc = "Register `FLCTL_ERASE_TIMCTL` reader"]
pub type R = crate::R<FlctlEraseTimctlSpec>;
#[doc = "Field `SETUP` reader - Length of the Setup phase for this operation"]
pub type SetupR = crate::FieldReader;
#[doc = "Field `ACTIVE` reader - Length of the Active phase for this operation"]
pub type ActiveR = crate::FieldReader<u32>;
#[doc = "Field `HOLD` reader - Length of the Hold phase for this operation"]
pub type HoldR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:27 - Length of the Active phase for this operation"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits >> 8) & 0x000f_ffff)
    }
    #[doc = "Bits 28:31 - Length of the Hold phase for this operation"]
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Erase Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_erase_timctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlEraseTimctlSpec;
impl crate::RegisterSpec for FlctlEraseTimctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_erase_timctl::R`](R) reader structure"]
impl crate::Readable for FlctlEraseTimctlSpec {}
