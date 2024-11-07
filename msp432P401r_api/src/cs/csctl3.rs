#[doc = "Register `CSCTL3` reader"]
pub type R = crate::R<Csctl3Spec>;
#[doc = "Register `CSCTL3` writer"]
pub type W = crate::W<Csctl3Spec>;
#[doc = "Start flag counter for LFXT\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fcntlf {
    #[doc = "0: 4096 cycles"]
    Fcntlf0 = 0,
    #[doc = "1: 8192 cycles"]
    Fcntlf1 = 1,
    #[doc = "2: 16384 cycles"]
    Fcntlf2 = 2,
    #[doc = "3: 32768 cycles"]
    Fcntlf3 = 3,
}
impl From<Fcntlf> for u8 {
    #[inline(always)]
    fn from(variant: Fcntlf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fcntlf {
    type Ux = u8;
}
impl crate::IsEnum for Fcntlf {}
#[doc = "Field `FCNTLF` reader - Start flag counter for LFXT"]
pub type FcntlfR = crate::FieldReader<Fcntlf>;
impl FcntlfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcntlf {
        match self.bits {
            0 => Fcntlf::Fcntlf0,
            1 => Fcntlf::Fcntlf1,
            2 => Fcntlf::Fcntlf2,
            3 => Fcntlf::Fcntlf3,
            _ => unreachable!(),
        }
    }
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn is_fcntlf_0(&self) -> bool {
        *self == Fcntlf::Fcntlf0
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn is_fcntlf_1(&self) -> bool {
        *self == Fcntlf::Fcntlf1
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn is_fcntlf_2(&self) -> bool {
        *self == Fcntlf::Fcntlf2
    }
    #[doc = "32768 cycles"]
    #[inline(always)]
    pub fn is_fcntlf_3(&self) -> bool {
        *self == Fcntlf::Fcntlf3
    }
}
#[doc = "Field `FCNTLF` writer - Start flag counter for LFXT"]
pub type FcntlfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fcntlf, crate::Safe>;
impl<'a, REG> FcntlfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn fcntlf_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntlf::Fcntlf0)
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn fcntlf_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntlf::Fcntlf1)
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn fcntlf_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntlf::Fcntlf2)
    }
    #[doc = "32768 cycles"]
    #[inline(always)]
    pub fn fcntlf_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fcntlf::Fcntlf3)
    }
}
#[doc = "Reset start fault counter for LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RfcntlfEnumWrite {
    #[doc = "0: Not applicable. Always reads as zero due to self clearing."]
    Rfcntlf0 = 0,
    #[doc = "1: Restarts the counter immediately."]
    Rfcntlf1 = 1,
}
impl From<RfcntlfEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: RfcntlfEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCNTLF` writer - Reset start fault counter for LFXT"]
pub type RfcntlfW<'a, REG> = crate::BitWriter<'a, REG, RfcntlfEnumWrite>;
impl<'a, REG> RfcntlfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not applicable. Always reads as zero due to self clearing."]
    #[inline(always)]
    pub fn rfcntlf_0(self) -> &'a mut crate::W<REG> {
        self.variant(RfcntlfEnumWrite::Rfcntlf0)
    }
    #[doc = "Restarts the counter immediately."]
    #[inline(always)]
    pub fn rfcntlf_1(self) -> &'a mut crate::W<REG> {
        self.variant(RfcntlfEnumWrite::Rfcntlf1)
    }
}
#[doc = "Enable start fault counter for LFXT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FcntlfEn {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    FcntlfEn0 = 0,
    #[doc = "1: Startup fault counter enabled."]
    FcntlfEn1 = 1,
}
impl From<FcntlfEn> for bool {
    #[inline(always)]
    fn from(variant: FcntlfEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCNTLF_EN` reader - Enable start fault counter for LFXT"]
