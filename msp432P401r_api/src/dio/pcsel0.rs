#[doc = "Register `PCSEL0` reader"]
pub type R = crate::R<Pcsel0Spec>;
#[doc = "Register `PCSEL0` writer"]
pub type W = crate::W<Pcsel0Spec>;
#[doc = "Field `P5SEL0` reader - Port 5 Select 0"]
pub type P5sel0R = crate::FieldReader;
#[doc = "Field `P5SEL0` writer - Port 5 Select 0"]
pub type P5sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P6SEL0` reader - Port 6 Select 0"]
pub type P6sel0R = crate::FieldReader;
#[doc = "Field `P6SEL0` writer - Port 6 Select 0"]
pub type P6sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Select 0"]
    #[inline(always)]
    pub fn p5sel0(&self) -> P5sel0R {
        P5sel0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Select 0"]
    #[inline(always)]
    pub fn p6sel0(&self) -> P6sel0R {
        P6sel0R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Select 0"]
    #[inline(always)]
    pub fn p5sel0(&mut self) -> P5sel0W<Pcsel0Spec> {
        P5sel0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 6 Select 0"]
    #[inline(always)]
    pub fn p6sel0(&mut self) -> P6sel0W<Pcsel0Spec> {
        P6sel0W::new(self, 8)
    }
}
#[doc = "Port C Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcsel0Spec;
impl crate::RegisterSpec for Pcsel0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcsel0::R`](R) reader structure"]
impl crate::Readable for Pcsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcsel0::W`](W) writer structure"]
impl crate::Writable for Pcsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PCSEL0 to value 0"]
impl crate::Resettable for Pcsel0Spec {
    const RESET_VALUE: u16 = 0;
}
