#[doc = "Register `PDIFG` reader"]
pub type R = crate::R<PdifgSpec>;
#[doc = "Register `PDIFG` writer"]
pub type W = crate::W<PdifgSpec>;
#[doc = "Field `P7IFG` reader - Port 7 Interrupt Flag"]
pub type P7ifgR = crate::FieldReader;
#[doc = "Field `P7IFG` writer - Port 7 Interrupt Flag"]
pub type P7ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P8IFG` reader - Port 8 Interrupt Flag"]
pub type P8ifgR = crate::FieldReader;
#[doc = "Field `P8IFG` writer - Port 8 Interrupt Flag"]
pub type P8ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Interrupt Flag"]
    #[inline(always)]
    pub fn p7ifg(&self) -> P7ifgR {
        P7ifgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Flag"]
    #[inline(always)]
    pub fn p8ifg(&self) -> P8ifgR {
        P8ifgR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Interrupt Flag"]
    #[inline(always)]
    pub fn p7ifg(&mut self) -> P7ifgW<PdifgSpec> {
        P7ifgW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Flag"]
    #[inline(always)]
    pub fn p8ifg(&mut self) -> P8ifgW<PdifgSpec> {
        P8ifgW::new(self, 8)
    }
}
#[doc = "Port D Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pdifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdifgSpec;
impl crate::RegisterSpec for PdifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdifg::R`](R) reader structure"]
impl crate::Readable for PdifgSpec {}
#[doc = "`write(|w| ..)` method takes [`pdifg::W`](W) writer structure"]
impl crate::Writable for PdifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDIFG to value 0"]
impl crate::Resettable for PdifgSpec {
    const RESET_VALUE: u16 = 0;
}