pub type FcntlfEnR = crate::BitReader<FcntlfEn>;
impl FcntlfEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FcntlfEn {
        match self.bits {
            false => FcntlfEn::FcntlfEn0,
            true => FcntlfEn::FcntlfEn1,
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn is_fcntlf_en_0(&self) -> bool {
        *self == FcntlfEn::FcntlfEn0
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn is_fcntlf_en_1(&self) -> bool {
        *self == FcntlfEn::FcntlfEn1
    }
}
#[doc = "Field `FCNTLF_EN` writer - Enable start fault counter for LFXT"]
pub type FcntlfEnW<'a, REG> = crate::BitWriter<'a, REG, FcntlfEn>;
impl<'a, REG> FcntlfEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn fcntlf_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(FcntlfEn::FcntlfEn0)
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn fcntlf_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(FcntlfEn::FcntlfEn1)
    }
}
#[doc = "Start flag counter for HFXT\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fcnthf {
    #[doc = "0: 2048 cycles"]
    Fcnthf0 = 0,
    #[doc = "1: 4096 cycles"]
    Fcnthf1 = 1,
    #[doc = "2: 8192 cycles"]
    Fcnthf2 = 2,
    #[doc = "3: 16384 cycles"]
    Fcnthf3 = 3,
}
impl From<Fcnthf> for u8 {
    #[inline(always)]
    fn from(variant: Fcnthf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fcnthf {
    type Ux = u8;
}
impl crate::IsEnum for Fcnthf {}
#[doc = "Field `FCNTHF` reader - Start flag counter for HFXT"]
pub type FcnthfR = crate::FieldReader<Fcnthf>;
impl FcnthfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcnthf {
        match self.bits {
            0 => Fcnthf::Fcnthf0,
            1 => Fcnthf::Fcnthf1,
            2 => Fcnthf::Fcnthf2,
            3 => Fcnthf::Fcnthf3,
            _ => unreachable!(),
        }
    }
    #[doc = "2048 cycles"]
    #[inline(always)]
    pub fn is_fcnthf_0(&self) -> bool {
        *self == Fcnthf::Fcnthf0
    }
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn is_fcnthf_1(&self) -> bool {
        *self == Fcnthf::Fcnthf1
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn is_fcnthf_2(&self) -> bool {
        *self == Fcnthf::Fcnthf2
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn is_fcnthf_3(&self) -> bool {
        *self == Fcnthf::Fcnthf3
    }
}
#[doc = "Field `FCNTHF` writer - Start flag counter for HFXT"]
pub type FcnthfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fcnthf, crate::Safe>;
impl<'a, REG> FcnthfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2048 cycles"]
    #[inline(always)]
    pub fn fcnthf_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf::Fcnthf0)
    }
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn fcnthf_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf::Fcnthf1)
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn fcnthf_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf::Fcnthf2)
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn fcnthf_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf::Fcnthf3)
    }
}
#[doc = "Reset start fault counter for HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RfcnthfEnumWrite {
    #[doc = "0: Not applicable. Always reads as zero due to self clearing."]
    Rfcnthf0 = 0,
    #[doc = "1: Restarts the counter immediately."]
    Rfcnthf1 = 1,
}
impl From<RfcnthfEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: RfcnthfEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCNTHF` writer - Reset start fault counter for HFXT"]
pub type RfcnthfW<'a, REG> = crate::BitWriter<'a, REG, RfcnthfEnumWrite>;
impl<'a, REG> RfcnthfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not applicable. Always reads as zero due to self clearing."]
    #[inline(always)]
    pub fn rfcnthf_0(self) -> &'a mut crate::W<REG> {
        self.variant(RfcnthfEnumWrite::Rfcnthf0)
    }
    #[doc = "Restarts the counter immediately."]
    #[inline(always)]
    pub fn rfcnthf_1(self) -> &'a mut crate::W<REG> {
        self.variant(RfcnthfEnumWrite::Rfcnthf1)
    }
}
#[doc = "Enable start fault counter for HFXT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FcnthfEn {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    FcnthfEn0 = 0,
    #[doc = "1: Startup fault counter enabled."]
    FcnthfEn1 = 1,
}
impl From<FcnthfEn> for bool {
    #[inline(always)]
    fn from(variant: FcnthfEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCNTHF_EN` reader - Enable start fault counter for HFXT"]
