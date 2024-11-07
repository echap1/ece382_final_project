#[doc = "Register `PESEL1` reader"]
pub type R = crate::R<Pesel1Spec>;
#[doc = "Register `PESEL1` writer"]
pub type W = crate::W<Pesel1Spec>;
#[doc = "Field `P9SEL1` reader - Port 9 Select 1"]
pub type P9sel1R = crate::FieldReader;
#[doc = "Field `P9SEL1` writer - Port 9 Select 1"]
pub type P9sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P10SEL1` reader - Port 10 Select 1"]
pub type P10sel1R = crate::FieldReader;
#[doc = "Field `P10SEL1` writer - Port 10 Select 1"]
pub type P10sel1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Select 1"]
    #[inline(always)]
    pub fn p9sel1(&self) -> P9sel1R {
        P9sel1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Select 1"]
    #[inline(always)]
    pub fn p10sel1(&self) -> P10sel1R {
        P10sel1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Select 1"]
    #[inline(always)]
    pub fn p9sel1(&mut self) -> P9sel1W<Pesel1Spec> {
        P9sel1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 10 Select 1"]
    #[inline(always)]
    pub fn p10sel1(&mut self) -> P10sel1W<Pesel1Spec> {
        P10sel1W::new(self, 8)
    }
}
#[doc = "Port E Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pesel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pesel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pesel1Spec;
impl crate::RegisterSpec for Pesel1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pesel1::R`](R) reader structure"]
impl crate::Readable for Pesel1Spec {}
#[doc = "`write(|w| ..)` method takes [`pesel1::W`](W) writer structure"]
impl crate::Writable for Pesel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PESEL1 to value 0"]
impl crate::Resettable for Pesel1Spec {
    const RESET_VALUE: u16 = 0;
}
