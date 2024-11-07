#[doc = "Register `ADC14_CAL_LEN` reader"]
pub type R = crate::R<Adc14CalLenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ADC14 Calibration Length\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14_cal_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14CalLenSpec;
impl crate::RegisterSpec for Adc14CalLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14_cal_len::R`](R) reader structure"]
impl crate::Readable for Adc14CalLenSpec {}
#[doc = "`reset()` method sets ADC14_CAL_LEN to value 0"]
impl crate::Resettable for Adc14CalLenSpec {
    const RESET_VALUE: u32 = 0;
}
