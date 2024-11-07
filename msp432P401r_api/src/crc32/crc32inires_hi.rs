#[doc = "Register `CRC32INIRES_HI` reader"]
pub type R = crate::R<Crc32iniresHiSpec>;
#[doc = "Register `CRC32INIRES_HI` writer"]
pub type W = crate::W<Crc32iniresHiSpec>;
#[doc = "Field `CRC32INIRES_HI` reader - CRC32 initialization and result, upper 16 bits"]
pub type Crc32iniresHiR = crate::FieldReader<u16>;
#[doc = "Field `CRC32INIRES_HI` writer - CRC32 initialization and result, upper 16 bits"]
pub type Crc32iniresHiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC32 initialization and result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32inires_hi(&self) -> Crc32iniresHiR {
        Crc32iniresHiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC32 initialization and result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32inires_hi(&mut self) -> Crc32iniresHiW<Crc32iniresHiSpec> {
        Crc32iniresHiW::new(self, 0)
    }
}
#[doc = "CRC32 Initialization and Result, upper 16 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32inires_hi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32inires_hi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc32iniresHiSpec;
impl crate::RegisterSpec for Crc32iniresHiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc32inires_hi::R`](R) reader structure"]
impl crate::Readable for Crc32iniresHiSpec {}
#[doc = "`write(|w| ..)` method takes [`crc32inires_hi::W`](W) writer structure"]
impl crate::Writable for Crc32iniresHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRC32INIRES_HI to value 0"]
impl crate::Resettable for Crc32iniresHiSpec {
    const RESET_VALUE: u16 = 0;
}
