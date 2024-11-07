#[doc = "Register `PDIN` reader"]
pub type R = crate::R<PdinSpec>;
#[doc = "Field `P7IN` reader - Port 7 Input"]
pub type P7inR = crate::FieldReader;
#[doc = "Field `P8IN` reader - Port 8 Input"]
pub type P8inR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Port 7 Input"]
    #[inline(always)]
    pub fn p7in(&self) -> P7inR {
        P7inR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Input"]
    #[inline(always)]
    pub fn p8in(&self) -> P8inR {
        P8inR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Port D Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pdin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdinSpec;
impl crate::RegisterSpec for PdinSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdin::R`](R) reader structure"]
impl crate::Readable for PdinSpec {}
#[doc = "`reset()` method sets PDIN to value 0"]
impl crate::Resettable for PdinSpec {
    const RESET_VALUE: u16 = 0;
}
