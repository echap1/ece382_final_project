#[doc = "Register `FLCTL_RDBRST_LEN` reader"]
pub type R = crate::R<FlctlRdbrstLenSpec>;
#[doc = "Register `FLCTL_RDBRST_LEN` writer"]
pub type W = crate::W<FlctlRdbrstLenSpec>;
#[doc = "Field `BURST_LENGTH` reader - Length of Burst Operation"]
pub type BurstLengthR = crate::FieldReader<u32>;
#[doc = "Field `BURST_LENGTH` writer - Length of Burst Operation"]
pub type BurstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - Length of Burst Operation"]
    #[inline(always)]
    pub fn burst_length(&self) -> BurstLengthR {
        BurstLengthR::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - Length of Burst Operation"]
    #[inline(always)]
    pub fn burst_length(&mut self) -> BurstLengthW<FlctlRdbrstLenSpec> {
        BurstLengthW::new(self, 0)
    }
}
#[doc = "Read Burst/Compare Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_rdbrst_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_rdbrst_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlRdbrstLenSpec;
impl crate::RegisterSpec for FlctlRdbrstLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_rdbrst_len::R`](R) reader structure"]
impl crate::Readable for FlctlRdbrstLenSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_rdbrst_len::W`](W) writer structure"]
impl crate::Writable for FlctlRdbrstLenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_RDBRST_LEN to value 0"]
impl crate::Resettable for FlctlRdbrstLenSpec {
    const RESET_VALUE: u32 = 0;
}
