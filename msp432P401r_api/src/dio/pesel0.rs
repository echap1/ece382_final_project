#[doc = "Register `PESEL0` reader"]
pub type R = crate::R<Pesel0Spec>;
#[doc = "Register `PESEL0` writer"]
pub type W = crate::W<Pesel0Spec>;
#[doc = "Field `P9SEL0` reader - Port 9 Select 0"]
pub type P9sel0R = crate::FieldReader;
#[doc = "Field `P9SEL0` writer - Port 9 Select 0"]
pub type P9sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P10SEL0` reader - Port 10 Select 0"]
pub type P10sel0R = crate::FieldReader;
#[doc = "Field `P10SEL0` writer - Port 10 Select 0"]
pub type P10sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Select 0"]
    #[inline(always)]
    pub fn p9sel0(&self) -> P9sel0R {
        P9sel0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Select 0"]
    #[inline(always)]
    pub fn p10sel0(&self) -> P10sel0R {
        P10sel0R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Select 0"]
    #[inline(always)]
    pub fn p9sel0(&mut self) -> P9sel0W<Pesel0Spec> {
        P9sel0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 10 Select 0"]
    #[inline(always)]
    pub fn p10sel0(&mut self) -> P10sel0W<Pesel0Spec> {
        P10sel0W::new(self, 8)
    }
}
#[doc = "Port E Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pesel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pesel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pesel0Spec;
impl crate::RegisterSpec for Pesel0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pesel0::R`](R) reader structure"]
impl crate::Readable for Pesel0Spec {}
#[doc = "`write(|w| ..)` method takes [`pesel0::W`](W) writer structure"]
impl crate::Writable for Pesel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PESEL0 to value 0"]
impl crate::Resettable for Pesel0Spec {
    const RESET_VALUE: u16 = 0;
}
