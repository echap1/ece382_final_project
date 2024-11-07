#[doc = "Register `PBIFG` reader"]
pub type R = crate::R<PbifgSpec>;
#[doc = "Register `PBIFG` writer"]
pub type W = crate::W<PbifgSpec>;
#[doc = "Field `P3IFG` reader - Port 3 Interrupt Flag"]
pub type P3ifgR = crate::FieldReader;
#[doc = "Field `P3IFG` writer - Port 3 Interrupt Flag"]
pub type P3ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P4IFG` reader - Port 4 Interrupt Flag"]
pub type P4ifgR = crate::FieldReader;
#[doc = "Field `P4IFG` writer - Port 4 Interrupt Flag"]
pub type P4ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Interrupt Flag"]
    #[inline(always)]
    pub fn p3ifg(&self) -> P3ifgR {
        P3ifgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Flag"]
    #[inline(always)]
    pub fn p4ifg(&self) -> P4ifgR {
        P4ifgR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Interrupt Flag"]
    #[inline(always)]
    pub fn p3ifg(&mut self) -> P3ifgW<PbifgSpec> {
        P3ifgW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Flag"]
    #[inline(always)]
    pub fn p4ifg(&mut self) -> P4ifgW<PbifgSpec> {
        P4ifgW::new(self, 8)
    }
}
#[doc = "Port B Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pbifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbifgSpec;
impl crate::RegisterSpec for PbifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbifg::R`](R) reader structure"]
impl crate::Readable for PbifgSpec {}
#[doc = "`write(|w| ..)` method takes [`pbifg::W`](W) writer structure"]
impl crate::Writable for PbifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBIFG to value 0"]
impl crate::Resettable for PbifgSpec {
    const RESET_VALUE: u16 = 0;
}
