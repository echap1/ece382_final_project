#[doc = "Register `SYS_SRAM_BANKEN` reader"]
pub type R = crate::R<SysSramBankenSpec>;
#[doc = "Register `SYS_SRAM_BANKEN` writer"]
pub type W = crate::W<SysSramBankenSpec>;
#[doc = "Field `BNK0_EN` reader - SRAM Bank0 enable"]
pub type Bnk0EnR = crate::BitReader;
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk1En {
    #[doc = "0: Disables Bank1 of the SRAM"]
    Bnk1En0 = 0,
    #[doc = "1: Enables Bank1 of the SRAM"]
    Bnk1En1 = 1,
}
impl From<Bnk1En> for bool {
    #[inline(always)]
    fn from(variant: Bnk1En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK1_EN` reader - SRAM Bank1 enable"]
pub type Bnk1EnR = crate::BitReader<Bnk1En>;
impl Bnk1EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk1En {
        match self.bits {
            false => Bnk1En::Bnk1En0,
            true => Bnk1En::Bnk1En1,
        }
    }
    #[doc = "Disables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk1_en_0(&self) -> bool {
        *self == Bnk1En::Bnk1En0
    }
    #[doc = "Enables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk1_en_1(&self) -> bool {
        *self == Bnk1En::Bnk1En1
    }
}
#[doc = "Field `BNK1_EN` writer - SRAM Bank1 enable"]
pub type Bnk1EnW<'a, REG> = crate::BitWriter<'a, REG, Bnk1En>;
impl<'a, REG> Bnk1EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn bnk1_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk1En::Bnk1En0)
    }
    #[doc = "Enables Bank1 of the SRAM"]
    #[inline(always)]
    pub fn bnk1_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk1En::Bnk1En1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk2En {
    #[doc = "0: Disables Bank2 of the SRAM"]
    Bnk2En0 = 0,
    #[doc = "1: Enables Bank2 of the SRAM"]
    Bnk2En1 = 1,
}
impl From<Bnk2En> for bool {
    #[inline(always)]
    fn from(variant: Bnk2En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK2_EN` reader - SRAM Bank1 enable"]
pub type Bnk2EnR = crate::BitReader<Bnk2En>;
impl Bnk2EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk2En {
        match self.bits {
            false => Bnk2En::Bnk2En0,
            true => Bnk2En::Bnk2En1,
        }
    }
    #[doc = "Disables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk2_en_0(&self) -> bool {
        *self == Bnk2En::Bnk2En0
    }
    #[doc = "Enables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk2_en_1(&self) -> bool {
        *self == Bnk2En::Bnk2En1
    }
}
#[doc = "Field `BNK2_EN` writer - SRAM Bank1 enable"]
pub type Bnk2EnW<'a, REG> = crate::BitWriter<'a, REG, Bnk2En>;
impl<'a, REG> Bnk2EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn bnk2_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk2En::Bnk2En0)
    }
    #[doc = "Enables Bank2 of the SRAM"]
    #[inline(always)]
    pub fn bnk2_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk2En::Bnk2En1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk3En {
    #[doc = "0: Disables Bank3 of the SRAM"]
    Bnk3En0 = 0,
    #[doc = "1: Enables Bank3 of the SRAM"]
    Bnk3En1 = 1,
}
impl From<Bnk3En> for bool {
    #[inline(always)]
    fn from(variant: Bnk3En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK3_EN` reader - SRAM Bank1 enable"]
pub type Bnk3EnR = crate::BitReader<Bnk3En>;
impl Bnk3EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk3En {
        match self.bits {
            false => Bnk3En::Bnk3En0,
            true => Bnk3En::Bnk3En1,
        }
    }
    #[doc = "Disables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk3_en_0(&self) -> bool {
        *self == Bnk3En::Bnk3En0
    }
    #[doc = "Enables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk3_en_1(&self) -> bool {
        *self == Bnk3En::Bnk3En1
    }
}
#[doc = "Field `BNK3_EN` writer - SRAM Bank1 enable"]
pub type Bnk3EnW<'a, REG> = crate::BitWriter<'a, REG, Bnk3En>;
impl<'a, REG> Bnk3EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn bnk3_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk3En::Bnk3En0)
    }
    #[doc = "Enables Bank3 of the SRAM"]
    #[inline(always)]
    pub fn bnk3_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk3En::Bnk3En1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk4En {
    #[doc = "0: Disables Bank4 of the SRAM"]
    Bnk4En0 = 0,
    #[doc = "1: Enables Bank4 of the SRAM"]
    Bnk4En1 = 1,
}
impl From<Bnk4En> for bool {
    #[inline(always)]
    fn from(variant: Bnk4En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK4_EN` reader - SRAM Bank1 enable"]
pub type Bnk4EnR = crate::BitReader<Bnk4En>;
impl Bnk4EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk4En {
        match self.bits {
            false => Bnk4En::Bnk4En0,
            true => Bnk4En::Bnk4En1,
        }
    }
    #[doc = "Disables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk4_en_0(&self) -> bool {
        *self == Bnk4En::Bnk4En0
    }
    #[doc = "Enables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk4_en_1(&self) -> bool {
        *self == Bnk4En::Bnk4En1
    }
}
#[doc = "Field `BNK4_EN` writer - SRAM Bank1 enable"]
pub type Bnk4EnW<'a, REG> = crate::BitWriter<'a, REG, Bnk4En>;
impl<'a, REG> Bnk4EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn bnk4_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk4En::Bnk4En0)
    }
    #[doc = "Enables Bank4 of the SRAM"]
    #[inline(always)]
    pub fn bnk4_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk4En::Bnk4En1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk5En {
    #[doc = "0: Disables Bank5 of the SRAM"]
    Bnk5En0 = 0,
    #[doc = "1: Enables Bank5 of the SRAM"]
    Bnk5En1 = 1,
}
impl From<Bnk5En> for bool {
    #[inline(always)]
    fn from(variant: Bnk5En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK5_EN` reader - SRAM Bank1 enable"]
pub type Bnk5EnR = crate::BitReader<Bnk5En>;
impl Bnk5EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk5En {
        match self.bits {
            false => Bnk5En::Bnk5En0,
            true => Bnk5En::Bnk5En1,
        }
    }
    #[doc = "Disables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk5_en_0(&self) -> bool {
        *self == Bnk5En::Bnk5En0
    }
    #[doc = "Enables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk5_en_1(&self) -> bool {
        *self == Bnk5En::Bnk5En1
    }
}
#[doc = "Field `BNK5_EN` writer - SRAM Bank1 enable"]
pub type Bnk5EnW<'a, REG> = crate::BitWriter<'a, REG, Bnk5En>;
impl<'a, REG> Bnk5EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn bnk5_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk5En::Bnk5En0)
    }
    #[doc = "Enables Bank5 of the SRAM"]
    #[inline(always)]
    pub fn bnk5_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk5En::Bnk5En1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk6En {
    #[doc = "0: Disables Bank6 of the SRAM"]
    Bnk6En0 = 0,
    #[doc = "1: Enables Bank6 of the SRAM"]
    Bnk6En1 = 1,
}
impl From<Bnk6En> for bool {
    #[inline(always)]
    fn from(variant: Bnk6En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK6_EN` reader - SRAM Bank1 enable"]
pub type Bnk6EnR = crate::BitReader<Bnk6En>;
impl Bnk6EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk6En {
        match self.bits {
            false => Bnk6En::Bnk6En0,
            true => Bnk6En::Bnk6En1,
        }
    }
    #[doc = "Disables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk6_en_0(&self) -> bool {
        *self == Bnk6En::Bnk6En0
    }
    #[doc = "Enables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk6_en_1(&self) -> bool {
        *self == Bnk6En::Bnk6En1
    }
}
#[doc = "Field `BNK6_EN` writer - SRAM Bank1 enable"]
pub type Bnk6EnW<'a, REG> = crate::BitWriter<'a, REG, Bnk6En>;
impl<'a, REG> Bnk6EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn bnk6_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk6En::Bnk6En0)
    }
    #[doc = "Enables Bank6 of the SRAM"]
    #[inline(always)]
    pub fn bnk6_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk6En::Bnk6En1)
    }
}
#[doc = "SRAM Bank1 enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bnk7En {
    #[doc = "0: Disables Bank7 of the SRAM"]
    Bnk7En0 = 0,
    #[doc = "1: Enables Bank7 of the SRAM"]
    Bnk7En1 = 1,
}
impl From<Bnk7En> for bool {
    #[inline(always)]
    fn from(variant: Bnk7En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK7_EN` reader - SRAM Bank1 enable"]
pub type Bnk7EnR = crate::BitReader<Bnk7En>;
impl Bnk7EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bnk7En {
        match self.bits {
            false => Bnk7En::Bnk7En0,
            true => Bnk7En::Bnk7En1,
        }
    }
    #[doc = "Disables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk7_en_0(&self) -> bool {
        *self == Bnk7En::Bnk7En0
    }
    #[doc = "Enables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn is_bnk7_en_1(&self) -> bool {
        *self == Bnk7En::Bnk7En1
    }
}
#[doc = "Field `BNK7_EN` writer - SRAM Bank1 enable"]
pub type Bnk7EnW<'a, REG> = crate::BitWriter<'a, REG, Bnk7En>;
impl<'a, REG> Bnk7EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn bnk7_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk7En::Bnk7En0)
    }
    #[doc = "Enables Bank7 of the SRAM"]
    #[inline(always)]
    pub fn bnk7_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bnk7En::Bnk7En1)
    }
}
#[doc = "SRAM ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramRdyEnumRead {
    #[doc = "0: SRAM is not ready for accesses. Banks are undergoing an enable or disable sequence, and reads or writes to SRAM are stalled until the banks are ready"]
    SramRdy0 = 0,
    #[doc = "1: SRAM is ready for accesses. All SRAM banks are enabled/disabled according to values of bits 7:0 of this register"]
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
    #[doc = "SRAM is not ready for accesses. Banks are undergoing an enable or disable sequence, and reads or writes to SRAM are stalled until the banks are ready"]
    #[inline(always)]
    pub fn is_sram_rdy_0(&self) -> bool {
        *self == SramRdyEnumRead::SramRdy0
    }
    #[doc = "SRAM is ready for accesses. All SRAM banks are enabled/disabled according to values of bits 7:0 of this register"]
    #[inline(always)]
    pub fn is_sram_rdy_1(&self) -> bool {
        *self == SramRdyEnumRead::SramRdy1
    }
}
impl R {
    #[doc = "Bit 0 - SRAM Bank0 enable"]
    #[inline(always)]
    pub fn bnk0_en(&self) -> Bnk0EnR {
        Bnk0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk1_en(&self) -> Bnk1EnR {
        Bnk1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk2_en(&self) -> Bnk2EnR {
        Bnk2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk3_en(&self) -> Bnk3EnR {
        Bnk3EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk4_en(&self) -> Bnk4EnR {
        Bnk4EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk5_en(&self) -> Bnk5EnR {
        Bnk5EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk6_en(&self) -> Bnk6EnR {
        Bnk6EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk7_en(&self) -> Bnk7EnR {
        Bnk7EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM ready"]
    #[inline(always)]
    pub fn sram_rdy(&self) -> SramRdyR {
        SramRdyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk1_en(&mut self) -> Bnk1EnW<SysSramBankenSpec> {
        Bnk1EnW::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk2_en(&mut self) -> Bnk2EnW<SysSramBankenSpec> {
        Bnk2EnW::new(self, 2)
    }
    #[doc = "Bit 3 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk3_en(&mut self) -> Bnk3EnW<SysSramBankenSpec> {
        Bnk3EnW::new(self, 3)
    }
    #[doc = "Bit 4 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk4_en(&mut self) -> Bnk4EnW<SysSramBankenSpec> {
        Bnk4EnW::new(self, 4)
    }
    #[doc = "Bit 5 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk5_en(&mut self) -> Bnk5EnW<SysSramBankenSpec> {
        Bnk5EnW::new(self, 5)
    }
    #[doc = "Bit 6 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk6_en(&mut self) -> Bnk6EnW<SysSramBankenSpec> {
        Bnk6EnW::new(self, 6)
    }
    #[doc = "Bit 7 - SRAM Bank1 enable"]
    #[inline(always)]
    pub fn bnk7_en(&mut self) -> Bnk7EnW<SysSramBankenSpec> {
        Bnk7EnW::new(self, 7)
    }
}
#[doc = "SRAM Bank Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sram_banken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_sram_banken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSramBankenSpec;
impl crate::RegisterSpec for SysSramBankenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sram_banken::R`](R) reader structure"]
impl crate::Readable for SysSramBankenSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_sram_banken::W`](W) writer structure"]
impl crate::Writable for SysSramBankenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_SRAM_BANKEN to value 0xff"]
impl crate::Resettable for SysSramBankenSpec {
    const RESET_VALUE: u32 = 0xff;
}
