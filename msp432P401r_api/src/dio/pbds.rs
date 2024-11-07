#[doc = "Register `PBDS` reader"]
pub type R = crate::R<PbdsSpec>;
#[doc = "Register `PBDS` writer"]
pub type W = crate::W<PbdsSpec>;
#[doc = "Field `P3DS` reader - Port 3 Drive Strength"]
pub type P3dsR = crate::FieldReader;
#[doc = "Field `P3DS` writer - Port 3 Drive Strength"]
pub type P3dsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P4DS` reader - Port 4 Drive Strength"]
pub type P4dsR = crate::FieldReader;
#[doc = "Field `P4DS` writer - Port 4 Drive Strength"]
pub type P4dsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Drive Strength"]
    #[inline(always)]
    pub fn p3ds(&self) -> P3dsR {
        P3dsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Drive Strength"]
    #[inline(always)]
    pub fn p4ds(&self) -> P4dsR {
        P4dsR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Drive Strength"]
    #[inline(always)]
    pub fn p3ds(&mut self) -> P3dsW<PbdsSpec> {
        P3dsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 4 Drive Strength"]
    #[inline(always)]
    pub fn p4ds(&mut self) -> P4dsW<PbdsSpec> {
        P4dsW::new(self, 8)
    }
}
#[doc = "Port B Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`pbds::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbdsSpec;
impl crate::RegisterSpec for PbdsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbds::R`](R) reader structure"]
impl crate::Readable for PbdsSpec {}
#[doc = "`write(|w| ..)` method takes [`pbds::W`](W) writer structure"]
impl crate::Writable for PbdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
