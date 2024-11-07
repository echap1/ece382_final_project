#[doc = "Register `CSCLRIFG` writer"]
pub type W = crate::W<CsclrifgSpec>;
#[doc = "Clear LFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrLfxtifgEnumWrite {
    #[doc = "0: No effect"]
    ClrLfxtifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    ClrLfxtifg1 = 1,
}
impl From<ClrLfxtifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrLfxtifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_LFXTIFG` writer - Clear LFXT oscillator fault interrupt flag"]
pub type ClrLfxtifgW<'a, REG> = crate::BitWriter<'a, REG, ClrLfxtifgEnumWrite>;
impl<'a, REG> ClrLfxtifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_lfxtifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrLfxtifgEnumWrite::ClrLfxtifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_lfxtifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrLfxtifgEnumWrite::ClrLfxtifg1)
    }
}
#[doc = "Clear HFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrHfxtifgEnumWrite {
    #[doc = "0: No effect"]
    ClrHfxtifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    ClrHfxtifg1 = 1,
}
impl From<ClrHfxtifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrHfxtifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_HFXTIFG` writer - Clear HFXT oscillator fault interrupt flag"]
pub type ClrHfxtifgW<'a, REG> = crate::BitWriter<'a, REG, ClrHfxtifgEnumWrite>;
impl<'a, REG> ClrHfxtifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_hfxtifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrHfxtifgEnumWrite::ClrHfxtifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxtifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrHfxtifgEnumWrite::ClrHfxtifg1)
    }
}
#[doc = "Clear HFXT2 oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrHfxt2ifgEnumWrite {
    #[doc = "0: No effect"]
    ClrHfxt2ifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    ClrHfxt2ifg1 = 1,
}
impl From<ClrHfxt2ifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrHfxt2ifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_HFXT2IFG` writer - Clear HFXT2 oscillator fault interrupt flag"]
pub type ClrHfxt2ifgW<'a, REG> = crate::BitWriter<'a, REG, ClrHfxt2ifgEnumWrite>;
impl<'a, REG> ClrHfxt2ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_hfxt2ifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrHfxt2ifgEnumWrite::ClrHfxt2ifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxt2ifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrHfxt2ifgEnumWrite::ClrHfxt2ifg1)
    }
}
#[doc = "Clear DCO external resistor open circuit fault interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrDcorOpnifgEnumWrite {
    #[doc = "0: No effect"]
    ClrDcorOpnifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    ClrDcorOpnifg1 = 1,
}
impl From<ClrDcorOpnifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrDcorOpnifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_DCOR_OPNIFG` writer - Clear DCO external resistor open circuit fault interrupt flag."]
pub type ClrDcorOpnifgW<'a, REG> = crate::BitWriter<'a, REG, ClrDcorOpnifgEnumWrite>;
impl<'a, REG> ClrDcorOpnifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_dcor_opnifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrDcorOpnifgEnumWrite::ClrDcorOpnifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_dcor_opnifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrDcorOpnifgEnumWrite::ClrDcorOpnifg1)
    }
}
#[doc = "Start fault counter clear interrupt flag LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrFcntlfifgEnumWrite {
    #[doc = "0: No effect"]
    ClrFcntlfifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    ClrFcntlfifg1 = 1,
}
impl From<ClrFcntlfifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrFcntlfifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_FCNTLFIFG` writer - Start fault counter clear interrupt flag LFXT"]
pub type ClrFcntlfifgW<'a, REG> = crate::BitWriter<'a, REG, ClrFcntlfifgEnumWrite>;
impl<'a, REG> ClrFcntlfifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_fcntlfifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrFcntlfifgEnumWrite::ClrFcntlfifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_fcntlfifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrFcntlfifgEnumWrite::ClrFcntlfifg1)
    }
}
#[doc = "Start fault counter clear interrupt flag HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrFcnthfifgEnumWrite {
    #[doc = "0: No effect"]
    ClrFcnthfifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    ClrFcnthfifg1 = 1,
}
impl From<ClrFcnthfifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrFcnthfifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_FCNTHFIFG` writer - Start fault counter clear interrupt flag HFXT"]
pub type ClrFcnthfifgW<'a, REG> = crate::BitWriter<'a, REG, ClrFcnthfifgEnumWrite>;
impl<'a, REG> ClrFcnthfifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_fcnthfifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrFcnthfifgEnumWrite::ClrFcnthfifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_fcnthfifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrFcnthfifgEnumWrite::ClrFcnthfifg1)
    }
}
#[doc = "Start fault counter clear interrupt flag HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrFcnthf2ifgEnumWrite {
    #[doc = "0: No effect"]
    ClrFcnthf2ifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    ClrFcnthf2ifg1 = 1,
}
impl From<ClrFcnthf2ifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrFcnthf2ifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_FCNTHF2IFG` writer - Start fault counter clear interrupt flag HFXT2"]
pub type ClrFcnthf2ifgW<'a, REG> = crate::BitWriter<'a, REG, ClrFcnthf2ifgEnumWrite>;
impl<'a, REG> ClrFcnthf2ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_fcnthf2ifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrFcnthf2ifgEnumWrite::ClrFcnthf2ifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_fcnthf2ifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrFcnthf2ifgEnumWrite::ClrFcnthf2ifg1)
    }
}
#[doc = "PLL out-of-lock clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrPlloolifgEnumWrite {
    #[doc = "0: No effect"]
    ClrPlloolifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    ClrPlloolifg1 = 1,
}
impl From<ClrPlloolifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrPlloolifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PLLOOLIFG` writer - PLL out-of-lock clear interrupt flag"]
pub type ClrPlloolifgW<'a, REG> = crate::BitWriter<'a, REG, ClrPlloolifgEnumWrite>;
impl<'a, REG> ClrPlloolifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_plloolifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPlloolifgEnumWrite::ClrPlloolifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_plloolifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPlloolifgEnumWrite::ClrPlloolifg1)
    }
}
#[doc = "PLL loss-of-signal clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrPlllosifgEnumWrite {
    #[doc = "0: No effect"]
    ClrPlllosifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    ClrPlllosifg1 = 1,
}
impl From<ClrPlllosifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrPlllosifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PLLLOSIFG` writer - PLL loss-of-signal clear interrupt flag"]
pub type ClrPlllosifgW<'a, REG> = crate::BitWriter<'a, REG, ClrPlllosifgEnumWrite>;
impl<'a, REG> ClrPlllosifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_plllosifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPlllosifgEnumWrite::ClrPlllosifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_plllosifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPlllosifgEnumWrite::ClrPlllosifg1)
    }
}
#[doc = "PLL out-of-range clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrPlloorifgEnumWrite {
    #[doc = "0: No effect"]
    ClrPlloorifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    ClrPlloorifg1 = 1,
}
impl From<ClrPlloorifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrPlloorifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PLLOORIFG` writer - PLL out-of-range clear interrupt flag"]
pub type ClrPlloorifgW<'a, REG> = crate::BitWriter<'a, REG, ClrPlloorifgEnumWrite>;
impl<'a, REG> ClrPlloorifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_plloorifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPlloorifgEnumWrite::ClrPlloorifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_plloorifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrPlloorifgEnumWrite::ClrPlloorifg1)
    }
}
#[doc = "REFCNT period counter clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrCalifgEnumWrite {
    #[doc = "0: No effect"]
    ClrCalifg0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    ClrCalifg1 = 1,
}
impl From<ClrCalifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: ClrCalifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CALIFG` writer - REFCNT period counter clear interrupt flag"]
pub type ClrCalifgW<'a, REG> = crate::BitWriter<'a, REG, ClrCalifgEnumWrite>;
impl<'a, REG> ClrCalifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_califg_0(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCalifgEnumWrite::ClrCalifg0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_califg_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClrCalifgEnumWrite::ClrCalifg1)
    }
}
impl W {
    #[doc = "Bit 0 - Clear LFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn clr_lfxtifg(&mut self) -> ClrLfxtifgW<CsclrifgSpec> {
        ClrLfxtifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear HFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxtifg(&mut self) -> ClrHfxtifgW<CsclrifgSpec> {
        ClrHfxtifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear HFXT2 oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxt2ifg(&mut self) -> ClrHfxt2ifgW<CsclrifgSpec> {
        ClrHfxt2ifgW::new(self, 2)
    }
    #[doc = "Bit 6 - Clear DCO external resistor open circuit fault interrupt flag."]
    #[inline(always)]
    pub fn clr_dcor_opnifg(&mut self) -> ClrDcorOpnifgW<CsclrifgSpec> {
        ClrDcorOpnifgW::new(self, 6)
    }
    #[doc = "Bit 8 - Start fault counter clear interrupt flag LFXT"]
    #[inline(always)]
    pub fn clr_fcntlfifg(&mut self) -> ClrFcntlfifgW<CsclrifgSpec> {
        ClrFcntlfifgW::new(self, 8)
    }
    #[doc = "Bit 9 - Start fault counter clear interrupt flag HFXT"]
    #[inline(always)]
    pub fn clr_fcnthfifg(&mut self) -> ClrFcnthfifgW<CsclrifgSpec> {
        ClrFcnthfifgW::new(self, 9)
    }
    #[doc = "Bit 10 - Start fault counter clear interrupt flag HFXT2"]
    #[inline(always)]
    pub fn clr_fcnthf2ifg(&mut self) -> ClrFcnthf2ifgW<CsclrifgSpec> {
        ClrFcnthf2ifgW::new(self, 10)
    }
    #[doc = "Bit 12 - PLL out-of-lock clear interrupt flag"]
    #[inline(always)]
    pub fn clr_plloolifg(&mut self) -> ClrPlloolifgW<CsclrifgSpec> {
        ClrPlloolifgW::new(self, 12)
    }
    #[doc = "Bit 13 - PLL loss-of-signal clear interrupt flag"]
    #[inline(always)]
    pub fn clr_plllosifg(&mut self) -> ClrPlllosifgW<CsclrifgSpec> {
        ClrPlllosifgW::new(self, 13)
    }
    #[doc = "Bit 14 - PLL out-of-range clear interrupt flag"]
    #[inline(always)]
    pub fn clr_plloorifg(&mut self) -> ClrPlloorifgW<CsclrifgSpec> {
        ClrPlloorifgW::new(self, 14)
    }
    #[doc = "Bit 15 - REFCNT period counter clear interrupt flag"]
    #[inline(always)]
    pub fn clr_califg(&mut self) -> ClrCalifgW<CsclrifgSpec> {
        ClrCalifgW::new(self, 15)
    }
}
#[doc = "Clear Interrupt Flag Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csclrifg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsclrifgSpec;
impl crate::RegisterSpec for CsclrifgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`csclrifg::W`](W) writer structure"]
impl crate::Writable for CsclrifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSCLRIFG to value 0"]
impl crate::Resettable for CsclrifgSpec {
    const RESET_VALUE: u32 = 0;
}
