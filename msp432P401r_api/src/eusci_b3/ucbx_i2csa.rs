#[doc = "Register `UCBxI2CSA` reader"]
pub type R = crate::R<UcbxI2csaSpec>;
#[doc = "Register `UCBxI2CSA` writer"]
pub type W = crate::W<UcbxI2csaSpec>;
#[doc = "Field `I2CSA` reader - I2C slave address"]
pub type I2csaR = crate::FieldReader<u16>;
#[doc = "Field `I2CSA` writer - I2C slave address"]
pub type I2csaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - I2C slave address"]
    #[inline(always)]
    pub fn i2csa(&self) -> I2csaR {
        I2csaR::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C slave address"]
    #[inline(always)]
    pub fn i2csa(&mut self) -> I2csaW<UcbxI2csaSpec> {
        I2csaW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx I2C Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_i2csa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_i2csa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcbxI2csaSpec;
impl crate::RegisterSpec for UcbxI2csaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucbx_i2csa::R`](R) reader structure"]
impl crate::Readable for UcbxI2csaSpec {}
#[doc = "`write(|w| ..)` method takes [`ucbx_i2csa::W`](W) writer structure"]
impl crate::Writable for UcbxI2csaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCBxI2CSA to value 0"]
impl crate::Resettable for UcbxI2csaSpec {
    const RESET_VALUE: u16 = 0;
}
