#[doc = "Register `CSSETIFG` writer"]
pub type W = crate::W<CssetifgSpec>;
#[doc = "Set LFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetLfxtifgEnumWrite {
    #[doc = "0: No effect"]
    SetLfxtifg0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SetLfxtifg1 = 1,
}
impl From<SetLfxtifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: SetLfxtifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_LFXTIFG` writer - Set LFXT oscillator fault interrupt flag"]
pub type SetLfxtifgW<'a, REG> = crate::BitWriter<'a, REG, SetLfxtifgEnumWrite>;
impl<'a, REG> SetLfxtifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_lfxtifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(SetLfxtifgEnumWrite::SetLfxtifg0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_lfxtifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(SetLfxtifgEnumWrite::SetLfxtifg1)
    }
}
#[doc = "Set HFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetHfxtifgEnumWrite {
    #[doc = "0: No effect"]
    SetHfxtifg0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SetHfxtifg1 = 1,
}
impl From<SetHfxtifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: SetHfxtifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_HFXTIFG` writer - Set HFXT oscillator fault interrupt flag"]
pub type SetHfxtifgW<'a, REG> = crate::BitWriter<'a, REG, SetHfxtifgEnumWrite>;
impl<'a, REG> SetHfxtifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_hfxtifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(SetHfxtifgEnumWrite::SetHfxtifg0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_hfxtifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(SetHfxtifgEnumWrite::SetHfxtifg1)
    }
}
#[doc = "Set HFXT2 oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetHfxt2ifgEnumWrite {
    #[doc = "0: No effect"]
    SetHfxt2ifg0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SetHfxt2ifg1 = 1,
}
impl From<SetHfxt2ifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: SetHfxt2ifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_HFXT2IFG` writer - Set HFXT2 oscillator fault interrupt flag"]
pub type SetHfxt2ifgW<'a, REG> = crate::BitWriter<'a, REG, SetHfxt2ifgEnumWrite>;
impl<'a, REG> SetHfxt2ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_hfxt2ifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(SetHfxt2ifgEnumWrite::SetHfxt2ifg0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_hfxt2ifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(SetHfxt2ifgEnumWrite::SetHfxt2ifg1)
    }
}
#[doc = "Set DCO external resistor open circuit fault interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetDcorOpnifgEnumWrite {
    #[doc = "0: No effect"]
    SetDcorOpnifg0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SetDcorOpnifg1 = 1,
}
impl From<SetDcorOpnifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: SetDcorOpnifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_DCOR_OPNIFG` writer - Set DCO external resistor open circuit fault interrupt flag."]
pub type SetDcorOpnifgW<'a, REG> = crate::BitWriter<'a, REG, SetDcorOpnifgEnumWrite>;
impl<'a, REG> SetDcorOpnifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_dcor_opnifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(SetDcorOpnifgEnumWrite::SetDcorOpnifg0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_dcor_opnifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(SetDcorOpnifgEnumWrite::SetDcorOpnifg1)
    }
}
#[doc = "Start fault counter set interrupt flag LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetFcntlfifgEnumWrite {
    #[doc = "0: No effect"]
    SetFcntlfifg0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SetFcntlfifg1 = 1,
}
impl From<SetFcntlfifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: SetFcntlfifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_FCNTLFIFG` writer - Start fault counter set interrupt flag LFXT"]
pub type SetFcntlfifgW<'a, REG> = crate::BitWriter<'a, REG, SetFcntlfifgEnumWrite>;
impl<'a, REG> SetFcntlfifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_fcntlfifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(SetFcntlfifgEnumWrite::SetFcntlfifg0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_fcntlfifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(SetFcntlfifgEnumWrite::SetFcntlfifg1)
    }
}
#[doc = "Start fault counter set interrupt flag HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetFcnthfifgEnumWrite {
    #[doc = "0: No effect"]
    SetFcnthfifg0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SetFcnthfifg1 = 1,
}
impl From<SetFcnthfifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: SetFcnthfifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_FCNTHFIFG` writer - Start fault counter set interrupt flag HFXT"]
pub type SetFcnthfifgW<'a, REG> = crate::BitWriter<'a, REG, SetFcnthfifgEnumWrite>;
impl<'a, REG> SetFcnthfifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_fcnthfifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(SetFcnthfifgEnumWrite::SetFcnthfifg0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_fcnthfifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(SetFcnthfifgEnumWrite::SetFcnthfifg1)
    }
}
#[doc = "Start fault counter set interrupt flag HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetFcnthf2ifgEnumWrite {
    #[doc = "0: No effect"]
    SetFcnthf2ifg0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SetFcnthf2ifg1 = 1,
}
impl From<SetFcnthf2ifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: SetFcnthf2ifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_FCNTHF2IFG` writer - Start fault counter set interrupt flag HFXT2"]
pub type SetFcnthf2ifgW<'a, REG> = crate::BitWriter<'a, REG, SetFcnthf2ifgEnumWrite>;
impl<'a, REG> SetFcnthf2ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_fcnthf2ifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(SetFcnthf2ifgEnumWrite::SetFcnthf2ifg0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_fcnthf2ifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(SetFcnthf2ifgEnumWrite::SetFcnthf2ifg1)
    }
}
#[doc = "PLL out-of-lock set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetPlloolifgEnumWrite {
    #[doc = "0: No effect"]
    SetPlloolifg0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SetPlloolifg1 = 1,
}
impl From<SetPlloolifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: SetPlloolifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_PLLOOLIFG` writer - PLL out-of-lock set interrupt flag"]
pub type SetPlloolifgW<'a, REG> = crate::BitWriter<'a, REG, SetPlloolifgEnumWrite>;
impl<'a, REG> SetPlloolifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_plloolifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(SetPlloolifgEnumWrite::SetPlloolifg0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_plloolifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(SetPlloolifgEnumWrite::SetPlloolifg1)
    }
}
#[doc = "PLL loss-of-signal set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetPlllosifgEnumWrite {
    #[doc = "0: No effect"]
    SetPlllosifg0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SetPlllosifg1 = 1,
}
impl From<SetPlllosifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: SetPlllosifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_PLLLOSIFG` writer - PLL loss-of-signal set interrupt flag"]
pub type SetPlllosifgW<'a, REG> = crate::BitWriter<'a, REG, SetPlllosifgEnumWrite>;
impl<'a, REG> SetPlllosifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_plllosifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(SetPlllosifgEnumWrite::SetPlllosifg0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_plllosifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(SetPlllosifgEnumWrite::SetPlllosifg1)
    }
}
#[doc = "PLL out-of-range set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetPlloorifgEnumWrite {
    #[doc = "0: No effect"]
    SetPlloorifg0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SetPlloorifg1 = 1,
}
impl From<SetPlloorifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: SetPlloorifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_PLLOORIFG` writer - PLL out-of-range set interrupt flag"]
pub type SetPlloorifgW<'a, REG> = crate::BitWriter<'a, REG, SetPlloorifgEnumWrite>;
impl<'a, REG> SetPlloorifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_plloorifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(SetPlloorifgEnumWrite::SetPlloorifg0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_plloorifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(SetPlloorifgEnumWrite::SetPlloorifg1)
    }
}
#[doc = "REFCNT period counter set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetCalifgEnumWrite {
    #[doc = "0: No effect"]
    SetCalifg0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SetCalifg1 = 1,
}
impl From<SetCalifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: SetCalifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_CALIFG` writer - REFCNT period counter set interrupt flag"]
pub type SetCalifgW<'a, REG> = crate::BitWriter<'a, REG, SetCalifgEnumWrite>;
impl<'a, REG> SetCalifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_califg_0(self) -> &'a mut crate::W<REG> {
        self.variant(SetCalifgEnumWrite::SetCalifg0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_califg_1(self) -> &'a mut crate::W<REG> {
        self.variant(SetCalifgEnumWrite::SetCalifg1)
    }
}
impl W {
    #[doc = "Bit 0 - Set LFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn set_lfxtifg(&mut self) -> SetLfxtifgW<CssetifgSpec> {
        SetLfxtifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Set HFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn set_hfxtifg(&mut self) -> SetHfxtifgW<CssetifgSpec> {
        SetHfxtifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Set HFXT2 oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn set_hfxt2ifg(&mut self) -> SetHfxt2ifgW<CssetifgSpec> {
        SetHfxt2ifgW::new(self, 2)
    }
    #[doc = "Bit 6 - Set DCO external resistor open circuit fault interrupt flag."]
    #[inline(always)]
    pub fn set_dcor_opnifg(&mut self) -> SetDcorOpnifgW<CssetifgSpec> {
        SetDcorOpnifgW::new(self, 6)
    }
    #[doc = "Bit 8 - Start fault counter set interrupt flag LFXT"]
    #[inline(always)]
    pub fn set_fcntlfifg(&mut self) -> SetFcntlfifgW<CssetifgSpec> {
        SetFcntlfifgW::new(self, 8)
    }
    #[doc = "Bit 9 - Start fault counter set interrupt flag HFXT"]
    #[inline(always)]
    pub fn set_fcnthfifg(&mut self) -> SetFcnthfifgW<CssetifgSpec> {
        SetFcnthfifgW::new(self, 9)
    }
    #[doc = "Bit 10 - Start fault counter set interrupt flag HFXT2"]
    #[inline(always)]
    pub fn set_fcnthf2ifg(&mut self) -> SetFcnthf2ifgW<CssetifgSpec> {
        SetFcnthf2ifgW::new(self, 10)
    }
    #[doc = "Bit 12 - PLL out-of-lock set interrupt flag"]
    #[inline(always)]
    pub fn set_plloolifg(&mut self) -> SetPlloolifgW<CssetifgSpec> {
        SetPlloolifgW::new(self, 12)
    }
    #[doc = "Bit 13 - PLL loss-of-signal set interrupt flag"]
    #[inline(always)]
    pub fn set_plllosifg(&mut self) -> SetPlllosifgW<CssetifgSpec> {
        SetPlllosifgW::new(self, 13)
    }
    #[doc = "Bit 14 - PLL out-of-range set interrupt flag"]
    #[inline(always)]
    pub fn set_plloorifg(&mut self) -> SetPlloorifgW<CssetifgSpec> {
        SetPlloorifgW::new(self, 14)
    }
    #[doc = "Bit 15 - REFCNT period counter set interrupt flag"]
    #[inline(always)]
    pub fn set_califg(&mut self) -> SetCalifgW<CssetifgSpec> {
        SetCalifgW::new(self, 15)
    }
}
#[doc = "Set Interrupt Flag Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cssetifg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CssetifgSpec;
impl crate::RegisterSpec for CssetifgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cssetifg::W`](W) writer structure"]
impl crate::Writable for CssetifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSSETIFG to value 0"]
impl crate::Resettable for CssetifgSpec {
    const RESET_VALUE: u32 = 0;
}
