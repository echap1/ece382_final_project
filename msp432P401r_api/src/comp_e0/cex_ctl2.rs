#[doc = "Register `CExCTL2` reader"]
pub type R = crate::R<CexCtl2Spec>;
#[doc = "Register `CExCTL2` writer"]
pub type W = crate::W<CexCtl2Spec>;
#[doc = "Reference resistor tap 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ceref0 {
    #[doc = "0: Reference resistor tap for setting 0."]
    Ceref0_0 = 0,
    #[doc = "1: Reference resistor tap for setting 1."]
    Ceref0_1 = 1,
    #[doc = "2: Reference resistor tap for setting 2."]
    Ceref0_2 = 2,
    #[doc = "3: Reference resistor tap for setting 3."]
    Ceref0_3 = 3,
    #[doc = "4: Reference resistor tap for setting 4."]
    Ceref0_4 = 4,
    #[doc = "5: Reference resistor tap for setting 5."]
    Ceref0_5 = 5,
    #[doc = "6: Reference resistor tap for setting 6."]
    Ceref0_6 = 6,
    #[doc = "7: Reference resistor tap for setting 7."]
    Ceref0_7 = 7,
    #[doc = "8: Reference resistor tap for setting 8."]
    Ceref0_8 = 8,
    #[doc = "9: Reference resistor tap for setting 9."]
    Ceref0_9 = 9,
    #[doc = "10: Reference resistor tap for setting 10."]
    Ceref0_10 = 10,
    #[doc = "11: Reference resistor tap for setting 11."]
    Ceref0_11 = 11,
    #[doc = "12: Reference resistor tap for setting 12."]
    Ceref0_12 = 12,
    #[doc = "13: Reference resistor tap for setting 13."]
    Ceref0_13 = 13,
    #[doc = "14: Reference resistor tap for setting 14."]
    Ceref0_14 = 14,
    #[doc = "15: Reference resistor tap for setting 15."]
    Ceref0_15 = 15,
    #[doc = "16: Reference resistor tap for setting 16."]
    Ceref0_16 = 16,
    #[doc = "17: Reference resistor tap for setting 17."]
    Ceref0_17 = 17,
    #[doc = "18: Reference resistor tap for setting 18."]
    Ceref0_18 = 18,
    #[doc = "19: Reference resistor tap for setting 19."]
    Ceref0_19 = 19,
    #[doc = "20: Reference resistor tap for setting 20."]
    Ceref0_20 = 20,
    #[doc = "21: Reference resistor tap for setting 21."]
    Ceref0_21 = 21,
    #[doc = "22: Reference resistor tap for setting 22."]
    Ceref0_22 = 22,
    #[doc = "23: Reference resistor tap for setting 23."]
    Ceref0_23 = 23,
    #[doc = "24: Reference resistor tap for setting 24."]
    Ceref0_24 = 24,
    #[doc = "25: Reference resistor tap for setting 25."]
    Ceref0_25 = 25,
    #[doc = "26: Reference resistor tap for setting 26."]
    Ceref0_26 = 26,
    #[doc = "27: Reference resistor tap for setting 27."]
    Ceref0_27 = 27,
    #[doc = "28: Reference resistor tap for setting 28."]
    Ceref0_28 = 28,
    #[doc = "29: Reference resistor tap for setting 29."]
    Ceref0_29 = 29,
    #[doc = "30: Reference resistor tap for setting 30."]
    Ceref0_30 = 30,
    #[doc = "31: Reference resistor tap for setting 31."]
    Ceref0_31 = 31,
}
impl From<Ceref0> for u8 {
    #[inline(always)]
    fn from(variant: Ceref0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ceref0 {
    type Ux = u8;
}
impl crate::IsEnum for Ceref0 {}
#[doc = "Field `CEREF0` reader - Reference resistor tap 0"]
pub type Ceref0R = crate::FieldReader<Ceref0>;
impl Ceref0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceref0 {
        match self.bits {
            0 => Ceref0::Ceref0_0,
            1 => Ceref0::Ceref0_1,
            2 => Ceref0::Ceref0_2,
            3 => Ceref0::Ceref0_3,
            4 => Ceref0::Ceref0_4,
            5 => Ceref0::Ceref0_5,
            6 => Ceref0::Ceref0_6,
            7 => Ceref0::Ceref0_7,
            8 => Ceref0::Ceref0_8,
            9 => Ceref0::Ceref0_9,
            10 => Ceref0::Ceref0_10,
            11 => Ceref0::Ceref0_11,
            12 => Ceref0::Ceref0_12,
            13 => Ceref0::Ceref0_13,
            14 => Ceref0::Ceref0_14,
            15 => Ceref0::Ceref0_15,
            16 => Ceref0::Ceref0_16,
            17 => Ceref0::Ceref0_17,
            18 => Ceref0::Ceref0_18,
            19 => Ceref0::Ceref0_19,
            20 => Ceref0::Ceref0_20,
            21 => Ceref0::Ceref0_21,
            22 => Ceref0::Ceref0_22,
            23 => Ceref0::Ceref0_23,
            24 => Ceref0::Ceref0_24,
            25 => Ceref0::Ceref0_25,
            26 => Ceref0::Ceref0_26,
            27 => Ceref0::Ceref0_27,
            28 => Ceref0::Ceref0_28,
            29 => Ceref0::Ceref0_29,
            30 => Ceref0::Ceref0_30,
            31 => Ceref0::Ceref0_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Reference resistor tap for setting 0."]
    #[inline(always)]
    pub fn is_ceref0_0(&self) -> bool {
        *self == Ceref0::Ceref0_0
    }
    #[doc = "Reference resistor tap for setting 1."]
    #[inline(always)]
    pub fn is_ceref0_1(&self) -> bool {
        *self == Ceref0::Ceref0_1
    }
    #[doc = "Reference resistor tap for setting 2."]
    #[inline(always)]
    pub fn is_ceref0_2(&self) -> bool {
        *self == Ceref0::Ceref0_2
    }
    #[doc = "Reference resistor tap for setting 3."]
    #[inline(always)]
    pub fn is_ceref0_3(&self) -> bool {
        *self == Ceref0::Ceref0_3
    }
    #[doc = "Reference resistor tap for setting 4."]
    #[inline(always)]
    pub fn is_ceref0_4(&self) -> bool {
        *self == Ceref0::Ceref0_4
    }
    #[doc = "Reference resistor tap for setting 5."]
    #[inline(always)]
    pub fn is_ceref0_5(&self) -> bool {
        *self == Ceref0::Ceref0_5
    }
    #[doc = "Reference resistor tap for setting 6."]
    #[inline(always)]
    pub fn is_ceref0_6(&self) -> bool {
        *self == Ceref0::Ceref0_6
    }
    #[doc = "Reference resistor tap for setting 7."]
    #[inline(always)]
    pub fn is_ceref0_7(&self) -> bool {
        *self == Ceref0::Ceref0_7
    }
    #[doc = "Reference resistor tap for setting 8."]
    #[inline(always)]
    pub fn is_ceref0_8(&self) -> bool {
        *self == Ceref0::Ceref0_8
    }
    #[doc = "Reference resistor tap for setting 9."]
    #[inline(always)]
    pub fn is_ceref0_9(&self) -> bool {
        *self == Ceref0::Ceref0_9
    }
    #[doc = "Reference resistor tap for setting 10."]
    #[inline(always)]
    pub fn is_ceref0_10(&self) -> bool {
        *self == Ceref0::Ceref0_10
    }
    #[doc = "Reference resistor tap for setting 11."]
    #[inline(always)]
    pub fn is_ceref0_11(&self) -> bool {
        *self == Ceref0::Ceref0_11
    }
    #[doc = "Reference resistor tap for setting 12."]
    #[inline(always)]
    pub fn is_ceref0_12(&self) -> bool {
        *self == Ceref0::Ceref0_12
    }
    #[doc = "Reference resistor tap for setting 13."]
    #[inline(always)]
    pub fn is_ceref0_13(&self) -> bool {
        *self == Ceref0::Ceref0_13
    }
    #[doc = "Reference resistor tap for setting 14."]
    #[inline(always)]
    pub fn is_ceref0_14(&self) -> bool {
        *self == Ceref0::Ceref0_14
    }
    #[doc = "Reference resistor tap for setting 15."]
    #[inline(always)]
    pub fn is_ceref0_15(&self) -> bool {
        *self == Ceref0::Ceref0_15
    }
    #[doc = "Reference resistor tap for setting 16."]
    #[inline(always)]
    pub fn is_ceref0_16(&self) -> bool {
        *self == Ceref0::Ceref0_16
    }
    #[doc = "Reference resistor tap for setting 17."]
    #[inline(always)]
    pub fn is_ceref0_17(&self) -> bool {
        *self == Ceref0::Ceref0_17
    }
    #[doc = "Reference resistor tap for setting 18."]
    #[inline(always)]
    pub fn is_ceref0_18(&self) -> bool {
        *self == Ceref0::Ceref0_18
    }
    #[doc = "Reference resistor tap for setting 19."]
    #[inline(always)]
    pub fn is_ceref0_19(&self) -> bool {
        *self == Ceref0::Ceref0_19
    }
    #[doc = "Reference resistor tap for setting 20."]
    #[inline(always)]
    pub fn is_ceref0_20(&self) -> bool {
        *self == Ceref0::Ceref0_20
    }
    #[doc = "Reference resistor tap for setting 21."]
    #[inline(always)]
    pub fn is_ceref0_21(&self) -> bool {
        *self == Ceref0::Ceref0_21
    }
    #[doc = "Reference resistor tap for setting 22."]
    #[inline(always)]
    pub fn is_ceref0_22(&self) -> bool {
        *self == Ceref0::Ceref0_22
    }
    #[doc = "Reference resistor tap for setting 23."]
    #[inline(always)]
    pub fn is_ceref0_23(&self) -> bool {
        *self == Ceref0::Ceref0_23
    }
    #[doc = "Reference resistor tap for setting 24."]
    #[inline(always)]
    pub fn is_ceref0_24(&self) -> bool {
        *self == Ceref0::Ceref0_24
    }
    #[doc = "Reference resistor tap for setting 25."]
    #[inline(always)]
    pub fn is_ceref0_25(&self) -> bool {
        *self == Ceref0::Ceref0_25
    }
    #[doc = "Reference resistor tap for setting 26."]
    #[inline(always)]
    pub fn is_ceref0_26(&self) -> bool {
        *self == Ceref0::Ceref0_26
    }
    #[doc = "Reference resistor tap for setting 27."]
    #[inline(always)]
    pub fn is_ceref0_27(&self) -> bool {
        *self == Ceref0::Ceref0_27
    }
    #[doc = "Reference resistor tap for setting 28."]
    #[inline(always)]
    pub fn is_ceref0_28(&self) -> bool {
        *self == Ceref0::Ceref0_28
    }
    #[doc = "Reference resistor tap for setting 29."]
    #[inline(always)]
    pub fn is_ceref0_29(&self) -> bool {
        *self == Ceref0::Ceref0_29
    }
    #[doc = "Reference resistor tap for setting 30."]
    #[inline(always)]
    pub fn is_ceref0_30(&self) -> bool {
        *self == Ceref0::Ceref0_30
    }
    #[doc = "Reference resistor tap for setting 31."]
    #[inline(always)]
    pub fn is_ceref0_31(&self) -> bool {
        *self == Ceref0::Ceref0_31
    }
}
#[doc = "Field `CEREF0` writer - Reference resistor tap 0"]
pub type Ceref0W<'a, REG> = crate::FieldWriter<'a, REG, 5, Ceref0, crate::Safe>;
impl<'a, REG> Ceref0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reference resistor tap for setting 0."]
    #[inline(always)]
    pub fn ceref0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_0)
    }
    #[doc = "Reference resistor tap for setting 1."]
    #[inline(always)]
    pub fn ceref0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_1)
    }
    #[doc = "Reference resistor tap for setting 2."]
    #[inline(always)]
    pub fn ceref0_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_2)
    }
    #[doc = "Reference resistor tap for setting 3."]
    #[inline(always)]
    pub fn ceref0_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_3)
    }
    #[doc = "Reference resistor tap for setting 4."]
    #[inline(always)]
    pub fn ceref0_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_4)
    }
    #[doc = "Reference resistor tap for setting 5."]
    #[inline(always)]
    pub fn ceref0_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_5)
    }
    #[doc = "Reference resistor tap for setting 6."]
    #[inline(always)]
    pub fn ceref0_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_6)
    }
    #[doc = "Reference resistor tap for setting 7."]
    #[inline(always)]
    pub fn ceref0_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_7)
    }
    #[doc = "Reference resistor tap for setting 8."]
    #[inline(always)]
    pub fn ceref0_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_8)
    }
    #[doc = "Reference resistor tap for setting 9."]
    #[inline(always)]
    pub fn ceref0_9(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_9)
    }
    #[doc = "Reference resistor tap for setting 10."]
    #[inline(always)]
    pub fn ceref0_10(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_10)
    }
    #[doc = "Reference resistor tap for setting 11."]
    #[inline(always)]
    pub fn ceref0_11(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_11)
    }
    #[doc = "Reference resistor tap for setting 12."]
    #[inline(always)]
    pub fn ceref0_12(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_12)
    }
    #[doc = "Reference resistor tap for setting 13."]
    #[inline(always)]
    pub fn ceref0_13(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_13)
    }
    #[doc = "Reference resistor tap for setting 14."]
    #[inline(always)]
    pub fn ceref0_14(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_14)
    }
    #[doc = "Reference resistor tap for setting 15."]
    #[inline(always)]
    pub fn ceref0_15(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_15)
    }
    #[doc = "Reference resistor tap for setting 16."]
    #[inline(always)]
    pub fn ceref0_16(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_16)
    }
    #[doc = "Reference resistor tap for setting 17."]
    #[inline(always)]
    pub fn ceref0_17(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_17)
    }
    #[doc = "Reference resistor tap for setting 18."]
    #[inline(always)]
    pub fn ceref0_18(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_18)
    }
    #[doc = "Reference resistor tap for setting 19."]
    #[inline(always)]
    pub fn ceref0_19(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_19)
    }
    #[doc = "Reference resistor tap for setting 20."]
    #[inline(always)]
    pub fn ceref0_20(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_20)
    }
    #[doc = "Reference resistor tap for setting 21."]
    #[inline(always)]
    pub fn ceref0_21(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_21)
    }
    #[doc = "Reference resistor tap for setting 22."]
    #[inline(always)]
    pub fn ceref0_22(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_22)
    }
    #[doc = "Reference resistor tap for setting 23."]
    #[inline(always)]
    pub fn ceref0_23(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_23)
    }
    #[doc = "Reference resistor tap for setting 24."]
    #[inline(always)]
    pub fn ceref0_24(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_24)
    }
    #[doc = "Reference resistor tap for setting 25."]
    #[inline(always)]
    pub fn ceref0_25(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_25)
    }
    #[doc = "Reference resistor tap for setting 26."]
    #[inline(always)]
    pub fn ceref0_26(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_26)
    }
    #[doc = "Reference resistor tap for setting 27."]
    #[inline(always)]
    pub fn ceref0_27(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_27)
    }
    #[doc = "Reference resistor tap for setting 28."]
    #[inline(always)]
    pub fn ceref0_28(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_28)
    }
    #[doc = "Reference resistor tap for setting 29."]
    #[inline(always)]
    pub fn ceref0_29(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_29)
    }
    #[doc = "Reference resistor tap for setting 30."]
    #[inline(always)]
    pub fn ceref0_30(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_30)
    }
    #[doc = "Reference resistor tap for setting 31."]
    #[inline(always)]
    pub fn ceref0_31(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref0::Ceref0_31)
    }
}
#[doc = "Reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cersel {
    #[doc = "0: When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    Cersel0 = 0,
    #[doc = "1: When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    Cersel1 = 1,
}
impl From<Cersel> for bool {
    #[inline(always)]
    fn from(variant: Cersel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERSEL` reader - Reference select"]
pub type CerselR = crate::BitReader<Cersel>;
impl CerselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cersel {
        match self.bits {
            false => Cersel::Cersel0,
            true => Cersel::Cersel1,
        }
    }
    #[doc = "When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    #[inline(always)]
    pub fn is_cersel_0(&self) -> bool {
        *self == Cersel::Cersel0
    }
    #[doc = "When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    #[inline(always)]
    pub fn is_cersel_1(&self) -> bool {
        *self == Cersel::Cersel1
    }
}
#[doc = "Field `CERSEL` writer - Reference select"]
pub type CerselW<'a, REG> = crate::BitWriter<'a, REG, Cersel>;
impl<'a, REG> CerselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When CEEX = 0, VREF is applied to the V+ terminal; When CEEX = 1, VREF is applied to the V- terminal"]
    #[inline(always)]
    pub fn cersel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cersel::Cersel0)
    }
    #[doc = "When CEEX = 0, VREF is applied to the V- terminal; When CEEX = 1, VREF is applied to the V+ terminal"]
    #[inline(always)]
    pub fn cersel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cersel::Cersel1)
    }
}
#[doc = "Reference source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cers {
    #[doc = "0: No current is drawn by the reference circuitry"]
    Cers0 = 0,
    #[doc = "1: VCC applied to the resistor ladder"]
    Cers1 = 1,
    #[doc = "2: Shared reference voltage applied to the resistor ladder"]
    Cers2 = 2,
    #[doc = "3: Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    Cers3 = 3,
}
impl From<Cers> for u8 {
    #[inline(always)]
    fn from(variant: Cers) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cers {
    type Ux = u8;
}
impl crate::IsEnum for Cers {}
#[doc = "Field `CERS` reader - Reference source"]
pub type CersR = crate::FieldReader<Cers>;
impl CersR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cers {
        match self.bits {
            0 => Cers::Cers0,
            1 => Cers::Cers1,
            2 => Cers::Cers2,
            3 => Cers::Cers3,
            _ => unreachable!(),
        }
    }
    #[doc = "No current is drawn by the reference circuitry"]
    #[inline(always)]
    pub fn is_cers_0(&self) -> bool {
        *self == Cers::Cers0
    }
    #[doc = "VCC applied to the resistor ladder"]
    #[inline(always)]
    pub fn is_cers_1(&self) -> bool {
        *self == Cers::Cers1
    }
    #[doc = "Shared reference voltage applied to the resistor ladder"]
    #[inline(always)]
    pub fn is_cers_2(&self) -> bool {
        *self == Cers::Cers2
    }
    #[doc = "Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    #[inline(always)]
    pub fn is_cers_3(&self) -> bool {
        *self == Cers::Cers3
    }
}
#[doc = "Field `CERS` writer - Reference source"]
pub type CersW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cers, crate::Safe>;
impl<'a, REG> CersW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No current is drawn by the reference circuitry"]
    #[inline(always)]
    pub fn cers_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cers::Cers0)
    }
    #[doc = "VCC applied to the resistor ladder"]
    #[inline(always)]
    pub fn cers_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cers::Cers1)
    }
    #[doc = "Shared reference voltage applied to the resistor ladder"]
    #[inline(always)]
    pub fn cers_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cers::Cers2)
    }
    #[doc = "Shared reference voltage supplied to V(CREF). Resistor ladder is off"]
    #[inline(always)]
    pub fn cers_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cers::Cers3)
    }
}
#[doc = "Reference resistor tap 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ceref1 {
    #[doc = "0: Reference resistor tap for setting 0."]
    Ceref1_0 = 0,
    #[doc = "1: Reference resistor tap for setting 1."]
    Ceref1_1 = 1,
    #[doc = "2: Reference resistor tap for setting 2."]
    Ceref1_2 = 2,
    #[doc = "3: Reference resistor tap for setting 3."]
    Ceref1_3 = 3,
    #[doc = "4: Reference resistor tap for setting 4."]
    Ceref1_4 = 4,
    #[doc = "5: Reference resistor tap for setting 5."]
    Ceref1_5 = 5,
    #[doc = "6: Reference resistor tap for setting 6."]
    Ceref1_6 = 6,
    #[doc = "7: Reference resistor tap for setting 7."]
    Ceref1_7 = 7,
    #[doc = "8: Reference resistor tap for setting 8."]
    Ceref1_8 = 8,
    #[doc = "9: Reference resistor tap for setting 9."]
    Ceref1_9 = 9,
    #[doc = "10: Reference resistor tap for setting 10."]
    Ceref1_10 = 10,
    #[doc = "11: Reference resistor tap for setting 11."]
    Ceref1_11 = 11,
    #[doc = "12: Reference resistor tap for setting 12."]
    Ceref1_12 = 12,
    #[doc = "13: Reference resistor tap for setting 13."]
    Ceref1_13 = 13,
    #[doc = "14: Reference resistor tap for setting 14."]
    Ceref1_14 = 14,
    #[doc = "15: Reference resistor tap for setting 15."]
    Ceref1_15 = 15,
    #[doc = "16: Reference resistor tap for setting 16."]
    Ceref1_16 = 16,
    #[doc = "17: Reference resistor tap for setting 17."]
    Ceref1_17 = 17,
    #[doc = "18: Reference resistor tap for setting 18."]
    Ceref1_18 = 18,
    #[doc = "19: Reference resistor tap for setting 19."]
    Ceref1_19 = 19,
    #[doc = "20: Reference resistor tap for setting 20."]
    Ceref1_20 = 20,
    #[doc = "21: Reference resistor tap for setting 21."]
    Ceref1_21 = 21,
    #[doc = "22: Reference resistor tap for setting 22."]
    Ceref1_22 = 22,
    #[doc = "23: Reference resistor tap for setting 23."]
    Ceref1_23 = 23,
    #[doc = "24: Reference resistor tap for setting 24."]
    Ceref1_24 = 24,
    #[doc = "25: Reference resistor tap for setting 25."]
    Ceref1_25 = 25,
    #[doc = "26: Reference resistor tap for setting 26."]
    Ceref1_26 = 26,
    #[doc = "27: Reference resistor tap for setting 27."]
    Ceref1_27 = 27,
    #[doc = "28: Reference resistor tap for setting 28."]
    Ceref1_28 = 28,
    #[doc = "29: Reference resistor tap for setting 29."]
    Ceref1_29 = 29,
    #[doc = "30: Reference resistor tap for setting 30."]
    Ceref1_30 = 30,
    #[doc = "31: Reference resistor tap for setting 31."]
    Ceref1_31 = 31,
}
impl From<Ceref1> for u8 {
    #[inline(always)]
    fn from(variant: Ceref1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ceref1 {
    type Ux = u8;
}
impl crate::IsEnum for Ceref1 {}
#[doc = "Field `CEREF1` reader - Reference resistor tap 1"]
pub type Ceref1R = crate::FieldReader<Ceref1>;
impl Ceref1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceref1 {
        match self.bits {
            0 => Ceref1::Ceref1_0,
            1 => Ceref1::Ceref1_1,
            2 => Ceref1::Ceref1_2,
            3 => Ceref1::Ceref1_3,
            4 => Ceref1::Ceref1_4,
            5 => Ceref1::Ceref1_5,
            6 => Ceref1::Ceref1_6,
            7 => Ceref1::Ceref1_7,
            8 => Ceref1::Ceref1_8,
            9 => Ceref1::Ceref1_9,
            10 => Ceref1::Ceref1_10,
            11 => Ceref1::Ceref1_11,
            12 => Ceref1::Ceref1_12,
            13 => Ceref1::Ceref1_13,
            14 => Ceref1::Ceref1_14,
            15 => Ceref1::Ceref1_15,
            16 => Ceref1::Ceref1_16,
            17 => Ceref1::Ceref1_17,
            18 => Ceref1::Ceref1_18,
            19 => Ceref1::Ceref1_19,
            20 => Ceref1::Ceref1_20,
            21 => Ceref1::Ceref1_21,
            22 => Ceref1::Ceref1_22,
            23 => Ceref1::Ceref1_23,
            24 => Ceref1::Ceref1_24,
            25 => Ceref1::Ceref1_25,
            26 => Ceref1::Ceref1_26,
            27 => Ceref1::Ceref1_27,
            28 => Ceref1::Ceref1_28,
            29 => Ceref1::Ceref1_29,
            30 => Ceref1::Ceref1_30,
            31 => Ceref1::Ceref1_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Reference resistor tap for setting 0."]
    #[inline(always)]
    pub fn is_ceref1_0(&self) -> bool {
        *self == Ceref1::Ceref1_0
    }
    #[doc = "Reference resistor tap for setting 1."]
    #[inline(always)]
    pub fn is_ceref1_1(&self) -> bool {
        *self == Ceref1::Ceref1_1
    }
    #[doc = "Reference resistor tap for setting 2."]
    #[inline(always)]
    pub fn is_ceref1_2(&self) -> bool {
        *self == Ceref1::Ceref1_2
    }
    #[doc = "Reference resistor tap for setting 3."]
    #[inline(always)]
    pub fn is_ceref1_3(&self) -> bool {
        *self == Ceref1::Ceref1_3
    }
    #[doc = "Reference resistor tap for setting 4."]
    #[inline(always)]
    pub fn is_ceref1_4(&self) -> bool {
        *self == Ceref1::Ceref1_4
    }
    #[doc = "Reference resistor tap for setting 5."]
    #[inline(always)]
    pub fn is_ceref1_5(&self) -> bool {
        *self == Ceref1::Ceref1_5
    }
    #[doc = "Reference resistor tap for setting 6."]
    #[inline(always)]
    pub fn is_ceref1_6(&self) -> bool {
        *self == Ceref1::Ceref1_6
    }
    #[doc = "Reference resistor tap for setting 7."]
    #[inline(always)]
    pub fn is_ceref1_7(&self) -> bool {
        *self == Ceref1::Ceref1_7
    }
    #[doc = "Reference resistor tap for setting 8."]
    #[inline(always)]
    pub fn is_ceref1_8(&self) -> bool {
        *self == Ceref1::Ceref1_8
    }
    #[doc = "Reference resistor tap for setting 9."]
    #[inline(always)]
    pub fn is_ceref1_9(&self) -> bool {
        *self == Ceref1::Ceref1_9
    }
    #[doc = "Reference resistor tap for setting 10."]
    #[inline(always)]
    pub fn is_ceref1_10(&self) -> bool {
        *self == Ceref1::Ceref1_10
    }
    #[doc = "Reference resistor tap for setting 11."]
    #[inline(always)]
    pub fn is_ceref1_11(&self) -> bool {
        *self == Ceref1::Ceref1_11
    }
    #[doc = "Reference resistor tap for setting 12."]
    #[inline(always)]
    pub fn is_ceref1_12(&self) -> bool {
        *self == Ceref1::Ceref1_12
    }
    #[doc = "Reference resistor tap for setting 13."]
    #[inline(always)]
    pub fn is_ceref1_13(&self) -> bool {
        *self == Ceref1::Ceref1_13
    }
    #[doc = "Reference resistor tap for setting 14."]
    #[inline(always)]
    pub fn is_ceref1_14(&self) -> bool {
        *self == Ceref1::Ceref1_14
    }
    #[doc = "Reference resistor tap for setting 15."]
    #[inline(always)]
    pub fn is_ceref1_15(&self) -> bool {
        *self == Ceref1::Ceref1_15
    }
    #[doc = "Reference resistor tap for setting 16."]
    #[inline(always)]
    pub fn is_ceref1_16(&self) -> bool {
        *self == Ceref1::Ceref1_16
    }
    #[doc = "Reference resistor tap for setting 17."]
    #[inline(always)]
    pub fn is_ceref1_17(&self) -> bool {
        *self == Ceref1::Ceref1_17
    }
    #[doc = "Reference resistor tap for setting 18."]
    #[inline(always)]
    pub fn is_ceref1_18(&self) -> bool {
        *self == Ceref1::Ceref1_18
    }
    #[doc = "Reference resistor tap for setting 19."]
    #[inline(always)]
    pub fn is_ceref1_19(&self) -> bool {
        *self == Ceref1::Ceref1_19
    }
    #[doc = "Reference resistor tap for setting 20."]
    #[inline(always)]
    pub fn is_ceref1_20(&self) -> bool {
        *self == Ceref1::Ceref1_20
    }
    #[doc = "Reference resistor tap for setting 21."]
    #[inline(always)]
    pub fn is_ceref1_21(&self) -> bool {
        *self == Ceref1::Ceref1_21
    }
    #[doc = "Reference resistor tap for setting 22."]
    #[inline(always)]
    pub fn is_ceref1_22(&self) -> bool {
        *self == Ceref1::Ceref1_22
    }
    #[doc = "Reference resistor tap for setting 23."]
    #[inline(always)]
    pub fn is_ceref1_23(&self) -> bool {
        *self == Ceref1::Ceref1_23
    }
    #[doc = "Reference resistor tap for setting 24."]
    #[inline(always)]
    pub fn is_ceref1_24(&self) -> bool {
        *self == Ceref1::Ceref1_24
    }
    #[doc = "Reference resistor tap for setting 25."]
    #[inline(always)]
    pub fn is_ceref1_25(&self) -> bool {
        *self == Ceref1::Ceref1_25
    }
    #[doc = "Reference resistor tap for setting 26."]
    #[inline(always)]
    pub fn is_ceref1_26(&self) -> bool {
        *self == Ceref1::Ceref1_26
    }
    #[doc = "Reference resistor tap for setting 27."]
    #[inline(always)]
    pub fn is_ceref1_27(&self) -> bool {
        *self == Ceref1::Ceref1_27
    }
    #[doc = "Reference resistor tap for setting 28."]
    #[inline(always)]
    pub fn is_ceref1_28(&self) -> bool {
        *self == Ceref1::Ceref1_28
    }
    #[doc = "Reference resistor tap for setting 29."]
    #[inline(always)]
    pub fn is_ceref1_29(&self) -> bool {
        *self == Ceref1::Ceref1_29
    }
    #[doc = "Reference resistor tap for setting 30."]
    #[inline(always)]
    pub fn is_ceref1_30(&self) -> bool {
        *self == Ceref1::Ceref1_30
    }
    #[doc = "Reference resistor tap for setting 31."]
    #[inline(always)]
    pub fn is_ceref1_31(&self) -> bool {
        *self == Ceref1::Ceref1_31
    }
}
#[doc = "Field `CEREF1` writer - Reference resistor tap 1"]
pub type Ceref1W<'a, REG> = crate::FieldWriter<'a, REG, 5, Ceref1, crate::Safe>;
impl<'a, REG> Ceref1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reference resistor tap for setting 0."]
    #[inline(always)]
    pub fn ceref1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_0)
    }
    #[doc = "Reference resistor tap for setting 1."]
    #[inline(always)]
    pub fn ceref1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_1)
    }
    #[doc = "Reference resistor tap for setting 2."]
    #[inline(always)]
    pub fn ceref1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_2)
    }
    #[doc = "Reference resistor tap for setting 3."]
    #[inline(always)]
    pub fn ceref1_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_3)
    }
    #[doc = "Reference resistor tap for setting 4."]
    #[inline(always)]
    pub fn ceref1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_4)
    }
    #[doc = "Reference resistor tap for setting 5."]
    #[inline(always)]
    pub fn ceref1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_5)
    }
    #[doc = "Reference resistor tap for setting 6."]
    #[inline(always)]
    pub fn ceref1_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_6)
    }
    #[doc = "Reference resistor tap for setting 7."]
    #[inline(always)]
    pub fn ceref1_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_7)
    }
    #[doc = "Reference resistor tap for setting 8."]
    #[inline(always)]
    pub fn ceref1_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_8)
    }
    #[doc = "Reference resistor tap for setting 9."]
    #[inline(always)]
    pub fn ceref1_9(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_9)
    }
    #[doc = "Reference resistor tap for setting 10."]
    #[inline(always)]
    pub fn ceref1_10(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_10)
    }
    #[doc = "Reference resistor tap for setting 11."]
    #[inline(always)]
    pub fn ceref1_11(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_11)
    }
    #[doc = "Reference resistor tap for setting 12."]
    #[inline(always)]
    pub fn ceref1_12(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_12)
    }
    #[doc = "Reference resistor tap for setting 13."]
    #[inline(always)]
    pub fn ceref1_13(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_13)
    }
    #[doc = "Reference resistor tap for setting 14."]
    #[inline(always)]
    pub fn ceref1_14(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_14)
    }
    #[doc = "Reference resistor tap for setting 15."]
    #[inline(always)]
    pub fn ceref1_15(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_15)
    }
    #[doc = "Reference resistor tap for setting 16."]
    #[inline(always)]
    pub fn ceref1_16(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_16)
    }
    #[doc = "Reference resistor tap for setting 17."]
    #[inline(always)]
    pub fn ceref1_17(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_17)
    }
    #[doc = "Reference resistor tap for setting 18."]
    #[inline(always)]
    pub fn ceref1_18(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_18)
    }
    #[doc = "Reference resistor tap for setting 19."]
    #[inline(always)]
    pub fn ceref1_19(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_19)
    }
    #[doc = "Reference resistor tap for setting 20."]
    #[inline(always)]
    pub fn ceref1_20(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_20)
    }
    #[doc = "Reference resistor tap for setting 21."]
    #[inline(always)]
    pub fn ceref1_21(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_21)
    }
    #[doc = "Reference resistor tap for setting 22."]
    #[inline(always)]
    pub fn ceref1_22(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_22)
    }
    #[doc = "Reference resistor tap for setting 23."]
    #[inline(always)]
    pub fn ceref1_23(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_23)
    }
    #[doc = "Reference resistor tap for setting 24."]
    #[inline(always)]
    pub fn ceref1_24(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_24)
    }
    #[doc = "Reference resistor tap for setting 25."]
    #[inline(always)]
    pub fn ceref1_25(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_25)
    }
    #[doc = "Reference resistor tap for setting 26."]
    #[inline(always)]
    pub fn ceref1_26(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_26)
    }
    #[doc = "Reference resistor tap for setting 27."]
    #[inline(always)]
    pub fn ceref1_27(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_27)
    }
    #[doc = "Reference resistor tap for setting 28."]
    #[inline(always)]
    pub fn ceref1_28(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_28)
    }
    #[doc = "Reference resistor tap for setting 29."]
    #[inline(always)]
    pub fn ceref1_29(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_29)
    }
    #[doc = "Reference resistor tap for setting 30."]
    #[inline(always)]
    pub fn ceref1_30(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_30)
    }
    #[doc = "Reference resistor tap for setting 31."]
    #[inline(always)]
    pub fn ceref1_31(self) -> &'a mut crate::W<REG> {
        self.variant(Ceref1::Ceref1_31)
    }
}
#[doc = "Reference voltage level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cerefl {
    #[doc = "0: Reference amplifier is disabled. No reference voltage is requested"]
    Cerefl0 = 0,
    #[doc = "1: 1.2 V is selected as shared reference voltage input"]
    Cerefl1 = 1,
    #[doc = "2: 2.0 V is selected as shared reference voltage input"]
    Cerefl2 = 2,
    #[doc = "3: 2.5 V is selected as shared reference voltage input"]
    Cerefl3 = 3,
}
impl From<Cerefl> for u8 {
    #[inline(always)]
    fn from(variant: Cerefl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cerefl {
    type Ux = u8;
}
impl crate::IsEnum for Cerefl {}
#[doc = "Field `CEREFL` reader - Reference voltage level"]
pub type CereflR = crate::FieldReader<Cerefl>;
impl CereflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cerefl {
        match self.bits {
            0 => Cerefl::Cerefl0,
            1 => Cerefl::Cerefl1,
            2 => Cerefl::Cerefl2,
            3 => Cerefl::Cerefl3,
            _ => unreachable!(),
        }
    }
    #[doc = "Reference amplifier is disabled. No reference voltage is requested"]
    #[inline(always)]
    pub fn is_cerefl_0(&self) -> bool {
        *self == Cerefl::Cerefl0
    }
    #[doc = "1.2 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn is_cerefl_1(&self) -> bool {
        *self == Cerefl::Cerefl1
    }
    #[doc = "2.0 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn is_cerefl_2(&self) -> bool {
        *self == Cerefl::Cerefl2
    }
    #[doc = "2.5 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn is_cerefl_3(&self) -> bool {
        *self == Cerefl::Cerefl3
    }
}
#[doc = "Field `CEREFL` writer - Reference voltage level"]
pub type CereflW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cerefl, crate::Safe>;
impl<'a, REG> CereflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reference amplifier is disabled. No reference voltage is requested"]
    #[inline(always)]
    pub fn cerefl_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefl::Cerefl0)
    }
    #[doc = "1.2 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn cerefl_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefl::Cerefl1)
    }
    #[doc = "2.0 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn cerefl_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefl::Cerefl2)
    }
    #[doc = "2.5 V is selected as shared reference voltage input"]
    #[inline(always)]
    pub fn cerefl_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefl::Cerefl3)
    }
}
#[doc = "Reference accuracy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cerefacc {
    #[doc = "0: Static mode"]
    Cerefacc0 = 0,
    #[doc = "1: Clocked (low power, low accuracy) mode"]
    Cerefacc1 = 1,
}
impl From<Cerefacc> for bool {
    #[inline(always)]
    fn from(variant: Cerefacc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEREFACC` reader - Reference accuracy"]
pub type CerefaccR = crate::BitReader<Cerefacc>;
impl CerefaccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cerefacc {
        match self.bits {
            false => Cerefacc::Cerefacc0,
            true => Cerefacc::Cerefacc1,
        }
    }
    #[doc = "Static mode"]
    #[inline(always)]
    pub fn is_cerefacc_0(&self) -> bool {
        *self == Cerefacc::Cerefacc0
    }
    #[doc = "Clocked (low power, low accuracy) mode"]
    #[inline(always)]
    pub fn is_cerefacc_1(&self) -> bool {
        *self == Cerefacc::Cerefacc1
    }
}
#[doc = "Field `CEREFACC` writer - Reference accuracy"]
pub type CerefaccW<'a, REG> = crate::BitWriter<'a, REG, Cerefacc>;
impl<'a, REG> CerefaccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Static mode"]
    #[inline(always)]
    pub fn cerefacc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefacc::Cerefacc0)
    }
    #[doc = "Clocked (low power, low accuracy) mode"]
    #[inline(always)]
    pub fn cerefacc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cerefacc::Cerefacc1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Reference resistor tap 0"]
    #[inline(always)]
    pub fn ceref0(&self) -> Ceref0R {
        Ceref0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    pub fn cersel(&self) -> CerselR {
        CerselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reference source"]
    #[inline(always)]
    pub fn cers(&self) -> CersR {
        CersR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Reference resistor tap 1"]
    #[inline(always)]
    pub fn ceref1(&self) -> Ceref1R {
        Ceref1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - Reference voltage level"]
    #[inline(always)]
    pub fn cerefl(&self) -> CereflR {
        CereflR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Reference accuracy"]
    #[inline(always)]
    pub fn cerefacc(&self) -> CerefaccR {
        CerefaccR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reference resistor tap 0"]
    #[inline(always)]
    pub fn ceref0(&mut self) -> Ceref0W<CexCtl2Spec> {
        Ceref0W::new(self, 0)
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    pub fn cersel(&mut self) -> CerselW<CexCtl2Spec> {
        CerselW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Reference source"]
    #[inline(always)]
    pub fn cers(&mut self) -> CersW<CexCtl2Spec> {
        CersW::new(self, 6)
    }
    #[doc = "Bits 8:12 - Reference resistor tap 1"]
    #[inline(always)]
    pub fn ceref1(&mut self) -> Ceref1W<CexCtl2Spec> {
        Ceref1W::new(self, 8)
    }
    #[doc = "Bits 13:14 - Reference voltage level"]
    #[inline(always)]
    pub fn cerefl(&mut self) -> CereflW<CexCtl2Spec> {
        CereflW::new(self, 13)
    }
    #[doc = "Bit 15 - Reference accuracy"]
    #[inline(always)]
    pub fn cerefacc(&mut self) -> CerefaccW<CexCtl2Spec> {
        CerefaccW::new(self, 15)
    }
}
#[doc = "Comparator Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cex_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cex_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CexCtl2Spec;
impl crate::RegisterSpec for CexCtl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cex_ctl2::R`](R) reader structure"]
impl crate::Readable for CexCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`cex_ctl2::W`](W) writer structure"]
impl crate::Writable for CexCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CExCTL2 to value 0"]
impl crate::Resettable for CexCtl2Spec {
    const RESET_VALUE: u16 = 0;
}
