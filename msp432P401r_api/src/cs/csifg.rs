#[doc = "Register `CSIFG` reader"]
pub type R = crate::R<CsifgSpec>;
#[doc = "LFXT oscillator fault flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfxtifgEnumRead {
    #[doc = "0: No fault condition occurred after the last reset"]
    Lfxtifg0 = 0,
    #[doc = "1: LFXT fault. A LFXT fault occurred after the last reset"]
    Lfxtifg1 = 1,
}
impl From<LfxtifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: LfxtifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTIFG` reader - LFXT oscillator fault flag"]
pub type LfxtifgR = crate::BitReader<LfxtifgEnumRead>;
impl LfxtifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LfxtifgEnumRead {
        match self.bits {
            false => LfxtifgEnumRead::Lfxtifg0,
            true => LfxtifgEnumRead::Lfxtifg1,
        }
    }
    #[doc = "No fault condition occurred after the last reset"]
    #[inline(always)]
    pub fn is_lfxtifg_0(&self) -> bool {
        *self == LfxtifgEnumRead::Lfxtifg0
    }
    #[doc = "LFXT fault. A LFXT fault occurred after the last reset"]
    #[inline(always)]
    pub fn is_lfxtifg_1(&self) -> bool {
        *self == LfxtifgEnumRead::Lfxtifg1
    }
}
#[doc = "HFXT oscillator fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HfxtifgEnumRead {
    #[doc = "0: No fault condition occurred after the last reset"]
    Hfxtifg0 = 0,
    #[doc = "1: HFXT fault. A HFXT fault occurred after the last reset"]
    Hfxtifg1 = 1,
}
impl From<HfxtifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: HfxtifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXTIFG` reader - HFXT oscillator fault flag"]
pub type HfxtifgR = crate::BitReader<HfxtifgEnumRead>;
impl HfxtifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HfxtifgEnumRead {
        match self.bits {
            false => HfxtifgEnumRead::Hfxtifg0,
            true => HfxtifgEnumRead::Hfxtifg1,
        }
    }
    #[doc = "No fault condition occurred after the last reset"]
    #[inline(always)]
    pub fn is_hfxtifg_0(&self) -> bool {
        *self == HfxtifgEnumRead::Hfxtifg0
    }
    #[doc = "HFXT fault. A HFXT fault occurred after the last reset"]
    #[inline(always)]
    pub fn is_hfxtifg_1(&self) -> bool {
        *self == HfxtifgEnumRead::Hfxtifg1
    }
}
#[doc = "HFXT2 oscillator fault flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfxt2ifgEnumRead {
    #[doc = "0: No fault condition occurred after the last reset"]
    Hfxt2ifg0 = 0,
    #[doc = "1: HFXT2 fault. A HFXT2 fault occurred after the last reset"]
    Hfxt2ifg1 = 1,
}
impl From<Hfxt2ifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: Hfxt2ifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXT2IFG` reader - HFXT2 oscillator fault flag"]
pub type Hfxt2ifgR = crate::BitReader<Hfxt2ifgEnumRead>;
impl Hfxt2ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxt2ifgEnumRead {
        match self.bits {
            false => Hfxt2ifgEnumRead::Hfxt2ifg0,
            true => Hfxt2ifgEnumRead::Hfxt2ifg1,
        }
    }
    #[doc = "No fault condition occurred after the last reset"]
    #[inline(always)]
    pub fn is_hfxt2ifg_0(&self) -> bool {
        *self == Hfxt2ifgEnumRead::Hfxt2ifg0
    }
    #[doc = "HFXT2 fault. A HFXT2 fault occurred after the last reset"]
    #[inline(always)]
    pub fn is_hfxt2ifg_1(&self) -> bool {
        *self == Hfxt2ifgEnumRead::Hfxt2ifg1
    }
}
#[doc = "DCO external resistor short circuit fault flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcorShtifgEnumRead {
    #[doc = "0: DCO external resistor present"]
    DcorShtifg0 = 0,
    #[doc = "1: DCO external resistor short circuit fault"]
    DcorShtifg1 = 1,
}
impl From<DcorShtifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: DcorShtifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCOR_SHTIFG` reader - DCO external resistor short circuit fault flag."]
pub type DcorShtifgR = crate::BitReader<DcorShtifgEnumRead>;
impl DcorShtifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcorShtifgEnumRead {
        match self.bits {
            false => DcorShtifgEnumRead::DcorShtifg0,
            true => DcorShtifgEnumRead::DcorShtifg1,
        }
    }
    #[doc = "DCO external resistor present"]
    #[inline(always)]
    pub fn is_dcor_shtifg_0(&self) -> bool {
        *self == DcorShtifgEnumRead::DcorShtifg0
    }
    #[doc = "DCO external resistor short circuit fault"]
    #[inline(always)]
    pub fn is_dcor_shtifg_1(&self) -> bool {
        *self == DcorShtifgEnumRead::DcorShtifg1
    }
}
#[doc = "DCO external resistor open circuit fault flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcorOpnifgEnumRead {
    #[doc = "0: DCO external resistor present"]
    DcorOpnifg0 = 0,
    #[doc = "1: DCO external resistor open circuit fault"]
    DcorOpnifg1 = 1,
}
impl From<DcorOpnifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: DcorOpnifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCOR_OPNIFG` reader - DCO external resistor open circuit fault flag."]
pub type DcorOpnifgR = crate::BitReader<DcorOpnifgEnumRead>;
impl DcorOpnifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcorOpnifgEnumRead {
        match self.bits {
            false => DcorOpnifgEnumRead::DcorOpnifg0,
            true => DcorOpnifgEnumRead::DcorOpnifg1,
        }
    }
    #[doc = "DCO external resistor present"]
    #[inline(always)]
    pub fn is_dcor_opnifg_0(&self) -> bool {
        *self == DcorOpnifgEnumRead::DcorOpnifg0
    }
    #[doc = "DCO external resistor open circuit fault"]
    #[inline(always)]
    pub fn is_dcor_opnifg_1(&self) -> bool {
        *self == DcorOpnifgEnumRead::DcorOpnifg1
    }
}
#[doc = "Start fault counter interrupt flag LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FcntlfifgEnumRead {
    #[doc = "0: Start counter not expired"]
    Fcntlfifg0 = 0,
    #[doc = "1: Start counter expired"]
    Fcntlfifg1 = 1,
}
impl From<FcntlfifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: FcntlfifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCNTLFIFG` reader - Start fault counter interrupt flag LFXT"]
pub type FcntlfifgR = crate::BitReader<FcntlfifgEnumRead>;
impl FcntlfifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FcntlfifgEnumRead {
        match self.bits {
            false => FcntlfifgEnumRead::Fcntlfifg0,
            true => FcntlfifgEnumRead::Fcntlfifg1,
        }
    }
    #[doc = "Start counter not expired"]
    #[inline(always)]
    pub fn is_fcntlfifg_0(&self) -> bool {
        *self == FcntlfifgEnumRead::Fcntlfifg0
    }
    #[doc = "Start counter expired"]
    #[inline(always)]
    pub fn is_fcntlfifg_1(&self) -> bool {
        *self == FcntlfifgEnumRead::Fcntlfifg1
    }
}
#[doc = "Start fault counter interrupt flag HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FcnthfifgEnumRead {
    #[doc = "0: Start counter not expired"]
    Fcnthfifg0 = 0,
    #[doc = "1: Start counter expired"]
    Fcnthfifg1 = 1,
}
impl From<FcnthfifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: FcnthfifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCNTHFIFG` reader - Start fault counter interrupt flag HFXT"]
pub type FcnthfifgR = crate::BitReader<FcnthfifgEnumRead>;
impl FcnthfifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FcnthfifgEnumRead {
        match self.bits {
            false => FcnthfifgEnumRead::Fcnthfifg0,
            true => FcnthfifgEnumRead::Fcnthfifg1,
        }
    }
    #[doc = "Start counter not expired"]
    #[inline(always)]
    pub fn is_fcnthfifg_0(&self) -> bool {
        *self == FcnthfifgEnumRead::Fcnthfifg0
    }
    #[doc = "Start counter expired"]
    #[inline(always)]
    pub fn is_fcnthfifg_1(&self) -> bool {
        *self == FcnthfifgEnumRead::Fcnthfifg1
    }
}
#[doc = "Start fault counter interrupt flag HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcnthf2ifgEnumRead {
    #[doc = "0: Start counter not expired"]
    Fcnthf2ifg0 = 0,
    #[doc = "1: Start counter expired"]
    Fcnthf2ifg1 = 1,
}
impl From<Fcnthf2ifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: Fcnthf2ifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCNTHF2IFG` reader - Start fault counter interrupt flag HFXT2"]
pub type Fcnthf2ifgR = crate::BitReader<Fcnthf2ifgEnumRead>;
impl Fcnthf2ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcnthf2ifgEnumRead {
        match self.bits {
            false => Fcnthf2ifgEnumRead::Fcnthf2ifg0,
            true => Fcnthf2ifgEnumRead::Fcnthf2ifg1,
        }
    }
    #[doc = "Start counter not expired"]
    #[inline(always)]
    pub fn is_fcnthf2ifg_0(&self) -> bool {
        *self == Fcnthf2ifgEnumRead::Fcnthf2ifg0
    }
    #[doc = "Start counter expired"]
    #[inline(always)]
    pub fn is_fcnthf2ifg_1(&self) -> bool {
        *self == Fcnthf2ifgEnumRead::Fcnthf2ifg1
    }
}
#[doc = "PLL out-of-lock interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlloolifgEnumRead {
    #[doc = "0: No interrupt pending"]
    Plloolifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Plloolifg1 = 1,
}
impl From<PlloolifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: PlloolifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLOOLIFG` reader - PLL out-of-lock interrupt flag"]
pub type PlloolifgR = crate::BitReader<PlloolifgEnumRead>;
impl PlloolifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PlloolifgEnumRead {
        match self.bits {
            false => PlloolifgEnumRead::Plloolifg0,
            true => PlloolifgEnumRead::Plloolifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_plloolifg_0(&self) -> bool {
        *self == PlloolifgEnumRead::Plloolifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_plloolifg_1(&self) -> bool {
        *self == PlloolifgEnumRead::Plloolifg1
    }
}
#[doc = "PLL loss-of-signal interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlllosifgEnumRead {
    #[doc = "0: No interrupt pending"]
    Plllosifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Plllosifg1 = 1,
}
impl From<PlllosifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: PlllosifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLLOSIFG` reader - PLL loss-of-signal interrupt flag"]
pub type PlllosifgR = crate::BitReader<PlllosifgEnumRead>;
impl PlllosifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PlllosifgEnumRead {
        match self.bits {
            false => PlllosifgEnumRead::Plllosifg0,
            true => PlllosifgEnumRead::Plllosifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_plllosifg_0(&self) -> bool {
        *self == PlllosifgEnumRead::Plllosifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_plllosifg_1(&self) -> bool {
        *self == PlllosifgEnumRead::Plllosifg1
    }
}
#[doc = "PLL out-of-range interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlloorifgEnumRead {
    #[doc = "0: No interrupt pending"]
    Plloorifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Plloorifg1 = 1,
}
impl From<PlloorifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: PlloorifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLOORIFG` reader - PLL out-of-range interrupt flag"]
pub type PlloorifgR = crate::BitReader<PlloorifgEnumRead>;
impl PlloorifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PlloorifgEnumRead {
        match self.bits {
            false => PlloorifgEnumRead::Plloorifg0,
            true => PlloorifgEnumRead::Plloorifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_plloorifg_0(&self) -> bool {
        *self == PlloorifgEnumRead::Plloorifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_plloorifg_1(&self) -> bool {
        *self == PlloorifgEnumRead::Plloorifg1
    }
}
#[doc = "REFCNT period counter expired\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalifgEnumRead {
    #[doc = "0: REFCNT period counter not expired"]
    Califg0 = 0,
    #[doc = "1: REFCNT period counter expired"]
    Califg1 = 1,
}
impl From<CalifgEnumRead> for bool {
    #[inline(always)]
    fn from(variant: CalifgEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALIFG` reader - REFCNT period counter expired"]
pub type CalifgR = crate::BitReader<CalifgEnumRead>;
impl CalifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CalifgEnumRead {
        match self.bits {
            false => CalifgEnumRead::Califg0,
            true => CalifgEnumRead::Califg1,
        }
    }
    #[doc = "REFCNT period counter not expired"]
    #[inline(always)]
    pub fn is_califg_0(&self) -> bool {
        *self == CalifgEnumRead::Califg0
    }
    #[doc = "REFCNT period counter expired"]
    #[inline(always)]
    pub fn is_califg_1(&self) -> bool {
        *self == CalifgEnumRead::Califg1
    }
}
impl R {
    #[doc = "Bit 0 - LFXT oscillator fault flag"]
    #[inline(always)]
    pub fn lfxtifg(&self) -> LfxtifgR {
        LfxtifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXT oscillator fault flag"]
    #[inline(always)]
    pub fn hfxtifg(&self) -> HfxtifgR {
        HfxtifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HFXT2 oscillator fault flag"]
    #[inline(always)]
    pub fn hfxt2ifg(&self) -> Hfxt2ifgR {
        Hfxt2ifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - DCO external resistor short circuit fault flag."]
    #[inline(always)]
    pub fn dcor_shtifg(&self) -> DcorShtifgR {
        DcorShtifgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DCO external resistor open circuit fault flag."]
    #[inline(always)]
    pub fn dcor_opnifg(&self) -> DcorOpnifgR {
        DcorOpnifgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Start fault counter interrupt flag LFXT"]
    #[inline(always)]
    pub fn fcntlfifg(&self) -> FcntlfifgR {
        FcntlfifgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start fault counter interrupt flag HFXT"]
    #[inline(always)]
    pub fn fcnthfifg(&self) -> FcnthfifgR {
        FcnthfifgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Start fault counter interrupt flag HFXT2"]
    #[inline(always)]
    pub fn fcnthf2ifg(&self) -> Fcnthf2ifgR {
        Fcnthf2ifgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL out-of-lock interrupt flag"]
    #[inline(always)]
    pub fn plloolifg(&self) -> PlloolifgR {
        PlloolifgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLL loss-of-signal interrupt flag"]
    #[inline(always)]
    pub fn plllosifg(&self) -> PlllosifgR {
        PlllosifgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PLL out-of-range interrupt flag"]
    #[inline(always)]
    pub fn plloorifg(&self) -> PlloorifgR {
        PlloorifgR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - REFCNT period counter expired"]
    #[inline(always)]
    pub fn califg(&self) -> CalifgR {
        CalifgR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csifg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsifgSpec;
impl crate::RegisterSpec for CsifgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csifg::R`](R) reader structure"]
impl crate::Readable for CsifgSpec {}
#[doc = "`reset()` method sets CSIFG to value 0x01"]
impl crate::Resettable for CsifgSpec {
    const RESET_VALUE: u32 = 0x01;
}
