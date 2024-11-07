#[doc = "Register `PEIFG` reader"]
pub type R = crate::R<PeifgSpec>;
#[doc = "Register `PEIFG` writer"]
pub type W = crate::W<PeifgSpec>;
#[doc = "Field `P9IFG` reader - Port 9 Interrupt Flag"]
pub type P9ifgR = crate::FieldReader;
#[doc = "Field `P9IFG` writer - Port 9 Interrupt Flag"]
pub type P9ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P10IFG` reader - Port 10 Interrupt Flag"]
pub type P10ifgR = crate::FieldReader;
#[doc = "Field `P10IFG` writer - Port 10 Interrupt Flag"]
pub type P10ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Interrupt Flag"]
    #[inline(always)]
    pub fn p9ifg(&self) -> P9ifgR {
        P9ifgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Flag"]
    #[inline(always)]
    pub fn p10ifg(&self) -> P10ifgR {
        P10ifgR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Interrupt Flag"]
    #[inline(always)]
    pub fn p9ifg(&mut self) -> P9ifgW<PeifgSpec> {
        P9ifgW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Flag"]
    #[inline(always)]
    pub fn p10ifg(&mut self) -> P10ifgW<PeifgSpec> {
        P10ifgW::new(self, 8)
    }
}
#[doc = "Port E Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`peifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeifgSpec;
impl crate::RegisterSpec for PeifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`peifg::R`](R) reader structure"]
impl crate::Readable for PeifgSpec {}
#[doc = "`write(|w| ..)` method takes [`peifg::W`](W) writer structure"]
impl crate::Writable for PeifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PEIFG to value 0"]
impl crate::Resettable for PeifgSpec {
    const RESET_VALUE: u16 = 0;
}
