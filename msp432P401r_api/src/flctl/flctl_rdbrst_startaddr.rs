#[doc = "Register `FLCTL_RDBRST_STARTADDR` reader"]
pub type R = crate::R<FlctlRdbrstStartaddrSpec>;
#[doc = "Register `FLCTL_RDBRST_STARTADDR` writer"]
pub type W = crate::W<FlctlRdbrstStartaddrSpec>;
#[doc = "Field `START_ADDRESS` reader - Start Address of Burst Operation"]
pub type StartAddressR = crate::FieldReader<u32>;
#[doc = "Field `START_ADDRESS` writer - Start Address of Burst Operation"]
pub type StartAddressW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - Start Address of Burst Operation"]
    #[inline(always)]
    pub fn start_address(&self) -> StartAddressR {
        StartAddressR::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - Start Address of Burst Operation"]
    #[inline(always)]
    pub fn start_address(&mut self) -> StartAddressW<FlctlRdbrstStartaddrSpec> {
        StartAddressW::new(self, 0)
    }
}
#[doc = "Read Burst/Compare Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_rdbrst_startaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_rdbrst_startaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlRdbrstStartaddrSpec;
impl crate::RegisterSpec for FlctlRdbrstStartaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_rdbrst_startaddr::R`](R) reader structure"]
impl crate::Readable for FlctlRdbrstStartaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_rdbrst_startaddr::W`](W) writer structure"]
impl crate::Writable for FlctlRdbrstStartaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_RDBRST_STARTADDR to value 0"]
impl crate::Resettable for FlctlRdbrstStartaddrSpec {
    const RESET_VALUE: u32 = 0;
}
