#[doc = "Register `PAIN` reader"]
pub type R = crate::R<PainSpec>;
#[doc = "Field `P1IN` reader - Port 1 Input"]
pub type P1inR = crate::FieldReader;
#[doc = "Field `P2IN` reader - Port 2 Input"]
pub type P2inR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Port 1 Input"]
    #[inline(always)]
    pub fn p1in(&self) -> P1inR {
        P1inR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Input"]
    #[inline(always)]
    pub fn p2in(&self) -> P2inR {
        P2inR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Port A Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pain::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PainSpec;
impl crate::RegisterSpec for PainSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pain::R`](R) reader structure"]
impl crate::Readable for PainSpec {}
#[doc = "`reset()` method sets PAIN to value 0"]
impl crate::Resettable for PainSpec {
    const RESET_VALUE: u16 = 0;
}
