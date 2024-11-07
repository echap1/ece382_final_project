#[doc = "Register `PCIN` reader"]
pub type R = crate::R<PcinSpec>;
#[doc = "Field `P5IN` reader - Port 5 Input"]
pub type P5inR = crate::FieldReader;
#[doc = "Field `P6IN` reader - Port 6 Input"]
pub type P6inR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Port 5 Input"]
    #[inline(always)]
    pub fn p5in(&self) -> P5inR {
        P5inR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Input"]
    #[inline(always)]
    pub fn p6in(&self) -> P6inR {
        P6inR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Port C Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pcin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcinSpec;
impl crate::RegisterSpec for PcinSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcin::R`](R) reader structure"]
impl crate::Readable for PcinSpec {}
#[doc = "`reset()` method sets PCIN to value 0"]
impl crate::Resettable for PcinSpec {
    const RESET_VALUE: u16 = 0;
}
