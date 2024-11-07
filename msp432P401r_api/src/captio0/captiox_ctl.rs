#[doc = "Register `CAPTIOxCTL` reader"]
pub type R = crate::R<CaptioxCtlSpec>;
#[doc = "Register `CAPTIOxCTL` writer"]
pub type W = crate::W<CaptioxCtlSpec>;
#[doc = "Capacitive Touch IO pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Captiopiselx {
    #[doc = "0: Px.0"]
    Captiopiselx0 = 0,
    #[doc = "1: Px.1"]
    Captiopiselx1 = 1,
    #[doc = "2: Px.2"]
    Captiopiselx2 = 2,
    #[doc = "3: Px.3"]
    Captiopiselx3 = 3,
    #[doc = "4: Px.4"]
    Captiopiselx4 = 4,
    #[doc = "5: Px.5"]
    Captiopiselx5 = 5,
    #[doc = "6: Px.6"]
    Captiopiselx6 = 6,
    #[doc = "7: Px.7"]
    Captiopiselx7 = 7,
}
impl From<Captiopiselx> for u8 {
    #[inline(always)]
    fn from(variant: Captiopiselx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Captiopiselx {
    type Ux = u8;
}
impl crate::IsEnum for Captiopiselx {}
#[doc = "Field `CAPTIOPISELx` reader - Capacitive Touch IO pin select"]
pub type CaptiopiselxR = crate::FieldReader<Captiopiselx>;
impl CaptiopiselxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captiopiselx {
        match self.bits {
            0 => Captiopiselx::Captiopiselx0,
            1 => Captiopiselx::Captiopiselx1,
            2 => Captiopiselx::Captiopiselx2,
            3 => Captiopiselx::Captiopiselx3,
            4 => Captiopiselx::Captiopiselx4,
            5 => Captiopiselx::Captiopiselx5,
            6 => Captiopiselx::Captiopiselx6,
            7 => Captiopiselx::Captiopiselx7,
            _ => unreachable!(),
        }
    }
    #[doc = "Px.0"]
    #[inline(always)]
    pub fn is_captiopiselx_0(&self) -> bool {
        *self == Captiopiselx::Captiopiselx0
    }
    #[doc = "Px.1"]
    #[inline(always)]
    pub fn is_captiopiselx_1(&self) -> bool {
        *self == Captiopiselx::Captiopiselx1
    }
    #[doc = "Px.2"]
    #[inline(always)]
    pub fn is_captiopiselx_2(&self) -> bool {
        *self == Captiopiselx::Captiopiselx2
    }
    #[doc = "Px.3"]
    #[inline(always)]
    pub fn is_captiopiselx_3(&self) -> bool {
        *self == Captiopiselx::Captiopiselx3
    }
    #[doc = "Px.4"]
    #[inline(always)]
    pub fn is_captiopiselx_4(&self) -> bool {
        *self == Captiopiselx::Captiopiselx4
    }
    #[doc = "Px.5"]
    #[inline(always)]
    pub fn is_captiopiselx_5(&self) -> bool {
        *self == Captiopiselx::Captiopiselx5
    }
    #[doc = "Px.6"]
    #[inline(always)]
    pub fn is_captiopiselx_6(&self) -> bool {
        *self == Captiopiselx::Captiopiselx6
    }
    #[doc = "Px.7"]
    #[inline(always)]
    pub fn is_captiopiselx_7(&self) -> bool {
        *self == Captiopiselx::Captiopiselx7
    }
}
#[doc = "Field `CAPTIOPISELx` writer - Capacitive Touch IO pin select"]
pub type CaptiopiselxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Captiopiselx, crate::Safe>;
impl<'a, REG> CaptiopiselxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Px.0"]
    #[inline(always)]
    pub fn captiopiselx_0(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopiselx::Captiopiselx0)
    }
    #[doc = "Px.1"]
    #[inline(always)]
    pub fn captiopiselx_1(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopiselx::Captiopiselx1)
    }
    #[doc = "Px.2"]
    #[inline(always)]
    pub fn captiopiselx_2(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopiselx::Captiopiselx2)
    }
    #[doc = "Px.3"]
    #[inline(always)]
    pub fn captiopiselx_3(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopiselx::Captiopiselx3)
    }
    #[doc = "Px.4"]
    #[inline(always)]
    pub fn captiopiselx_4(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopiselx::Captiopiselx4)
    }
    #[doc = "Px.5"]
    #[inline(always)]
    pub fn captiopiselx_5(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopiselx::Captiopiselx5)
    }
    #[doc = "Px.6"]
    #[inline(always)]
    pub fn captiopiselx_6(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopiselx::Captiopiselx6)
    }
    #[doc = "Px.7"]
    #[inline(always)]
    pub fn captiopiselx_7(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopiselx::Captiopiselx7)
    }
}
#[doc = "Capacitive Touch IO port select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Captioposelx {
    #[doc = "0: Px = PJ"]
    Captioposelx0 = 0,
    #[doc = "1: Px = P1"]
    Captioposelx1 = 1,
    #[doc = "2: Px = P2"]
    Captioposelx2 = 2,
    #[doc = "3: Px = P3"]
    Captioposelx3 = 3,
    #[doc = "4: Px = P4"]
    Captioposelx4 = 4,
    #[doc = "5: Px = P5"]
    Captioposelx5 = 5,
    #[doc = "6: Px = P6"]
    Captioposelx6 = 6,
    #[doc = "7: Px = P7"]
    Captioposelx7 = 7,
    #[doc = "8: Px = P8"]
    Captioposelx8 = 8,
    #[doc = "9: Px = P9"]
    Captioposelx9 = 9,
    #[doc = "10: Px = P10"]
    Captioposelx10 = 10,
    #[doc = "11: Px = P11"]
    Captioposelx11 = 11,
    #[doc = "12: Px = P12"]
    Captioposelx12 = 12,
    #[doc = "13: Px = P13"]
    Captioposelx13 = 13,
    #[doc = "14: Px = P14"]
    Captioposelx14 = 14,
    #[doc = "15: Px = P15"]
    Captioposelx15 = 15,
}
impl From<Captioposelx> for u8 {
    #[inline(always)]
    fn from(variant: Captioposelx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Captioposelx {
    type Ux = u8;
}
impl crate::IsEnum for Captioposelx {}
#[doc = "Field `CAPTIOPOSELx` reader - Capacitive Touch IO port select"]
pub type CaptioposelxR = crate::FieldReader<Captioposelx>;
impl CaptioposelxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captioposelx {
        match self.bits {
            0 => Captioposelx::Captioposelx0,
            1 => Captioposelx::Captioposelx1,
            2 => Captioposelx::Captioposelx2,
            3 => Captioposelx::Captioposelx3,
            4 => Captioposelx::Captioposelx4,
            5 => Captioposelx::Captioposelx5,
            6 => Captioposelx::Captioposelx6,
            7 => Captioposelx::Captioposelx7,
            8 => Captioposelx::Captioposelx8,
            9 => Captioposelx::Captioposelx9,
            10 => Captioposelx::Captioposelx10,
            11 => Captioposelx::Captioposelx11,
            12 => Captioposelx::Captioposelx12,
            13 => Captioposelx::Captioposelx13,
            14 => Captioposelx::Captioposelx14,
            15 => Captioposelx::Captioposelx15,
            _ => unreachable!(),
        }
    }
    #[doc = "Px = PJ"]
    #[inline(always)]
    pub fn is_captioposelx_0(&self) -> bool {
        *self == Captioposelx::Captioposelx0
    }
    #[doc = "Px = P1"]
    #[inline(always)]
    pub fn is_captioposelx_1(&self) -> bool {
        *self == Captioposelx::Captioposelx1
    }
    #[doc = "Px = P2"]
    #[inline(always)]
    pub fn is_captioposelx_2(&self) -> bool {
        *self == Captioposelx::Captioposelx2
    }
    #[doc = "Px = P3"]
    #[inline(always)]
    pub fn is_captioposelx_3(&self) -> bool {
        *self == Captioposelx::Captioposelx3
    }
    #[doc = "Px = P4"]
    #[inline(always)]
    pub fn is_captioposelx_4(&self) -> bool {
        *self == Captioposelx::Captioposelx4
    }
    #[doc = "Px = P5"]
    #[inline(always)]
    pub fn is_captioposelx_5(&self) -> bool {
        *self == Captioposelx::Captioposelx5
    }
    #[doc = "Px = P6"]
    #[inline(always)]
    pub fn is_captioposelx_6(&self) -> bool {
        *self == Captioposelx::Captioposelx6
    }
    #[doc = "Px = P7"]
    #[inline(always)]
    pub fn is_captioposelx_7(&self) -> bool {
        *self == Captioposelx::Captioposelx7
    }
    #[doc = "Px = P8"]
    #[inline(always)]
    pub fn is_captioposelx_8(&self) -> bool {
        *self == Captioposelx::Captioposelx8
    }
    #[doc = "Px = P9"]
    #[inline(always)]
    pub fn is_captioposelx_9(&self) -> bool {
        *self == Captioposelx::Captioposelx9
    }
    #[doc = "Px = P10"]
    #[inline(always)]
    pub fn is_captioposelx_10(&self) -> bool {
        *self == Captioposelx::Captioposelx10
    }
    #[doc = "Px = P11"]
    #[inline(always)]
    pub fn is_captioposelx_11(&self) -> bool {
        *self == Captioposelx::Captioposelx11
    }
    #[doc = "Px = P12"]
    #[inline(always)]
    pub fn is_captioposelx_12(&self) -> bool {
        *self == Captioposelx::Captioposelx12
    }
    #[doc = "Px = P13"]
    #[inline(always)]
    pub fn is_captioposelx_13(&self) -> bool {
        *self == Captioposelx::Captioposelx13
    }
    #[doc = "Px = P14"]
    #[inline(always)]
    pub fn is_captioposelx_14(&self) -> bool {
        *self == Captioposelx::Captioposelx14
    }
    #[doc = "Px = P15"]
    #[inline(always)]
    pub fn is_captioposelx_15(&self) -> bool {
        *self == Captioposelx::Captioposelx15
    }
}
#[doc = "Field `CAPTIOPOSELx` writer - Capacitive Touch IO port select"]
pub type CaptioposelxW<'a, REG> = crate::FieldWriter<'a, REG, 4, Captioposelx, crate::Safe>;
impl<'a, REG> CaptioposelxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Px = PJ"]
    #[inline(always)]
    pub fn captioposelx_0(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx0)
    }
    #[doc = "Px = P1"]
    #[inline(always)]
    pub fn captioposelx_1(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx1)
    }
    #[doc = "Px = P2"]
    #[inline(always)]
    pub fn captioposelx_2(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx2)
    }
    #[doc = "Px = P3"]
    #[inline(always)]
    pub fn captioposelx_3(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx3)
    }
    #[doc = "Px = P4"]
    #[inline(always)]
    pub fn captioposelx_4(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx4)
    }
    #[doc = "Px = P5"]
    #[inline(always)]
    pub fn captioposelx_5(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx5)
    }
    #[doc = "Px = P6"]
    #[inline(always)]
    pub fn captioposelx_6(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx6)
    }
    #[doc = "Px = P7"]
    #[inline(always)]
    pub fn captioposelx_7(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx7)
    }
    #[doc = "Px = P8"]
    #[inline(always)]
    pub fn captioposelx_8(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx8)
    }
    #[doc = "Px = P9"]
    #[inline(always)]
    pub fn captioposelx_9(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx9)
    }
    #[doc = "Px = P10"]
    #[inline(always)]
    pub fn captioposelx_10(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx10)
    }
    #[doc = "Px = P11"]
    #[inline(always)]
    pub fn captioposelx_11(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx11)
    }
    #[doc = "Px = P12"]
    #[inline(always)]
    pub fn captioposelx_12(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx12)
    }
    #[doc = "Px = P13"]
    #[inline(always)]
    pub fn captioposelx_13(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx13)
    }
    #[doc = "Px = P14"]
    #[inline(always)]
    pub fn captioposelx_14(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx14)
    }
    #[doc = "Px = P15"]
    #[inline(always)]
    pub fn captioposelx_15(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposelx::Captioposelx15)
    }
}
#[doc = "Capacitive Touch IO enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Captioen {
    #[doc = "0: All Capacitive Touch IOs are disabled. Signal towards timers is 0."]
    Captioen0 = 0,
    #[doc = "1: Selected Capacitive Touch IO is enabled"]
    Captioen1 = 1,
}
impl From<Captioen> for bool {
    #[inline(always)]
    fn from(variant: Captioen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTIOEN` reader - Capacitive Touch IO enable"]
pub type CaptioenR = crate::BitReader<Captioen>;
impl CaptioenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captioen {
        match self.bits {
            false => Captioen::Captioen0,
            true => Captioen::Captioen1,
        }
    }
    #[doc = "All Capacitive Touch IOs are disabled. Signal towards timers is 0."]
    #[inline(always)]
    pub fn is_captioen_0(&self) -> bool {
        *self == Captioen::Captioen0
    }
    #[doc = "Selected Capacitive Touch IO is enabled"]
    #[inline(always)]
    pub fn is_captioen_1(&self) -> bool {
        *self == Captioen::Captioen1
    }
}
#[doc = "Field `CAPTIOEN` writer - Capacitive Touch IO enable"]
pub type CaptioenW<'a, REG> = crate::BitWriter<'a, REG, Captioen>;
impl<'a, REG> CaptioenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All Capacitive Touch IOs are disabled. Signal towards timers is 0."]
    #[inline(always)]
    pub fn captioen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Captioen::Captioen0)
    }
    #[doc = "Selected Capacitive Touch IO is enabled"]
    #[inline(always)]
    pub fn captioen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Captioen::Captioen1)
    }
}
#[doc = "Capacitive Touch IO state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptiostateEnumRead {
    #[doc = "0: Curent state 0 or Capacitive Touch IO is disabled"]
    Captiostate0 = 0,
    #[doc = "1: Current state 1"]
    Captiostate1 = 1,
}
impl From<CaptiostateEnumRead> for bool {
    #[inline(always)]
    fn from(variant: CaptiostateEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTIOSTATE` reader - Capacitive Touch IO state"]
pub type CaptiostateR = crate::BitReader<CaptiostateEnumRead>;
impl CaptiostateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CaptiostateEnumRead {
        match self.bits {
            false => CaptiostateEnumRead::Captiostate0,
            true => CaptiostateEnumRead::Captiostate1,
        }
    }
    #[doc = "Curent state 0 or Capacitive Touch IO is disabled"]
    #[inline(always)]
    pub fn is_captiostate_0(&self) -> bool {
        *self == CaptiostateEnumRead::Captiostate0
    }
    #[doc = "Current state 1"]
    #[inline(always)]
    pub fn is_captiostate_1(&self) -> bool {
        *self == CaptiostateEnumRead::Captiostate1
    }
}
impl R {
    #[doc = "Bits 1:3 - Capacitive Touch IO pin select"]
    #[inline(always)]
    pub fn captiopiselx(&self) -> CaptiopiselxR {
        CaptiopiselxR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - Capacitive Touch IO port select"]
    #[inline(always)]
    pub fn captioposelx(&self) -> CaptioposelxR {
        CaptioposelxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Capacitive Touch IO enable"]
    #[inline(always)]
    pub fn captioen(&self) -> CaptioenR {
        CaptioenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capacitive Touch IO state"]
    #[inline(always)]
    pub fn captiostate(&self) -> CaptiostateR {
        CaptiostateR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:3 - Capacitive Touch IO pin select"]
    #[inline(always)]
    pub fn captiopiselx(&mut self) -> CaptiopiselxW<CaptioxCtlSpec> {
        CaptiopiselxW::new(self, 1)
    }
    #[doc = "Bits 4:7 - Capacitive Touch IO port select"]
    #[inline(always)]
    pub fn captioposelx(&mut self) -> CaptioposelxW<CaptioxCtlSpec> {
        CaptioposelxW::new(self, 4)
    }
    #[doc = "Bit 8 - Capacitive Touch IO enable"]
    #[inline(always)]
    pub fn captioen(&mut self) -> CaptioenW<CaptioxCtlSpec> {
        CaptioenW::new(self, 8)
    }
}
#[doc = "Capacitive Touch IO x Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`captiox_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`captiox_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaptioxCtlSpec;
impl crate::RegisterSpec for CaptioxCtlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`captiox_ctl::R`](R) reader structure"]
impl crate::Readable for CaptioxCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`captiox_ctl::W`](W) writer structure"]
impl crate::Writable for CaptioxCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CAPTIOxCTL to value 0"]
impl crate::Resettable for CaptioxCtlSpec {
    const RESET_VALUE: u16 = 0;
}