pub type FcnthfEnR = crate::BitReader<FcnthfEn>;
impl FcnthfEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FcnthfEn {
        match self.bits {
            false => FcnthfEn::FcnthfEn0,
            true => FcnthfEn::FcnthfEn1,
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn is_fcnthf_en_0(&self) -> bool {
        *self == FcnthfEn::FcnthfEn0
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn is_fcnthf_en_1(&self) -> bool {
        *self == FcnthfEn::FcnthfEn1
    }
}
#[doc = "Field `FCNTHF_EN` writer - Enable start fault counter for HFXT"]
pub type FcnthfEnW<'a, REG> = crate::BitWriter<'a, REG, FcnthfEn>;
impl<'a, REG> FcnthfEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn fcnthf_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(FcnthfEn::FcnthfEn0)
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn fcnthf_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(FcnthfEn::FcnthfEn1)
    }
}
#[doc = "Start flag counter for HFXT2\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fcnthf2 {
    #[doc = "0: 2048 cycles"]
    Fcnthf2_0 = 0,
    #[doc = "1: 4096 cycles"]
    Fcnthf2_1 = 1,
    #[doc = "2: 8192 cycles"]
    Fcnthf2_2 = 2,
    #[doc = "3: 16384 cycles"]
    Fcnthf2_3 = 3,
}
impl From<Fcnthf2> for u8 {
    #[inline(always)]
    fn from(variant: Fcnthf2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fcnthf2 {
    type Ux = u8;
}
impl crate::IsEnum for Fcnthf2 {}
#[doc = "Field `FCNTHF2` reader - Start flag counter for HFXT2"]
pub type Fcnthf2R = crate::FieldReader<Fcnthf2>;
impl Fcnthf2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcnthf2 {
        match self.bits {
            0 => Fcnthf2::Fcnthf2_0,
            1 => Fcnthf2::Fcnthf2_1,
            2 => Fcnthf2::Fcnthf2_2,
            3 => Fcnthf2::Fcnthf2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "2048 cycles"]
    #[inline(always)]
    pub fn is_fcnthf2_0(&self) -> bool {
        *self == Fcnthf2::Fcnthf2_0
    }
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn is_fcnthf2_1(&self) -> bool {
        *self == Fcnthf2::Fcnthf2_1
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn is_fcnthf2_2(&self) -> bool {
        *self == Fcnthf2::Fcnthf2_2
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn is_fcnthf2_3(&self) -> bool {
        *self == Fcnthf2::Fcnthf2_3
    }
}
#[doc = "Field `FCNTHF2` writer - Start flag counter for HFXT2"]
pub type Fcnthf2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Fcnthf2, crate::Safe>;
impl<'a, REG> Fcnthf2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2048 cycles"]
    #[inline(always)]
    pub fn fcnthf2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf2::Fcnthf2_0)
    }
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn fcnthf2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf2::Fcnthf2_1)
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn fcnthf2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf2::Fcnthf2_2)
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn fcnthf2_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf2::Fcnthf2_3)
    }
}
#[doc = "Reset start fault counter for HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfcnthf2EnumWrite {
    #[doc = "0: Not applicable. Always reads as zero due to self clearing."]
    Rfcnthf2_0 = 0,
    #[doc = "1: Restarts the counter immediately."]
    Rfcnthf2_1 = 1,
}
impl From<Rfcnthf2EnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Rfcnthf2EnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCNTHF2` writer - Reset start fault counter for HFXT2"]
pub type Rfcnthf2W<'a, REG> = crate::BitWriter<'a, REG, Rfcnthf2EnumWrite>;
impl<'a, REG> Rfcnthf2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not applicable. Always reads as zero due to self clearing."]
    #[inline(always)]
    pub fn rfcnthf2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rfcnthf2EnumWrite::Rfcnthf2_0)
    }
    #[doc = "Restarts the counter immediately."]
    #[inline(always)]
    pub fn rfcnthf2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rfcnthf2EnumWrite::Rfcnthf2_1)
    }
}
#[doc = "Enable start fault counter for HFXT2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcnthf2En {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    Fcnthf2En0 = 0,
    #[doc = "1: Startup fault counter enabled."]
    Fcnthf2En1 = 1,
}
impl From<Fcnthf2En> for bool {
    #[inline(always)]
    fn from(variant: Fcnthf2En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCNTHF2_EN` reader - Enable start fault counter for HFXT2"]
