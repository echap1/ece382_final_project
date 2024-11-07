#[doc = "Register `TAxCTL` reader"]
pub type R = crate::R<TaxCtlSpec>;
#[doc = "Register `TAxCTL` writer"]
pub type W = crate::W<TaxCtlSpec>;
#[doc = "TimerA interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taifg {
    #[doc = "0: No interrupt pending"]
    Taifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Taifg1 = 1,
}
impl From<Taifg> for bool {
    #[inline(always)]
    fn from(variant: Taifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAIFG` reader - TimerA interrupt flag"]
pub type TaifgR = crate::BitReader<Taifg>;
impl TaifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taifg {
        match self.bits {
            false => Taifg::Taifg0,
            true => Taifg::Taifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_taifg_0(&self) -> bool {
        *self == Taifg::Taifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_taifg_1(&self) -> bool {
        *self == Taifg::Taifg1
    }
}
#[doc = "Field `TAIFG` writer - TimerA interrupt flag"]
pub type TaifgW<'a, REG> = crate::BitWriter<'a, REG, Taifg>;
impl<'a, REG> TaifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn taifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Taifg::Taifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn taifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Taifg::Taifg1)
    }
}
#[doc = "TimerA interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taie {
    #[doc = "0: Interrupt disabled"]
    Taie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Taie1 = 1,
}
impl From<Taie> for bool {
    #[inline(always)]
    fn from(variant: Taie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAIE` reader - TimerA interrupt enable"]
pub type TaieR = crate::BitReader<Taie>;
impl TaieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taie {
        match self.bits {
            false => Taie::Taie0,
            true => Taie::Taie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_taie_0(&self) -> bool {
        *self == Taie::Taie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_taie_1(&self) -> bool {
        *self == Taie::Taie1
    }
}
#[doc = "Field `TAIE` writer - TimerA interrupt enable"]
pub type TaieW<'a, REG> = crate::BitWriter<'a, REG, Taie>;
impl<'a, REG> TaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn taie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Taie::Taie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn taie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Taie::Taie1)
    }
}
#[doc = "Field `TACLR` reader - TimerA clear"]
pub type TaclrR = crate::BitReader;
#[doc = "Field `TACLR` writer - TimerA clear"]
pub type TaclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mc {
    #[doc = "0: Stop mode: Timer is halted"]
    Mc0 = 0,
    #[doc = "1: Up mode: Timer counts up to TAxCCR0"]
    Mc1 = 1,
    #[doc = "2: Continuous mode: Timer counts up to 0FFFFh"]
    Mc2 = 2,
    #[doc = "3: Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    Mc3 = 3,
}
impl From<Mc> for u8 {
    #[inline(always)]
    fn from(variant: Mc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mc {
    type Ux = u8;
}
impl crate::IsEnum for Mc {}
#[doc = "Field `MC` reader - Mode control"]
pub type McR = crate::FieldReader<Mc>;
impl McR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mc {
        match self.bits {
            0 => Mc::Mc0,
            1 => Mc::Mc1,
            2 => Mc::Mc2,
            3 => Mc::Mc3,
            _ => unreachable!(),
        }
    }
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn is_mc_0(&self) -> bool {
        *self == Mc::Mc0
    }
    #[doc = "Up mode: Timer counts up to TAxCCR0"]
    #[inline(always)]
    pub fn is_mc_1(&self) -> bool {
        *self == Mc::Mc1
    }
    #[doc = "Continuous mode: Timer counts up to 0FFFFh"]
    #[inline(always)]
    pub fn is_mc_2(&self) -> bool {
        *self == Mc::Mc2
    }
    #[doc = "Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    #[inline(always)]
    pub fn is_mc_3(&self) -> bool {
        *self == Mc::Mc3
    }
}
#[doc = "Field `MC` writer - Mode control"]
pub type McW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mc, crate::Safe>;
impl<'a, REG> McW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop mode: Timer is halted"]
    #[inline(always)]
    pub fn mc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Mc0)
    }
    #[doc = "Up mode: Timer counts up to TAxCCR0"]
    #[inline(always)]
    pub fn mc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Mc1)
    }
    #[doc = "Continuous mode: Timer counts up to 0FFFFh"]
    #[inline(always)]
    pub fn mc_2(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Mc2)
    }
    #[doc = "Up/down mode: Timer counts up to TAxCCR0 then down to 0000h"]
    #[inline(always)]
    pub fn mc_3(self) -> &'a mut crate::W<REG> {
        self.variant(Mc::Mc3)
    }
}
#[doc = "Input divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Id {
    #[doc = "0: /1"]
    Id0 = 0,
    #[doc = "1: /2"]
    Id1 = 1,
    #[doc = "2: /4"]
    Id2 = 2,
    #[doc = "3: /8"]
    Id3 = 3,
}
impl From<Id> for u8 {
    #[inline(always)]
    fn from(variant: Id) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Id {
    type Ux = u8;
}
impl crate::IsEnum for Id {}
#[doc = "Field `ID` reader - Input divider"]
pub type IdR = crate::FieldReader<Id>;
impl IdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Id {
        match self.bits {
            0 => Id::Id0,
            1 => Id::Id1,
            2 => Id::Id2,
            3 => Id::Id3,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_id_0(&self) -> bool {
        *self == Id::Id0
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_id_1(&self) -> bool {
        *self == Id::Id1
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_id_2(&self) -> bool {
        *self == Id::Id2
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_id_3(&self) -> bool {
        *self == Id::Id3
    }
}
#[doc = "Field `ID` writer - Input divider"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Id, crate::Safe>;
impl<'a, REG> IdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn id_0(self) -> &'a mut crate::W<REG> {
        self.variant(Id::Id0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn id_1(self) -> &'a mut crate::W<REG> {
        self.variant(Id::Id1)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn id_2(self) -> &'a mut crate::W<REG> {
        self.variant(Id::Id2)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn id_3(self) -> &'a mut crate::W<REG> {
        self.variant(Id::Id3)
    }
}
#[doc = "TimerA clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tassel {
    #[doc = "0: TAxCLK"]
    Tassel0 = 0,
    #[doc = "1: ACLK"]
    Tassel1 = 1,
    #[doc = "2: SMCLK"]
    Tassel2 = 2,
    #[doc = "3: INCLK"]
    Tassel3 = 3,
}
impl From<Tassel> for u8 {
    #[inline(always)]
    fn from(variant: Tassel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tassel {
    type Ux = u8;
}
impl crate::IsEnum for Tassel {}
#[doc = "Field `TASSEL` reader - TimerA clock source select"]
pub type TasselR = crate::FieldReader<Tassel>;
impl TasselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tassel {
        match self.bits {
            0 => Tassel::Tassel0,
            1 => Tassel::Tassel1,
            2 => Tassel::Tassel2,
            3 => Tassel::Tassel3,
            _ => unreachable!(),
        }
    }
    #[doc = "TAxCLK"]
    #[inline(always)]
    pub fn is_tassel_0(&self) -> bool {
        *self == Tassel::Tassel0
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_tassel_1(&self) -> bool {
        *self == Tassel::Tassel1
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_tassel_2(&self) -> bool {
        *self == Tassel::Tassel2
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn is_tassel_3(&self) -> bool {
        *self == Tassel::Tassel3
    }
}
#[doc = "Field `TASSEL` writer - TimerA clock source select"]
pub type TasselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tassel, crate::Safe>;
impl<'a, REG> TasselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TAxCLK"]
    #[inline(always)]
    pub fn tassel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Tassel0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn tassel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Tassel1)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn tassel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Tassel2)
    }
    #[doc = "INCLK"]
    #[inline(always)]
    pub fn tassel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Tassel::Tassel3)
    }
}
impl R {
    #[doc = "Bit 0 - TimerA interrupt flag"]
    #[inline(always)]
    pub fn taifg(&self) -> TaifgR {
        TaifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TimerA interrupt enable"]
    #[inline(always)]
    pub fn taie(&self) -> TaieR {
        TaieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TimerA clear"]
    #[inline(always)]
    pub fn taclr(&self) -> TaclrR {
        TaclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&self) -> McR {
        McR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TimerA clock source select"]
    #[inline(always)]
    pub fn tassel(&self) -> TasselR {
        TasselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TimerA interrupt flag"]
    #[inline(always)]
    pub fn taifg(&mut self) -> TaifgW<TaxCtlSpec> {
        TaifgW::new(self, 0)
    }
    #[doc = "Bit 1 - TimerA interrupt enable"]
    #[inline(always)]
    pub fn taie(&mut self) -> TaieW<TaxCtlSpec> {
        TaieW::new(self, 1)
    }
    #[doc = "Bit 2 - TimerA clear"]
    #[inline(always)]
    pub fn taclr(&mut self) -> TaclrW<TaxCtlSpec> {
        TaclrW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Mode control"]
    #[inline(always)]
    pub fn mc(&mut self) -> McW<TaxCtlSpec> {
        McW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Input divider"]
    #[inline(always)]
    pub fn id(&mut self) -> IdW<TaxCtlSpec> {
        IdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - TimerA clock source select"]
    #[inline(always)]
    pub fn tassel(&mut self) -> TasselW<TaxCtlSpec> {
        TasselW::new(self, 8)
    }
}
#[doc = "TimerAx Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tax_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaxCtlSpec;
impl crate::RegisterSpec for TaxCtlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tax_ctl::R`](R) reader structure"]
impl crate::Readable for TaxCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`tax_ctl::W`](W) writer structure"]
impl crate::Writable for TaxCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TAxCTL to value 0"]
impl crate::Resettable for TaxCtlSpec {
    const RESET_VALUE: u16 = 0;
}
