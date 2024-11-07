#[doc = "Register `FLCTL_RDBRST_FAILADDR` reader"]
pub type R = crate::R<FlctlRdbrstFailaddrSpec>;
#[doc = "Register `FLCTL_RDBRST_FAILADDR` writer"]
pub type W = crate::W<FlctlRdbrstFailaddrSpec>;
#[doc = "Field `FAIL_ADDRESS` reader - Reflects address of last failed compare"]
pub type FailAddressR = crate::FieldReader<u32>;
#[doc = "Field `FAIL_ADDRESS` writer - Reflects address of last failed compare"]
pub type FailAddressW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - Reflects address of last failed compare"]
    #[inline(always)]
    pub fn fail_address(&self) -> FailAddressR {
        FailAddressR::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - Reflects address of last failed compare"]
    #[inline(always)]
    pub fn fail_address(&mut self) -> FailAddressW<FlctlRdbrstFailaddrSpec> {
        FailAddressW::new(self, 0)
    }
}
#[doc = "Read Burst/Compare Fail Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_rdbrst_failaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_rdbrst_failaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlRdbrstFailaddrSpec;
impl crate::RegisterSpec for FlctlRdbrstFailaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_rdbrst_failaddr::R`](R) reader structure"]
impl crate::Readable for FlctlRdbrstFailaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_rdbrst_failaddr::W`](W) writer structure"]
impl crate::Writable for FlctlRdbrstFailaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_RDBRST_FAILADDR to value 0"]
impl crate::Resettable for FlctlRdbrstFailaddrSpec {
    const RESET_VALUE: u32 = 0;
}
