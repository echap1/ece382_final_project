#[doc = "Register `FLCTL_BMRK_DREAD` reader"]
pub type R = crate::R<FlctlBmrkDreadSpec>;
#[doc = "Register `FLCTL_BMRK_DREAD` writer"]
pub type W = crate::W<FlctlBmrkDreadSpec>;
#[doc = "Field `COUNT` reader - Reflects the number of Data Read operations to the Flash (increments by one on each read)"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Reflects the number of Data Read operations to the Flash (increments by one on each read)"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reflects the number of Data Read operations to the Flash (increments by one on each read)"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reflects the number of Data Read operations to the Flash (increments by one on each read)"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<FlctlBmrkDreadSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Benchmark Data Read Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bmrk_dread::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bmrk_dread::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlBmrkDreadSpec;
impl crate::RegisterSpec for FlctlBmrkDreadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_bmrk_dread::R`](R) reader structure"]
impl crate::Readable for FlctlBmrkDreadSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_bmrk_dread::W`](W) writer structure"]
impl crate::Writable for FlctlBmrkDreadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_BMRK_DREAD to value 0"]
impl crate::Resettable for FlctlBmrkDreadSpec {
    const RESET_VALUE: u32 = 0;
}
