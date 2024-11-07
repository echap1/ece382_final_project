#[doc = "Register `ADC14IV` reader"]
pub type R = crate::R<Adc14ivSpec>;
#[doc = "Register `ADC14IV` writer"]
pub type W = crate::W<Adc14ivSpec>;
#[doc = "ADC14 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Adc14iv {
    #[doc = "0: No interrupt pending"]
    Adc14iv0 = 0,
    #[doc = "2: Interrupt Source: ADC14MEMx overflow; Interrupt Flag: ADC14OVIFG; Interrupt Priority: Highest"]
    Adc14iv2 = 2,
    #[doc = "4: Interrupt Source: Conversion time overflow; Interrupt Flag: ADC14TOVIFG"]
    Adc14iv4 = 4,
    #[doc = "6: Interrupt Source: ADC14 window high interrupt flag; Interrupt Flag: ADC14HIIFG"]
    Adc14iv6 = 6,
    #[doc = "8: Interrupt Source: ADC14 window low interrupt flag; Interrupt Flag: ADC14LOIFG"]
    Adc14iv8 = 8,
    #[doc = "10: Interrupt Source: ADC14 in-window interrupt flag; Interrupt Flag: ADC14INIFG"]
    Adc14iv10 = 10,
    #[doc = "12: Interrupt Source: ADC14MEM0 interrupt flag; Interrupt Flag: ADC14IFG0"]
    Adc14iv12 = 12,
    #[doc = "14: Interrupt Source: ADC14MEM1 interrupt flag; Interrupt Flag: ADC14IFG1"]
    Adc14iv14 = 14,
    #[doc = "16: Interrupt Source: ADC14MEM2 interrupt flag; Interrupt Flag: ADC14IFG2"]
    Adc14iv16 = 16,
    #[doc = "18: Interrupt Source: ADC14MEM3 interrupt flag; Interrupt Flag: ADC14IFG3"]
    Adc14iv18 = 18,
    #[doc = "20: Interrupt Source: ADC14MEM4 interrupt flag; Interrupt Flag: ADC14IFG4"]
    Adc14iv20 = 20,
    #[doc = "22: Interrupt Source: ADC14MEM5 interrupt flag; Interrupt Flag: ADC14IFG5"]
    Adc14iv22 = 22,
    #[doc = "24: Interrupt Source: ADC14MEM6 interrupt flag; Interrupt Flag: ADC14IFG6"]
    Adc14iv24 = 24,
    #[doc = "26: Interrupt Source: ADC14MEM7 interrupt flag; Interrupt Flag: ADC14IFG7"]
    Adc14iv26 = 26,
    #[doc = "28: Interrupt Source: ADC14MEM8 interrupt flag; Interrupt Flag: ADC14IFG8"]
    Adc14iv28 = 28,
    #[doc = "30: Interrupt Source: ADC14MEM9 interrupt flag; Interrupt Flag: ADC14IFG9"]
    Adc14iv30 = 30,
    #[doc = "32: Interrupt Source: ADC14MEM10 interrupt flag; Interrupt Flag: ADC14IFG10"]
    Adc14iv32 = 32,
    #[doc = "34: Interrupt Source: ADC14MEM11 interrupt flag; Interrupt Flag: ADC14IFG11"]
    Adc14iv34 = 34,
    #[doc = "36: Interrupt Source: ADC14MEM12 interrupt flag; Interrupt Flag: ADC14IFG12"]
    Adc14iv36 = 36,
    #[doc = "38: Interrupt Source: ADC14MEM13 interrupt flag; Interrupt Flag: ADC14IFG13"]
    Adc14iv38 = 38,
    #[doc = "40: Interrupt Source: ADC14MEM14 interrupt flag; Interrupt Flag: ADC14IFG14"]
    Adc14iv40 = 40,
    #[doc = "42: Interrupt Source: ADC14MEM15 interrupt flag; Interrupt Flag: ADC14IFG15"]
    Adc14iv42 = 42,
    #[doc = "44: Interrupt Source: ADC14MEM16 interrupt flag; Interrupt Flag: ADC14IFG16"]
    Adc14iv44 = 44,
    #[doc = "46: Interrupt Source: ADC14MEM17 interrupt flag; Interrupt Flag: ADC14IFG17"]
    Adc14iv46 = 46,
    #[doc = "48: Interrupt Source: ADC14MEM18 interrupt flag; Interrupt Flag: ADC14IFG18"]
    Adc14iv48 = 48,
    #[doc = "50: Interrupt Source: ADC14MEM19 interrupt flag; Interrupt Flag: ADC14IFG19"]
    Adc14iv50 = 50,
    #[doc = "52: Interrupt Source: ADC14MEM20 interrupt flag; Interrupt Flag: ADC14IFG20"]
    Adc14iv52 = 52,
    #[doc = "54: Interrupt Source: ADC14MEM22 interrupt flag; Interrupt Flag: ADC14IFG22"]
    Adc14iv54 = 54,
    #[doc = "56: Interrupt Source: ADC14MEM22 interrupt flag; Interrupt Flag: ADC14IFG22"]
    Adc14iv56 = 56,
    #[doc = "58: Interrupt Source: ADC14MEM23 interrupt flag; Interrupt Flag: ADC14IFG23"]
    Adc14iv58 = 58,
    #[doc = "60: Interrupt Source: ADC14MEM24 interrupt flag; Interrupt Flag: ADC14IFG24"]
    Adc14iv60 = 60,
    #[doc = "62: Interrupt Source: ADC14MEM25 interrupt flag; Interrupt Flag: ADC14IFG25"]
    Adc14iv62 = 62,
    #[doc = "64: Interrupt Source: ADC14MEM26 interrupt flag; Interrupt Flag: ADC14IFG26"]
    Adc14iv64 = 64,
    #[doc = "66: Interrupt Source: ADC14MEM27 interrupt flag; Interrupt Flag: ADC14IFG27"]
    Adc14iv66 = 66,
    #[doc = "68: Interrupt Source: ADC14MEM28 interrupt flag; Interrupt Flag: ADC14IFG28"]
    Adc14iv68 = 68,
    #[doc = "70: Interrupt Source: ADC14MEM29 interrupt flag; Interrupt Flag: ADC14IFG29"]
    Adc14iv70 = 70,
    #[doc = "72: Interrupt Source: ADC14MEM30 interrupt flag; Interrupt Flag: ADC14IFG30"]
    Adc14iv72 = 72,
    #[doc = "74: Interrupt Source: ADC14MEM31 interrupt flag; Interrupt Flag: ADC14IFG31"]
    Adc14iv74 = 74,
    #[doc = "76: Interrupt Source: ADC14RDYIFG interrupt flag; Interrupt Flag: ADC14RDYIFG; Interrupt Priority: Lowest"]
    Adc14iv76 = 76,
}
impl From<Adc14iv> for u32 {
    #[inline(always)]
    fn from(variant: Adc14iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14iv {
    type Ux = u32;
}
impl crate::IsEnum for Adc14iv {}
#[doc = "Field `ADC14IV` reader - ADC14 interrupt vector value"]
pub type Adc14ivR = crate::FieldReader<Adc14iv>;
impl Adc14ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc14iv> {
        match self.bits {
            0 => Some(Adc14iv::Adc14iv0),
            2 => Some(Adc14iv::Adc14iv2),
            4 => Some(Adc14iv::Adc14iv4),
            6 => Some(Adc14iv::Adc14iv6),
            8 => Some(Adc14iv::Adc14iv8),
            10 => Some(Adc14iv::Adc14iv10),
            12 => Some(Adc14iv::Adc14iv12),
            14 => Some(Adc14iv::Adc14iv14),
            16 => Some(Adc14iv::Adc14iv16),
            18 => Some(Adc14iv::Adc14iv18),
            20 => Some(Adc14iv::Adc14iv20),
            22 => Some(Adc14iv::Adc14iv22),
            24 => Some(Adc14iv::Adc14iv24),
            26 => Some(Adc14iv::Adc14iv26),
            28 => Some(Adc14iv::Adc14iv28),
            30 => Some(Adc14iv::Adc14iv30),
            32 => Some(Adc14iv::Adc14iv32),
            34 => Some(Adc14iv::Adc14iv34),
            36 => Some(Adc14iv::Adc14iv36),
            38 => Some(Adc14iv::Adc14iv38),
            40 => Some(Adc14iv::Adc14iv40),
            42 => Some(Adc14iv::Adc14iv42),
            44 => Some(Adc14iv::Adc14iv44),
            46 => Some(Adc14iv::Adc14iv46),
            48 => Some(Adc14iv::Adc14iv48),
            50 => Some(Adc14iv::Adc14iv50),
            52 => Some(Adc14iv::Adc14iv52),
            54 => Some(Adc14iv::Adc14iv54),
            56 => Some(Adc14iv::Adc14iv56),
            58 => Some(Adc14iv::Adc14iv58),
            60 => Some(Adc14iv::Adc14iv60),
            62 => Some(Adc14iv::Adc14iv62),
            64 => Some(Adc14iv::Adc14iv64),
            66 => Some(Adc14iv::Adc14iv66),
            68 => Some(Adc14iv::Adc14iv68),
            70 => Some(Adc14iv::Adc14iv70),
            72 => Some(Adc14iv::Adc14iv72),
            74 => Some(Adc14iv::Adc14iv74),
            76 => Some(Adc14iv::Adc14iv76),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adc14iv_0(&self) -> bool {
        *self == Adc14iv::Adc14iv0
    }
    #[doc = "Interrupt Source: ADC14MEMx overflow; Interrupt Flag: ADC14OVIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_adc14iv_2(&self) -> bool {
        *self == Adc14iv::Adc14iv2
    }
    #[doc = "Interrupt Source: Conversion time overflow; Interrupt Flag: ADC14TOVIFG"]
    #[inline(always)]
    pub fn is_adc14iv_4(&self) -> bool {
        *self == Adc14iv::Adc14iv4
    }
    #[doc = "Interrupt Source: ADC14 window high interrupt flag; Interrupt Flag: ADC14HIIFG"]
    #[inline(always)]
    pub fn is_adc14iv_6(&self) -> bool {
        *self == Adc14iv::Adc14iv6
    }
    #[doc = "Interrupt Source: ADC14 window low interrupt flag; Interrupt Flag: ADC14LOIFG"]
    #[inline(always)]
    pub fn is_adc14iv_8(&self) -> bool {
        *self == Adc14iv::Adc14iv8
    }
    #[doc = "Interrupt Source: ADC14 in-window interrupt flag; Interrupt Flag: ADC14INIFG"]
    #[inline(always)]
    pub fn is_adc14iv_10(&self) -> bool {
        *self == Adc14iv::Adc14iv10
    }
    #[doc = "Interrupt Source: ADC14MEM0 interrupt flag; Interrupt Flag: ADC14IFG0"]
    #[inline(always)]
    pub fn is_adc14iv_12(&self) -> bool {
        *self == Adc14iv::Adc14iv12
    }
    #[doc = "Interrupt Source: ADC14MEM1 interrupt flag; Interrupt Flag: ADC14IFG1"]
    #[inline(always)]
    pub fn is_adc14iv_14(&self) -> bool {
        *self == Adc14iv::Adc14iv14
    }
    #[doc = "Interrupt Source: ADC14MEM2 interrupt flag; Interrupt Flag: ADC14IFG2"]
    #[inline(always)]
    pub fn is_adc14iv_16(&self) -> bool {
        *self == Adc14iv::Adc14iv16
    }
    #[doc = "Interrupt Source: ADC14MEM3 interrupt flag; Interrupt Flag: ADC14IFG3"]
    #[inline(always)]
    pub fn is_adc14iv_18(&self) -> bool {
        *self == Adc14iv::Adc14iv18
    }
    #[doc = "Interrupt Source: ADC14MEM4 interrupt flag; Interrupt Flag: ADC14IFG4"]
    #[inline(always)]
    pub fn is_adc14iv_20(&self) -> bool {
        *self == Adc14iv::Adc14iv20
    }
    #[doc = "Interrupt Source: ADC14MEM5 interrupt flag; Interrupt Flag: ADC14IFG5"]
    #[inline(always)]
    pub fn is_adc14iv_22(&self) -> bool {
        *self == Adc14iv::Adc14iv22
    }
    #[doc = "Interrupt Source: ADC14MEM6 interrupt flag; Interrupt Flag: ADC14IFG6"]
    #[inline(always)]
    pub fn is_adc14iv_24(&self) -> bool {
        *self == Adc14iv::Adc14iv24
    }
    #[doc = "Interrupt Source: ADC14MEM7 interrupt flag; Interrupt Flag: ADC14IFG7"]
    #[inline(always)]
    pub fn is_adc14iv_26(&self) -> bool {
        *self == Adc14iv::Adc14iv26
    }
    #[doc = "Interrupt Source: ADC14MEM8 interrupt flag; Interrupt Flag: ADC14IFG8"]
    #[inline(always)]
    pub fn is_adc14iv_28(&self) -> bool {
        *self == Adc14iv::Adc14iv28
    }
    #[doc = "Interrupt Source: ADC14MEM9 interrupt flag; Interrupt Flag: ADC14IFG9"]
    #[inline(always)]
    pub fn is_adc14iv_30(&self) -> bool {
        *self == Adc14iv::Adc14iv30
    }
    #[doc = "Interrupt Source: ADC14MEM10 interrupt flag; Interrupt Flag: ADC14IFG10"]
    #[inline(always)]
    pub fn is_adc14iv_32(&self) -> bool {
        *self == Adc14iv::Adc14iv32
    }
    #[doc = "Interrupt Source: ADC14MEM11 interrupt flag; Interrupt Flag: ADC14IFG11"]
    #[inline(always)]
    pub fn is_adc14iv_34(&self) -> bool {
        *self == Adc14iv::Adc14iv34
    }
    #[doc = "Interrupt Source: ADC14MEM12 interrupt flag; Interrupt Flag: ADC14IFG12"]
    #[inline(always)]
    pub fn is_adc14iv_36(&self) -> bool {
        *self == Adc14iv::Adc14iv36
    }
    #[doc = "Interrupt Source: ADC14MEM13 interrupt flag; Interrupt Flag: ADC14IFG13"]
    #[inline(always)]
    pub fn is_adc14iv_38(&self) -> bool {
        *self == Adc14iv::Adc14iv38
    }
    #[doc = "Interrupt Source: ADC14MEM14 interrupt flag; Interrupt Flag: ADC14IFG14"]
    #[inline(always)]
    pub fn is_adc14iv_40(&self) -> bool {
        *self == Adc14iv::Adc14iv40
    }
    #[doc = "Interrupt Source: ADC14MEM15 interrupt flag; Interrupt Flag: ADC14IFG15"]
    #[inline(always)]
    pub fn is_adc14iv_42(&self) -> bool {
        *self == Adc14iv::Adc14iv42
    }
    #[doc = "Interrupt Source: ADC14MEM16 interrupt flag; Interrupt Flag: ADC14IFG16"]
    #[inline(always)]
    pub fn is_adc14iv_44(&self) -> bool {
        *self == Adc14iv::Adc14iv44
    }
    #[doc = "Interrupt Source: ADC14MEM17 interrupt flag; Interrupt Flag: ADC14IFG17"]
    #[inline(always)]
    pub fn is_adc14iv_46(&self) -> bool {
        *self == Adc14iv::Adc14iv46
    }
    #[doc = "Interrupt Source: ADC14MEM18 interrupt flag; Interrupt Flag: ADC14IFG18"]
    #[inline(always)]
    pub fn is_adc14iv_48(&self) -> bool {
        *self == Adc14iv::Adc14iv48
    }
    #[doc = "Interrupt Source: ADC14MEM19 interrupt flag; Interrupt Flag: ADC14IFG19"]
    #[inline(always)]
    pub fn is_adc14iv_50(&self) -> bool {
        *self == Adc14iv::Adc14iv50
    }
    #[doc = "Interrupt Source: ADC14MEM20 interrupt flag; Interrupt Flag: ADC14IFG20"]
    #[inline(always)]
    pub fn is_adc14iv_52(&self) -> bool {
        *self == Adc14iv::Adc14iv52
    }
    #[doc = "Interrupt Source: ADC14MEM22 interrupt flag; Interrupt Flag: ADC14IFG22"]
    #[inline(always)]
    pub fn is_adc14iv_54(&self) -> bool {
        *self == Adc14iv::Adc14iv54
    }
    #[doc = "Interrupt Source: ADC14MEM22 interrupt flag; Interrupt Flag: ADC14IFG22"]
    #[inline(always)]
    pub fn is_adc14iv_56(&self) -> bool {
        *self == Adc14iv::Adc14iv56
    }
    #[doc = "Interrupt Source: ADC14MEM23 interrupt flag; Interrupt Flag: ADC14IFG23"]
    #[inline(always)]
    pub fn is_adc14iv_58(&self) -> bool {
        *self == Adc14iv::Adc14iv58
    }
    #[doc = "Interrupt Source: ADC14MEM24 interrupt flag; Interrupt Flag: ADC14IFG24"]
    #[inline(always)]
    pub fn is_adc14iv_60(&self) -> bool {
        *self == Adc14iv::Adc14iv60
    }
    #[doc = "Interrupt Source: ADC14MEM25 interrupt flag; Interrupt Flag: ADC14IFG25"]
    #[inline(always)]
    pub fn is_adc14iv_62(&self) -> bool {
        *self == Adc14iv::Adc14iv62
    }
    #[doc = "Interrupt Source: ADC14MEM26 interrupt flag; Interrupt Flag: ADC14IFG26"]
    #[inline(always)]
    pub fn is_adc14iv_64(&self) -> bool {
        *self == Adc14iv::Adc14iv64
    }
    #[doc = "Interrupt Source: ADC14MEM27 interrupt flag; Interrupt Flag: ADC14IFG27"]
    #[inline(always)]
    pub fn is_adc14iv_66(&self) -> bool {
        *self == Adc14iv::Adc14iv66
    }
    #[doc = "Interrupt Source: ADC14MEM28 interrupt flag; Interrupt Flag: ADC14IFG28"]
    #[inline(always)]
    pub fn is_adc14iv_68(&self) -> bool {
        *self == Adc14iv::Adc14iv68
    }
    #[doc = "Interrupt Source: ADC14MEM29 interrupt flag; Interrupt Flag: ADC14IFG29"]
    #[inline(always)]
    pub fn is_adc14iv_70(&self) -> bool {
        *self == Adc14iv::Adc14iv70
    }
    #[doc = "Interrupt Source: ADC14MEM30 interrupt flag; Interrupt Flag: ADC14IFG30"]
    #[inline(always)]
    pub fn is_adc14iv_72(&self) -> bool {
        *self == Adc14iv::Adc14iv72
    }
    #[doc = "Interrupt Source: ADC14MEM31 interrupt flag; Interrupt Flag: ADC14IFG31"]
    #[inline(always)]
    pub fn is_adc14iv_74(&self) -> bool {
        *self == Adc14iv::Adc14iv74
    }
    #[doc = "Interrupt Source: ADC14RDYIFG interrupt flag; Interrupt Flag: ADC14RDYIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_adc14iv_76(&self) -> bool {
        *self == Adc14iv::Adc14iv76
    }
}
#[doc = "Field `ADC14IV` writer - ADC14 interrupt vector value"]
pub type Adc14ivW<'a, REG> = crate::FieldWriter<'a, REG, 32, Adc14iv>;
impl<'a, REG> Adc14ivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adc14iv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv0)
    }
    #[doc = "Interrupt Source: ADC14MEMx overflow; Interrupt Flag: ADC14OVIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn adc14iv_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv2)
    }
    #[doc = "Interrupt Source: Conversion time overflow; Interrupt Flag: ADC14TOVIFG"]
    #[inline(always)]
    pub fn adc14iv_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv4)
    }
    #[doc = "Interrupt Source: ADC14 window high interrupt flag; Interrupt Flag: ADC14HIIFG"]
    #[inline(always)]
    pub fn adc14iv_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv6)
    }
    #[doc = "Interrupt Source: ADC14 window low interrupt flag; Interrupt Flag: ADC14LOIFG"]
    #[inline(always)]
    pub fn adc14iv_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv8)
    }
    #[doc = "Interrupt Source: ADC14 in-window interrupt flag; Interrupt Flag: ADC14INIFG"]
    #[inline(always)]
    pub fn adc14iv_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv10)
    }
    #[doc = "Interrupt Source: ADC14MEM0 interrupt flag; Interrupt Flag: ADC14IFG0"]
    #[inline(always)]
    pub fn adc14iv_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv12)
    }
    #[doc = "Interrupt Source: ADC14MEM1 interrupt flag; Interrupt Flag: ADC14IFG1"]
    #[inline(always)]
    pub fn adc14iv_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv14)
    }
    #[doc = "Interrupt Source: ADC14MEM2 interrupt flag; Interrupt Flag: ADC14IFG2"]
    #[inline(always)]
    pub fn adc14iv_16(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv16)
    }
    #[doc = "Interrupt Source: ADC14MEM3 interrupt flag; Interrupt Flag: ADC14IFG3"]
    #[inline(always)]
    pub fn adc14iv_18(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv18)
    }
    #[doc = "Interrupt Source: ADC14MEM4 interrupt flag; Interrupt Flag: ADC14IFG4"]
    #[inline(always)]
    pub fn adc14iv_20(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv20)
    }
    #[doc = "Interrupt Source: ADC14MEM5 interrupt flag; Interrupt Flag: ADC14IFG5"]
    #[inline(always)]
    pub fn adc14iv_22(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv22)
    }
    #[doc = "Interrupt Source: ADC14MEM6 interrupt flag; Interrupt Flag: ADC14IFG6"]
    #[inline(always)]
    pub fn adc14iv_24(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv24)
    }
    #[doc = "Interrupt Source: ADC14MEM7 interrupt flag; Interrupt Flag: ADC14IFG7"]
    #[inline(always)]
    pub fn adc14iv_26(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv26)
    }
    #[doc = "Interrupt Source: ADC14MEM8 interrupt flag; Interrupt Flag: ADC14IFG8"]
    #[inline(always)]
    pub fn adc14iv_28(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv28)
    }
    #[doc = "Interrupt Source: ADC14MEM9 interrupt flag; Interrupt Flag: ADC14IFG9"]
    #[inline(always)]
    pub fn adc14iv_30(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv30)
    }
    #[doc = "Interrupt Source: ADC14MEM10 interrupt flag; Interrupt Flag: ADC14IFG10"]
    #[inline(always)]
    pub fn adc14iv_32(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv32)
    }
    #[doc = "Interrupt Source: ADC14MEM11 interrupt flag; Interrupt Flag: ADC14IFG11"]
    #[inline(always)]
    pub fn adc14iv_34(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv34)
    }
    #[doc = "Interrupt Source: ADC14MEM12 interrupt flag; Interrupt Flag: ADC14IFG12"]
    #[inline(always)]
    pub fn adc14iv_36(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv36)
    }
    #[doc = "Interrupt Source: ADC14MEM13 interrupt flag; Interrupt Flag: ADC14IFG13"]
    #[inline(always)]
    pub fn adc14iv_38(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv38)
    }
    #[doc = "Interrupt Source: ADC14MEM14 interrupt flag; Interrupt Flag: ADC14IFG14"]
    #[inline(always)]
    pub fn adc14iv_40(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv40)
    }
    #[doc = "Interrupt Source: ADC14MEM15 interrupt flag; Interrupt Flag: ADC14IFG15"]
    #[inline(always)]
    pub fn adc14iv_42(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv42)
    }
    #[doc = "Interrupt Source: ADC14MEM16 interrupt flag; Interrupt Flag: ADC14IFG16"]
    #[inline(always)]
    pub fn adc14iv_44(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv44)
    }
    #[doc = "Interrupt Source: ADC14MEM17 interrupt flag; Interrupt Flag: ADC14IFG17"]
    #[inline(always)]
    pub fn adc14iv_46(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv46)
    }
    #[doc = "Interrupt Source: ADC14MEM18 interrupt flag; Interrupt Flag: ADC14IFG18"]
    #[inline(always)]
    pub fn adc14iv_48(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv48)
    }
    #[doc = "Interrupt Source: ADC14MEM19 interrupt flag; Interrupt Flag: ADC14IFG19"]
    #[inline(always)]
    pub fn adc14iv_50(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv50)
    }
    #[doc = "Interrupt Source: ADC14MEM20 interrupt flag; Interrupt Flag: ADC14IFG20"]
    #[inline(always)]
    pub fn adc14iv_52(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv52)
    }
    #[doc = "Interrupt Source: ADC14MEM22 interrupt flag; Interrupt Flag: ADC14IFG22"]
    #[inline(always)]
    pub fn adc14iv_54(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv54)
    }
    #[doc = "Interrupt Source: ADC14MEM22 interrupt flag; Interrupt Flag: ADC14IFG22"]
    #[inline(always)]
    pub fn adc14iv_56(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv56)
    }
    #[doc = "Interrupt Source: ADC14MEM23 interrupt flag; Interrupt Flag: ADC14IFG23"]
    #[inline(always)]
    pub fn adc14iv_58(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv58)
    }
    #[doc = "Interrupt Source: ADC14MEM24 interrupt flag; Interrupt Flag: ADC14IFG24"]
    #[inline(always)]
    pub fn adc14iv_60(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv60)
    }
    #[doc = "Interrupt Source: ADC14MEM25 interrupt flag; Interrupt Flag: ADC14IFG25"]
    #[inline(always)]
    pub fn adc14iv_62(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv62)
    }
    #[doc = "Interrupt Source: ADC14MEM26 interrupt flag; Interrupt Flag: ADC14IFG26"]
    #[inline(always)]
    pub fn adc14iv_64(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv64)
    }
    #[doc = "Interrupt Source: ADC14MEM27 interrupt flag; Interrupt Flag: ADC14IFG27"]
    #[inline(always)]
    pub fn adc14iv_66(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv66)
    }
    #[doc = "Interrupt Source: ADC14MEM28 interrupt flag; Interrupt Flag: ADC14IFG28"]
    #[inline(always)]
    pub fn adc14iv_68(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv68)
    }
    #[doc = "Interrupt Source: ADC14MEM29 interrupt flag; Interrupt Flag: ADC14IFG29"]
    #[inline(always)]
    pub fn adc14iv_70(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv70)
    }
    #[doc = "Interrupt Source: ADC14MEM30 interrupt flag; Interrupt Flag: ADC14IFG30"]
    #[inline(always)]
    pub fn adc14iv_72(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv72)
    }
    #[doc = "Interrupt Source: ADC14MEM31 interrupt flag; Interrupt Flag: ADC14IFG31"]
    #[inline(always)]
    pub fn adc14iv_74(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv74)
    }
    #[doc = "Interrupt Source: ADC14RDYIFG interrupt flag; Interrupt Flag: ADC14RDYIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn adc14iv_76(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14iv::Adc14iv76)
    }
}
impl R {
    #[doc = "Bits 0:31 - ADC14 interrupt vector value"]
    #[inline(always)]
    pub fn adc14iv(&self) -> Adc14ivR {
        Adc14ivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADC14 interrupt vector value"]
    #[inline(always)]
    pub fn adc14iv(&mut self) -> Adc14ivW<Adc14ivSpec> {
        Adc14ivW::new(self, 0)
    }
}
#[doc = "Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14ivSpec;
impl crate::RegisterSpec for Adc14ivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14iv::R`](R) reader structure"]
impl crate::Readable for Adc14ivSpec {}
#[doc = "`write(|w| ..)` method takes [`adc14iv::W`](W) writer structure"]
impl crate::Writable for Adc14ivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14IV to value 0"]
impl crate::Resettable for Adc14ivSpec {
    const RESET_VALUE: u32 = 0;
}
