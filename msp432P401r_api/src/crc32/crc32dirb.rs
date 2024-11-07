#[doc = "Register `CRC32DIRB` reader"]
pub type R = crate::R<Crc32dirbSpec>;
#[doc = "Register `CRC32DIRB` writer"]
pub type W = crate::W<Crc32dirbSpec>;
#[doc = "Field `CRC32DIRB` reader - Data input register reversed"]
pub type Crc32dirbR = crate::FieldReader<u16>;
#[doc = "Field `CRC32DIRB` writer - Data input register reversed"]
pub type Crc32dirbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data input register reversed"]
    #[inline(always)]
    pub fn crc32dirb(&self) -> Crc32dirbR {
        Crc32dirbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data input register reversed"]
    #[inline(always)]
    pub fn crc32dirb(&mut self) -> Crc32dirbW<Crc32dirbSpec> {
        Crc32dirbW::new(self, 0)
    }
}
#[doc = "Data In Reverse for CRC32 Computation\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32dirb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32dirb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc32dirbSpec;
impl crate::RegisterSpec for Crc32dirbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc32dirb::R`](R) reader structure"]
impl crate::Readable for Crc32dirbSpec {}
#[doc = "`write(|w| ..)` method takes [`crc32dirb::W`](W) writer structure"]
impl crate::Writable for Crc32dirbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRC32DIRB to value 0"]
impl crate::Resettable for Crc32dirbSpec {
    const RESET_VALUE: u16 = 0;
}
