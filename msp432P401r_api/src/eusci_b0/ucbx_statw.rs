#[doc = "Register `UCBxSTATW` reader"]
pub type R = crate::R<UcbxStatwSpec>;
#[doc = "Register `UCBxSTATW` writer"]
pub type W = crate::W<UcbxStatwSpec>;
#[doc = "Bus busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UcbbusyEnumRead {
    #[doc = "0: Bus inactive"]
    Ucbbusy0 = 0,
    #[doc = "1: Bus busy"]
    Ucbbusy1 = 1,
}
impl From<UcbbusyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: UcbbusyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBBUSY` reader - Bus busy"]
pub type UcbbusyR = crate::BitReader<UcbbusyEnumRead>;
impl UcbbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UcbbusyEnumRead {
        match self.bits {
            false => UcbbusyEnumRead::Ucbbusy0,
            true => UcbbusyEnumRead::Ucbbusy1,
        }
    }
    #[doc = "Bus inactive"]
    #[inline(always)]
    pub fn is_ucbbusy_0(&self) -> bool {
        *self == UcbbusyEnumRead::Ucbbusy0
    }
    #[doc = "Bus busy"]
    #[inline(always)]
    pub fn is_ucbbusy_1(&self) -> bool {
        *self == UcbbusyEnumRead::Ucbbusy1
    }
}
#[doc = "General call address received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UcgcEnumRead {
    #[doc = "0: No general call address received"]
    Ucgc0 = 0,
    #[doc = "1: General call address received"]
    Ucgc1 = 1,
}
impl From<UcgcEnumRead> for bool {
    #[inline(always)]
    fn from(variant: UcgcEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCGC` reader - General call address received"]
pub type UcgcR = crate::BitReader<UcgcEnumRead>;
impl UcgcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UcgcEnumRead {
        match self.bits {
            false => UcgcEnumRead::Ucgc0,
            true => UcgcEnumRead::Ucgc1,
        }
    }
    #[doc = "No general call address received"]
    #[inline(always)]
    pub fn is_ucgc_0(&self) -> bool {
        *self == UcgcEnumRead::Ucgc0
    }
    #[doc = "General call address received"]
    #[inline(always)]
    pub fn is_ucgc_1(&self) -> bool {
        *self == UcgcEnumRead::Ucgc1
    }
}
#[doc = "SCL low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UcscllowEnumRead {
    #[doc = "0: SCL is not held low"]
    Ucscllow0 = 0,
    #[doc = "1: SCL is held low"]
    Ucscllow1 = 1,
}
impl From<UcscllowEnumRead> for bool {
    #[inline(always)]
    fn from(variant: UcscllowEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSCLLOW` reader - SCL low"]
pub type UcscllowR = crate::BitReader<UcscllowEnumRead>;
impl UcscllowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UcscllowEnumRead {
        match self.bits {
            false => UcscllowEnumRead::Ucscllow0,
            true => UcscllowEnumRead::Ucscllow1,
        }
    }
    #[doc = "SCL is not held low"]
    #[inline(always)]
    pub fn is_ucscllow_0(&self) -> bool {
        *self == UcscllowEnumRead::Ucscllow0
    }
    #[doc = "SCL is held low"]
    #[inline(always)]
    pub fn is_ucscllow_1(&self) -> bool {
        *self == UcscllowEnumRead::Ucscllow1
    }
}
#[doc = "Field `UCBCNT` reader - Hardware byte counter value"]
pub type UcbcntR = crate::FieldReader;
impl R {
    #[doc = "Bit 4 - Bus busy"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UcbbusyR {
        UcbbusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General call address received"]
    #[inline(always)]
    pub fn ucgc(&self) -> UcgcR {
        UcgcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UcscllowR {
        UcscllowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Hardware byte counter value"]
    #[inline(always)]
    pub fn ucbcnt(&self) -> UcbcntR {
        UcbcntR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {}
#[doc = "eUSCI_Bx Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_statw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_statw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcbxStatwSpec;
impl crate::RegisterSpec for UcbxStatwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucbx_statw::R`](R) reader structure"]
impl crate::Readable for UcbxStatwSpec {}
#[doc = "`write(|w| ..)` method takes [`ucbx_statw::W`](W) writer structure"]
impl crate::Writable for UcbxStatwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCBxSTATW to value 0"]
impl crate::Resettable for UcbxStatwSpec {
    const RESET_VALUE: u16 = 0;
}
