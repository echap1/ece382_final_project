#[doc = "Register `PBOUT` reader"]
pub type R = crate::R<PboutSpec>;
#[doc = "Register `PBOUT` writer"]
pub type W = crate::W<PboutSpec>;
#[doc = "Field `P3OUT` reader - Port 3 Output"]
pub type P3outR = crate::FieldReader;
#[doc = "Field `P3OUT` writer - Port 3 Output"]
pub type P3outW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `P4OUT` reader - Port 4 Output"]
pub type P4outR = crate::FieldReader;
#[doc = "Field `P4OUT` writer - Port 4 Output"]
pub type P4outW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Output"]
    #[inline(always)]
    pub fn p3out(&self) -> P3outR {
        P3outR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Output"]
    #[inline(always)]
    pub fn p4out(&self) -> P4outR {
        P4outR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Output"]
    #[inline(always)]
    pub fn p3out(&mut self) -> P3outW<PboutSpec> {
        P3outW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port 4 Output"]
    #[inline(always)]
    pub fn p4out(&mut self) -> P4outW<PboutSpec> {
        P4outW::new(self, 8)
    }
}
#[doc = "Port B Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pbout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PboutSpec;
impl crate::RegisterSpec for PboutSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pbout::R`](R) reader structure"]
impl crate::Readable for PboutSpec {}
#[doc = "`write(|w| ..)` method takes [`pbout::W`](W) writer structure"]
impl crate::Writable for PboutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PBOUT to value 0"]
impl crate::Resettable for PboutSpec {
    const RESET_VALUE: u16 = 0;
}