pub type Fcnthf2EnR = crate::BitReader<Fcnthf2En>;
impl Fcnthf2EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcnthf2En {
        match self.bits {
            false => Fcnthf2En::Fcnthf2En0,
            true => Fcnthf2En::Fcnthf2En1,
        }
    }
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn is_fcnthf2_en_0(&self) -> bool {
        *self == Fcnthf2En::Fcnthf2En0
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn is_fcnthf2_en_1(&self) -> bool {
        *self == Fcnthf2En::Fcnthf2En1
    }
}
#[doc = "Field `FCNTHF2_EN` writer - Enable start fault counter for HFXT2"]
pub type Fcnthf2EnW<'a, REG> = crate::BitWriter<'a, REG, Fcnthf2En>;
impl<'a, REG> Fcnthf2EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn fcnthf2_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf2En::Fcnthf2En0)
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn fcnthf2_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcnthf2En::Fcnthf2En1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Start flag counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf(&self) -> FcntlfR {
        FcntlfR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Enable start fault counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf_en(&self) -> FcntlfEnR {
        FcntlfEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Start flag counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf(&self) -> FcnthfR {
        FcnthfR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Enable start fault counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf_en(&self) -> FcnthfEnR {
        FcnthfEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Start flag counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2(&self) -> Fcnthf2R {
        Fcnthf2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Enable start fault counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2_en(&self) -> Fcnthf2EnR {
        Fcnthf2EnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Start flag counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf(&mut self) -> FcntlfW<Csctl3Spec> {
        FcntlfW::new(self, 0)
    }
    #[doc = "Bit 2 - Reset start fault counter for LFXT"]
    #[inline(always)]
    pub fn rfcntlf(&mut self) -> RfcntlfW<Csctl3Spec> {
        RfcntlfW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable start fault counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf_en(&mut self) -> FcntlfEnW<Csctl3Spec> {
        FcntlfEnW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Start flag counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf(&mut self) -> FcnthfW<Csctl3Spec> {
        FcnthfW::new(self, 4)
    }
    #[doc = "Bit 6 - Reset start fault counter for HFXT"]
    #[inline(always)]
    pub fn rfcnthf(&mut self) -> RfcnthfW<Csctl3Spec> {
        RfcnthfW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable start fault counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf_en(&mut self) -> FcnthfEnW<Csctl3Spec> {
        FcnthfEnW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Start flag counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2(&mut self) -> Fcnthf2W<Csctl3Spec> {
        Fcnthf2W::new(self, 8)
    }
    #[doc = "Bit 10 - Reset start fault counter for HFXT2"]
    #[inline(always)]
    pub fn rfcnthf2(&mut self) -> Rfcnthf2W<Csctl3Spec> {
        Rfcnthf2W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable start fault counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2_en(&mut self) -> Fcnthf2EnW<Csctl3Spec> {
        Fcnthf2EnW::new(self, 11)
    }
}
#[doc = "Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl3Spec;
impl crate::RegisterSpec for Csctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csctl3::R`](R) reader structure"]
impl crate::Readable for Csctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl3::W`](W) writer structure"]
impl crate::Writable for Csctl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSCTL3 to value 0x0bbb"]
impl crate::Resettable for Csctl3Spec {
    const RESET_VALUE: u32 = 0x0bbb;
}
