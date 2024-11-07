#[doc = "Register `PBSEL1` reader"]
pub type R = crate::R<Pbsel1Spec>;
#[doc = "Register `PBSEL1` writer"]
pub type W = crate::W<Pbsel1Spec>;
#[doc = "Field `P3SEL1` reader - Port 3 Select 1"]
pub type P3sel1R = crate::FieldReader;
#[doc = "Field `P3SEL1` writer - Port 3 Select 1"]
pub type P3sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P4SEL1` reader - Port 4 Select 1"]
pub type P4sel1R = crate::FieldReader;
#[doc = "Field `P4SEL1` writer - Port 4 Select 1"]
pub type P4sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Select 1"]
    #[inline(always)]
    pub fn p3sel1(&self) -> P3sel1R {
        P3sel1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Select 1"]
    #[inline(always)]
    pub fn p4sel1(&self) -> P4sel1R {
        P4sel1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Select 1"]
    #[inline(always)]
    pub fn p3sel1(&mut self) -> P3sel1W<Pbsel1Spec> {
        P3sel1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 4 Select 1"]
    #[inline(always)]
    pub fn p4sel1(&mut self) -> P4sel1W<Pbsel1Spec> {
        P4sel1W::new(self, 8)
    }
}
#[doc = "Port B Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pbsel1Spec;
impl crate::RegisterSpec for Pbsel1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbsel1::R`](R) reader structure"]
impl crate::Readable for Pbsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`pbsel1::W`](W) writer structure"]
impl crate::Writable for Pbsel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBSEL1 to value 0"]
impl crate::Resettable for Pbsel1Spec {
    const RESET_VALUE: u16 = 0;
}
