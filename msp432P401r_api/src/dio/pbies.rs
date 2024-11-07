#[doc = "Register `PBIES` reader"]
pub type R = crate::R<PbiesSpec>;
#[doc = "Register `PBIES` writer"]
pub type W = crate::W<PbiesSpec>;
#[doc = "Field `P3IES` reader - Port 3 Interrupt Edge Select"]
pub type P3iesR = crate::FieldReader;
#[doc = "Field `P3IES` writer - Port 3 Interrupt Edge Select"]
pub type P3iesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P4IES` reader - Port 4 Interrupt Edge Select"]
pub type P4iesR = crate::FieldReader;
#[doc = "Field `P4IES` writer - Port 4 Interrupt Edge Select"]
pub type P4iesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p3ies(&self) -> P3iesR {
        P3iesR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p4ies(&self) -> P4iesR {
        P4iesR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p3ies(&mut self) -> P3iesW<PbiesSpec> {
        P3iesW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p4ies(&mut self) -> P4iesW<PbiesSpec> {
        P4iesW::new(self, 8)
    }
}
#[doc = "Port B Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pbies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbiesSpec;
impl crate::RegisterSpec for PbiesSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbies::R`](R) reader structure"]
impl crate::Readable for PbiesSpec {}
#[doc = "`write(|w| ..)` method takes [`pbies::W`](W) writer structure"]
impl crate::Writable for PbiesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBIES to value 0"]
impl crate::Resettable for PbiesSpec {
    const RESET_VALUE: u16 = 0;
}
