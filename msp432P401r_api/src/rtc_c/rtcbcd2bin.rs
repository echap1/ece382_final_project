#[doc = "Register `RTCBCD2BIN` reader"]
pub type R = crate::R<Rtcbcd2binSpec>;
#[doc = "Register `RTCBCD2BIN` writer"]
pub type W = crate::W<Rtcbcd2binSpec>;
#[doc = "Field `BCD2BIN` reader - bcd to bin conversion"]
pub type Bcd2binR = crate::FieldReader<u16>;
#[doc = "Field `BCD2BIN` writer - bcd to bin conversion"]
pub type Bcd2binW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - bcd to bin conversion"]
    #[inline(always)]
    pub fn bcd2bin(&self) -> Bcd2binR {
        Bcd2binR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - bcd to bin conversion"]
    #[inline(always)]
    pub fn bcd2bin(&mut self) -> Bcd2binW<Rtcbcd2binSpec> {
        Bcd2binW::new(self, 0)
    }
}
#[doc = "BCD-to-Binary Conversion Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcbcd2bin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcbcd2bin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcbcd2binSpec;
impl crate::RegisterSpec for Rtcbcd2binSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcbcd2bin::R`](R) reader structure"]
impl crate::Readable for Rtcbcd2binSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcbcd2bin::W`](W) writer structure"]
impl crate::Writable for Rtcbcd2binSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCBCD2BIN to value 0"]
impl crate::Resettable for Rtcbcd2binSpec {
    const RESET_VALUE: u16 = 0;
}
