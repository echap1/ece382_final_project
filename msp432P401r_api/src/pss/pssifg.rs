#[doc = "Register `PSSIFG` reader"]
pub type R = crate::R<PssifgSpec>;
#[doc = "High-side SVSM interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvsmhifgEnumRead {
    #[doc = "0: No interrupt pending"]
    Svsmhifg0 = 0,
    #[doc = "1: Interrupt due to SVSMH"]
    Svsmhifg1 = 1,
}
impl From<SvsmhifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: SvsmhifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSMHIFG` reader - High-side SVSM interrupt flag"]
pub type SvsmhifgR = crate::BitReader<SvsmhifgEnumRead>;
impl SvsmhifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SvsmhifgEnumRead {
        match self.bits {
            false => SvsmhifgEnumRead::Svsmhifg0,
            true => SvsmhifgEnumRead::Svsmhifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_svsmhifg_0(&self) -> bool {
        *self == SvsmhifgEnumRead::Svsmhifg0
    }
    #[doc = "Interrupt due to SVSMH"]
    #[inline(always)]
    pub fn is_svsmhifg_1(&self) -> bool {
        *self == SvsmhifgEnumRead::Svsmhifg1
    }
}
impl R {
    #[doc = "Bit 1 - High-side SVSM interrupt flag"]
    #[inline(always)]
    pub fn svsmhifg(&self) -> SvsmhifgR {
        SvsmhifgR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pssifg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PssifgSpec;
impl crate::RegisterSpec for PssifgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pssifg::R`](R) reader structure"]
impl crate::Readable for PssifgSpec {}
#[doc = "`reset()` method sets PSSIFG to value 0"]
impl crate::Resettable for PssifgSpec {
    const RESET_VALUE: u32 = 0;
}
