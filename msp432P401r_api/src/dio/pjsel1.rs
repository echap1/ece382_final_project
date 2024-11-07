#[doc = "Register `PJSEL1` reader"]
pub type R = crate::R<Pjsel1Spec>;
#[doc = "Register `PJSEL1` writer"]
pub type W = crate::W<Pjsel1Spec>;
#[doc = "Field `PJSEL1` reader - Port J Select 1"]
pub type Pjsel1R = crate::FieldReader<u16>;
#[doc = "Field `PJSEL1` writer - Port J Select 1"]
pub type Pjsel1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port J Select 1"]
    #[inline(always)]
    pub fn pjsel1(&self) -> Pjsel1R {
        Pjsel1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Select 1"]
    #[inline(always)]
    pub fn pjsel1(&mut self) -> Pjsel1W<Pjsel1Spec> {
        Pjsel1W::new(self, 0)
    }
}
#[doc = "Port J Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pjsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pjsel1Spec;
impl crate::RegisterSpec for Pjsel1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pjsel1::R`](R) reader structure"]
impl crate::Readable for Pjsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`pjsel1::W`](W) writer structure"]
impl crate::Writable for Pjsel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PJSEL1 to value 0"]
impl crate::Resettable for Pjsel1Spec {
    const RESET_VALUE: u16 = 0;
}
