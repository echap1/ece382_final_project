#[doc = "Register `CRC32DI` reader"]
pub type R = crate::R<Crc32diSpec>;
#[doc = "Register `CRC32DI` writer"]
pub type W = crate::W<Crc32diSpec>;
#[doc = "Field `CRC32DI` reader - Data input register"]
pub type Crc32diR = crate::FieldReader<u16>;
#[doc = "Field `CRC32DI` writer - Data input register"]
pub type Crc32diW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data input register"]
    #[inline(always)]
    pub fn crc32di(&self) -> Crc32diR {
        Crc32diR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data input register"]
    #[inline(always)]
    pub fn crc32di(&mut self) -> Crc32diW<Crc32diSpec> {
        Crc32diW::new(self, 0)
    }
}
#[doc = "Data Input for CRC32 Signature Computation\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32di::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32di::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc32diSpec;
impl crate::RegisterSpec for Crc32diSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc32di::R`](R) reader structure"]
impl crate::Readable for Crc32diSpec {}
#[doc = "`write(|w| ..)` method takes [`crc32di::W`](W) writer structure"]
impl crate::Writable for Crc32diSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRC32DI to value 0"]
impl crate::Resettable for Crc32diSpec {
    const RESET_VALUE: u16 = 0;
}
