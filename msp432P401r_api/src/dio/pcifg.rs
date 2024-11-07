#[doc = "Register `PCIFG` reader"]
pub type R = crate::R<PcifgSpec>;
#[doc = "Register `PCIFG` writer"]
pub type W = crate::W<PcifgSpec>;
#[doc = "Field `P5IFG` reader - Port 5 Interrupt Flag"]
pub type P5ifgR = crate::FieldReader;
#[doc = "Field `P5IFG` writer - Port 5 Interrupt Flag"]
pub type P5ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P6IFG` reader - Port 6 Interrupt Flag"]
pub type P6ifgR = crate::FieldReader;
#[doc = "Field `P6IFG` writer - Port 6 Interrupt Flag"]
pub type P6ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Interrupt Flag"]
    #[inline(always)]
    pub fn p5ifg(&self) -> P5ifgR {
        P5ifgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Flag"]
    #[inline(always)]
    pub fn p6ifg(&self) -> P6ifgR {
        P6ifgR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Interrupt Flag"]
    #[inline(always)]
    pub fn p5ifg(&mut self) -> P5ifgW<PcifgSpec> {
        P5ifgW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Flag"]
    #[inline(always)]
    pub fn p6ifg(&mut self) -> P6ifgW<PcifgSpec> {
        P6ifgW::new(self, 8)
    }
}
#[doc = "Port C Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pcifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcifgSpec;
impl crate::RegisterSpec for PcifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcifg::R`](R) reader structure"]
impl crate::Readable for PcifgSpec {}
#[doc = "`write(|w| ..)` method takes [`pcifg::W`](W) writer structure"]
impl crate::Writable for PcifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PCIFG to value 0"]
impl crate::Resettable for PcifgSpec {
    const RESET_VALUE: u16 = 0;
}
