#[doc = "Register `CSCTL1` reader"]
pub type R = crate::R<Csctl1Spec>;
#[doc = "Register `CSCTL1` writer"]
pub type W = crate::W<Csctl1Spec>;
#[doc = "Selects the MCLK source\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selm {
    #[doc = "0: when LFXT available, otherwise REFOCLK"]
    Selm0 = 0,
    #[doc = "1: `1`"]
    Selm1 = 1,
    #[doc = "2: `10`"]
    Selm2 = 2,
    #[doc = "3: `11`"]
    Selm3 = 3,
    #[doc = "4: `100`"]
    Selm4 = 4,
    #[doc = "5: when HFXT available, otherwise DCOCLK"]
    Selm5 = 5,
    #[doc = "6: when HFXT2 available, otherwise DCOCLK"]
    Selm6 = 6,
    #[doc = "7: for future use. Defaults to DCOCLK. Not recommended for use to ensure future compatibilities."]
    Selm7 = 7,
}
impl From<Selm> for u8 {
    #[inline(always)]
    fn from(variant: Selm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selm {
    type Ux = u8;
}
impl crate::IsEnum for Selm {}
#[doc = "Field `SELM` reader - Selects the MCLK source"]
pub type SelmR = crate::FieldReader<Selm>;
impl SelmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selm {
        match self.bits {
            0 => Selm::Selm0,
            1 => Selm::Selm1,
            2 => Selm::Selm2,
            3 => Selm::Selm3,
            4 => Selm::Selm4,
            5 => Selm::Selm5,
            6 => Selm::Selm6,
            7 => Selm::Selm7,
            _ => unreachable!(),
        }
    }
    #[doc = "when LFXT available, otherwise REFOCLK"]
    #[inline(always)]
    pub fn is_selm_0(&self) -> bool {
        *self == Selm::Selm0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_selm_1(&self) -> bool {
        *self == Selm::Selm1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_selm_2(&self) -> bool {
        *self == Selm::Selm2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_selm_3(&self) -> bool {
        *self == Selm::Selm3
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_selm_4(&self) -> bool {
        *self == Selm::Selm4
    }
    #[doc = "when HFXT available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn is_selm_5(&self) -> bool {
        *self == Selm::Selm5
    }
    #[doc = "when HFXT2 available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn is_selm_6(&self) -> bool {
        *self == Selm::Selm6
    }
    #[doc = "for future use. Defaults to DCOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn is_selm_7(&self) -> bool {
        *self == Selm::Selm7
    }
}
#[doc = "Field `SELM` writer - Selects the MCLK source"]
pub type SelmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selm, crate::Safe>;
impl<'a, REG> SelmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "when LFXT available, otherwise REFOCLK"]
    #[inline(always)]
    pub fn selm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn selm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn selm_2(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn selm_3(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn selm_4(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm4)
    }
    #[doc = "when HFXT available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn selm_5(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm5)
    }
    #[doc = "when HFXT2 available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn selm_6(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm6)
    }
    #[doc = "for future use. Defaults to DCOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn selm_7(self) -> &'a mut crate::W<REG> {
        self.variant(Selm::Selm7)
    }
}
#[doc = "Selects the SMCLK and HSMCLK source\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sels {
    #[doc = "0: when LFXT available, otherwise REFOCLK"]
    Sels0 = 0,
    #[doc = "1: `1`"]
    Sels1 = 1,
    #[doc = "2: `10`"]
    Sels2 = 2,
    #[doc = "3: `11`"]
    Sels3 = 3,
    #[doc = "4: `100`"]
    Sels4 = 4,
    #[doc = "5: when HFXT available, otherwise DCOCLK"]
    Sels5 = 5,
    #[doc = "6: when HFXT2 available, otherwise DCOCLK"]
    Sels6 = 6,
    #[doc = "7: for furture use. Defaults to DCOCLK. Do not use to ensure future compatibilities."]
    Sels7 = 7,
}
impl From<Sels> for u8 {
    #[inline(always)]
    fn from(variant: Sels) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sels {
    type Ux = u8;
}
impl crate::IsEnum for Sels {}
#[doc = "Field `SELS` reader - Selects the SMCLK and HSMCLK source"]
pub type SelsR = crate::FieldReader<Sels>;
impl SelsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sels {
        match self.bits {
            0 => Sels::Sels0,
            1 => Sels::Sels1,
            2 => Sels::Sels2,
            3 => Sels::Sels3,
            4 => Sels::Sels4,
            5 => Sels::Sels5,
            6 => Sels::Sels6,
            7 => Sels::Sels7,
            _ => unreachable!(),
        }
    }
    #[doc = "when LFXT available, otherwise REFOCLK"]
    #[inline(always)]
    pub fn is_sels_0(&self) -> bool {
        *self == Sels::Sels0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sels_1(&self) -> bool {
        *self == Sels::Sels1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sels_2(&self) -> bool {
        *self == Sels::Sels2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_sels_3(&self) -> bool {
        *self == Sels::Sels3
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_sels_4(&self) -> bool {
        *self == Sels::Sels4
    }
    #[doc = "when HFXT available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn is_sels_5(&self) -> bool {
        *self == Sels::Sels5
    }
    #[doc = "when HFXT2 available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn is_sels_6(&self) -> bool {
        *self == Sels::Sels6
    }
    #[doc = "for furture use. Defaults to DCOCLK. Do not use to ensure future compatibilities."]
    #[inline(always)]
    pub fn is_sels_7(&self) -> bool {
        *self == Sels::Sels7
    }
}
#[doc = "Field `SELS` writer - Selects the SMCLK and HSMCLK source"]
pub type SelsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sels, crate::Safe>;
impl<'a, REG> SelsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "when LFXT available, otherwise REFOCLK"]
    #[inline(always)]
    pub fn sels_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Sels0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sels_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Sels1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sels_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Sels2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sels_3(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Sels3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sels_4(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Sels4)
    }
    #[doc = "when HFXT available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn sels_5(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Sels5)
    }
    #[doc = "when HFXT2 available, otherwise DCOCLK"]
    #[inline(always)]
    pub fn sels_6(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Sels6)
    }
    #[doc = "for furture use. Defaults to DCOCLK. Do not use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sels_7(self) -> &'a mut crate::W<REG> {
        self.variant(Sels::Sels7)
    }
}
#[doc = "Selects the ACLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sela {
    #[doc = "0: when LFXT available, otherwise REFOCLK"]
    Sela0 = 0,
    #[doc = "1: `1`"]
    Sela1 = 1,
    #[doc = "2: `10`"]
    Sela2 = 2,
    #[doc = "3: for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    Sela3 = 3,
    #[doc = "4: for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    Sela4 = 4,
    #[doc = "5: for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    Sela5 = 5,
    #[doc = "6: for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    Sela6 = 6,
    #[doc = "7: for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    Sela7 = 7,
}
impl From<Sela> for u8 {
    #[inline(always)]
    fn from(variant: Sela) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sela {
    type Ux = u8;
}
impl crate::IsEnum for Sela {}
#[doc = "Field `SELA` reader - Selects the ACLK source"]
pub type SelaR = crate::FieldReader<Sela>;
impl SelaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sela {
        match self.bits {
            0 => Sela::Sela0,
            1 => Sela::Sela1,
            2 => Sela::Sela2,
            3 => Sela::Sela3,
            4 => Sela::Sela4,
            5 => Sela::Sela5,
            6 => Sela::Sela6,
            7 => Sela::Sela7,
            _ => unreachable!(),
        }
    }
    #[doc = "when LFXT available, otherwise REFOCLK"]
    #[inline(always)]
    pub fn is_sela_0(&self) -> bool {
        *self == Sela::Sela0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_sela_1(&self) -> bool {
        *self == Sela::Sela1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_sela_2(&self) -> bool {
        *self == Sela::Sela2
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn is_sela_3(&self) -> bool {
        *self == Sela::Sela3
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn is_sela_4(&self) -> bool {
        *self == Sela::Sela4
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn is_sela_5(&self) -> bool {
        *self == Sela::Sela5
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn is_sela_6(&self) -> bool {
        *self == Sela::Sela6
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn is_sela_7(&self) -> bool {
        *self == Sela::Sela7
    }
}
#[doc = "Field `SELA` writer - Selects the ACLK source"]
pub type SelaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sela, crate::Safe>;
impl<'a, REG> SelaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "when LFXT available, otherwise REFOCLK"]
    #[inline(always)]
    pub fn sela_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Sela0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sela_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Sela1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sela_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Sela2)
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sela_3(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Sela3)
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sela_4(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Sela4)
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sela_5(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Sela5)
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sela_6(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Sela6)
    }
    #[doc = "for future use. Defaults to REFOCLK. Not recommended for use to ensure future compatibilities."]
    #[inline(always)]
    pub fn sela_7(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Sela7)
    }
}
#[doc = "Selects the BCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selb {
    #[doc = "0: LFXTCLK"]
    Selb0 = 0,
    #[doc = "1: REFOCLK"]
    Selb1 = 1,
}
impl From<Selb> for bool {
    #[inline(always)]
    fn from(variant: Selb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELB` reader - Selects the BCLK source"]
pub type SelbR = crate::BitReader<Selb>;
impl SelbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selb {
        match self.bits {
            false => Selb::Selb0,
            true => Selb::Selb1,
        }
    }
    #[doc = "LFXTCLK"]
    #[inline(always)]
    pub fn is_selb_0(&self) -> bool {
        *self == Selb::Selb0
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn is_selb_1(&self) -> bool {
        *self == Selb::Selb1
    }
}
#[doc = "Field `SELB` writer - Selects the BCLK source"]
pub type SelbW<'a, REG> = crate::BitWriter<'a, REG, Selb>;
impl<'a, REG> SelbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LFXTCLK"]
    #[inline(always)]
    pub fn selb_0(self) -> &'a mut crate::W<REG> {
        self.variant(Selb::Selb0)
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn selb_1(self) -> &'a mut crate::W<REG> {
        self.variant(Selb::Selb1)
    }
}
#[doc = "MCLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divm {
    #[doc = "0: f(MCLK)/1"]
    Divm0 = 0,
    #[doc = "1: f(MCLK)/2"]
    Divm1 = 1,
    #[doc = "2: f(MCLK)/4"]
    Divm2 = 2,
    #[doc = "3: f(MCLK)/8"]
    Divm3 = 3,
    #[doc = "4: f(MCLK)/16"]
    Divm4 = 4,
    #[doc = "5: f(MCLK)/32"]
    Divm5 = 5,
    #[doc = "6: f(MCLK)/64"]
    Divm6 = 6,
    #[doc = "7: f(MCLK)/128"]
    Divm7 = 7,
}
impl From<Divm> for u8 {
    #[inline(always)]
    fn from(variant: Divm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divm {
    type Ux = u8;
}
impl crate::IsEnum for Divm {}
#[doc = "Field `DIVM` reader - MCLK source divider"]
pub type DivmR = crate::FieldReader<Divm>;
impl DivmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divm {
        match self.bits {
            0 => Divm::Divm0,
            1 => Divm::Divm1,
            2 => Divm::Divm2,
            3 => Divm::Divm3,
            4 => Divm::Divm4,
            5 => Divm::Divm5,
            6 => Divm::Divm6,
            7 => Divm::Divm7,
            _ => unreachable!(),
        }
    }
    #[doc = "f(MCLK)/1"]
    #[inline(always)]
    pub fn is_divm_0(&self) -> bool {
        *self == Divm::Divm0
    }
    #[doc = "f(MCLK)/2"]
    #[inline(always)]
    pub fn is_divm_1(&self) -> bool {
        *self == Divm::Divm1
    }
    #[doc = "f(MCLK)/4"]
    #[inline(always)]
    pub fn is_divm_2(&self) -> bool {
        *self == Divm::Divm2
    }
    #[doc = "f(MCLK)/8"]
    #[inline(always)]
    pub fn is_divm_3(&self) -> bool {
        *self == Divm::Divm3
    }
    #[doc = "f(MCLK)/16"]
    #[inline(always)]
    pub fn is_divm_4(&self) -> bool {
        *self == Divm::Divm4
    }
    #[doc = "f(MCLK)/32"]
    #[inline(always)]
    pub fn is_divm_5(&self) -> bool {
        *self == Divm::Divm5
    }
    #[doc = "f(MCLK)/64"]
    #[inline(always)]
    pub fn is_divm_6(&self) -> bool {
        *self == Divm::Divm6
    }
    #[doc = "f(MCLK)/128"]
    #[inline(always)]
    pub fn is_divm_7(&self) -> bool {
        *self == Divm::Divm7
    }
}
#[doc = "Field `DIVM` writer - MCLK source divider"]
pub type DivmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Divm, crate::Safe>;
impl<'a, REG> DivmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "f(MCLK)/1"]
    #[inline(always)]
    pub fn divm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm0)
    }
    #[doc = "f(MCLK)/2"]
    #[inline(always)]
    pub fn divm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm1)
    }
    #[doc = "f(MCLK)/4"]
    #[inline(always)]
    pub fn divm_2(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm2)
    }
    #[doc = "f(MCLK)/8"]
    #[inline(always)]
    pub fn divm_3(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm3)
    }
    #[doc = "f(MCLK)/16"]
    #[inline(always)]
    pub fn divm_4(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm4)
    }
    #[doc = "f(MCLK)/32"]
    #[inline(always)]
    pub fn divm_5(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm5)
    }
    #[doc = "f(MCLK)/64"]
    #[inline(always)]
    pub fn divm_6(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm6)
    }
    #[doc = "f(MCLK)/128"]
    #[inline(always)]
    pub fn divm_7(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Divm7)
    }
}
#[doc = "HSMCLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divhs {
    #[doc = "0: f(HSMCLK)/1"]
    Divhs0 = 0,
    #[doc = "1: f(HSMCLK)/2"]
    Divhs1 = 1,
    #[doc = "2: f(HSMCLK)/4"]
    Divhs2 = 2,
    #[doc = "3: f(HSMCLK)/8"]
    Divhs3 = 3,
    #[doc = "4: f(HSMCLK)/16"]
    Divhs4 = 4,
    #[doc = "5: f(HSMCLK)/32"]
    Divhs5 = 5,
    #[doc = "6: f(HSMCLK)/64"]
    Divhs6 = 6,
    #[doc = "7: f(HSMCLK)/128"]
    Divhs7 = 7,
}
impl From<Divhs> for u8 {
    #[inline(always)]
    fn from(variant: Divhs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divhs {
    type Ux = u8;
}
impl crate::IsEnum for Divhs {}
#[doc = "Field `DIVHS` reader - HSMCLK source divider"]
pub type DivhsR = crate::FieldReader<Divhs>;
impl DivhsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divhs {
        match self.bits {
            0 => Divhs::Divhs0,
            1 => Divhs::Divhs1,
            2 => Divhs::Divhs2,
            3 => Divhs::Divhs3,
            4 => Divhs::Divhs4,
            5 => Divhs::Divhs5,
            6 => Divhs::Divhs6,
            7 => Divhs::Divhs7,
            _ => unreachable!(),
        }
    }
    #[doc = "f(HSMCLK)/1"]
    #[inline(always)]
    pub fn is_divhs_0(&self) -> bool {
        *self == Divhs::Divhs0
    }
    #[doc = "f(HSMCLK)/2"]
    #[inline(always)]
    pub fn is_divhs_1(&self) -> bool {
        *self == Divhs::Divhs1
    }
    #[doc = "f(HSMCLK)/4"]
    #[inline(always)]
    pub fn is_divhs_2(&self) -> bool {
        *self == Divhs::Divhs2
    }
    #[doc = "f(HSMCLK)/8"]
    #[inline(always)]
    pub fn is_divhs_3(&self) -> bool {
        *self == Divhs::Divhs3
    }
    #[doc = "f(HSMCLK)/16"]
    #[inline(always)]
    pub fn is_divhs_4(&self) -> bool {
        *self == Divhs::Divhs4
    }
    #[doc = "f(HSMCLK)/32"]
    #[inline(always)]
    pub fn is_divhs_5(&self) -> bool {
        *self == Divhs::Divhs5
    }
    #[doc = "f(HSMCLK)/64"]
    #[inline(always)]
    pub fn is_divhs_6(&self) -> bool {
        *self == Divhs::Divhs6
    }
    #[doc = "f(HSMCLK)/128"]
    #[inline(always)]
    pub fn is_divhs_7(&self) -> bool {
        *self == Divhs::Divhs7
    }
}
#[doc = "Field `DIVHS` writer - HSMCLK source divider"]
pub type DivhsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Divhs, crate::Safe>;
impl<'a, REG> DivhsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "f(HSMCLK)/1"]
    #[inline(always)]
    pub fn divhs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Divhs::Divhs0)
    }
    #[doc = "f(HSMCLK)/2"]
    #[inline(always)]
    pub fn divhs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Divhs::Divhs1)
    }
    #[doc = "f(HSMCLK)/4"]
    #[inline(always)]
    pub fn divhs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Divhs::Divhs2)
    }
    #[doc = "f(HSMCLK)/8"]
    #[inline(always)]
    pub fn divhs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Divhs::Divhs3)
    }
    #[doc = "f(HSMCLK)/16"]
    #[inline(always)]
    pub fn divhs_4(self) -> &'a mut crate::W<REG> {
        self.variant(Divhs::Divhs4)
    }
    #[doc = "f(HSMCLK)/32"]
    #[inline(always)]
    pub fn divhs_5(self) -> &'a mut crate::W<REG> {
        self.variant(Divhs::Divhs5)
    }
    #[doc = "f(HSMCLK)/64"]
    #[inline(always)]
    pub fn divhs_6(self) -> &'a mut crate::W<REG> {
        self.variant(Divhs::Divhs6)
    }
    #[doc = "f(HSMCLK)/128"]
    #[inline(always)]
    pub fn divhs_7(self) -> &'a mut crate::W<REG> {
        self.variant(Divhs::Divhs7)
    }
}
#[doc = "ACLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diva {
    #[doc = "0: f(ACLK)/1"]
    Diva0 = 0,
    #[doc = "1: f(ACLK)/2"]
    Diva1 = 1,
    #[doc = "2: f(ACLK)/4"]
    Diva2 = 2,
    #[doc = "3: f(ACLK)/8"]
    Diva3 = 3,
    #[doc = "4: f(ACLK)/16"]
    Diva4 = 4,
    #[doc = "5: f(ACLK)/32"]
    Diva5 = 5,
    #[doc = "6: f(ACLK)/64"]
    Diva6 = 6,
    #[doc = "7: f(ACLK)/128"]
    Diva7 = 7,
}
impl From<Diva> for u8 {
    #[inline(always)]
    fn from(variant: Diva) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diva {
    type Ux = u8;
}
impl crate::IsEnum for Diva {}
#[doc = "Field `DIVA` reader - ACLK source divider"]
pub type DivaR = crate::FieldReader<Diva>;
impl DivaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diva {
        match self.bits {
            0 => Diva::Diva0,
            1 => Diva::Diva1,
            2 => Diva::Diva2,
            3 => Diva::Diva3,
            4 => Diva::Diva4,
            5 => Diva::Diva5,
            6 => Diva::Diva6,
            7 => Diva::Diva7,
            _ => unreachable!(),
        }
    }
    #[doc = "f(ACLK)/1"]
    #[inline(always)]
    pub fn is_diva_0(&self) -> bool {
        *self == Diva::Diva0
    }
    #[doc = "f(ACLK)/2"]
    #[inline(always)]
    pub fn is_diva_1(&self) -> bool {
        *self == Diva::Diva1
    }
    #[doc = "f(ACLK)/4"]
    #[inline(always)]
    pub fn is_diva_2(&self) -> bool {
        *self == Diva::Diva2
    }
    #[doc = "f(ACLK)/8"]
    #[inline(always)]
    pub fn is_diva_3(&self) -> bool {
        *self == Diva::Diva3
    }
    #[doc = "f(ACLK)/16"]
    #[inline(always)]
    pub fn is_diva_4(&self) -> bool {
        *self == Diva::Diva4
    }
    #[doc = "f(ACLK)/32"]
    #[inline(always)]
    pub fn is_diva_5(&self) -> bool {
        *self == Diva::Diva5
    }
    #[doc = "f(ACLK)/64"]
    #[inline(always)]
    pub fn is_diva_6(&self) -> bool {
        *self == Diva::Diva6
    }
    #[doc = "f(ACLK)/128"]
    #[inline(always)]
    pub fn is_diva_7(&self) -> bool {
        *self == Diva::Diva7
    }
}
#[doc = "Field `DIVA` writer - ACLK source divider"]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Diva, crate::Safe>;
impl<'a, REG> DivaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "f(ACLK)/1"]
    #[inline(always)]
    pub fn diva_0(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva0)
    }
    #[doc = "f(ACLK)/2"]
    #[inline(always)]
    pub fn diva_1(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva1)
    }
    #[doc = "f(ACLK)/4"]
    #[inline(always)]
    pub fn diva_2(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva2)
    }
    #[doc = "f(ACLK)/8"]
    #[inline(always)]
    pub fn diva_3(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva3)
    }
    #[doc = "f(ACLK)/16"]
    #[inline(always)]
    pub fn diva_4(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva4)
    }
    #[doc = "f(ACLK)/32"]
    #[inline(always)]
    pub fn diva_5(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva5)
    }
    #[doc = "f(ACLK)/64"]
    #[inline(always)]
    pub fn diva_6(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva6)
    }
    #[doc = "f(ACLK)/128"]
    #[inline(always)]
    pub fn diva_7(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva7)
    }
}
#[doc = "SMCLK source divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divs {
    #[doc = "0: f(SMCLK)/1"]
    Divs0 = 0,
    #[doc = "1: f(SMCLK)/2"]
    Divs1 = 1,
    #[doc = "2: f(SMCLK)/4"]
    Divs2 = 2,
    #[doc = "3: f(SMCLK)/8"]
    Divs3 = 3,
    #[doc = "4: f(SMCLK)/16"]
    Divs4 = 4,
    #[doc = "5: f(SMCLK)/32"]
    Divs5 = 5,
    #[doc = "6: f(SMCLK)/64"]
    Divs6 = 6,
    #[doc = "7: f(SMCLK)/128"]
    Divs7 = 7,
}
impl From<Divs> for u8 {
    #[inline(always)]
    fn from(variant: Divs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divs {
    type Ux = u8;
}
impl crate::IsEnum for Divs {}
#[doc = "Field `DIVS` reader - SMCLK source divider"]
pub type DivsR = crate::FieldReader<Divs>;
impl DivsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divs {
        match self.bits {
            0 => Divs::Divs0,
            1 => Divs::Divs1,
            2 => Divs::Divs2,
            3 => Divs::Divs3,
            4 => Divs::Divs4,
            5 => Divs::Divs5,
            6 => Divs::Divs6,
            7 => Divs::Divs7,
            _ => unreachable!(),
        }
    }
    #[doc = "f(SMCLK)/1"]
    #[inline(always)]
    pub fn is_divs_0(&self) -> bool {
        *self == Divs::Divs0
    }
    #[doc = "f(SMCLK)/2"]
    #[inline(always)]
    pub fn is_divs_1(&self) -> bool {
        *self == Divs::Divs1
    }
    #[doc = "f(SMCLK)/4"]
    #[inline(always)]
    pub fn is_divs_2(&self) -> bool {
        *self == Divs::Divs2
    }
    #[doc = "f(SMCLK)/8"]
    #[inline(always)]
    pub fn is_divs_3(&self) -> bool {
        *self == Divs::Divs3
    }
    #[doc = "f(SMCLK)/16"]
    #[inline(always)]
    pub fn is_divs_4(&self) -> bool {
        *self == Divs::Divs4
    }
    #[doc = "f(SMCLK)/32"]
    #[inline(always)]
    pub fn is_divs_5(&self) -> bool {
        *self == Divs::Divs5
    }
    #[doc = "f(SMCLK)/64"]
    #[inline(always)]
    pub fn is_divs_6(&self) -> bool {
        *self == Divs::Divs6
    }
    #[doc = "f(SMCLK)/128"]
    #[inline(always)]
    pub fn is_divs_7(&self) -> bool {
        *self == Divs::Divs7
    }
}
#[doc = "Field `DIVS` writer - SMCLK source divider"]
pub type DivsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Divs, crate::Safe>;
impl<'a, REG> DivsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "f(SMCLK)/1"]
    #[inline(always)]
    pub fn divs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs0)
    }
    #[doc = "f(SMCLK)/2"]
    #[inline(always)]
    pub fn divs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs1)
    }
    #[doc = "f(SMCLK)/4"]
    #[inline(always)]
    pub fn divs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs2)
    }
    #[doc = "f(SMCLK)/8"]
    #[inline(always)]
    pub fn divs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs3)
    }
    #[doc = "f(SMCLK)/16"]
    #[inline(always)]
    pub fn divs_4(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs4)
    }
    #[doc = "f(SMCLK)/32"]
    #[inline(always)]
    pub fn divs_5(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs5)
    }
    #[doc = "f(SMCLK)/64"]
    #[inline(always)]
    pub fn divs_6(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs6)
    }
    #[doc = "f(SMCLK)/128"]
    #[inline(always)]
    pub fn divs_7(self) -> &'a mut crate::W<REG> {
        self.variant(Divs::Divs7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the MCLK source"]
    #[inline(always)]
    pub fn selm(&self) -> SelmR {
        SelmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Selects the SMCLK and HSMCLK source"]
    #[inline(always)]
    pub fn sels(&self) -> SelsR {
        SelsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&self) -> SelaR {
        SelaR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Selects the BCLK source"]
    #[inline(always)]
    pub fn selb(&self) -> SelbR {
        SelbR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:18 - MCLK source divider"]
    #[inline(always)]
    pub fn divm(&self) -> DivmR {
        DivmR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - HSMCLK source divider"]
    #[inline(always)]
    pub fn divhs(&self) -> DivhsR {
        DivhsR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - ACLK source divider"]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - SMCLK source divider"]
    #[inline(always)]
    pub fn divs(&self) -> DivsR {
        DivsR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the MCLK source"]
    #[inline(always)]
    pub fn selm(&mut self) -> SelmW<Csctl1Spec> {
        SelmW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Selects the SMCLK and HSMCLK source"]
    #[inline(always)]
    pub fn sels(&mut self) -> SelsW<Csctl1Spec> {
        SelsW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&mut self) -> SelaW<Csctl1Spec> {
        SelaW::new(self, 8)
    }
    #[doc = "Bit 12 - Selects the BCLK source"]
    #[inline(always)]
    pub fn selb(&mut self) -> SelbW<Csctl1Spec> {
        SelbW::new(self, 12)
    }
    #[doc = "Bits 16:18 - MCLK source divider"]
    #[inline(always)]
    pub fn divm(&mut self) -> DivmW<Csctl1Spec> {
        DivmW::new(self, 16)
    }
    #[doc = "Bits 20:22 - HSMCLK source divider"]
    #[inline(always)]
    pub fn divhs(&mut self) -> DivhsW<Csctl1Spec> {
        DivhsW::new(self, 20)
    }
    #[doc = "Bits 24:26 - ACLK source divider"]
    #[inline(always)]
    pub fn diva(&mut self) -> DivaW<Csctl1Spec> {
        DivaW::new(self, 24)
    }
    #[doc = "Bits 28:30 - SMCLK source divider"]
    #[inline(always)]
    pub fn divs(&mut self) -> DivsW<Csctl1Spec> {
        DivsW::new(self, 28)
    }
}
#[doc = "Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl1Spec;
impl crate::RegisterSpec for Csctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csctl1::R`](R) reader structure"]
impl crate::Readable for Csctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl1::W`](W) writer structure"]
impl crate::Writable for Csctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSCTL1 to value 0x33"]
impl crate::Resettable for Csctl1Spec {
    const RESET_VALUE: u32 = 0x33;
}
