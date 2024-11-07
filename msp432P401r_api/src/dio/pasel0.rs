#[doc = "Register `PASEL0` reader"]
pub type R = crate::R<Pasel0Spec>;
#[doc = "Register `PASEL0` writer"]
pub type W = crate::W<Pasel0Spec>;
#[doc = "Field `P1SEL0` reader - Port 1 Select 0"]
pub type P1sel0R = crate::FieldReader;
#[doc = "Field `P1SEL0` writer - Port 1 Select 0"]
pub type P1sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P2SEL0` reader - Port 2 Select 0"]
pub type P2sel0R = crate::FieldReader;
#[doc = "Field `P2SEL0` writer - Port 2 Select 0"]
pub type P2sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Select 0"]
    #[inline(always)]
    pub fn p1sel0(&self) -> P1sel0R {
        P1sel0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Select 0"]
    #[inline(always)]
    pub fn p2sel0(&self) -> P2sel0R {
        P2sel0R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Select 0"]
    #[inline(always)]
    pub fn p1sel0(&mut self) -> P1sel0W<Pasel0Spec> {
        P1sel0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 2 Select 0"]
    #[inline(always)]
    pub fn p2sel0(&mut self) -> P2sel0W<Pasel0Spec> {
        P2sel0W::new(self, 8)
    }
}
#[doc = "Port A Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pasel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pasel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pasel0Spec;
impl crate::RegisterSpec for Pasel0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pasel0::R`](R) reader structure"]
impl crate::Readable for Pasel0Spec {}
#[doc = "`write(|w| ..)` method takes [`pasel0::W`](W) writer structure"]
impl crate::Writable for Pasel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PASEL0 to value 0"]
impl crate::Resettable for Pasel0Spec {
    const RESET_VALUE: u16 = 0;
}
