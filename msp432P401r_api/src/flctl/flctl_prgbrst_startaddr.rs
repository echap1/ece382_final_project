#[doc = "Register `FLCTL_PRGBRST_STARTADDR` reader"]
pub type R = crate::R<FlctlPrgbrstStartaddrSpec>;
#[doc = "Register `FLCTL_PRGBRST_STARTADDR` writer"]
pub type W = crate::W<FlctlPrgbrstStartaddrSpec>;
#[doc = "Field `START_ADDRESS` reader - Start Address of Program Burst Operation"]
pub type StartAddressR = crate::FieldReader<u32>;
#[doc = "Field `START_ADDRESS` writer - Start Address of Program Burst Operation"]
pub type StartAddressW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Start Address of Program Burst Operation"]
    #[inline(always)]
    pub fn start_address(&self) -> StartAddressR {
        StartAddressR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Start Address of Program Burst Operation"]
    #[inline(always)]
    pub fn start_address(&mut self) -> StartAddressW<FlctlPrgbrstStartaddrSpec> {
        StartAddressW::new(self, 0)
    }
}
#[doc = "Program Burst Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_startaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_startaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlPrgbrstStartaddrSpec;
impl crate::RegisterSpec for FlctlPrgbrstStartaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_prgbrst_startaddr::R`](R) reader structure"]
impl crate::Readable for FlctlPrgbrstStartaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_prgbrst_startaddr::W`](W) writer structure"]
impl crate::Writable for FlctlPrgbrstStartaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_PRGBRST_STARTADDR to value 0"]
impl crate::Resettable for FlctlPrgbrstStartaddrSpec {
    const RESET_VALUE: u32 = 0;
}
