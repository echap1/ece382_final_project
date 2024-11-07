#[doc = "Register `CRC16INIRES` reader"]
pub type R = crate::R<Crc16iniresSpec>;
#[doc = "Register `CRC16INIRES` writer"]
pub type W = crate::W<Crc16iniresSpec>;
#[doc = "Field `CRC16INIRES` reader - CRC16 initialization and result"]
pub type Crc16iniresR = crate::FieldReader<u16>;
#[doc = "Field `CRC16INIRES` writer - CRC16 initialization and result"]
pub type Crc16iniresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC16 initialization and result"]
    #[inline(always)]
    pub fn crc16inires(&self) -> Crc16iniresR {
        Crc16iniresR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16 initialization and result"]
    #[inline(always)]
    pub fn crc16inires(&mut self) -> Crc16iniresW<Crc16iniresSpec> {
        Crc16iniresW::new(self, 0)
    }
}
#[doc = "CRC16 Initialization and Result register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16inires::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16inires::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc16iniresSpec;
impl crate::RegisterSpec for Crc16iniresSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc16inires::R`](R) reader structure"]
impl crate::Readable for Crc16iniresSpec {}
#[doc = "`write(|w| ..)` method takes [`crc16inires::W`](W) writer structure"]
impl crate::Writable for Crc16iniresSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRC16INIRES to value 0xffff"]
impl crate::Resettable for Crc16iniresSpec {
    const RESET_VALUE: u16 = 0xffff;
}
