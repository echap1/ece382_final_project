#[doc = "Register `UCBxI2COA3` reader"]
pub type R = crate::R<UcbxI2coa3Spec>;
#[doc = "Register `UCBxI2COA3` writer"]
pub type W = crate::W<UcbxI2coa3Spec>;
#[doc = "Field `I2COA3` reader - I2C own address"]
pub type I2coa3R = crate::FieldReader<u16>;
#[doc = "Field `I2COA3` writer - I2C own address"]
pub type I2coa3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Own Address enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucoaen {
    #[doc = "0: The slave address defined in I2COA3 is disabled"]
    Ucoaen0 = 0,
    #[doc = "1: The slave address defined in I2COA3 is enabled"]
    Ucoaen1 = 1,
}
impl From<Ucoaen> for bool {
    #[inline(always)]
    fn from(variant: Ucoaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCOAEN` reader - Own Address enable register"]
pub type UcoaenR = crate::BitReader<Ucoaen>;
impl UcoaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucoaen {
        match self.bits {
            false => Ucoaen::Ucoaen0,
            true => Ucoaen::Ucoaen1,
        }
    }
    #[doc = "The slave address defined in I2COA3 is disabled"]
    #[inline(always)]
    pub fn is_ucoaen_0(&self) -> bool {
        *self == Ucoaen::Ucoaen0
    }
    #[doc = "The slave address defined in I2COA3 is enabled"]
    #[inline(always)]
    pub fn is_ucoaen_1(&self) -> bool {
        *self == Ucoaen::Ucoaen1
    }
}
#[doc = "Field `UCOAEN` writer - Own Address enable register"]
pub type UcoaenW<'a, REG> = crate::BitWriter<'a, REG, Ucoaen>;
impl<'a, REG> UcoaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The slave address defined in I2COA3 is disabled"]
    #[inline(always)]
    pub fn ucoaen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucoaen::Ucoaen0)
    }
    #[doc = "The slave address defined in I2COA3 is enabled"]
    #[inline(always)]
    pub fn ucoaen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucoaen::Ucoaen1)
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa3(&self) -> I2coa3R {
        I2coa3R::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&self) -> UcoaenR {
        UcoaenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa3(&mut self) -> I2coa3W<UcbxI2coa3Spec> {
        I2coa3W::new(self, 0)
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&mut self) -> UcoaenW<UcbxI2coa3Spec> {
        UcoaenW::new(self, 10)
    }
}
#[doc = "eUSCI_Bx I2C Own Address 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_i2coa3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_i2coa3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcbxI2coa3Spec;
impl crate::RegisterSpec for UcbxI2coa3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucbx_i2coa3::R`](R) reader structure"]
impl crate::Readable for UcbxI2coa3Spec {}
#[doc = "`write(|w| ..)` method takes [`ucbx_i2coa3::W`](W) writer structure"]
impl crate::Writable for UcbxI2coa3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCBxI2COA3 to value 0"]
impl crate::Resettable for UcbxI2coa3Spec {
    const RESET_VALUE: u16 = 0;
}