#[doc = "Register `FLCTL_RDBRST_FAILCNT` reader"]
pub type R = crate::R<FlctlRdbrstFailcntSpec>;
#[doc = "Register `FLCTL_RDBRST_FAILCNT` writer"]
pub type W = crate::W<FlctlRdbrstFailcntSpec>;
#[doc = "Field `FAIL_COUNT` reader - Number of failures encountered in burst operation"]
pub type FailCountR = crate::FieldReader<u32>;
#[doc = "Field `FAIL_COUNT` writer - Number of failures encountered in burst operation"]
pub type FailCountW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - Number of failures encountered in burst operation"]
    #[inline(always)]
    pub fn fail_count(&self) -> FailCountR {
        FailCountR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Number of failures encountered in burst operation"]
    #[inline(always)]
    pub fn fail_count(&mut self) -> FailCountW<FlctlRdbrstFailcntSpec> {
        FailCountW::new(self, 0)
    }
}
#[doc = "Read Burst/Compare Fail Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_rdbrst_failcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_rdbrst_failcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlRdbrstFailcntSpec;
impl crate::RegisterSpec for FlctlRdbrstFailcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_rdbrst_failcnt::R`](R) reader structure"]
impl crate::Readable for FlctlRdbrstFailcntSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_rdbrst_failcnt::W`](W) writer structure"]
impl crate::Writable for FlctlRdbrstFailcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_RDBRST_FAILCNT to value 0"]
impl crate::Resettable for FlctlRdbrstFailcntSpec {
    const RESET_VALUE: u32 = 0;
}
