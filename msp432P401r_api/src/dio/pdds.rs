#[doc = "Register `PDDS` reader"]
pub type R = crate::R<PddsSpec>;
#[doc = "Register `PDDS` writer"]
pub type W = crate::W<PddsSpec>;
#[doc = "Field `P7DS` reader - Port 7 Drive Strength"]
pub type P7dsR = crate::FieldReader;
#[doc = "Field `P7DS` writer - Port 7 Drive Strength"]
pub type P7dsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P8DS` reader - Port 8 Drive Strength"]
pub type P8dsR = crate::FieldReader;
#[doc = "Field `P8DS` writer - Port 8 Drive Strength"]
pub type P8dsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Drive Strength"]
    #[inline(always)]
    pub fn p7ds(&self) -> P7dsR {
        P7dsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Drive Strength"]
    #[inline(always)]
    pub fn p8ds(&self) -> P8dsR {
        P8dsR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Drive Strength"]
    #[inline(always)]
    pub fn p7ds(&mut self) -> P7dsW<PddsSpec> {
        P7dsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 8 Drive Strength"]
    #[inline(always)]
    pub fn p8ds(&mut self) -> P8dsW<PddsSpec> {
        P8dsW::new(self, 8)
    }
}
#[doc = "Port D Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`pdds::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PddsSpec;
impl crate::RegisterSpec for PddsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdds::R`](R) reader structure"]
impl crate::Readable for PddsSpec {}
#[doc = "`write(|w| ..)` method takes [`pdds::W`](W) writer structure"]
impl crate::Writable for PddsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
