#[doc = "Register `PCSEL1` reader"]
pub type R = crate::R<Pcsel1Spec>;
#[doc = "Register `PCSEL1` writer"]
pub type W = crate::W<Pcsel1Spec>;
#[doc = "Field `P5SEL1` reader - Port 5 Select 1"]
pub type P5sel1R = crate::FieldReader;
#[doc = "Field `P5SEL1` writer - Port 5 Select 1"]
pub type P5sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P6SEL1` reader - Port 6 Select 1"]
pub type P6sel1R = crate::FieldReader;
#[doc = "Field `P6SEL1` writer - Port 6 Select 1"]
pub type P6sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Select 1"]
    #[inline(always)]
    pub fn p5sel1(&self) -> P5sel1R {
        P5sel1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Select 1"]
    #[inline(always)]
    pub fn p6sel1(&self) -> P6sel1R {
        P6sel1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Select 1"]
    #[inline(always)]
    pub fn p5sel1(&mut self) -> P5sel1W<Pcsel1Spec> {
        P5sel1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 6 Select 1"]
    #[inline(always)]
    pub fn p6sel1(&mut self) -> P6sel1W<Pcsel1Spec> {
        P6sel1W::new(self, 8)
    }
}
#[doc = "Port C Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcsel1Spec;
impl crate::RegisterSpec for Pcsel1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcsel1::R`](R) reader structure"]
impl crate::Readable for Pcsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcsel1::W`](W) writer structure"]
impl crate::Writable for Pcsel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PCSEL1 to value 0"]
impl crate::Resettable for Pcsel1Spec {
    const RESET_VALUE: u16 = 0;
}
