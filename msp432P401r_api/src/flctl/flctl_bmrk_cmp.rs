#[doc = "Register `FLCTL_BMRK_CMP` reader"]
pub type R = crate::R<FlctlBmrkCmpSpec>;
#[doc = "Register `FLCTL_BMRK_CMP` writer"]
pub type W = crate::W<FlctlBmrkCmpSpec>;
#[doc = "Field `COUNT` reader - Reflects the threshold value that is compared against either the IFETCH or DREAD Benchmark Counters"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - Reflects the threshold value that is compared against either the IFETCH or DREAD Benchmark Counters"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reflects the threshold value that is compared against either the IFETCH or DREAD Benchmark Counters"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reflects the threshold value that is compared against either the IFETCH or DREAD Benchmark Counters"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<FlctlBmrkCmpSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Benchmark Count Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bmrk_cmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bmrk_cmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlBmrkCmpSpec;
impl crate::RegisterSpec for FlctlBmrkCmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_bmrk_cmp::R`](R) reader structure"]
impl crate::Readable for FlctlBmrkCmpSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_bmrk_cmp::W`](W) writer structure"]
impl crate::Writable for FlctlBmrkCmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_BMRK_CMP to value 0x0001_0000"]
impl crate::Resettable for FlctlBmrkCmpSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
