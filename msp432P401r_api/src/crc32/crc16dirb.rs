#[doc = "Register `CRC16DIRB` reader"]
pub type R = crate::R<Crc16dirbSpec>;
#[doc = "Register `CRC16DIRB` writer"]
pub type W = crate::W<Crc16dirbSpec>;
#[doc = "Field `CRC16DIRB` reader - CRC16 data in reverse byte"]
pub type Crc16dirbR = crate::FieldReader<u16>;
#[doc = "Field `CRC16DIRB` writer - CRC16 data in reverse byte"]
pub type Crc16dirbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC16 data in reverse byte"]
    #[inline(always)]
    pub fn crc16dirb(&self) -> Crc16dirbR {
        Crc16dirbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16 data in reverse byte"]
    #[inline(always)]
    pub fn crc16dirb(&mut self) -> Crc16dirbW<Crc16dirbSpec> {
        Crc16dirbW::new(self, 0)
    }
}
#[doc = "CRC16 Data In Reverse\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16dirb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16dirb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crc16dirbSpec;
impl crate::RegisterSpec for Crc16dirbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crc16dirb::R`](R) reader structure"]
impl crate::Readable for Crc16dirbSpec {}
#[doc = "`write(|w| ..)` method takes [`crc16dirb::W`](W) writer structure"]
impl crate::Writable for Crc16dirbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CRC16DIRB to value 0"]
impl crate::Resettable for Crc16dirbSpec {
    const RESET_VALUE: u16 = 0;
}
