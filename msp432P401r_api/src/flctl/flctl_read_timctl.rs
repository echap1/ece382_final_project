#[doc = "Register `FLCTL_READ_TIMCTL` reader"]
pub type R = crate::R<FlctlReadTimctlSpec>;
#[doc = "Field `SETUP` reader - Configures the length of the Setup phase for this operation"]
pub type SetupR = crate::FieldReader;
#[doc = "Field `IREF_BOOST1` reader - Length of the IREF_BOOST1 signal of the IP"]
pub type IrefBoost1R = crate::FieldReader;
#[doc = "Field `SETUP_LONG` reader - Length of the Setup time into read mode when the device is recovering from one of the following conditions: Moving from Power-down or Standby back to Active and device is not trimmed. Moving from standby to active state in low-frequency active mode. Recovering from the LDO Boost operation after a Mass Erase."]
pub type SetupLongR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Configures the length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - Length of the IREF_BOOST1 signal of the IP"]
    #[inline(always)]
    pub fn iref_boost1(&self) -> IrefBoost1R {
        IrefBoost1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Length of the Setup time into read mode when the device is recovering from one of the following conditions: Moving from Power-down or Standby back to Active and device is not trimmed. Moving from standby to active state in low-frequency active mode. Recovering from the LDO Boost operation after a Mass Erase."]
    #[inline(always)]
    pub fn setup_long(&self) -> SetupLongR {
        SetupLongR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Read Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_read_timctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlReadTimctlSpec;
impl crate::RegisterSpec for FlctlReadTimctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_read_timctl::R`](R) reader structure"]
impl crate::Readable for FlctlReadTimctlSpec {}
