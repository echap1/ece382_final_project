#[doc = "Register `CRC32RESR_HI` reader"]
pub type R = crate::R<Crc32resrHiSpec>;
#[doc = "Register `CRC32RESR_HI` writer"]
pub type W = crate::W<Crc32resrHiSpec>;
#[doc = "Field `CRC32RESR_HI` reader - CRC32 reverse result, upper 16 bits"]
pub type Crc32resrHiR = crate::FieldReader<u16>;
#[doc = "Field `CRC32RESR_HI` writer - CRC32 reverse result, upper 16 bits"]
pub type Crc32resrHiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC32 reverse result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32resr_hi(&self) -> Crc32resrHiR {
        Crc32resrHiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC32 reverse result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32resr_hi(&mut self) -> Crc32resrHiW<Crc32resrHiSpec> {
        Crc32resrHiW::new(self, 0)
    }
}
#[doc = "CRC32 Result Reverse, Upper 16 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32resr_hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32resr_hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc32resrHiSpec;
impl crate::RegisterSpec for Crc32resrHiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc32resr_hi::R`](R) reader structure"]
impl crate::Readable for Crc32resrHiSpec {}
#[doc = "`write(|w| ..)` method takes [`crc32resr_hi::W`](W) writer structure"]
impl crate::Writable for Crc32resrHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRC32RESR_HI to value 0xffff"]
impl crate::Resettable for Crc32resrHiSpec {
    const RESET_VALUE: u16 = 0xffff;
}
