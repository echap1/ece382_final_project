#[doc = "Register `ADC14_REF2P5V_TS30C` reader"]
pub type R = crate::R<Adc14Ref2p5vTs30cSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ADC14 2.5V Reference Temp. Sensor 30C\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14_ref2p5v_ts30c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14Ref2p5vTs30cSpec;
impl crate::RegisterSpec for Adc14Ref2p5vTs30cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14_ref2p5v_ts30c::R`](R) reader structure"]
impl crate::Readable for Adc14Ref2p5vTs30cSpec {}
#[doc = "`reset()` method sets ADC14_REF2P5V_TS30C to value 0"]
impl crate::Resettable for Adc14Ref2p5vTs30cSpec {
    const RESET_VALUE: u32 = 0;
}
