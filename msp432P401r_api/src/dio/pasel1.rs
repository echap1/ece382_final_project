#[doc = "Register `PASEL1` reader"]
pub type R = crate::R<Pasel1Spec>;
#[doc = "Register `PASEL1` writer"]
pub type W = crate::W<Pasel1Spec>;
#[doc = "Field `P1SEL1` reader - Port 1 Select 1"]
pub type P1sel1R = crate::FieldReader;
#[doc = "Field `P1SEL1` writer - Port 1 Select 1"]
pub type P1sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P2SEL1` reader - Port 2 Select 1"]
pub type P2sel1R = crate::FieldReader;
#[doc = "Field `P2SEL1` writer - Port 2 Select 1"]
pub type P2sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Select 1"]
    #[inline(always)]
    pub fn p1sel1(&self) -> P1sel1R {
        P1sel1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Select 1"]
    #[inline(always)]
    pub fn p2sel1(&self) -> P2sel1R {
        P2sel1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Select 1"]
    #[inline(always)]
    pub fn p1sel1(&mut self) -> P1sel1W<Pasel1Spec> {
        P1sel1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 2 Select 1"]
    #[inline(always)]
    pub fn p2sel1(&mut self) -> P2sel1W<Pasel1Spec> {
        P2sel1W::new(self, 8)
    }
}
#[doc = "Port A Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pasel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pasel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pasel1Spec;
impl crate::RegisterSpec for Pasel1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pasel1::R`](R) reader structure"]
impl crate::Readable for Pasel1Spec {}
#[doc = "`write(|w| ..)` method takes [`pasel1::W`](W) writer structure"]
impl crate::Writable for Pasel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PASEL1 to value 0"]
impl crate::Resettable for Pasel1Spec {
    const RESET_VALUE: u16 = 0;
}
