#[doc = "Register `P7MAP67` reader"]
pub type R = crate::R<P7map67Spec>;
#[doc = "Register `P7MAP67` writer"]
pub type W = crate::W<P7map67Spec>;
#[doc = "Field `PMAPx` reader - Selects secondary port function"]
pub type PmapxR = crate::FieldReader<u16>;
#[doc = "Field `PMAPx` writer - Selects secondary port function"]
pub type PmapxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Selects secondary port function"]
    #[inline(always)]
    pub fn pmapx(&self) -> PmapxR {
        PmapxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Selects secondary port function"]
    #[inline(always)]
    pub fn pmapx(&mut self) -> PmapxW<P7map67Spec> {
        PmapxW::new(self, 0)
    }
}
#[doc = "Port mapping register, P7.6 and P7.7\n\nYou can [`read`](crate::Reg::read) this register and get [`p7map67::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7map67::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7map67Spec;
impl crate::RegisterSpec for P7map67Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p7map67::R`](R) reader structure"]
impl crate::Readable for P7map67Spec {}
#[doc = "`write(|w| ..)` method takes [`p7map67::W`](W) writer structure"]
impl crate::Writable for P7map67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
