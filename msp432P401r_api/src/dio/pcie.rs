#[doc = "Register `PCIE` reader"]
pub type R = crate::R<PcieSpec>;
#[doc = "Register `PCIE` writer"]
pub type W = crate::W<PcieSpec>;
#[doc = "Field `P5IE` reader - Port 5 Interrupt Enable"]
pub type P5ieR = crate::FieldReader;
#[doc = "Field `P5IE` writer - Port 5 Interrupt Enable"]
pub type P5ieW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P6IE` reader - Port 6 Interrupt Enable"]
pub type P6ieR = crate::FieldReader;
#[doc = "Field `P6IE` writer - Port 6 Interrupt Enable"]
pub type P6ieW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Interrupt Enable"]
    #[inline(always)]
    pub fn p5ie(&self) -> P5ieR {
        P5ieR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Enable"]
    #[inline(always)]
    pub fn p6ie(&self) -> P6ieR {
        P6ieR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Interrupt Enable"]
    #[inline(always)]
    pub fn p5ie(&mut self) -> P5ieW<PcieSpec> {
        P5ieW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Enable"]
    #[inline(always)]
    pub fn p6ie(&mut self) -> P6ieW<PcieSpec> {
        P6ieW::new(self, 8)
    }
}
#[doc = "Port C Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieSpec;
impl crate::RegisterSpec for PcieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcie::R`](R) reader structure"]
impl crate::Readable for PcieSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie::W`](W) writer structure"]
impl crate::Writable for PcieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PCIE to value 0"]
impl crate::Resettable for PcieSpec {
    const RESET_VALUE: u16 = 0;
}
