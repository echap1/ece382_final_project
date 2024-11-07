#[doc = "Register `RTCTIM0` reader"]
pub type R = crate::R<Rtctim0Spec>;
#[doc = "Register `RTCTIM0` writer"]
pub type W = crate::W<Rtctim0Spec>;
#[doc = "Field `Seconds` reader - Seconds (0 to 59)"]
pub type SecondsR = crate::FieldReader;
#[doc = "Field `Seconds` writer - Seconds (0 to 59)"]
pub type SecondsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Minutes` reader - Minutes (0 to 59)"]
pub type MinutesR = crate::FieldReader;
#[doc = "Field `Minutes` writer - Minutes (0 to 59)"]
pub type MinutesW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Seconds (0 to 59)"]
    #[inline(always)]
    pub fn seconds(&self) -> SecondsR {
        SecondsR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&self) -> MinutesR {
        MinutesR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds (0 to 59)"]
    #[inline(always)]
    pub fn seconds(&mut self) -> SecondsW<Rtctim0Spec> {
        SecondsW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&mut self) -> MinutesW<Rtctim0Spec> {
        MinutesW::new(self, 8)
    }
}
#[doc = "RTCTIM0 Register Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctim0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctim0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtctim0Spec;
impl crate::RegisterSpec for Rtctim0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtctim0::R`](R) reader structure"]
impl crate::Readable for Rtctim0Spec {}
#[doc = "`write(|w| ..)` method takes [`rtctim0::W`](W) writer structure"]
impl crate::Writable for Rtctim0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCTIM0 to value 0"]
impl crate::Resettable for Rtctim0Spec {
    const RESET_VALUE: u16 = 0;
}
