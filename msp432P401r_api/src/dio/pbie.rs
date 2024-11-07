#[doc = "Register `PBIE` reader"]
pub type R = crate::R<PbieSpec>;
#[doc = "Register `PBIE` writer"]
pub type W = crate::W<PbieSpec>;
#[doc = "Field `P3IE` reader - Port 3 Interrupt Enable"]
pub type P3ieR = crate::FieldReader;
#[doc = "Field `P3IE` writer - Port 3 Interrupt Enable"]
pub type P3ieW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P4IE` reader - Port 4 Interrupt Enable"]
pub type P4ieR = crate::FieldReader;
#[doc = "Field `P4IE` writer - Port 4 Interrupt Enable"]
pub type P4ieW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Interrupt Enable"]
    #[inline(always)]
    pub fn p3ie(&self) -> P3ieR {
        P3ieR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Enable"]
    #[inline(always)]
    pub fn p4ie(&self) -> P4ieR {
        P4ieR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Interrupt Enable"]
    #[inline(always)]
    pub fn p3ie(&mut self) -> P3ieW<PbieSpec> {
        P3ieW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Enable"]
    #[inline(always)]
    pub fn p4ie(&mut self) -> P4ieW<PbieSpec> {
        P4ieW::new(self, 8)
    }
}
#[doc = "Port B Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pbie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbieSpec;
impl crate::RegisterSpec for PbieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbie::R`](R) reader structure"]
impl crate::Readable for PbieSpec {}
#[doc = "`write(|w| ..)` method takes [`pbie::W`](W) writer structure"]
impl crate::Writable for PbieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBIE to value 0"]
impl crate::Resettable for PbieSpec {
    const RESET_VALUE: u16 = 0;
}
