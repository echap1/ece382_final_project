#[doc = "Register `PBIN` reader"]
pub type R = crate::R<PbinSpec>;
#[doc = "Field `P3IN` reader - Port 3 Input"]
pub type P3inR = crate::FieldReader;
#[doc = "Field `P4IN` reader - Port 4 Input"]
pub type P4inR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Port 3 Input"]
    #[inline(always)]
    pub fn p3in(&self) -> P3inR {
        P3inR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Input"]
    #[inline(always)]
    pub fn p4in(&self) -> P4inR {
        P4inR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Port B Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pbin::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbinSpec;
impl crate::RegisterSpec for PbinSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbin::R`](R) reader structure"]
impl crate::Readable for PbinSpec {}
#[doc = "`reset()` method sets PBIN to value 0"]
impl crate::Resettable for PbinSpec {
    const RESET_VALUE: u16 = 0;
}
