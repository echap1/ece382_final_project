#[doc = "Register `PEDS` reader"]
pub type R = crate::R<PedsSpec>;
#[doc = "Register `PEDS` writer"]
pub type W = crate::W<PedsSpec>;
#[doc = "Field `P9DS` reader - Port 9 Drive Strength"]
pub type P9dsR = crate::FieldReader;
#[doc = "Field `P9DS` writer - Port 9 Drive Strength"]
pub type P9dsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P10DS` reader - Port 10 Drive Strength"]
pub type P10dsR = crate::FieldReader;
#[doc = "Field `P10DS` writer - Port 10 Drive Strength"]
pub type P10dsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Drive Strength"]
    #[inline(always)]
    pub fn p9ds(&self) -> P9dsR {
        P9dsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Drive Strength"]
    #[inline(always)]
    pub fn p10ds(&self) -> P10dsR {
        P10dsR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Drive Strength"]
    #[inline(always)]
    pub fn p9ds(&mut self) -> P9dsW<PedsSpec> {
        P9dsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 10 Drive Strength"]
    #[inline(always)]
    pub fn p10ds(&mut self) -> P10dsW<PedsSpec> {
        P10dsW::new(self, 8)
    }
}
#[doc = "Port E Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`peds::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PedsSpec;
impl crate::RegisterSpec for PedsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`peds::R`](R) reader structure"]
impl crate::Readable for PedsSpec {}
#[doc = "`write(|w| ..)` method takes [`peds::W`](W) writer structure"]
impl crate::Writable for PedsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
