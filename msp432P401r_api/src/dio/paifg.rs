#[doc = "Register `PAIFG` reader"]
pub type R = crate::R<PaifgSpec>;
#[doc = "Register `PAIFG` writer"]
pub type W = crate::W<PaifgSpec>;
#[doc = "Field `P1IFG` reader - Port 1 Interrupt Flag"]
pub type P1ifgR = crate::FieldReader;
#[doc = "Field `P1IFG` writer - Port 1 Interrupt Flag"]
pub type P1ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P2IFG` reader - Port 2 Interrupt Flag"]
pub type P2ifgR = crate::FieldReader;
#[doc = "Field `P2IFG` writer - Port 2 Interrupt Flag"]
pub type P2ifgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Interrupt Flag"]
    #[inline(always)]
    pub fn p1ifg(&self) -> P1ifgR {
        P1ifgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Flag"]
    #[inline(always)]
    pub fn p2ifg(&self) -> P2ifgR {
        P2ifgR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Interrupt Flag"]
    #[inline(always)]
    pub fn p1ifg(&mut self) -> P1ifgW<PaifgSpec> {
        P1ifgW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Flag"]
    #[inline(always)]
    pub fn p2ifg(&mut self) -> P2ifgW<PaifgSpec> {
        P2ifgW::new(self, 8)
    }
}
#[doc = "Port A Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`paifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaifgSpec;
impl crate::RegisterSpec for PaifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`paifg::R`](R) reader structure"]
impl crate::Readable for PaifgSpec {}
#[doc = "`write(|w| ..)` method takes [`paifg::W`](W) writer structure"]
impl crate::Writable for PaifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PAIFG to value 0"]
impl crate::Resettable for PaifgSpec {
    const RESET_VALUE: u16 = 0;
}
