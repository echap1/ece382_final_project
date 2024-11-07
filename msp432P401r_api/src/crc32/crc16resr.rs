#[doc = "Register `CRC16RESR` reader"]
pub type R = crate::R<Crc16resrSpec>;
#[doc = "Register `CRC16RESR` writer"]
pub type W = crate::W<Crc16resrSpec>;
#[doc = "Field `CRC16RESR` reader - CRC16 reverse result"]
pub type Crc16resrR = crate::FieldReader<u16>;
#[doc = "Field `CRC16RESR` writer - CRC16 reverse result"]
pub type Crc16resrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC16 reverse result"]
    #[inline(always)]
    pub fn crc16resr(&self) -> Crc16resrR {
        Crc16resrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16 reverse result"]
    #[inline(always)]
    pub fn crc16resr(&mut self) -> Crc16resrW<Crc16resrSpec> {
        Crc16resrW::new(self, 0)
    }
}
#[doc = "CRC16 Result Reverse\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16resr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16resr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc16resrSpec;
impl crate::RegisterSpec for Crc16resrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc16resr::R`](R) reader structure"]
impl crate::Readable for Crc16resrSpec {}
#[doc = "`write(|w| ..)` method takes [`crc16resr::W`](W) writer structure"]
impl crate::Writable for Crc16resrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRC16RESR to value 0xffff"]
impl crate::Resettable for Crc16resrSpec {
    const RESET_VALUE: u16 = 0xffff;
}
