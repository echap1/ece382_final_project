#[doc = "Register `PDSEL1` reader"]
pub type R = crate::R<Pdsel1Spec>;
#[doc = "Register `PDSEL1` writer"]
pub type W = crate::W<Pdsel1Spec>;
#[doc = "Field `P7SEL1` reader - Port 7 Select 1"]
pub type P7sel1R = crate::FieldReader;
#[doc = "Field `P7SEL1` writer - Port 7 Select 1"]
pub type P7sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P8SEL1` reader - Port 8 Select 1"]
pub type P8sel1R = crate::FieldReader;
#[doc = "Field `P8SEL1` writer - Port 8 Select 1"]
pub type P8sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Select 1"]
    #[inline(always)]
    pub fn p7sel1(&self) -> P7sel1R {
        P7sel1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Select 1"]
    #[inline(always)]
    pub fn p8sel1(&self) -> P8sel1R {
        P8sel1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Select 1"]
    #[inline(always)]
    pub fn p7sel1(&mut self) -> P7sel1W<Pdsel1Spec> {
        P7sel1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 8 Select 1"]
    #[inline(always)]
    pub fn p8sel1(&mut self) -> P8sel1W<Pdsel1Spec> {
        P8sel1W::new(self, 8)
    }
}
#[doc = "Port D Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdsel1Spec;
impl crate::RegisterSpec for Pdsel1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdsel1::R`](R) reader structure"]
impl crate::Readable for Pdsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`pdsel1::W`](W) writer structure"]
impl crate::Writable for Pdsel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDSEL1 to value 0"]
impl crate::Resettable for Pdsel1Spec {
    const RESET_VALUE: u16 = 0;
}
