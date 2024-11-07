#[doc = "Register `FLCTL_PRGBRST_DATA2_2` reader"]
pub type R = crate::R<FlctlPrgbrstData2_2Spec>;
#[doc = "Register `FLCTL_PRGBRST_DATA2_2` writer"]
pub type W = crate::W<FlctlPrgbrstData2_2Spec>;
#[doc = "Field `DATAIN` reader - Program Burst 128 bit Data Word 2"]
pub type DatainR = crate::FieldReader<u32>;
#[doc = "Field `DATAIN` writer - Program Burst 128 bit Data Word 2"]
pub type DatainW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Program Burst 128 bit Data Word 2"]
    #[inline(always)]
    pub fn datain(&self) -> DatainR {
        DatainR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Program Burst 128 bit Data Word 2"]
    #[inline(always)]
    pub fn datain(&mut self) -> DatainW<FlctlPrgbrstData2_2Spec> {
        DatainW::new(self, 0)
    }
}
#[doc = "Program Burst Data2 Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data2_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data2_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlPrgbrstData2_2Spec;
impl crate::RegisterSpec for FlctlPrgbrstData2_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_prgbrst_data2_2::R`](R) reader structure"]
impl crate::Readable for FlctlPrgbrstData2_2Spec {}
#[doc = "`write(|w| ..)` method takes [`flctl_prgbrst_data2_2::W`](W) writer structure"]
impl crate::Writable for FlctlPrgbrstData2_2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_PRGBRST_DATA2_2 to value 0xffff_ffff"]
impl crate::Resettable for FlctlPrgbrstData2_2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
