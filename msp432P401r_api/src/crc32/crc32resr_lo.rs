#[doc = "Register `CRC32RESR_LO` reader"]
pub type R = crate::R<Crc32resrLoSpec>;
#[doc = "Register `CRC32RESR_LO` writer"]
pub type W = crate::W<Crc32resrLoSpec>;
#[doc = "Field `CRC32RESR_LO` reader - CRC32 reverse result, lower 16 bits"]
pub type Crc32resrLoR = crate::FieldReader<u16>;
#[doc = "Field `CRC32RESR_LO` writer - CRC32 reverse result, lower 16 bits"]
pub type Crc32resrLoW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC32 reverse result, lower 16 bits"]
    #[inline(always)]
    pub fn crc32resr_lo(&self) -> Crc32resrLoR {
        Crc32resrLoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC32 reverse result, lower 16 bits"]
    #[inline(always)]
    pub fn crc32resr_lo(&mut self) -> Crc32resrLoW<Crc32resrLoSpec> {
        Crc32resrLoW::new(self, 0)
    }
}
#[doc = "CRC32 Result Reverse, lower 16 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32resr_lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32resr_lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc32resrLoSpec;
impl crate::RegisterSpec for Crc32resrLoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc32resr_lo::R`](R) reader structure"]
impl crate::Readable for Crc32resrLoSpec {}
#[doc = "`write(|w| ..)` method takes [`crc32resr_lo::W`](W) writer structure"]
impl crate::Writable for Crc32resrLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRC32RESR_LO to value 0xffff"]
impl crate::Resettable for Crc32resrLoSpec {
    const RESET_VALUE: u16 = 0xffff;
}
