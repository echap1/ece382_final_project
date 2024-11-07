#[doc = "Register `PAIES` reader"]
pub type R = crate::R<PaiesSpec>;
#[doc = "Register `PAIES` writer"]
pub type W = crate::W<PaiesSpec>;
#[doc = "Field `P1IES` reader - Port 1 Interrupt Edge Select"]
pub type P1iesR = crate::FieldReader;
#[doc = "Field `P1IES` writer - Port 1 Interrupt Edge Select"]
pub type P1iesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P2IES` reader - Port 2 Interrupt Edge Select"]
pub type P2iesR = crate::FieldReader;
#[doc = "Field `P2IES` writer - Port 2 Interrupt Edge Select"]
pub type P2iesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p1ies(&self) -> P1iesR {
        P1iesR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p2ies(&self) -> P2iesR {
        P2iesR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p1ies(&mut self) -> P1iesW<PaiesSpec> {
        P1iesW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p2ies(&mut self) -> P2iesW<PaiesSpec> {
        P2iesW::new(self, 8)
    }
}
#[doc = "Port A Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`paies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaiesSpec;
impl crate::RegisterSpec for PaiesSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`paies::R`](R) reader structure"]
impl crate::Readable for PaiesSpec {}
#[doc = "`write(|w| ..)` method takes [`paies::W`](W) writer structure"]
impl crate::Writable for PaiesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PAIES to value 0"]
impl crate::Resettable for PaiesSpec {
    const RESET_VALUE: u16 = 0;
}
