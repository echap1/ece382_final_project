#[doc = "Register `ADC14_CAL_TAG` reader"]
pub type R = crate::R<Adc14CalTagSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ADC14 Calibration Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14_cal_tag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14CalTagSpec;
impl crate::RegisterSpec for Adc14CalTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14_cal_tag::R`](R) reader structure"]
impl crate::Readable for Adc14CalTagSpec {}
#[doc = "`reset()` method sets ADC14_CAL_TAG to value 0x05"]
impl crate::Resettable for Adc14CalTagSpec {
    const RESET_VALUE: u32 = 0x05;
}
