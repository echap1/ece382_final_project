#[doc = "Register `CRC32INIRES_LO` reader"]
pub type R = crate::R<Crc32iniresLoSpec>;
#[doc = "Register `CRC32INIRES_LO` writer"]
pub type W = crate::W<Crc32iniresLoSpec>;
#[doc = "Field `CRC32INIRES_LO` reader - CRC32 initialization and result, lower 16 bits"]
pub type Crc32iniresLoR = crate::FieldReader<u16>;
#[doc = "Field `CRC32INIRES_LO` writer - CRC32 initialization and result, lower 16 bits"]
pub type Crc32iniresLoW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC32 initialization and result, lower 16 bits"]
    #[inline(always)]
    pub fn crc32inires_lo(&self) -> Crc32iniresLoR {
        Crc32iniresLoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC32 initialization and result, lower 16 bits"]
    #[inline(always)]
    pub fn crc32inires_lo(&mut self) -> Crc32iniresLoW<Crc32iniresLoSpec> {
        Crc32iniresLoW::new(self, 0)
    }
}
#[doc = "CRC32 Initialization and Result, lower 16 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32inires_lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32inires_lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc32iniresLoSpec;
impl crate::RegisterSpec for Crc32iniresLoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc32inires_lo::R`](R) reader structure"]
impl crate::Readable for Crc32iniresLoSpec {}
#[doc = "`write(|w| ..)` method takes [`crc32inires_lo::W`](W) writer structure"]
impl crate::Writable for Crc32iniresLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRC32INIRES_LO to value 0"]
impl crate::Resettable for Crc32iniresLoSpec {
    const RESET_VALUE: u16 = 0;
}
