#[doc = "Register `FLCTL_BANK0_RDCTL` reader"]
pub type R = crate::R<FlctlBank0RdctlSpec>;
#[doc = "Register `FLCTL_BANK0_RDCTL` writer"]
pub type W = crate::W<FlctlBank0RdctlSpec>;
#[doc = "Flash read mode control setting for Bank 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RdMode {
    #[doc = "0: Normal read mode"]
    RdMode0 = 0,
    #[doc = "1: Read Margin 0"]
    RdMode1 = 1,
    #[doc = "2: Read Margin 1"]
    RdMode2 = 2,
    #[doc = "3: Program Verify"]
    RdMode3 = 3,
    #[doc = "4: Erase Verify"]
    RdMode4 = 4,
    #[doc = "5: Leakage Verify"]
    RdMode5 = 5,
    #[doc = "9: Read Margin 0B"]
    RdMode9 = 9,
    #[doc = "10: Read Margin 1B"]
    RdMode10 = 10,
}
impl From<RdMode> for u8 {
    #[inline(always)]
    fn from(variant: RdMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RdMode {
    type Ux = u8;
}
impl crate::IsEnum for RdMode {}
#[doc = "Field `RD_MODE` reader - Flash read mode control setting for Bank 0"]
pub type RdModeR = crate::FieldReader<RdMode>;
impl RdModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RdMode> {
        match self.bits {
            0 => Some(RdMode::RdMode0),
            1 => Some(RdMode::RdMode1),
            2 => Some(RdMode::RdMode2),
            3 => Some(RdMode::RdMode3),
            4 => Some(RdMode::RdMode4),
            5 => Some(RdMode::RdMode5),
            9 => Some(RdMode::RdMode9),
            10 => Some(RdMode::RdMode10),
            _ => None,
        }
    }
    #[doc = "Normal read mode"]
    #[inline(always)]
    pub fn is_rd_mode_0(&self) -> bool {
        *self == RdMode::RdMode0
    }
    #[doc = "Read Margin 0"]
    #[inline(always)]
    pub fn is_rd_mode_1(&self) -> bool {
        *self == RdMode::RdMode1
    }
    #[doc = "Read Margin 1"]
    #[inline(always)]
    pub fn is_rd_mode_2(&self) -> bool {
        *self == RdMode::RdMode2
    }
    #[doc = "Program Verify"]
    #[inline(always)]
    pub fn is_rd_mode_3(&self) -> bool {
        *self == RdMode::RdMode3
    }
    #[doc = "Erase Verify"]
    #[inline(always)]
    pub fn is_rd_mode_4(&self) -> bool {
        *self == RdMode::RdMode4
    }
    #[doc = "Leakage Verify"]
    #[inline(always)]
    pub fn is_rd_mode_5(&self) -> bool {
        *self == RdMode::RdMode5
    }
    #[doc = "Read Margin 0B"]
    #[inline(always)]
    pub fn is_rd_mode_9(&self) -> bool {
        *self == RdMode::RdMode9
    }
    #[doc = "Read Margin 1B"]
    #[inline(always)]
    pub fn is_rd_mode_10(&self) -> bool {
        *self == RdMode::RdMode10
    }
}
#[doc = "Field `RD_MODE` writer - Flash read mode control setting for Bank 0"]
pub type RdModeW<'a, REG> = crate::FieldWriter<'a, REG, 4, RdMode>;
impl<'a, REG> RdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal read mode"]
    #[inline(always)]
    pub fn rd_mode_0(self) -> &'a mut crate::W<REG> {
        self.variant(RdMode::RdMode0)
    }
    #[doc = "Read Margin 0"]
    #[inline(always)]
    pub fn rd_mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(RdMode::RdMode1)
    }
    #[doc = "Read Margin 1"]
    #[inline(always)]
    pub fn rd_mode_2(self) -> &'a mut crate::W<REG> {
        self.variant(RdMode::RdMode2)
    }
    #[doc = "Program Verify"]
    #[inline(always)]
    pub fn rd_mode_3(self) -> &'a mut crate::W<REG> {
        self.variant(RdMode::RdMode3)
    }
    #[doc = "Erase Verify"]
    #[inline(always)]
    pub fn rd_mode_4(self) -> &'a mut crate::W<REG> {
        self.variant(RdMode::RdMode4)
    }
    #[doc = "Leakage Verify"]
    #[inline(always)]
    pub fn rd_mode_5(self) -> &'a mut crate::W<REG> {
        self.variant(RdMode::RdMode5)
    }
    #[doc = "Read Margin 0B"]
    #[inline(always)]
    pub fn rd_mode_9(self) -> &'a mut crate::W<REG> {
        self.variant(RdMode::RdMode9)
    }
    #[doc = "Read Margin 1B"]
    #[inline(always)]
    pub fn rd_mode_10(self) -> &'a mut crate::W<REG> {
        self.variant(RdMode::RdMode10)
    }
}
#[doc = "Field `BUFI` reader - Enables read buffering feature for instruction fetches to this Bank"]
pub type BufiR = crate::BitReader;
#[doc = "Field `BUFI` writer - Enables read buffering feature for instruction fetches to this Bank"]
pub type BufiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFD` reader - Enables read buffering feature for data reads to this Bank"]
pub type BufdR = crate::BitReader;
#[doc = "Field `BUFD` writer - Enables read buffering feature for data reads to this Bank"]
pub type BufdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Number of wait states for read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wait {
    #[doc = "0: 0 wait states"]
    Wait0 = 0,
    #[doc = "1: 1 wait states"]
    Wait1 = 1,
    #[doc = "2: 2 wait states"]
    Wait2 = 2,
    #[doc = "3: 3 wait states"]
    Wait3 = 3,
    #[doc = "4: 4 wait states"]
    Wait4 = 4,
    #[doc = "5: 5 wait states"]
    Wait5 = 5,
    #[doc = "6: 6 wait states"]
    Wait6 = 6,
    #[doc = "7: 7 wait states"]
    Wait7 = 7,
    #[doc = "8: 8 wait states"]
    Wait8 = 8,
    #[doc = "9: 9 wait states"]
    Wait9 = 9,
    #[doc = "10: 10 wait states"]
    Wait10 = 10,
    #[doc = "11: 11 wait states"]
    Wait11 = 11,
    #[doc = "12: 12 wait states"]
    Wait12 = 12,
    #[doc = "13: 13 wait states"]
    Wait13 = 13,
    #[doc = "14: 14 wait states"]
    Wait14 = 14,
    #[doc = "15: 15 wait states"]
    Wait15 = 15,
}
impl From<Wait> for u8 {
    #[inline(always)]
    fn from(variant: Wait) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wait {
    type Ux = u8;
}
impl crate::IsEnum for Wait {}
#[doc = "Field `WAIT` reader - Number of wait states for read"]
pub type WaitR = crate::FieldReader<Wait>;
impl WaitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wait {
        match self.bits {
            0 => Wait::Wait0,
            1 => Wait::Wait1,
            2 => Wait::Wait2,
            3 => Wait::Wait3,
            4 => Wait::Wait4,
            5 => Wait::Wait5,
            6 => Wait::Wait6,
            7 => Wait::Wait7,
            8 => Wait::Wait8,
            9 => Wait::Wait9,
            10 => Wait::Wait10,
            11 => Wait::Wait11,
            12 => Wait::Wait12,
            13 => Wait::Wait13,
            14 => Wait::Wait14,
            15 => Wait::Wait15,
            _ => unreachable!(),
        }
    }
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn is_wait_0(&self) -> bool {
        *self == Wait::Wait0
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn is_wait_1(&self) -> bool {
        *self == Wait::Wait1
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn is_wait_2(&self) -> bool {
        *self == Wait::Wait2
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn is_wait_3(&self) -> bool {
        *self == Wait::Wait3
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn is_wait_4(&self) -> bool {
        *self == Wait::Wait4
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn is_wait_5(&self) -> bool {
        *self == Wait::Wait5
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn is_wait_6(&self) -> bool {
        *self == Wait::Wait6
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn is_wait_7(&self) -> bool {
        *self == Wait::Wait7
    }
    #[doc = "8 wait states"]
    #[inline(always)]
    pub fn is_wait_8(&self) -> bool {
        *self == Wait::Wait8
    }
    #[doc = "9 wait states"]
    #[inline(always)]
    pub fn is_wait_9(&self) -> bool {
        *self == Wait::Wait9
    }
    #[doc = "10 wait states"]
    #[inline(always)]
    pub fn is_wait_10(&self) -> bool {
        *self == Wait::Wait10
    }
    #[doc = "11 wait states"]
    #[inline(always)]
    pub fn is_wait_11(&self) -> bool {
        *self == Wait::Wait11
    }
    #[doc = "12 wait states"]
    #[inline(always)]
    pub fn is_wait_12(&self) -> bool {
        *self == Wait::Wait12
    }
    #[doc = "13 wait states"]
    #[inline(always)]
    pub fn is_wait_13(&self) -> bool {
        *self == Wait::Wait13
    }
    #[doc = "14 wait states"]
    #[inline(always)]
    pub fn is_wait_14(&self) -> bool {
        *self == Wait::Wait14
    }
    #[doc = "15 wait states"]
    #[inline(always)]
    pub fn is_wait_15(&self) -> bool {
        *self == Wait::Wait15
    }
}
#[doc = "Field `WAIT` writer - Number of wait states for read"]
pub type WaitW<'a, REG> = crate::FieldWriter<'a, REG, 4, Wait, crate::Safe>;
impl<'a, REG> WaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn wait_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait0)
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn wait_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait1)
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn wait_2(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait2)
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn wait_3(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait3)
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn wait_4(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait4)
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn wait_5(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait5)
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn wait_6(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait6)
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn wait_7(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait7)
    }
    #[doc = "8 wait states"]
    #[inline(always)]
    pub fn wait_8(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait8)
    }
    #[doc = "9 wait states"]
    #[inline(always)]
    pub fn wait_9(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait9)
    }
    #[doc = "10 wait states"]
    #[inline(always)]
    pub fn wait_10(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait10)
    }
    #[doc = "11 wait states"]
    #[inline(always)]
    pub fn wait_11(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait11)
    }
    #[doc = "12 wait states"]
    #[inline(always)]
    pub fn wait_12(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait12)
    }
    #[doc = "13 wait states"]
    #[inline(always)]
    pub fn wait_13(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait13)
    }
    #[doc = "14 wait states"]
    #[inline(always)]
    pub fn wait_14(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait14)
    }
    #[doc = "15 wait states"]
    #[inline(always)]
    pub fn wait_15(self) -> &'a mut crate::W<REG> {
        self.variant(Wait::Wait15)
    }
}
#[doc = "Read mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RdModeStatus {
    #[doc = "0: Normal read mode"]
    RdModeStatus0 = 0,
    #[doc = "1: Read Margin 0"]
    RdModeStatus1 = 1,
    #[doc = "2: Read Margin 1"]
    RdModeStatus2 = 2,
    #[doc = "3: Program Verify"]
    RdModeStatus3 = 3,
    #[doc = "4: Erase Verify"]
    RdModeStatus4 = 4,
    #[doc = "5: Leakage Verify"]
    RdModeStatus5 = 5,
    #[doc = "9: Read Margin 0B"]
    RdModeStatus9 = 9,
    #[doc = "10: Read Margin 1B"]
    RdModeStatus10 = 10,
}
impl From<RdModeStatus> for u8 {
    #[inline(always)]
    fn from(variant: RdModeStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RdModeStatus {
    type Ux = u8;
}
impl crate::IsEnum for RdModeStatus {}
#[doc = "Field `RD_MODE_STATUS` reader - Read mode"]
pub type RdModeStatusR = crate::FieldReader<RdModeStatus>;
impl RdModeStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RdModeStatus> {
        match self.bits {
            0 => Some(RdModeStatus::RdModeStatus0),
            1 => Some(RdModeStatus::RdModeStatus1),
            2 => Some(RdModeStatus::RdModeStatus2),
            3 => Some(RdModeStatus::RdModeStatus3),
            4 => Some(RdModeStatus::RdModeStatus4),
            5 => Some(RdModeStatus::RdModeStatus5),
            9 => Some(RdModeStatus::RdModeStatus9),
            10 => Some(RdModeStatus::RdModeStatus10),
            _ => None,
        }
    }
    #[doc = "Normal read mode"]
    #[inline(always)]
    pub fn is_rd_mode_status_0(&self) -> bool {
        *self == RdModeStatus::RdModeStatus0
    }
    #[doc = "Read Margin 0"]
    #[inline(always)]
    pub fn is_rd_mode_status_1(&self) -> bool {
        *self == RdModeStatus::RdModeStatus1
    }
    #[doc = "Read Margin 1"]
    #[inline(always)]
    pub fn is_rd_mode_status_2(&self) -> bool {
        *self == RdModeStatus::RdModeStatus2
    }
    #[doc = "Program Verify"]
    #[inline(always)]
    pub fn is_rd_mode_status_3(&self) -> bool {
        *self == RdModeStatus::RdModeStatus3
    }
    #[doc = "Erase Verify"]
    #[inline(always)]
    pub fn is_rd_mode_status_4(&self) -> bool {
        *self == RdModeStatus::RdModeStatus4
    }
    #[doc = "Leakage Verify"]
    #[inline(always)]
    pub fn is_rd_mode_status_5(&self) -> bool {
        *self == RdModeStatus::RdModeStatus5
    }
    #[doc = "Read Margin 0B"]
    #[inline(always)]
    pub fn is_rd_mode_status_9(&self) -> bool {
        *self == RdModeStatus::RdModeStatus9
    }
    #[doc = "Read Margin 1B"]
    #[inline(always)]
    pub fn is_rd_mode_status_10(&self) -> bool {
        *self == RdModeStatus::RdModeStatus10
    }
}
impl R {
    #[doc = "Bits 0:3 - Flash read mode control setting for Bank 0"]
    #[inline(always)]
    pub fn rd_mode(&self) -> RdModeR {
        RdModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enables read buffering feature for instruction fetches to this Bank"]
    #[inline(always)]
    pub fn bufi(&self) -> BufiR {
        BufiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables read buffering feature for data reads to this Bank"]
    #[inline(always)]
    pub fn bufd(&self) -> BufdR {
        BufdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Number of wait states for read"]
    #[inline(always)]
    pub fn wait(&self) -> WaitR {
        WaitR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Read mode"]
    #[inline(always)]
    pub fn rd_mode_status(&self) -> RdModeStatusR {
        RdModeStatusR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash read mode control setting for Bank 0"]
    #[inline(always)]
    pub fn rd_mode(&mut self) -> RdModeW<FlctlBank0RdctlSpec> {
        RdModeW::new(self, 0)
    }
    #[doc = "Bit 4 - Enables read buffering feature for instruction fetches to this Bank"]
    #[inline(always)]
    pub fn bufi(&mut self) -> BufiW<FlctlBank0RdctlSpec> {
        BufiW::new(self, 4)
    }
    #[doc = "Bit 5 - Enables read buffering feature for data reads to this Bank"]
    #[inline(always)]
    pub fn bufd(&mut self) -> BufdW<FlctlBank0RdctlSpec> {
        BufdW::new(self, 5)
    }
    #[doc = "Bits 12:15 - Number of wait states for read"]
    #[inline(always)]
    pub fn wait(&mut self) -> WaitW<FlctlBank0RdctlSpec> {
        WaitW::new(self, 12)
    }
}
#[doc = "Bank0 Read Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bank0_rdctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bank0_rdctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlBank0RdctlSpec;
impl crate::RegisterSpec for FlctlBank0RdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_bank0_rdctl::R`](R) reader structure"]
impl crate::Readable for FlctlBank0RdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_bank0_rdctl::W`](W) writer structure"]
impl crate::Writable for FlctlBank0RdctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_BANK0_RDCTL to value 0"]
impl crate::Resettable for FlctlBank0RdctlSpec {
    const RESET_VALUE: u32 = 0;
}
