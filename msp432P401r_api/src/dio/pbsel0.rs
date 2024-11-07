#[doc = "Register `PBSEL0` reader"]
pub type R = crate::R<Pbsel0Spec>;
#[doc = "Register `PBSEL0` writer"]
pub type W = crate::W<Pbsel0Spec>;
#[doc = "Field `P3SEL0` reader - Port 3 Select 0"]
pub type P3sel0R = crate::FieldReader;
#[doc = "Field `P3SEL0` writer - Port 3 Select 0"]
pub type P3sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P4SEL0` reader - Port 4 Select 0"]
pub type P4sel0R = crate::FieldReader;
#[doc = "Field `P4SEL0` writer - Port 4 Select 0"]
pub type P4sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Select 0"]
    #[inline(always)]
    pub fn p3sel0(&self) -> P3sel0R {
        P3sel0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Select 0"]
    #[inline(always)]
    pub fn p4sel0(&self) -> P4sel0R {
        P4sel0R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Select 0"]
    #[inline(always)]
    pub fn p3sel0(&mut self) -> P3sel0W<Pbsel0Spec> {
        P3sel0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 4 Select 0"]
    #[inline(always)]
    pub fn p4sel0(&mut self) -> P4sel0W<Pbsel0Spec> {
        P4sel0W::new(self, 8)
    }
}
#[doc = "Port B Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pbsel0Spec;
impl crate::RegisterSpec for Pbsel0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbsel0::R`](R) reader structure"]
impl crate::Readable for Pbsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`pbsel0::W`](W) writer structure"]
impl crate::Writable for Pbsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBSEL0 to value 0"]
impl crate::Resettable for Pbsel0Spec {
    const RESET_VALUE: u16 = 0;
}
