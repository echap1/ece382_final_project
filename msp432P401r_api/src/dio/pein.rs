#[doc = "Register `PEIN` reader"]
pub type R = crate::R<PeinSpec>;
#[doc = "Field `P9IN` reader - Port 9 Input"]
pub type P9inR = crate::FieldReader;
#[doc = "Field `P10IN` reader - Port 10 Input"]
pub type P10inR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Port 9 Input"]
    #[inline(always)]
    pub fn p9in(&self) -> P9inR {
        P9inR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Input"]
    #[inline(always)]
    pub fn p10in(&self) -> P10inR {
        P10inR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Port E Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pein::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeinSpec;
impl crate::RegisterSpec for PeinSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pein::R`](R) reader structure"]
impl crate::Readable for PeinSpec {}
#[doc = "`reset()` method sets PEIN to value 0"]
impl crate::Resettable for PeinSpec {
    const RESET_VALUE: u16 = 0;
}
