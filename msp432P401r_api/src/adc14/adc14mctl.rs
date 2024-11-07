#[doc = "Register `ADC14MCTL[%s]` reader"]
pub type R = crate::R<Adc14mctlSpec>;
#[doc = "Register `ADC14MCTL[%s]` writer"]
pub type W = crate::W<Adc14mctlSpec>;
#[doc = "Input channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc14inch {
    #[doc = "0: If ADC14DIF = 0: A0; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    Adc14inch0 = 0,
    #[doc = "1: If ADC14DIF = 0: A1; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    Adc14inch1 = 1,
    #[doc = "2: If ADC14DIF = 0: A2; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    Adc14inch2 = 2,
    #[doc = "3: If ADC14DIF = 0: A3; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    Adc14inch3 = 3,
    #[doc = "4: If ADC14DIF = 0: A4; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    Adc14inch4 = 4,
    #[doc = "5: If ADC14DIF = 0: A5; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    Adc14inch5 = 5,
    #[doc = "6: If ADC14DIF = 0: A6; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    Adc14inch6 = 6,
    #[doc = "7: If ADC14DIF = 0: A7; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    Adc14inch7 = 7,
    #[doc = "8: If ADC14DIF = 0: A8; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    Adc14inch8 = 8,
    #[doc = "9: If ADC14DIF = 0: A9; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    Adc14inch9 = 9,
    #[doc = "10: If ADC14DIF = 0: A10; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    Adc14inch10 = 10,
    #[doc = "11: If ADC14DIF = 0: A11; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    Adc14inch11 = 11,
    #[doc = "12: If ADC14DIF = 0: A12; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    Adc14inch12 = 12,
    #[doc = "13: If ADC14DIF = 0: A13; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    Adc14inch13 = 13,
    #[doc = "14: If ADC14DIF = 0: A14; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    Adc14inch14 = 14,
    #[doc = "15: If ADC14DIF = 0: A15; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    Adc14inch15 = 15,
    #[doc = "16: If ADC14DIF = 0: A16; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    Adc14inch16 = 16,
    #[doc = "17: If ADC14DIF = 0: A17; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    Adc14inch17 = 17,
    #[doc = "18: If ADC14DIF = 0: A18; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    Adc14inch18 = 18,
    #[doc = "19: If ADC14DIF = 0: A19; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    Adc14inch19 = 19,
    #[doc = "20: If ADC14DIF = 0: A20; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    Adc14inch20 = 20,
    #[doc = "21: If ADC14DIF = 0: A21; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    Adc14inch21 = 21,
    #[doc = "22: If ADC14DIF = 0: A22; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    Adc14inch22 = 22,
    #[doc = "23: If ADC14DIF = 0: A23; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    Adc14inch23 = 23,
    #[doc = "24: If ADC14DIF = 0: A24; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    Adc14inch24 = 24,
    #[doc = "25: If ADC14DIF = 0: A25; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    Adc14inch25 = 25,
    #[doc = "26: If ADC14DIF = 0: A26; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    Adc14inch26 = 26,
    #[doc = "27: If ADC14DIF = 0: A27; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    Adc14inch27 = 27,
    #[doc = "28: If ADC14DIF = 0: A28; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    Adc14inch28 = 28,
    #[doc = "29: If ADC14DIF = 0: A29; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    Adc14inch29 = 29,
    #[doc = "30: If ADC14DIF = 0: A30; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    Adc14inch30 = 30,
    #[doc = "31: If ADC14DIF = 0: A31; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    Adc14inch31 = 31,
}
impl From<Adc14inch> for u8 {
    #[inline(always)]
    fn from(variant: Adc14inch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14inch {
    type Ux = u8;
}
impl crate::IsEnum for Adc14inch {}
#[doc = "Field `ADC14INCH` reader - Input channel select"]
pub type Adc14inchR = crate::FieldReader<Adc14inch>;
impl Adc14inchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14inch {
        match self.bits {
            0 => Adc14inch::Adc14inch0,
            1 => Adc14inch::Adc14inch1,
            2 => Adc14inch::Adc14inch2,
            3 => Adc14inch::Adc14inch3,
            4 => Adc14inch::Adc14inch4,
            5 => Adc14inch::Adc14inch5,
            6 => Adc14inch::Adc14inch6,
            7 => Adc14inch::Adc14inch7,
            8 => Adc14inch::Adc14inch8,
            9 => Adc14inch::Adc14inch9,
            10 => Adc14inch::Adc14inch10,
            11 => Adc14inch::Adc14inch11,
            12 => Adc14inch::Adc14inch12,
            13 => Adc14inch::Adc14inch13,
            14 => Adc14inch::Adc14inch14,
            15 => Adc14inch::Adc14inch15,
            16 => Adc14inch::Adc14inch16,
            17 => Adc14inch::Adc14inch17,
            18 => Adc14inch::Adc14inch18,
            19 => Adc14inch::Adc14inch19,
            20 => Adc14inch::Adc14inch20,
            21 => Adc14inch::Adc14inch21,
            22 => Adc14inch::Adc14inch22,
            23 => Adc14inch::Adc14inch23,
            24 => Adc14inch::Adc14inch24,
            25 => Adc14inch::Adc14inch25,
            26 => Adc14inch::Adc14inch26,
            27 => Adc14inch::Adc14inch27,
            28 => Adc14inch::Adc14inch28,
            29 => Adc14inch::Adc14inch29,
            30 => Adc14inch::Adc14inch30,
            31 => Adc14inch::Adc14inch31,
            _ => unreachable!(),
        }
    }
    #[doc = "If ADC14DIF = 0: A0; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn is_adc14inch_0(&self) -> bool {
        *self == Adc14inch::Adc14inch0
    }
    #[doc = "If ADC14DIF = 0: A1; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn is_adc14inch_1(&self) -> bool {
        *self == Adc14inch::Adc14inch1
    }
    #[doc = "If ADC14DIF = 0: A2; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn is_adc14inch_2(&self) -> bool {
        *self == Adc14inch::Adc14inch2
    }
    #[doc = "If ADC14DIF = 0: A3; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn is_adc14inch_3(&self) -> bool {
        *self == Adc14inch::Adc14inch3
    }
    #[doc = "If ADC14DIF = 0: A4; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn is_adc14inch_4(&self) -> bool {
        *self == Adc14inch::Adc14inch4
    }
    #[doc = "If ADC14DIF = 0: A5; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn is_adc14inch_5(&self) -> bool {
        *self == Adc14inch::Adc14inch5
    }
    #[doc = "If ADC14DIF = 0: A6; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn is_adc14inch_6(&self) -> bool {
        *self == Adc14inch::Adc14inch6
    }
    #[doc = "If ADC14DIF = 0: A7; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn is_adc14inch_7(&self) -> bool {
        *self == Adc14inch::Adc14inch7
    }
    #[doc = "If ADC14DIF = 0: A8; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn is_adc14inch_8(&self) -> bool {
        *self == Adc14inch::Adc14inch8
    }
    #[doc = "If ADC14DIF = 0: A9; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn is_adc14inch_9(&self) -> bool {
        *self == Adc14inch::Adc14inch9
    }
    #[doc = "If ADC14DIF = 0: A10; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn is_adc14inch_10(&self) -> bool {
        *self == Adc14inch::Adc14inch10
    }
    #[doc = "If ADC14DIF = 0: A11; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn is_adc14inch_11(&self) -> bool {
        *self == Adc14inch::Adc14inch11
    }
    #[doc = "If ADC14DIF = 0: A12; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn is_adc14inch_12(&self) -> bool {
        *self == Adc14inch::Adc14inch12
    }
    #[doc = "If ADC14DIF = 0: A13; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn is_adc14inch_13(&self) -> bool {
        *self == Adc14inch::Adc14inch13
    }
    #[doc = "If ADC14DIF = 0: A14; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn is_adc14inch_14(&self) -> bool {
        *self == Adc14inch::Adc14inch14
    }
    #[doc = "If ADC14DIF = 0: A15; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn is_adc14inch_15(&self) -> bool {
        *self == Adc14inch::Adc14inch15
    }
    #[doc = "If ADC14DIF = 0: A16; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn is_adc14inch_16(&self) -> bool {
        *self == Adc14inch::Adc14inch16
    }
    #[doc = "If ADC14DIF = 0: A17; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn is_adc14inch_17(&self) -> bool {
        *self == Adc14inch::Adc14inch17
    }
    #[doc = "If ADC14DIF = 0: A18; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn is_adc14inch_18(&self) -> bool {
        *self == Adc14inch::Adc14inch18
    }
    #[doc = "If ADC14DIF = 0: A19; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn is_adc14inch_19(&self) -> bool {
        *self == Adc14inch::Adc14inch19
    }
    #[doc = "If ADC14DIF = 0: A20; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn is_adc14inch_20(&self) -> bool {
        *self == Adc14inch::Adc14inch20
    }
    #[doc = "If ADC14DIF = 0: A21; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn is_adc14inch_21(&self) -> bool {
        *self == Adc14inch::Adc14inch21
    }
    #[doc = "If ADC14DIF = 0: A22; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn is_adc14inch_22(&self) -> bool {
        *self == Adc14inch::Adc14inch22
    }
    #[doc = "If ADC14DIF = 0: A23; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn is_adc14inch_23(&self) -> bool {
        *self == Adc14inch::Adc14inch23
    }
    #[doc = "If ADC14DIF = 0: A24; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn is_adc14inch_24(&self) -> bool {
        *self == Adc14inch::Adc14inch24
    }
    #[doc = "If ADC14DIF = 0: A25; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn is_adc14inch_25(&self) -> bool {
        *self == Adc14inch::Adc14inch25
    }
    #[doc = "If ADC14DIF = 0: A26; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    #[inline(always)]
    pub fn is_adc14inch_26(&self) -> bool {
        *self == Adc14inch::Adc14inch26
    }
    #[doc = "If ADC14DIF = 0: A27; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    #[inline(always)]
    pub fn is_adc14inch_27(&self) -> bool {
        *self == Adc14inch::Adc14inch27
    }
    #[doc = "If ADC14DIF = 0: A28; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn is_adc14inch_28(&self) -> bool {
        *self == Adc14inch::Adc14inch28
    }
    #[doc = "If ADC14DIF = 0: A29; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn is_adc14inch_29(&self) -> bool {
        *self == Adc14inch::Adc14inch29
    }
    #[doc = "If ADC14DIF = 0: A30; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn is_adc14inch_30(&self) -> bool {
        *self == Adc14inch::Adc14inch30
    }
    #[doc = "If ADC14DIF = 0: A31; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn is_adc14inch_31(&self) -> bool {
        *self == Adc14inch::Adc14inch31
    }
}
#[doc = "Field `ADC14INCH` writer - Input channel select"]
pub type Adc14inchW<'a, REG> = crate::FieldWriter<'a, REG, 5, Adc14inch, crate::Safe>;
impl<'a, REG> Adc14inchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If ADC14DIF = 0: A0; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn adc14inch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch0)
    }
    #[doc = "If ADC14DIF = 0: A1; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn adc14inch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch1)
    }
    #[doc = "If ADC14DIF = 0: A2; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn adc14inch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch2)
    }
    #[doc = "If ADC14DIF = 0: A3; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn adc14inch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch3)
    }
    #[doc = "If ADC14DIF = 0: A4; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn adc14inch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch4)
    }
    #[doc = "If ADC14DIF = 0: A5; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn adc14inch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch5)
    }
    #[doc = "If ADC14DIF = 0: A6; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn adc14inch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch6)
    }
    #[doc = "If ADC14DIF = 0: A7; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn adc14inch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch7)
    }
    #[doc = "If ADC14DIF = 0: A8; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn adc14inch_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch8)
    }
    #[doc = "If ADC14DIF = 0: A9; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn adc14inch_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch9)
    }
    #[doc = "If ADC14DIF = 0: A10; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn adc14inch_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch10)
    }
    #[doc = "If ADC14DIF = 0: A11; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn adc14inch_11(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch11)
    }
    #[doc = "If ADC14DIF = 0: A12; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn adc14inch_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch12)
    }
    #[doc = "If ADC14DIF = 0: A13; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn adc14inch_13(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch13)
    }
    #[doc = "If ADC14DIF = 0: A14; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn adc14inch_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch14)
    }
    #[doc = "If ADC14DIF = 0: A15; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn adc14inch_15(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch15)
    }
    #[doc = "If ADC14DIF = 0: A16; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn adc14inch_16(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch16)
    }
    #[doc = "If ADC14DIF = 0: A17; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn adc14inch_17(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch17)
    }
    #[doc = "If ADC14DIF = 0: A18; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn adc14inch_18(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch18)
    }
    #[doc = "If ADC14DIF = 0: A19; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn adc14inch_19(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch19)
    }
    #[doc = "If ADC14DIF = 0: A20; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn adc14inch_20(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch20)
    }
    #[doc = "If ADC14DIF = 0: A21; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn adc14inch_21(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch21)
    }
    #[doc = "If ADC14DIF = 0: A22; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn adc14inch_22(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch22)
    }
    #[doc = "If ADC14DIF = 0: A23; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn adc14inch_23(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch23)
    }
    #[doc = "If ADC14DIF = 0: A24; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn adc14inch_24(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch24)
    }
    #[doc = "If ADC14DIF = 0: A25; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn adc14inch_25(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch25)
    }
    #[doc = "If ADC14DIF = 0: A26; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    #[inline(always)]
    pub fn adc14inch_26(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch26)
    }
    #[doc = "If ADC14DIF = 0: A27; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    #[inline(always)]
    pub fn adc14inch_27(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch27)
    }
    #[doc = "If ADC14DIF = 0: A28; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn adc14inch_28(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch28)
    }
    #[doc = "If ADC14DIF = 0: A29; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn adc14inch_29(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch29)
    }
    #[doc = "If ADC14DIF = 0: A30; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn adc14inch_30(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch30)
    }
    #[doc = "If ADC14DIF = 0: A31; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn adc14inch_31(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14inch::Adc14inch31)
    }
}
#[doc = "End of sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14eos {
    #[doc = "0: Not end of sequence"]
    Adc14eos0 = 0,
    #[doc = "1: End of sequence"]
    Adc14eos1 = 1,
}
impl From<Adc14eos> for bool {
    #[inline(always)]
    fn from(variant: Adc14eos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14EOS` reader - End of sequence"]
pub type Adc14eosR = crate::BitReader<Adc14eos>;
impl Adc14eosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14eos {
        match self.bits {
            false => Adc14eos::Adc14eos0,
            true => Adc14eos::Adc14eos1,
        }
    }
    #[doc = "Not end of sequence"]
    #[inline(always)]
    pub fn is_adc14eos_0(&self) -> bool {
        *self == Adc14eos::Adc14eos0
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn is_adc14eos_1(&self) -> bool {
        *self == Adc14eos::Adc14eos1
    }
}
#[doc = "Field `ADC14EOS` writer - End of sequence"]
pub type Adc14eosW<'a, REG> = crate::BitWriter<'a, REG, Adc14eos>;
impl<'a, REG> Adc14eosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not end of sequence"]
    #[inline(always)]
    pub fn adc14eos_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14eos::Adc14eos0)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn adc14eos_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14eos::Adc14eos1)
    }
}
#[doc = "Selects combinations of V(R+) and V(R-) sources\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc14vrsel {
    #[doc = "0: V(R+) = AVCC, V(R-) = AVSS"]
    Adc14vrsel0 = 0,
    #[doc = "1: V(R+) = VREF buffered, V(R-) = AVSS"]
    Adc14vrsel1 = 1,
    #[doc = "14: V(R+) = VeREF+, V(R-) = VeREF-"]
    Adc14vrsel14 = 14,
    #[doc = "15: V(R+) = VeREF+ buffered, V(R-) = VeREF"]
    Adc14vrsel15 = 15,
}
impl From<Adc14vrsel> for u8 {
    #[inline(always)]
    fn from(variant: Adc14vrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14vrsel {
    type Ux = u8;
}
impl crate::IsEnum for Adc14vrsel {}
#[doc = "Field `ADC14VRSEL` reader - Selects combinations of V(R+) and V(R-) sources"]
pub type Adc14vrselR = crate::FieldReader<Adc14vrsel>;
impl Adc14vrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc14vrsel> {
        match self.bits {
            0 => Some(Adc14vrsel::Adc14vrsel0),
            1 => Some(Adc14vrsel::Adc14vrsel1),
            14 => Some(Adc14vrsel::Adc14vrsel14),
            15 => Some(Adc14vrsel::Adc14vrsel15),
            _ => None,
        }
    }
    #[doc = "V(R+) = AVCC, V(R-) = AVSS"]
    #[inline(always)]
    pub fn is_adc14vrsel_0(&self) -> bool {
        *self == Adc14vrsel::Adc14vrsel0
    }
    #[doc = "V(R+) = VREF buffered, V(R-) = AVSS"]
    #[inline(always)]
    pub fn is_adc14vrsel_1(&self) -> bool {
        *self == Adc14vrsel::Adc14vrsel1
    }
    #[doc = "V(R+) = VeREF+, V(R-) = VeREF-"]
    #[inline(always)]
    pub fn is_adc14vrsel_14(&self) -> bool {
        *self == Adc14vrsel::Adc14vrsel14
    }
    #[doc = "V(R+) = VeREF+ buffered, V(R-) = VeREF"]
    #[inline(always)]
    pub fn is_adc14vrsel_15(&self) -> bool {
        *self == Adc14vrsel::Adc14vrsel15
    }
}
#[doc = "Field `ADC14VRSEL` writer - Selects combinations of V(R+) and V(R-) sources"]
pub type Adc14vrselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Adc14vrsel>;
impl<'a, REG> Adc14vrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "V(R+) = AVCC, V(R-) = AVSS"]
    #[inline(always)]
    pub fn adc14vrsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14vrsel::Adc14vrsel0)
    }
    #[doc = "V(R+) = VREF buffered, V(R-) = AVSS"]
    #[inline(always)]
    pub fn adc14vrsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14vrsel::Adc14vrsel1)
    }
    #[doc = "V(R+) = VeREF+, V(R-) = VeREF-"]
    #[inline(always)]
    pub fn adc14vrsel_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14vrsel::Adc14vrsel14)
    }
    #[doc = "V(R+) = VeREF+ buffered, V(R-) = VeREF"]
    #[inline(always)]
    pub fn adc14vrsel_15(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14vrsel::Adc14vrsel15)
    }
}
#[doc = "Differential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14dif {
    #[doc = "0: Single-ended mode enabled"]
    Adc14dif0 = 0,
    #[doc = "1: Differential mode enabled"]
    Adc14dif1 = 1,
}
impl From<Adc14dif> for bool {
    #[inline(always)]
    fn from(variant: Adc14dif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14DIF` reader - Differential mode"]
pub type Adc14difR = crate::BitReader<Adc14dif>;
impl Adc14difR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14dif {
        match self.bits {
            false => Adc14dif::Adc14dif0,
            true => Adc14dif::Adc14dif1,
        }
    }
    #[doc = "Single-ended mode enabled"]
    #[inline(always)]
    pub fn is_adc14dif_0(&self) -> bool {
        *self == Adc14dif::Adc14dif0
    }
    #[doc = "Differential mode enabled"]
    #[inline(always)]
    pub fn is_adc14dif_1(&self) -> bool {
        *self == Adc14dif::Adc14dif1
    }
}
#[doc = "Field `ADC14DIF` writer - Differential mode"]
pub type Adc14difW<'a, REG> = crate::BitWriter<'a, REG, Adc14dif>;
impl<'a, REG> Adc14difW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-ended mode enabled"]
    #[inline(always)]
    pub fn adc14dif_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14dif::Adc14dif0)
    }
    #[doc = "Differential mode enabled"]
    #[inline(always)]
    pub fn adc14dif_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14dif::Adc14dif1)
    }
}
#[doc = "Comparator window enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14winc {
    #[doc = "0: Comparator window disabled"]
    Adc14winc0 = 0,
    #[doc = "1: Comparator window enabled"]
    Adc14winc1 = 1,
}
impl From<Adc14winc> for bool {
    #[inline(always)]
    fn from(variant: Adc14winc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14WINC` reader - Comparator window enable"]
pub type Adc14wincR = crate::BitReader<Adc14winc>;
impl Adc14wincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14winc {
        match self.bits {
            false => Adc14winc::Adc14winc0,
            true => Adc14winc::Adc14winc1,
        }
    }
    #[doc = "Comparator window disabled"]
    #[inline(always)]
    pub fn is_adc14winc_0(&self) -> bool {
        *self == Adc14winc::Adc14winc0
    }
    #[doc = "Comparator window enabled"]
    #[inline(always)]
    pub fn is_adc14winc_1(&self) -> bool {
        *self == Adc14winc::Adc14winc1
    }
}
#[doc = "Field `ADC14WINC` writer - Comparator window enable"]
pub type Adc14wincW<'a, REG> = crate::BitWriter<'a, REG, Adc14winc>;
impl<'a, REG> Adc14wincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator window disabled"]
    #[inline(always)]
    pub fn adc14winc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14winc::Adc14winc0)
    }
    #[doc = "Comparator window enabled"]
    #[inline(always)]
    pub fn adc14winc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14winc::Adc14winc1)
    }
}
#[doc = "Window comparator threshold register selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14wincth {
    #[doc = "0: Use window comparator thresholds 0, ADC14LO0 and ADC14HI0"]
    Adc14wincth0 = 0,
    #[doc = "1: Use window comparator thresholds 1, ADC14LO1 and ADC14HI1"]
    Adc14wincth1 = 1,
}
impl From<Adc14wincth> for bool {
    #[inline(always)]
    fn from(variant: Adc14wincth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14WINCTH` reader - Window comparator threshold register selection"]
pub type Adc14wincthR = crate::BitReader<Adc14wincth>;
impl Adc14wincthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14wincth {
        match self.bits {
            false => Adc14wincth::Adc14wincth0,
            true => Adc14wincth::Adc14wincth1,
        }
    }
    #[doc = "Use window comparator thresholds 0, ADC14LO0 and ADC14HI0"]
    #[inline(always)]
    pub fn is_adc14wincth_0(&self) -> bool {
        *self == Adc14wincth::Adc14wincth0
    }
    #[doc = "Use window comparator thresholds 1, ADC14LO1 and ADC14HI1"]
    #[inline(always)]
    pub fn is_adc14wincth_1(&self) -> bool {
        *self == Adc14wincth::Adc14wincth1
    }
}
#[doc = "Field `ADC14WINCTH` writer - Window comparator threshold register selection"]
pub type Adc14wincthW<'a, REG> = crate::BitWriter<'a, REG, Adc14wincth>;
impl<'a, REG> Adc14wincthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use window comparator thresholds 0, ADC14LO0 and ADC14HI0"]
    #[inline(always)]
    pub fn adc14wincth_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14wincth::Adc14wincth0)
    }
    #[doc = "Use window comparator thresholds 1, ADC14LO1 and ADC14HI1"]
    #[inline(always)]
    pub fn adc14wincth_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14wincth::Adc14wincth1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adc14inch(&self) -> Adc14inchR {
        Adc14inchR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - End of sequence"]
    #[inline(always)]
    pub fn adc14eos(&self) -> Adc14eosR {
        Adc14eosR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Selects combinations of V(R+) and V(R-) sources"]
    #[inline(always)]
    pub fn adc14vrsel(&self) -> Adc14vrselR {
        Adc14vrselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Differential mode"]
    #[inline(always)]
    pub fn adc14dif(&self) -> Adc14difR {
        Adc14difR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparator window enable"]
    #[inline(always)]
    pub fn adc14winc(&self) -> Adc14wincR {
        Adc14wincR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Window comparator threshold register selection"]
    #[inline(always)]
    pub fn adc14wincth(&self) -> Adc14wincthR {
        Adc14wincthR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adc14inch(&mut self) -> Adc14inchW<Adc14mctlSpec> {
        Adc14inchW::new(self, 0)
    }
    #[doc = "Bit 7 - End of sequence"]
    #[inline(always)]
    pub fn adc14eos(&mut self) -> Adc14eosW<Adc14mctlSpec> {
        Adc14eosW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Selects combinations of V(R+) and V(R-) sources"]
    #[inline(always)]
    pub fn adc14vrsel(&mut self) -> Adc14vrselW<Adc14mctlSpec> {
        Adc14vrselW::new(self, 8)
    }
    #[doc = "Bit 13 - Differential mode"]
    #[inline(always)]
    pub fn adc14dif(&mut self) -> Adc14difW<Adc14mctlSpec> {
        Adc14difW::new(self, 13)
    }
    #[doc = "Bit 14 - Comparator window enable"]
    #[inline(always)]
    pub fn adc14winc(&mut self) -> Adc14wincW<Adc14mctlSpec> {
        Adc14wincW::new(self, 14)
    }
    #[doc = "Bit 15 - Window comparator threshold register selection"]
    #[inline(always)]
    pub fn adc14wincth(&mut self) -> Adc14wincthW<Adc14mctlSpec> {
        Adc14wincthW::new(self, 15)
    }
}
#[doc = "Conversion Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14mctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14mctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14mctlSpec;
impl crate::RegisterSpec for Adc14mctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14mctl::R`](R) reader structure"]
impl crate::Readable for Adc14mctlSpec {}
#[doc = "`write(|w| ..)` method takes [`adc14mctl::W`](W) writer structure"]
impl crate::Writable for Adc14mctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14MCTL[%s]
to value 0"]
impl crate::Resettable for Adc14mctlSpec {
    const RESET_VALUE: u32 = 0;
}
