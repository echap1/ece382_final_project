#[doc = "Register `CSIE` reader"]
pub type R = crate::R<CsieSpec>;
#[doc = "Register `CSIE` writer"]
pub type W = crate::W<CsieSpec>;
#[doc = "LFXT oscillator fault flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfxtie {
    #[doc = "0: Interrupt disabled"]
    Lfxtie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Lfxtie1 = 1,
}
impl From<Lfxtie> for bool {
    #[inline(always)]
    fn from(variant: Lfxtie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTIE` reader - LFXT oscillator fault flag interrupt enable"]
pub type LfxtieR = crate::BitReader<Lfxtie>;
impl LfxtieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfxtie {
        match self.bits {
            false => Lfxtie::Lfxtie0,
            true => Lfxtie::Lfxtie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_lfxtie_0(&self) -> bool {
        *self == Lfxtie::Lfxtie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_lfxtie_1(&self) -> bool {
        *self == Lfxtie::Lfxtie1
    }
}
#[doc = "Field `LFXTIE` writer - LFXT oscillator fault flag interrupt enable"]
pub type LfxtieW<'a, REG> = crate::BitWriter<'a, REG, Lfxtie>;
impl<'a, REG> LfxtieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lfxtie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtie::Lfxtie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lfxtie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lfxtie::Lfxtie1)
    }
}
#[doc = "HFXT oscillator fault flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfxtie {
    #[doc = "0: Interrupt disabled"]
    Hfxtie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Hfxtie1 = 1,
}
impl From<Hfxtie> for bool {
    #[inline(always)]
    fn from(variant: Hfxtie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXTIE` reader - HFXT oscillator fault flag interrupt enable"]
pub type HfxtieR = crate::BitReader<Hfxtie>;
impl HfxtieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxtie {
        match self.bits {
            false => Hfxtie::Hfxtie0,
            true => Hfxtie::Hfxtie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_hfxtie_0(&self) -> bool {
        *self == Hfxtie::Hfxtie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_hfxtie_1(&self) -> bool {
        *self == Hfxtie::Hfxtie1
    }
}
#[doc = "Field `HFXTIE` writer - HFXT oscillator fault flag interrupt enable"]
pub type HfxtieW<'a, REG> = crate::BitWriter<'a, REG, Hfxtie>;
impl<'a, REG> HfxtieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn hfxtie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtie::Hfxtie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn hfxtie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxtie::Hfxtie1)
    }
}
#[doc = "HFXT2 oscillator fault flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfxt2ie {
    #[doc = "0: Interrupt disabled"]
    Hfxt2ie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Hfxt2ie1 = 1,
}
impl From<Hfxt2ie> for bool {
    #[inline(always)]
    fn from(variant: Hfxt2ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFXT2IE` reader - HFXT2 oscillator fault flag interrupt enable"]
pub type Hfxt2ieR = crate::BitReader<Hfxt2ie>;
impl Hfxt2ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfxt2ie {
        match self.bits {
            false => Hfxt2ie::Hfxt2ie0,
            true => Hfxt2ie::Hfxt2ie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_hfxt2ie_0(&self) -> bool {
        *self == Hfxt2ie::Hfxt2ie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_hfxt2ie_1(&self) -> bool {
        *self == Hfxt2ie::Hfxt2ie1
    }
}
#[doc = "Field `HFXT2IE` writer - HFXT2 oscillator fault flag interrupt enable"]
pub type Hfxt2ieW<'a, REG> = crate::BitWriter<'a, REG, Hfxt2ie>;
impl<'a, REG> Hfxt2ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn hfxt2ie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxt2ie::Hfxt2ie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn hfxt2ie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfxt2ie::Hfxt2ie1)
    }
}
#[doc = "DCO external resistor open circuit fault flag interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcorOpnie {
    #[doc = "0: Interrupt disabled"]
    DcorOpnie0 = 0,
    #[doc = "1: Interrupt enabled"]
    DcorOpnie1 = 1,
}
impl From<DcorOpnie> for bool {
    #[inline(always)]
    fn from(variant: DcorOpnie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCOR_OPNIE` reader - DCO external resistor open circuit fault flag interrupt enable."]
pub type DcorOpnieR = crate::BitReader<DcorOpnie>;
impl DcorOpnieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DcorOpnie {
        match self.bits {
            false => DcorOpnie::DcorOpnie0,
            true => DcorOpnie::DcorOpnie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_dcor_opnie_0(&self) -> bool {
        *self == DcorOpnie::DcorOpnie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_dcor_opnie_1(&self) -> bool {
        *self == DcorOpnie::DcorOpnie1
    }
}
#[doc = "Field `DCOR_OPNIE` writer - DCO external resistor open circuit fault flag interrupt enable."]
pub type DcorOpnieW<'a, REG> = crate::BitWriter<'a, REG, DcorOpnie>;
impl<'a, REG> DcorOpnieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn dcor_opnie_0(self) -> &'a mut crate::W<REG> {
        self.variant(DcorOpnie::DcorOpnie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn dcor_opnie_1(self) -> &'a mut crate::W<REG> {
        self.variant(DcorOpnie::DcorOpnie1)
    }
}
#[doc = "Start fault counter interrupt enable LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcntlfie {
    #[doc = "0: Interrupt disabled"]
    Fcntlfie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Fcntlfie1 = 1,
}
impl From<Fcntlfie> for bool {
    #[inline(always)]
    fn from(variant: Fcntlfie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCNTLFIE` reader - Start fault counter interrupt enable LFXT"]
pub type FcntlfieR = crate::BitReader<Fcntlfie>;
impl FcntlfieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcntlfie {
        match self.bits {
            false => Fcntlfie::Fcntlfie0,
            true => Fcntlfie::Fcntlfie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_fcntlfie_0(&self) -> bool {
        *self == Fcntlfie::Fcntlfie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_fcntlfie_1(&self) -> bool {
        *self == Fcntlfie::Fcntlfie1
    }
}
#[doc = "Field `FCNTLFIE` writer - Start fault counter interrupt enable LFXT"]
pub type FcntlfieW<'a, REG> = crate::BitWriter<'a, REG, Fcntlfie>;
impl<'a, REG> FcntlfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn fcntlfie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntlfie::Fcntlfie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn fcntlfie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntlfie::Fcntlfie1)
    }
}
#[doc = "Start fault counter interrupt enable HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcnthfie {
    #[doc = "0: Interrupt disabled"]
    Fcnthfie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Fcnthfie1 = 1,
}
impl From<Fcnthfie> for bool {
    #[inline(always)]
    fn from(variant: Fcnthfie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCNTHFIE` reader - Start fault counter interrupt enable HFXT"]
pub type FcnthfieR = crate::BitReader<Fcnthfie>;
impl FcnthfieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcnthfie {
        match self.bits {
            false => Fcnthfie::Fcnthfie0,
            true => Fcnthfie::Fcnthfie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_fcnthfie_0(&self) -> bool {
        *self == Fcnthfie::Fcnthfie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_fcnthfie_1(&self) -> bool {
        *self == Fcnthfie::Fcnthfie1
    }
}
#[doc = "Field `FCNTHFIE` writer - Start fault counter interrupt enable HFXT"]
pub type FcnthfieW<'a, REG> = crate::BitWriter<'a, REG, Fcnthfie>;
impl<'a, REG> FcnthfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn fcnthfie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthfie::Fcnthfie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn fcnthfie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthfie::Fcnthfie1)
    }
}
#[doc = "Start fault counter interrupt enable HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcnthf2ie {
    #[doc = "0: Interrupt disabled"]
    Fcnthf2ie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Fcnthf2ie1 = 1,
}
impl From<Fcnthf2ie> for bool {
    #[inline(always)]
    fn from(variant: Fcnthf2ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCNTHF2IE` reader - Start fault counter interrupt enable HFXT2"]
pub type Fcnthf2ieR = crate::BitReader<Fcnthf2ie>;
impl Fcnthf2ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcnthf2ie {
        match self.bits {
            false => Fcnthf2ie::Fcnthf2ie0,
            true => Fcnthf2ie::Fcnthf2ie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_fcnthf2ie_0(&self) -> bool {
        *self == Fcnthf2ie::Fcnthf2ie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_fcnthf2ie_1(&self) -> bool {
        *self == Fcnthf2ie::Fcnthf2ie1
    }
}
#[doc = "Field `FCNTHF2IE` writer - Start fault counter interrupt enable HFXT2"]
pub type Fcnthf2ieW<'a, REG> = crate::BitWriter<'a, REG, Fcnthf2ie>;
impl<'a, REG> Fcnthf2ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn fcnthf2ie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf2ie::Fcnthf2ie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn fcnthf2ie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf2ie::Fcnthf2ie1)
    }
}
#[doc = "PLL out-of-lock interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plloolie {
    #[doc = "0: Interrupt disabled"]
    Plloolie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Plloolie1 = 1,
}
impl From<Plloolie> for bool {
    #[inline(always)]
    fn from(variant: Plloolie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLOOLIE` reader - PLL out-of-lock interrupt enable"]
pub type PlloolieR = crate::BitReader<Plloolie>;
impl PlloolieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plloolie {
        match self.bits {
            false => Plloolie::Plloolie0,
            true => Plloolie::Plloolie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_plloolie_0(&self) -> bool {
        *self == Plloolie::Plloolie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_plloolie_1(&self) -> bool {
        *self == Plloolie::Plloolie1
    }
}
#[doc = "Field `PLLOOLIE` writer - PLL out-of-lock interrupt enable"]
pub type PlloolieW<'a, REG> = crate::BitWriter<'a, REG, Plloolie>;
impl<'a, REG> PlloolieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn plloolie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Plloolie::Plloolie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn plloolie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Plloolie::Plloolie1)
    }
}
#[doc = "PLL loss-of-signal interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plllosie {
    #[doc = "0: Interrupt disabled"]
    Plllosie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Plllosie1 = 1,
}
impl From<Plllosie> for bool {
    #[inline(always)]
    fn from(variant: Plllosie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLLOSIE` reader - PLL loss-of-signal interrupt enable"]
pub type PlllosieR = crate::BitReader<Plllosie>;
impl PlllosieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plllosie {
        match self.bits {
            false => Plllosie::Plllosie0,
            true => Plllosie::Plllosie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_plllosie_0(&self) -> bool {
        *self == Plllosie::Plllosie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_plllosie_1(&self) -> bool {
        *self == Plllosie::Plllosie1
    }
}
#[doc = "Field `PLLLOSIE` writer - PLL loss-of-signal interrupt enable"]
pub type PlllosieW<'a, REG> = crate::BitWriter<'a, REG, Plllosie>;
impl<'a, REG> PlllosieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn plllosie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Plllosie::Plllosie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn plllosie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Plllosie::Plllosie1)
    }
}
#[doc = "PLL out-of-range interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plloorie {
    #[doc = "0: Interrupt disabled"]
    Plloorie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Plloorie1 = 1,
}
impl From<Plloorie> for bool {
    #[inline(always)]
    fn from(variant: Plloorie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLOORIE` reader - PLL out-of-range interrupt enable"]
pub type PlloorieR = crate::BitReader<Plloorie>;
impl PlloorieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plloorie {
        match self.bits {
            false => Plloorie::Plloorie0,
            true => Plloorie::Plloorie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_plloorie_0(&self) -> bool {
        *self == Plloorie::Plloorie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_plloorie_1(&self) -> bool {
        *self == Plloorie::Plloorie1
    }
}
#[doc = "Field `PLLOORIE` writer - PLL out-of-range interrupt enable"]
pub type PlloorieW<'a, REG> = crate::BitWriter<'a, REG, Plloorie>;
impl<'a, REG> PlloorieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn plloorie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Plloorie::Plloorie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn plloorie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Plloorie::Plloorie1)
    }
}
#[doc = "REFCNT period counter interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calie {
    #[doc = "0: Interrupt disabled"]
    Calie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Calie1 = 1,
}
impl From<Calie> for bool {
    #[inline(always)]
    fn from(variant: Calie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALIE` reader - REFCNT period counter interrupt enable"]
pub type CalieR = crate::BitReader<Calie>;
impl CalieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calie {
        match self.bits {
            false => Calie::Calie0,
            true => Calie::Calie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_calie_0(&self) -> bool {
        *self == Calie::Calie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_calie_1(&self) -> bool {
        *self == Calie::Calie1
    }
}
#[doc = "Field `CALIE` writer - REFCNT period counter interrupt enable"]
pub type CalieW<'a, REG> = crate::BitWriter<'a, REG, Calie>;
impl<'a, REG> CalieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn calie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Calie::Calie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn calie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Calie::Calie1)
    }
}
impl R {
    #[doc = "Bit 0 - LFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn lfxtie(&self) -> LfxtieR {
        LfxtieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxtie(&self) -> HfxtieR {
        HfxtieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HFXT2 oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxt2ie(&self) -> Hfxt2ieR {
        Hfxt2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - DCO external resistor open circuit fault flag interrupt enable."]
    #[inline(always)]
    pub fn dcor_opnie(&self) -> DcorOpnieR {
        DcorOpnieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Start fault counter interrupt enable LFXT"]
    #[inline(always)]
    pub fn fcntlfie(&self) -> FcntlfieR {
        FcntlfieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start fault counter interrupt enable HFXT"]
    #[inline(always)]
    pub fn fcnthfie(&self) -> FcnthfieR {
        FcnthfieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Start fault counter interrupt enable HFXT2"]
    #[inline(always)]
    pub fn fcnthf2ie(&self) -> Fcnthf2ieR {
        Fcnthf2ieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL out-of-lock interrupt enable"]
    #[inline(always)]
    pub fn plloolie(&self) -> PlloolieR {
        PlloolieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLL loss-of-signal interrupt enable"]
    #[inline(always)]
    pub fn plllosie(&self) -> PlllosieR {
        PlllosieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PLL out-of-range interrupt enable"]
    #[inline(always)]
    pub fn plloorie(&self) -> PlloorieR {
        PlloorieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - REFCNT period counter interrupt enable"]
    #[inline(always)]
    pub fn calie(&self) -> CalieR {
        CalieR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn lfxtie(&mut self) -> LfxtieW<CsieSpec> {
        LfxtieW::new(self, 0)
    }
    #[doc = "Bit 1 - HFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxtie(&mut self) -> HfxtieW<CsieSpec> {
        HfxtieW::new(self, 1)
    }
    #[doc = "Bit 2 - HFXT2 oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxt2ie(&mut self) -> Hfxt2ieW<CsieSpec> {
        Hfxt2ieW::new(self, 2)
    }
    #[doc = "Bit 6 - DCO external resistor open circuit fault flag interrupt enable."]
    #[inline(always)]
    pub fn dcor_opnie(&mut self) -> DcorOpnieW<CsieSpec> {
        DcorOpnieW::new(self, 6)
    }
    #[doc = "Bit 8 - Start fault counter interrupt enable LFXT"]
    #[inline(always)]
    pub fn fcntlfie(&mut self) -> FcntlfieW<CsieSpec> {
        FcntlfieW::new(self, 8)
    }
    #[doc = "Bit 9 - Start fault counter interrupt enable HFXT"]
    #[inline(always)]
    pub fn fcnthfie(&mut self) -> FcnthfieW<CsieSpec> {
        FcnthfieW::new(self, 9)
    }
    #[doc = "Bit 10 - Start fault counter interrupt enable HFXT2"]
    #[inline(always)]
    pub fn fcnthf2ie(&mut self) -> Fcnthf2ieW<CsieSpec> {
        Fcnthf2ieW::new(self, 10)
    }
    #[doc = "Bit 12 - PLL out-of-lock interrupt enable"]
    #[inline(always)]
    pub fn plloolie(&mut self) -> PlloolieW<CsieSpec> {
        PlloolieW::new(self, 12)
    }
    #[doc = "Bit 13 - PLL loss-of-signal interrupt enable"]
    #[inline(always)]
    pub fn plllosie(&mut self) -> PlllosieW<CsieSpec> {
        PlllosieW::new(self, 13)
    }
    #[doc = "Bit 14 - PLL out-of-range interrupt enable"]
    #[inline(always)]
    pub fn plloorie(&mut self) -> PlloorieW<CsieSpec> {
        PlloorieW::new(self, 14)
    }
    #[doc = "Bit 15 - REFCNT period counter interrupt enable"]
    #[inline(always)]
    pub fn calie(&mut self) -> CalieW<CsieSpec> {
        CalieW::new(self, 15)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsieSpec;
impl crate::RegisterSpec for CsieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csie::R`](R) reader structure"]
impl crate::Readable for CsieSpec {}
#[doc = "`write(|w| ..)` method takes [`csie::W`](W) writer structure"]
impl crate::Writable for CsieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSIE to value 0"]
impl crate::Resettable for CsieSpec {
    const RESET_VALUE: u32 = 0;
}
