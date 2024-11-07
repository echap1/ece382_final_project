#[doc = "Register `PADS` reader"]
pub type R = crate::R<PadsSpec>;
#[doc = "Register `PADS` writer"]
pub type W = crate::W<PadsSpec>;
#[doc = "Field `P1DS` reader - Port 1 Drive Strength"]
pub type P1dsR = crate::FieldReader;
#[doc = "Field `P1DS` writer - Port 1 Drive Strength"]
pub type P1dsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P2DS` reader - Port 2 Drive Strength"]
pub type P2dsR = crate::FieldReader;
#[doc = "Field `P2DS` writer - Port 2 Drive Strength"]
pub type P2dsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Drive Strength"]
    #[inline(always)]
    pub fn p1ds(&self) -> P1dsR {
        P1dsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Drive Strength"]
    #[inline(always)]
    pub fn p2ds(&self) -> P2dsR {
        P2dsR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Drive Strength"]
    #[inline(always)]
    pub fn p1ds(&mut self) -> P1dsW<PadsSpec> {
        P1dsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 2 Drive Strength"]
    #[inline(always)]
    pub fn p2ds(&mut self) -> P2dsW<PadsSpec> {
        P2dsW::new(self, 8)
    }
}
#[doc = "Port A Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`pads::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pads::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadsSpec;
impl crate::RegisterSpec for PadsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pads::R`](R) reader structure"]
impl crate::Readable for PadsSpec {}
#[doc = "`write(|w| ..)` method takes [`pads::W`](W) writer structure"]
impl crate::Writable for PadsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
