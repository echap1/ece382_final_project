#[doc = "Register `RTCBIN2BCD` reader"]
pub type R = crate::R<Rtcbin2bcdSpec>;
#[doc = "Register `RTCBIN2BCD` writer"]
pub type W = crate::W<Rtcbin2bcdSpec>;
#[doc = "Field `BIN2BCD` reader - bin to bcd conversion"]
pub type Bin2bcdR = crate::FieldReader<u16>;
#[doc = "Field `BIN2BCD` writer - bin to bcd conversion"]
pub type Bin2bcdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - bin to bcd conversion"]
    #[inline(always)]
    pub fn bin2bcd(&self) -> Bin2bcdR {
        Bin2bcdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - bin to bcd conversion"]
    #[inline(always)]
    pub fn bin2bcd(&mut self) -> Bin2bcdW<Rtcbin2bcdSpec> {
        Bin2bcdW::new(self, 0)
    }
}
#[doc = "Binary-to-BCD Conversion Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcbin2bcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcbin2bcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcbin2bcdSpec;
impl crate::RegisterSpec for Rtcbin2bcdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcbin2bcd::R`](R) reader structure"]
impl crate::Readable for Rtcbin2bcdSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcbin2bcd::W`](W) writer structure"]
impl crate::Writable for Rtcbin2bcdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCBIN2BCD to value 0"]
impl crate::Resettable for Rtcbin2bcdSpec {
    const RESET_VALUE: u16 = 0;
}
