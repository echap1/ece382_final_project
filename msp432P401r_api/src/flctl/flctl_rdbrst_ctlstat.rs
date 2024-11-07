#[doc = "Register `FLCTL_RDBRST_CTLSTAT` reader"]
pub type R = crate::R<FlctlRdbrstCtlstatSpec>;
#[doc = "Register `FLCTL_RDBRST_CTLSTAT` writer"]
pub type W = crate::W<FlctlRdbrstCtlstatSpec>;
#[doc = "Field `START` writer - Start of burst/compare operation"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Type of memory that burst is carried out on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MemType {
    #[doc = "0: Main Memory"]
    MemType0 = 0,
    #[doc = "1: Information Memory"]
    MemType1 = 1,
    #[doc = "3: Engineering Memory"]
    MemType3 = 3,
}
impl From<MemType> for u8 {
    #[inline(always)]
    fn from(variant: MemType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MemType {
    type Ux = u8;
}
impl crate::IsEnum for MemType {}
#[doc = "Field `MEM_TYPE` reader - Type of memory that burst is carried out on"]
pub type MemTypeR = crate::FieldReader<MemType>;
impl MemTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MemType> {
        match self.bits {
            0 => Some(MemType::MemType0),
            1 => Some(MemType::MemType1),
            3 => Some(MemType::MemType3),
            _ => None,
        }
    }
    #[doc = "Main Memory"]
    #[inline(always)]
    pub fn is_mem_type_0(&self) -> bool {
        *self == MemType::MemType0
    }
    #[doc = "Information Memory"]
    #[inline(always)]
    pub fn is_mem_type_1(&self) -> bool {
        *self == MemType::MemType1
    }
    #[doc = "Engineering Memory"]
    #[inline(always)]
    pub fn is_mem_type_3(&self) -> bool {
        *self == MemType::MemType3
    }
}
#[doc = "Field `MEM_TYPE` writer - Type of memory that burst is carried out on"]
pub type MemTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, MemType>;
impl<'a, REG> MemTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main Memory"]
    #[inline(always)]
    pub fn mem_type_0(self) -> &'a mut crate::W<REG> {
        self.variant(MemType::MemType0)
    }
    #[doc = "Information Memory"]
    #[inline(always)]
    pub fn mem_type_1(self) -> &'a mut crate::W<REG> {
        self.variant(MemType::MemType1)
    }
    #[doc = "Engineering Memory"]
    #[inline(always)]
    pub fn mem_type_3(self) -> &'a mut crate::W<REG> {
        self.variant(MemType::MemType3)
    }
}
#[doc = "Field `STOP_FAIL` reader - Terminate burst/compare operation"]
pub type StopFailR = crate::BitReader;
#[doc = "Field `STOP_FAIL` writer - Terminate burst/compare operation"]
pub type StopFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data pattern used for comparison against memory read data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataCmp {
    #[doc = "0: 0000_0000_0000_0000_0000_0000_0000_0000"]
    DataCmp0 = 0,
    #[doc = "1: FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF"]
    DataCmp1 = 1,
}
impl From<DataCmp> for bool {
    #[inline(always)]
    fn from(variant: DataCmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_CMP` reader - Data pattern used for comparison against memory read data"]
pub type DataCmpR = crate::BitReader<DataCmp>;
impl DataCmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataCmp {
        match self.bits {
            false => DataCmp::DataCmp0,
            true => DataCmp::DataCmp1,
        }
    }
    #[doc = "0000_0000_0000_0000_0000_0000_0000_0000"]
    #[inline(always)]
    pub fn is_data_cmp_0(&self) -> bool {
        *self == DataCmp::DataCmp0
    }
    #[doc = "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF"]
    #[inline(always)]
    pub fn is_data_cmp_1(&self) -> bool {
        *self == DataCmp::DataCmp1
    }
}
#[doc = "Field `DATA_CMP` writer - Data pattern used for comparison against memory read data"]
pub type DataCmpW<'a, REG> = crate::BitWriter<'a, REG, DataCmp>;
impl<'a, REG> DataCmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0000_0000_0000_0000_0000_0000_0000_0000"]
    #[inline(always)]
    pub fn data_cmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(DataCmp::DataCmp0)
    }
    #[doc = "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF"]
    #[inline(always)]
    pub fn data_cmp_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataCmp::DataCmp1)
    }
}
#[doc = "Field `TEST_EN` reader - Enable comparison against test data compare registers"]
pub type TestEnR = crate::BitReader;
#[doc = "Field `TEST_EN` writer - Enable comparison against test data compare registers"]
pub type TestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Status of Burst/Compare operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BrstStat {
    #[doc = "0: Idle"]
    BrstStat0 = 0,
    #[doc = "1: Burst/Compare START bit written, but operation pending"]
    BrstStat1 = 1,
    #[doc = "2: Burst/Compare in progress"]
    BrstStat2 = 2,
    #[doc = "3: Burst complete (status of completed burst remains in this state unless explicitly cleared by SW)"]
    BrstStat3 = 3,
}
impl From<BrstStat> for u8 {
    #[inline(always)]
    fn from(variant: BrstStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BrstStat {
    type Ux = u8;
}
impl crate::IsEnum for BrstStat {}
#[doc = "Field `BRST_STAT` reader - Status of Burst/Compare operation"]
pub type BrstStatR = crate::FieldReader<BrstStat>;
impl BrstStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BrstStat {
        match self.bits {
            0 => BrstStat::BrstStat0,
            1 => BrstStat::BrstStat1,
            2 => BrstStat::BrstStat2,
            3 => BrstStat::BrstStat3,
            _ => unreachable!(),
        }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_brst_stat_0(&self) -> bool {
        *self == BrstStat::BrstStat0
    }
    #[doc = "Burst/Compare START bit written, but operation pending"]
    #[inline(always)]
    pub fn is_brst_stat_1(&self) -> bool {
        *self == BrstStat::BrstStat1
    }
    #[doc = "Burst/Compare in progress"]
    #[inline(always)]
    pub fn is_brst_stat_2(&self) -> bool {
        *self == BrstStat::BrstStat2
    }
    #[doc = "Burst complete (status of completed burst remains in this state unless explicitly cleared by SW)"]
    #[inline(always)]
    pub fn is_brst_stat_3(&self) -> bool {
        *self == BrstStat::BrstStat3
    }
}
#[doc = "Field `CMP_ERR` reader - Burst/Compare Operation encountered atleast one data"]
pub type CmpErrR = crate::BitReader;
#[doc = "Field `ADDR_ERR` reader - Burst/Compare Operation was terminated due to access to"]
pub type AddrErrR = crate::BitReader;
#[doc = "Field `CLR_STAT` writer - Clear status bits 19-16 of this register"]
pub type ClrStatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:2 - Type of memory that burst is carried out on"]
    #[inline(always)]
    pub fn mem_type(&self) -> MemTypeR {
        MemTypeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Terminate burst/compare operation"]
    #[inline(always)]
    pub fn stop_fail(&self) -> StopFailR {
        StopFailR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data pattern used for comparison against memory read data"]
    #[inline(always)]
    pub fn data_cmp(&self) -> DataCmpR {
        DataCmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable comparison against test data compare registers"]
    #[inline(always)]
    pub fn test_en(&self) -> TestEnR {
        TestEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Status of Burst/Compare operation"]
    #[inline(always)]
    pub fn brst_stat(&self) -> BrstStatR {
        BrstStatR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Burst/Compare Operation encountered atleast one data"]
    #[inline(always)]
    pub fn cmp_err(&self) -> CmpErrR {
        CmpErrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Burst/Compare Operation was terminated due to access to"]
    #[inline(always)]
    pub fn addr_err(&self) -> AddrErrR {
        AddrErrR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of burst/compare operation"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<FlctlRdbrstCtlstatSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Type of memory that burst is carried out on"]
    #[inline(always)]
    pub fn mem_type(&mut self) -> MemTypeW<FlctlRdbrstCtlstatSpec> {
        MemTypeW::new(self, 1)
    }
    #[doc = "Bit 3 - Terminate burst/compare operation"]
    #[inline(always)]
    pub fn stop_fail(&mut self) -> StopFailW<FlctlRdbrstCtlstatSpec> {
        StopFailW::new(self, 3)
    }
    #[doc = "Bit 4 - Data pattern used for comparison against memory read data"]
    #[inline(always)]
    pub fn data_cmp(&mut self) -> DataCmpW<FlctlRdbrstCtlstatSpec> {
        DataCmpW::new(self, 4)
    }
    #[doc = "Bit 6 - Enable comparison against test data compare registers"]
    #[inline(always)]
    pub fn test_en(&mut self) -> TestEnW<FlctlRdbrstCtlstatSpec> {
        TestEnW::new(self, 6)
    }
    #[doc = "Bit 23 - Clear status bits 19-16 of this register"]
    #[inline(always)]
    pub fn clr_stat(&mut self) -> ClrStatW<FlctlRdbrstCtlstatSpec> {
        ClrStatW::new(self, 23)
    }
}
#[doc = "Read Burst/Compare Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_rdbrst_ctlstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_rdbrst_ctlstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlRdbrstCtlstatSpec;
impl crate::RegisterSpec for FlctlRdbrstCtlstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_rdbrst_ctlstat::R`](R) reader structure"]
impl crate::Readable for FlctlRdbrstCtlstatSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_rdbrst_ctlstat::W`](W) writer structure"]
impl crate::Writable for FlctlRdbrstCtlstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_RDBRST_CTLSTAT to value 0"]
impl crate::Resettable for FlctlRdbrstCtlstatSpec {
    const RESET_VALUE: u32 = 0;
}
