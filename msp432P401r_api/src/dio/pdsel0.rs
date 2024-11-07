#[doc = "Register `PDSEL0` reader"]
pub type R = crate::R<Pdsel0Spec>;
#[doc = "Register `PDSEL0` writer"]
pub type W = crate::W<Pdsel0Spec>;
#[doc = "Field `P7SEL0` reader - Port 7 Select 0"]
pub type P7sel0R = crate::FieldReader;
#[doc = "Field `P7SEL0` writer - Port 7 Select 0"]
pub type P7sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P8SEL0` reader - Port 8 Select 0"]
pub type P8sel0R = crate::FieldReader;
#[doc = "Field `P8SEL0` writer - Port 8 Select 0"]
pub type P8sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Select 0"]
    #[inline(always)]
    pub fn p7sel0(&self) -> P7sel0R {
        P7sel0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Select 0"]
    #[inline(always)]
    pub fn p8sel0(&self) -> P8sel0R {
        P8sel0R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Select 0"]
    #[inline(always)]
    pub fn p7sel0(&mut self) -> P7sel0W<Pdsel0Spec> {
        P7sel0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 8 Select 0"]
    #[inline(always)]
    pub fn p8sel0(&mut self) -> P8sel0W<Pdsel0Spec> {
        P8sel0W::new(self, 8)
    }
}
#[doc = "Port D Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdsel0Spec;
impl crate::RegisterSpec for Pdsel0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdsel0::R`](R) reader structure"]
impl crate::Readable for Pdsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`pdsel0::W`](W) writer structure"]
impl crate::Writable for Pdsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDSEL0 to value 0"]
impl crate::Resettable for Pdsel0Spec {
    const RESET_VALUE: u16 = 0;
}
