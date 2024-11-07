#[doc = "Register `PCDS` reader"]
pub type R = crate::R<PcdsSpec>;
#[doc = "Register `PCDS` writer"]
pub type W = crate::W<PcdsSpec>;
#[doc = "Field `P5DS` reader - Port 5 Drive Strength"]
pub type P5dsR = crate::FieldReader;
#[doc = "Field `P5DS` writer - Port 5 Drive Strength"]
pub type P5dsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P6DS` reader - Port 6 Drive Strength"]
pub type P6dsR = crate::FieldReader;
#[doc = "Field `P6DS` writer - Port 6 Drive Strength"]
pub type P6dsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Drive Strength"]
    #[inline(always)]
    pub fn p5ds(&self) -> P5dsR {
        P5dsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Drive Strength"]
    #[inline(always)]
    pub fn p6ds(&self) -> P6dsR {
        P6dsR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Drive Strength"]
    #[inline(always)]
    pub fn p5ds(&mut self) -> P5dsW<PcdsSpec> {
        P5dsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 6 Drive Strength"]
    #[inline(always)]
    pub fn p6ds(&mut self) -> P6dsW<PcdsSpec> {
        P6dsW::new(self, 8)
    }
}
#[doc = "Port C Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`pcds::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcdsSpec;
impl crate::RegisterSpec for PcdsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcds::R`](R) reader structure"]
impl crate::Readable for PcdsSpec {}
#[doc = "`write(|w| ..)` method takes [`pcds::W`](W) writer structure"]
impl crate::Writable for PcdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
