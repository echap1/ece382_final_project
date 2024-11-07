#[doc = "Register `SYS_SRAM_BANKRET` reader"]
pub type R = crate::R<SysSramBankretSpec>;
#[doc = "Register `SYS_SRAM_BANKRET` writer"]
pub type W = crate::W<SysSramBankretSpec>;
#[doc = "Field `BNK0_RET` reader - Bank0 retention"]
pub type Bnk0RetR = crate::BitReader;
#[doc = "Bank1 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk1Ret {
    #[doc = "0: Bank1 of the SRAM is not retained in LPM3 or LPM4"]
    Bnk1Ret0 = 0,
    #[doc = "1: Bank1 of the SRAM is retained in LPM3 and LPM4"]
    Bnk1Ret1 = 1,
}
impl From<Bnk1Ret> for bool {
    #[inline(always)]
    fn from(variant: Bnk1Ret) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK1_RET` reader - Bank1 retention"]
pub type Bnk1RetR = crate::BitReader<Bnk1Ret>;
impl Bnk1RetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk1Ret {
        match self.bits {
            false => Bnk1Ret::Bnk1Ret0,
            true => Bnk1Ret::Bnk1Ret1,
        }
    }
    #[doc = "Bank1 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn is_bnk1_ret_0(&self) -> bool {
        *self == Bnk1Ret::Bnk1Ret0
    }
    #[doc = "Bank1 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn is_bnk1_ret_1(&self) -> bool {
        *self == Bnk1Ret::Bnk1Ret1
    }
}
#[doc = "Field `BNK1_RET` writer - Bank1 retention"]
pub type Bnk1RetW<'a, REG> = crate::BitWriter<'a, REG, Bnk1Ret>;
impl<'a, REG> Bnk1RetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank1 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk1_ret_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk1Ret::Bnk1Ret0)
    }
    #[doc = "Bank1 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk1_ret_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk1Ret::Bnk1Ret1)
    }
}
#[doc = "Bank2 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk2Ret {
    #[doc = "0: Bank2 of the SRAM is not retained in LPM3 or LPM4"]
    Bnk2Ret0 = 0,
    #[doc = "1: Bank2 of the SRAM is retained in LPM3 and LPM4"]
    Bnk2Ret1 = 1,
}
impl From<Bnk2Ret> for bool {
    #[inline(always)]
    fn from(variant: Bnk2Ret) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK2_RET` reader - Bank2 retention"]
pub type Bnk2RetR = crate::BitReader<Bnk2Ret>;
impl Bnk2RetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk2Ret {
        match self.bits {
            false => Bnk2Ret::Bnk2Ret0,
            true => Bnk2Ret::Bnk2Ret1,
        }
    }
    #[doc = "Bank2 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn is_bnk2_ret_0(&self) -> bool {
        *self == Bnk2Ret::Bnk2Ret0
    }
    #[doc = "Bank2 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn is_bnk2_ret_1(&self) -> bool {
        *self == Bnk2Ret::Bnk2Ret1
    }
}
#[doc = "Field `BNK2_RET` writer - Bank2 retention"]
pub type Bnk2RetW<'a, REG> = crate::BitWriter<'a, REG, Bnk2Ret>;
impl<'a, REG> Bnk2RetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank2 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk2_ret_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk2Ret::Bnk2Ret0)
    }
    #[doc = "Bank2 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk2_ret_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk2Ret::Bnk2Ret1)
    }
}
#[doc = "Bank3 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk3Ret {
    #[doc = "0: Bank3 of the SRAM is not retained in LPM3 or LPM4"]
    Bnk3Ret0 = 0,
    #[doc = "1: Bank3 of the SRAM is retained in LPM3 and LPM4"]
    Bnk3Ret1 = 1,
}
impl From<Bnk3Ret> for bool {
    #[inline(always)]
    fn from(variant: Bnk3Ret) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK3_RET` reader - Bank3 retention"]
pub type Bnk3RetR = crate::BitReader<Bnk3Ret>;
impl Bnk3RetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk3Ret {
        match self.bits {
            false => Bnk3Ret::Bnk3Ret0,
            true => Bnk3Ret::Bnk3Ret1,
        }
    }
    #[doc = "Bank3 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn is_bnk3_ret_0(&self) -> bool {
        *self == Bnk3Ret::Bnk3Ret0
    }
    #[doc = "Bank3 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn is_bnk3_ret_1(&self) -> bool {
        *self == Bnk3Ret::Bnk3Ret1
    }
}
#[doc = "Field `BNK3_RET` writer - Bank3 retention"]
pub type Bnk3RetW<'a, REG> = crate::BitWriter<'a, REG, Bnk3Ret>;
impl<'a, REG> Bnk3RetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank3 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk3_ret_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk3Ret::Bnk3Ret0)
    }
    #[doc = "Bank3 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk3_ret_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk3Ret::Bnk3Ret1)
    }
}
#[doc = "Bank4 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk4Ret {
    #[doc = "0: Bank4 of the SRAM is not retained in LPM3 or LPM4"]
    Bnk4Ret0 = 0,
    #[doc = "1: Bank4 of the SRAM is retained in LPM3 and LPM4"]
    Bnk4Ret1 = 1,
}
impl From<Bnk4Ret> for bool {
    #[inline(always)]
    fn from(variant: Bnk4Ret) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK4_RET` reader - Bank4 retention"]
pub type Bnk4RetR = crate::BitReader<Bnk4Ret>;
impl Bnk4RetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk4Ret {
        match self.bits {
            false => Bnk4Ret::Bnk4Ret0,
            true => Bnk4Ret::Bnk4Ret1,
        }
    }
    #[doc = "Bank4 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn is_bnk4_ret_0(&self) -> bool {
        *self == Bnk4Ret::Bnk4Ret0
    }
    #[doc = "Bank4 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn is_bnk4_ret_1(&self) -> bool {
        *self == Bnk4Ret::Bnk4Ret1
    }
}
#[doc = "Field `BNK4_RET` writer - Bank4 retention"]
pub type Bnk4RetW<'a, REG> = crate::BitWriter<'a, REG, Bnk4Ret>;
impl<'a, REG> Bnk4RetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank4 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk4_ret_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk4Ret::Bnk4Ret0)
    }
    #[doc = "Bank4 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk4_ret_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk4Ret::Bnk4Ret1)
    }
}
#[doc = "Bank5 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk5Ret {
    #[doc = "0: Bank5 of the SRAM is not retained in LPM3 or LPM4"]
    Bnk5Ret0 = 0,
    #[doc = "1: Bank5 of the SRAM is retained in LPM3 and LPM4"]
    Bnk5Ret1 = 1,
}
impl From<Bnk5Ret> for bool {
    #[inline(always)]
    fn from(variant: Bnk5Ret) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK5_RET` reader - Bank5 retention"]
pub type Bnk5RetR = crate::BitReader<Bnk5Ret>;
impl Bnk5RetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk5Ret {
        match self.bits {
            false => Bnk5Ret::Bnk5Ret0,
            true => Bnk5Ret::Bnk5Ret1,
        }
    }
    #[doc = "Bank5 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn is_bnk5_ret_0(&self) -> bool {
        *self == Bnk5Ret::Bnk5Ret0
    }
    #[doc = "Bank5 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn is_bnk5_ret_1(&self) -> bool {
        *self == Bnk5Ret::Bnk5Ret1
    }
}
#[doc = "Field `BNK5_RET` writer - Bank5 retention"]
pub type Bnk5RetW<'a, REG> = crate::BitWriter<'a, REG, Bnk5Ret>;
impl<'a, REG> Bnk5RetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank5 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk5_ret_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk5Ret::Bnk5Ret0)
    }
    #[doc = "Bank5 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk5_ret_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk5Ret::Bnk5Ret1)
    }
}
#[doc = "Bank6 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk6Ret {
    #[doc = "0: Bank6 of the SRAM is not retained in LPM3 or LPM4"]
    Bnk6Ret0 = 0,
    #[doc = "1: Bank6 of the SRAM is retained in LPM3 and LPM4"]
    Bnk6Ret1 = 1,
}
impl From<Bnk6Ret> for bool {
    #[inline(always)]
    fn from(variant: Bnk6Ret) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK6_RET` reader - Bank6 retention"]
pub type Bnk6RetR = crate::BitReader<Bnk6Ret>;
impl Bnk6RetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk6Ret {
        match self.bits {
            false => Bnk6Ret::Bnk6Ret0,
            true => Bnk6Ret::Bnk6Ret1,
        }
    }
    #[doc = "Bank6 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn is_bnk6_ret_0(&self) -> bool {
        *self == Bnk6Ret::Bnk6Ret0
    }
    #[doc = "Bank6 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn is_bnk6_ret_1(&self) -> bool {
        *self == Bnk6Ret::Bnk6Ret1
    }
}
#[doc = "Field `BNK6_RET` writer - Bank6 retention"]
pub type Bnk6RetW<'a, REG> = crate::BitWriter<'a, REG, Bnk6Ret>;
impl<'a, REG> Bnk6RetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank6 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk6_ret_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk6Ret::Bnk6Ret0)
    }
    #[doc = "Bank6 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk6_ret_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk6Ret::Bnk6Ret1)
    }
}
#[doc = "Bank7 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk7Ret {
    #[doc = "0: Bank7 of the SRAM is not retained in LPM3 or LPM4"]
    Bnk7Ret0 = 0,
    #[doc = "1: Bank7 of the SRAM is retained in LPM3 and LPM4"]
    Bnk7Ret1 = 1,
}
impl From<Bnk7Ret> for bool {
    #[inline(always)]
    fn from(variant: Bnk7Ret) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK7_RET` reader - Bank7 retention"]
pub type Bnk7RetR = crate::BitReader<Bnk7Ret>;
impl Bnk7RetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk7Ret {
        match self.bits {
            false => Bnk7Ret::Bnk7Ret0,
            true => Bnk7Ret::Bnk7Ret1,
        }
    }
    #[doc = "Bank7 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn is_bnk7_ret_0(&self) -> bool {
        *self == Bnk7Ret::Bnk7Ret0
    }
    #[doc = "Bank7 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn is_bnk7_ret_1(&self) -> bool {
        *self == Bnk7Ret::Bnk7Ret1
    }
}
#[doc = "Field `BNK7_RET` writer - Bank7 retention"]
pub type Bnk7RetW<'a, REG> = crate::BitWriter<'a, REG, Bnk7Ret>;
impl<'a, REG> Bnk7RetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bank7 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk7_ret_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk7Ret::Bnk7Ret0)
    }
    #[doc = "Bank7 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk7_ret_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk7Ret::Bnk7Ret1)
    }
}
#[doc = "SRAM ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramRdyEnumRead {
    #[doc = "0: SRAM banks are being set up for retention. Entry into LPM3, LPM4 should not be attempted until this bit is set to 1"]
    SramRdy0 = 0,
    #[doc = "1: SRAM is ready for accesses. All SRAM banks are enabled/disabled for retention according to values of bits 7:0 of this register"]
    SramRdy1 = 1,
}
impl From<SramRdyEnumRead> for bool {
    #[inline(always)]
    fn from(variant: SramRdyEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_RDY` reader - SRAM ready"]
pub type SramRdyR = crate::BitReader<SramRdyEnumRead>;
impl SramRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramRdyEnumRead {
        match self.bits {
            false => SramRdyEnumRead::SramRdy0,
            true => SramRdyEnumRead::SramRdy1,
        }
    }
    #[doc = "SRAM banks are being set up for retention. Entry into LPM3, LPM4 should not be attempted until this bit is set to 1"]
    #[inline(always)]
    pub fn is_sram_rdy_0(&self) -> bool {
        *self == SramRdyEnumRead::SramRdy0
    }
    #[doc = "SRAM is ready for accesses. All SRAM banks are enabled/disabled for retention according to values of bits 7:0 of this register"]
    #[inline(always)]
    pub fn is_sram_rdy_1(&self) -> bool {
        *self == SramRdyEnumRead::SramRdy1
    }
}
impl R {
    #[doc = "Bit 0 - Bank0 retention"]
    #[inline(always)]
    pub fn bnk0_ret(&self) -> Bnk0RetR {
        Bnk0RetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank1 retention"]
    #[inline(always)]
    pub fn bnk1_ret(&self) -> Bnk1RetR {
        Bnk1RetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank2 retention"]
    #[inline(always)]
    pub fn bnk2_ret(&self) -> Bnk2RetR {
        Bnk2RetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank3 retention"]
    #[inline(always)]
    pub fn bnk3_ret(&self) -> Bnk3RetR {
        Bnk3RetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bank4 retention"]
    #[inline(always)]
    pub fn bnk4_ret(&self) -> Bnk4RetR {
        Bnk4RetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bank5 retention"]
    #[inline(always)]
    pub fn bnk5_ret(&self) -> Bnk5RetR {
        Bnk5RetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bank6 retention"]
    #[inline(always)]
    pub fn bnk6_ret(&self) -> Bnk6RetR {
        Bnk6RetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bank7 retention"]
    #[inline(always)]
    pub fn bnk7_ret(&self) -> Bnk7RetR {
        Bnk7RetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM ready"]
    #[inline(always)]
    pub fn sram_rdy(&self) -> SramRdyR {
        SramRdyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Bank1 retention"]
    #[inline(always)]
    pub fn bnk1_ret(&mut self) -> Bnk1RetW<SysSramBankretSpec> {
        Bnk1RetW::new(self, 1)
    }
    #[doc = "Bit 2 - Bank2 retention"]
    #[inline(always)]
    pub fn bnk2_ret(&mut self) -> Bnk2RetW<SysSramBankretSpec> {
        Bnk2RetW::new(self, 2)
    }
    #[doc = "Bit 3 - Bank3 retention"]
    #[inline(always)]
    pub fn bnk3_ret(&mut self) -> Bnk3RetW<SysSramBankretSpec> {
        Bnk3RetW::new(self, 3)
    }
    #[doc = "Bit 4 - Bank4 retention"]
    #[inline(always)]
    pub fn bnk4_ret(&mut self) -> Bnk4RetW<SysSramBankretSpec> {
        Bnk4RetW::new(self, 4)
    }
    #[doc = "Bit 5 - Bank5 retention"]
    #[inline(always)]
    pub fn bnk5_ret(&mut self) -> Bnk5RetW<SysSramBankretSpec> {
        Bnk5RetW::new(self, 5)
    }
    #[doc = "Bit 6 - Bank6 retention"]
    #[inline(always)]
    pub fn bnk6_ret(&mut self) -> Bnk6RetW<SysSramBankretSpec> {
        Bnk6RetW::new(self, 6)
    }
    #[doc = "Bit 7 - Bank7 retention"]
    #[inline(always)]
    pub fn bnk7_ret(&mut self) -> Bnk7RetW<SysSramBankretSpec> {
        Bnk7RetW::new(self, 7)
    }
}
#[doc = "SRAM Bank Retention Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sram_bankret::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_sram_bankret::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSramBankretSpec;
impl crate::RegisterSpec for SysSramBankretSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sram_bankret::R`](R) reader structure"]
impl crate::Readable for SysSramBankretSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_sram_bankret::W`](W) writer structure"]
impl crate::Writable for SysSramBankretSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_SRAM_BANKRET to value 0xff"]
impl crate::Resettable for SysSramBankretSpec {
    const RESET_VALUE: u32 = 0xff;
}
