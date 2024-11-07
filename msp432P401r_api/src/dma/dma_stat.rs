#[doc = "Register `DMA_STAT` reader"]
pub type R = crate::R<DmaStatSpec>;
#[doc = "Enable status of the controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MastenEnumRead {
    #[doc = "0: Controller disabled"]
    Masten0 = 0,
    #[doc = "1: Controller enabled"]
    Masten1 = 1,
}
impl From<MastenEnumRead> for bool {
    #[inline(always)]
    fn from(variant: MastenEnumRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTEN` reader - Enable status of the controller"]
pub type MastenR = crate::BitReader<MastenEnumRead>;
impl MastenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MastenEnumRead {
        match self.bits {
            false => MastenEnumRead::Masten0,
            true => MastenEnumRead::Masten1,
        }
    }
    #[doc = "Controller disabled"]
    #[inline(always)]
    pub fn is_masten_0(&self) -> bool {
        *self == MastenEnumRead::Masten0
    }
    #[doc = "Controller enabled"]
    #[inline(always)]
    pub fn is_masten_1(&self) -> bool {
        *self == MastenEnumRead::Masten1
    }
}
#[doc = "Current state of the control state machine. State can be one of the following:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StateEnumRead {
    #[doc = "0: idle"]
    State0 = 0,
    #[doc = "1: reading channel controller data"]
    State1 = 1,
    #[doc = "2: reading source data end pointer"]
    State2 = 2,
    #[doc = "3: reading destination data end pointer"]
    State3 = 3,
    #[doc = "4: reading source data"]
    State4 = 4,
    #[doc = "5: writing destination data"]
    State5 = 5,
    #[doc = "6: waiting for DMA request to clear"]
    State6 = 6,
    #[doc = "7: writing channel controller data"]
    State7 = 7,
    #[doc = "8: stalled"]
    State8 = 8,
    #[doc = "9: done"]
    State9 = 9,
    #[doc = "10: peripheral scatter-gather transition"]
    State10 = 10,
}
impl From<StateEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: StateEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StateEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for StateEnumRead {}
#[doc = "Field `STATE` reader - Current state of the control state machine. State can be one of the following:"]
pub type StateR = crate::FieldReader<StateEnumRead>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<StateEnumRead> {
        match self.bits {
            0 => Some(StateEnumRead::State0),
            1 => Some(StateEnumRead::State1),
            2 => Some(StateEnumRead::State2),
            3 => Some(StateEnumRead::State3),
            4 => Some(StateEnumRead::State4),
            5 => Some(StateEnumRead::State5),
            6 => Some(StateEnumRead::State6),
            7 => Some(StateEnumRead::State7),
            8 => Some(StateEnumRead::State8),
            9 => Some(StateEnumRead::State9),
            10 => Some(StateEnumRead::State10),
            _ => None,
        }
    }
    #[doc = "idle"]
    #[inline(always)]
    pub fn is_state_0(&self) -> bool {
        *self == StateEnumRead::State0
    }
    #[doc = "reading channel controller data"]
    #[inline(always)]
    pub fn is_state_1(&self) -> bool {
        *self == StateEnumRead::State1
    }
    #[doc = "reading source data end pointer"]
    #[inline(always)]
    pub fn is_state_2(&self) -> bool {
        *self == StateEnumRead::State2
    }
    #[doc = "reading destination data end pointer"]
    #[inline(always)]
    pub fn is_state_3(&self) -> bool {
        *self == StateEnumRead::State3
    }
    #[doc = "reading source data"]
    #[inline(always)]
    pub fn is_state_4(&self) -> bool {
        *self == StateEnumRead::State4
    }
    #[doc = "writing destination data"]
    #[inline(always)]
    pub fn is_state_5(&self) -> bool {
        *self == StateEnumRead::State5
    }
    #[doc = "waiting for DMA request to clear"]
    #[inline(always)]
    pub fn is_state_6(&self) -> bool {
        *self == StateEnumRead::State6
    }
    #[doc = "writing channel controller data"]
    #[inline(always)]
    pub fn is_state_7(&self) -> bool {
        *self == StateEnumRead::State7
    }
    #[doc = "stalled"]
    #[inline(always)]
    pub fn is_state_8(&self) -> bool {
        *self == StateEnumRead::State8
    }
    #[doc = "done"]
    #[inline(always)]
    pub fn is_state_9(&self) -> bool {
        *self == StateEnumRead::State9
    }
    #[doc = "peripheral scatter-gather transition"]
    #[inline(always)]
    pub fn is_state_10(&self) -> bool {
        *self == StateEnumRead::State10
    }
}
#[doc = "Number of available DMA channels minus one.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmachansEnumRead {
    #[doc = "0: Controller configured to use 1 DMA channel"]
    Dmachans0 = 0,
    #[doc = "1: Controller configured to use 2 DMA channels"]
    Dmachans1 = 1,
    #[doc = "30: Controller configured to use 31 DMA channels"]
    Dmachans30 = 30,
    #[doc = "31: Controller configured to use 32 DMA channels"]
    Dmachans31 = 31,
}
impl From<DmachansEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: DmachansEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmachansEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for DmachansEnumRead {}
#[doc = "Field `DMACHANS` reader - Number of available DMA channels minus one."]
pub type DmachansR = crate::FieldReader<DmachansEnumRead>;
impl DmachansR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmachansEnumRead> {
        match self.bits {
            0 => Some(DmachansEnumRead::Dmachans0),
            1 => Some(DmachansEnumRead::Dmachans1),
            30 => Some(DmachansEnumRead::Dmachans30),
            31 => Some(DmachansEnumRead::Dmachans31),
            _ => None,
        }
    }
    #[doc = "Controller configured to use 1 DMA channel"]
    #[inline(always)]
    pub fn is_dmachans_0(&self) -> bool {
        *self == DmachansEnumRead::Dmachans0
    }
    #[doc = "Controller configured to use 2 DMA channels"]
    #[inline(always)]
    pub fn is_dmachans_1(&self) -> bool {
        *self == DmachansEnumRead::Dmachans1
    }
    #[doc = "Controller configured to use 31 DMA channels"]
    #[inline(always)]
    pub fn is_dmachans_30(&self) -> bool {
        *self == DmachansEnumRead::Dmachans30
    }
    #[doc = "Controller configured to use 32 DMA channels"]
    #[inline(always)]
    pub fn is_dmachans_31(&self) -> bool {
        *self == DmachansEnumRead::Dmachans31
    }
}
#[doc = "To reduce the gate count the controller can be configured to exclude the integration test logic. The values 2h to Fh are Reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TeststatEnumRead {
    #[doc = "0: Controller does not include the integration test logic"]
    Teststat0 = 0,
    #[doc = "1: Controller includes the integration test logic"]
    Teststat1 = 1,
}
impl From<TeststatEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: TeststatEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TeststatEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for TeststatEnumRead {}
#[doc = "Field `TESTSTAT` reader - To reduce the gate count the controller can be configured to exclude the integration test logic. The values 2h to Fh are Reserved."]
pub type TeststatR = crate::FieldReader<TeststatEnumRead>;
impl TeststatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TeststatEnumRead> {
        match self.bits {
            0 => Some(TeststatEnumRead::Teststat0),
            1 => Some(TeststatEnumRead::Teststat1),
            _ => None,
        }
    }
    #[doc = "Controller does not include the integration test logic"]
    #[inline(always)]
    pub fn is_teststat_0(&self) -> bool {
        *self == TeststatEnumRead::Teststat0
    }
    #[doc = "Controller includes the integration test logic"]
    #[inline(always)]
    pub fn is_teststat_1(&self) -> bool {
        *self == TeststatEnumRead::Teststat1
    }
}
impl R {
    #[doc = "Bit 0 - Enable status of the controller"]
    #[inline(always)]
    pub fn masten(&self) -> MastenR {
        MastenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Current state of the control state machine. State can be one of the following:"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Number of available DMA channels minus one."]
    #[inline(always)]
    pub fn dmachans(&self) -> DmachansR {
        DmachansR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - To reduce the gate count the controller can be configured to exclude the integration test logic. The values 2h to Fh are Reserved."]
    #[inline(always)]
    pub fn teststat(&self) -> TeststatR {
        TeststatR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaStatSpec;
impl crate::RegisterSpec for DmaStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_stat::R`](R) reader structure"]
impl crate::Readable for DmaStatSpec {}
#[doc = "`reset()` method sets DMA_STAT to value 0"]
impl crate::Resettable for DmaStatSpec {
    const RESET_VALUE: u32 = 0;
}
