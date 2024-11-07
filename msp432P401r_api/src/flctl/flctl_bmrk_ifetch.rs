#[doc = "Register `FLCTL_BMRK_IFETCH` reader"]
pub type R = crate::R<FlctlBmrkIfetchSpec>;
#[doc = "Register `FLCTL_BMRK_IFETCH` writer"]
pub type W = crate::W<FlctlBmrkIfetchSpec>;
#[doc = "Field `COUNT` reader - Reflects the number of Instruction Fetches to the Flash (increments by one on each fetch)"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Reflects the number of Instruction Fetches to the Flash (increments by one on each fetch)"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reflects the number of Instruction Fetches to the Flash (increments by one on each fetch)"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reflects the number of Instruction Fetches to the Flash (increments by one on each fetch)"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<FlctlBmrkIfetchSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Benchmark Instruction Fetch Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bmrk_ifetch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bmrk_ifetch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlBmrkIfetchSpec;
impl crate::RegisterSpec for FlctlBmrkIfetchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_bmrk_ifetch::R`](R) reader structure"]
impl crate::Readable for FlctlBmrkIfetchSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_bmrk_ifetch::W`](W) writer structure"]
impl crate::Writable for FlctlBmrkIfetchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_BMRK_IFETCH to value 0"]
impl crate::Resettable for FlctlBmrkIfetchSpec {
    const RESET_VALUE: u32 = 0;
}
