#[doc = "Register `FLCTL_MASSERASE_TIMCTL` reader"]
pub type R = crate::R<FlctlMasseraseTimctlSpec>;
#[doc = "Field `BOOST_ACTIVE` reader - Length of the time for which LDO Boost Signal is kept active"]
pub type BoostActiveR = crate::FieldReader;
#[doc = "Field `BOOST_HOLD` reader - Length for which Flash deactivates the LDO Boost signal before processing any new commands"]
pub type BoostHoldR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Length of the time for which LDO Boost Signal is kept active"]
    #[inline(always)]
    pub fn boost_active(&self) -> BoostActiveR {
        BoostActiveR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Length for which Flash deactivates the LDO Boost signal before processing any new commands"]
    #[inline(always)]
    pub fn boost_hold(&self) -> BoostHoldR {
        BoostHoldR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Mass Erase Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_masserase_timctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlMasseraseTimctlSpec;
impl crate::RegisterSpec for FlctlMasseraseTimctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_masserase_timctl::R`](R) reader structure"]
impl crate::Readable for FlctlMasseraseTimctlSpec {}
