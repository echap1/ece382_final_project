#[doc = "Register `CRC16DI` reader"]
pub type R = crate::R<Crc16diSpec>;
#[doc = "Register `CRC16DI` writer"]
pub type W = crate::W<Crc16diSpec>;
#[doc = "Field `CRC16DI` reader - CRC16 data in"]
pub type Crc16diR = crate::FieldReader<u16>;
#[doc = "Field `CRC16DI` writer - CRC16 data in"]
pub type Crc16diW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC16 data in"]
    #[inline(always)]
    pub fn crc16di(&self) -> Crc16diR {
        Crc16diR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16 data in"]
    #[inline(always)]
    pub fn crc16di(&mut self) -> Crc16diW<Crc16diSpec> {
        Crc16diW::new(self, 0)
    }
}
#[doc = "Data Input for CRC16 computation\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16di::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16di::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc16diSpec;
impl crate::RegisterSpec for Crc16diSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc16di::R`](R) reader structure"]
impl crate::Readable for Crc16diSpec {}
#[doc = "`write(|w| ..)` method takes [`crc16di::W`](W) writer structure"]
impl crate::Writable for Crc16diSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRC16DI to value 0"]
impl crate::Resettable for Crc16diSpec {
    const RESET_VALUE: u16 = 0;
}
