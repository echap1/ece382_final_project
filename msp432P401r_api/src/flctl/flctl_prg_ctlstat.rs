#[doc = "Register `FLCTL_PRG_CTLSTAT` reader"]
pub type R = crate::R<FlctlPrgCtlstatSpec>;
#[doc = "Register `FLCTL_PRG_CTLSTAT` writer"]
pub type W = crate::W<FlctlPrgCtlstatSpec>;
#[doc = "Master control for all word program operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Word program operation disabled"]
    Enable0 = 0,
    #[doc = "1: Word program operation enabled"]
    Enable1 = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Master control for all word program operations"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Enable0,
            true => Enable::Enable1,
        }
    }
    #[doc = "Word program operation disabled"]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == Enable::Enable0
    }
    #[doc = "Word program operation enabled"]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == Enable::Enable1
    }
}
#[doc = "Field `ENABLE` writer - Master control for all word program operations"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Word program operation disabled"]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable0)
    }
    #[doc = "Word program operation enabled"]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable1)
    }
}
#[doc = "Write mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Write immediate mode. Starts program operation immediately on each write to the Flash"]
    Mode0 = 0,
    #[doc = "1: Full word write mode. Flash controller collates data over multiple writes to compose the full 128bit word before initiating the program operation"]
    Mode1 = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Write mode"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Mode0,
            true => Mode::Mode1,
        }
    }
    #[doc = "Write immediate mode. Starts program operation immediately on each write to the Flash"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == Mode::Mode0
    }
    #[doc = "Full word write mode. Flash controller collates data over multiple writes to compose the full 128bit word before initiating the program operation"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == Mode::Mode1
    }
}
#[doc = "Field `MODE` writer - Write mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write immediate mode. Starts program operation immediately on each write to the Flash"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode0)
    }
    #[doc = "Full word write mode. Flash controller collates data over multiple writes to compose the full 128bit word before initiating the program operation"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode1)
    }
}
#[doc = "Controls automatic pre program verify operations\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerPre {
    #[doc = "0: No pre program verification"]
    VerPre0 = 0,
    #[doc = "1: Pre verify feature automatically invoked for each write operation (irrespective of the mode)"]
    VerPre1 = 1,
}
impl From<VerPre> for bool {
    #[inline(always)]
    fn from(variant: VerPre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VER_PRE` reader - Controls automatic pre program verify operations"]
pub type VerPreR = crate::BitReader<VerPre>;
impl VerPreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VerPre {
        match self.bits {
            false => VerPre::VerPre0,
            true => VerPre::VerPre1,
        }
    }
    #[doc = "No pre program verification"]
    #[inline(always)]
    pub fn is_ver_pre_0(&self) -> bool {
        *self == VerPre::VerPre0
    }
    #[doc = "Pre verify feature automatically invoked for each write operation (irrespective of the mode)"]
    #[inline(always)]
    pub fn is_ver_pre_1(&self) -> bool {
        *self == VerPre::VerPre1
    }
}
#[doc = "Field `VER_PRE` writer - Controls automatic pre program verify operations"]
pub type VerPreW<'a, REG> = crate::BitWriter<'a, REG, VerPre>;
impl<'a, REG> VerPreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No pre program verification"]
    #[inline(always)]
    pub fn ver_pre_0(self) -> &'a mut crate::W<REG> {
        self.variant(VerPre::VerPre0)
    }
    #[doc = "Pre verify feature automatically invoked for each write operation (irrespective of the mode)"]
    #[inline(always)]
    pub fn ver_pre_1(self) -> &'a mut crate::W<REG> {
        self.variant(VerPre::VerPre1)
    }
}
#[doc = "Controls automatic post program verify operations\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerPst {
    #[doc = "0: No post program verification"]
    VerPst0 = 0,
    #[doc = "1: Post verify feature automatically invoked for each write operation (irrespective of the mode)"]
    VerPst1 = 1,
}
impl From<VerPst> for bool {
    #[inline(always)]
    fn from(variant: VerPst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VER_PST` reader - Controls automatic post program verify operations"]
pub type VerPstR = crate::BitReader<VerPst>;
impl VerPstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VerPst {
        match self.bits {
            false => VerPst::VerPst0,
            true => VerPst::VerPst1,
        }
    }
    #[doc = "No post program verification"]
    #[inline(always)]
    pub fn is_ver_pst_0(&self) -> bool {
        *self == VerPst::VerPst0
    }
    #[doc = "Post verify feature automatically invoked for each write operation (irrespective of the mode)"]
    #[inline(always)]
    pub fn is_ver_pst_1(&self) -> bool {
        *self == VerPst::VerPst1
    }
}
#[doc = "Field `VER_PST` writer - Controls automatic post program verify operations"]
pub type VerPstW<'a, REG> = crate::BitWriter<'a, REG, VerPst>;
impl<'a, REG> VerPstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No post program verification"]
    #[inline(always)]
    pub fn ver_pst_0(self) -> &'a mut crate::W<REG> {
        self.variant(VerPst::VerPst0)
    }
    #[doc = "Post verify feature automatically invoked for each write operation (irrespective of the mode)"]
    #[inline(always)]
    pub fn ver_pst_1(self) -> &'a mut crate::W<REG> {
        self.variant(VerPst::VerPst1)
    }
}
#[doc = "Status of program operations in the Flash memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    #[doc = "0: Idle (no program operation currently active)"]
    Status0 = 0,
    #[doc = "1: Single word program operation triggered, but pending"]
    Status1 = 1,
    #[doc = "2: Single word program in progress"]
    Status2 = 2,
}
impl From<Status> for u8 {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Status {
    type Ux = u8;
}
impl crate::IsEnum for Status {}
#[doc = "Field `STATUS` reader - Status of program operations in the Flash memory"]
pub type StatusR = crate::FieldReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Status> {
        match self.bits {
            0 => Some(Status::Status0),
            1 => Some(Status::Status1),
            2 => Some(Status::Status2),
            _ => None,
        }
    }
    #[doc = "Idle (no program operation currently active)"]
    #[inline(always)]
    pub fn is_status_0(&self) -> bool {
        *self == Status::Status0
    }
    #[doc = "Single word program operation triggered, but pending"]
    #[inline(always)]
    pub fn is_status_1(&self) -> bool {
        *self == Status::Status1
    }
    #[doc = "Single word program in progress"]
    #[inline(always)]
    pub fn is_status_2(&self) -> bool {
        *self == Status::Status2
    }
}
#[doc = "Bank active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BnkAct {
    #[doc = "0: Word in Bank0 being programmed"]
    BnkAct0 = 0,
    #[doc = "1: Word in Bank1 being programmed"]
    BnkAct1 = 1,
}
impl From<BnkAct> for bool {
    #[inline(always)]
    fn from(variant: BnkAct) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK_ACT` reader - Bank active"]
pub type BnkActR = crate::BitReader<BnkAct>;
impl BnkActR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BnkAct {
        match self.bits {
            false => BnkAct::BnkAct0,
            true => BnkAct::BnkAct1,
        }
    }
    #[doc = "Word in Bank0 being programmed"]
    #[inline(always)]
    pub fn is_bnk_act_0(&self) -> bool {
        *self == BnkAct::BnkAct0
    }
    #[doc = "Word in Bank1 being programmed"]
    #[inline(always)]
    pub fn is_bnk_act_1(&self) -> bool {
        *self == BnkAct::BnkAct1
    }
}
impl R {
    #[doc = "Bit 0 - Master control for all word program operations"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controls automatic pre program verify operations"]
    #[inline(always)]
    pub fn ver_pre(&self) -> VerPreR {
        VerPreR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls automatic post program verify operations"]
    #[inline(always)]
    pub fn ver_pst(&self) -> VerPstR {
        VerPstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Status of program operations in the Flash memory"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Bank active"]
    #[inline(always)]
    pub fn bnk_act(&self) -> BnkActR {
        BnkActR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master control for all word program operations"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<FlctlPrgCtlstatSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Write mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<FlctlPrgCtlstatSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Controls automatic pre program verify operations"]
    #[inline(always)]
    pub fn ver_pre(&mut self) -> VerPreW<FlctlPrgCtlstatSpec> {
        VerPreW::new(self, 2)
    }
    #[doc = "Bit 3 - Controls automatic post program verify operations"]
    #[inline(always)]
    pub fn ver_pst(&mut self) -> VerPstW<FlctlPrgCtlstatSpec> {
        VerPstW::new(self, 3)
    }
}
#[doc = "Program Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prg_ctlstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prg_ctlstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlPrgCtlstatSpec;
impl crate::RegisterSpec for FlctlPrgCtlstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_prg_ctlstat::R`](R) reader structure"]
impl crate::Readable for FlctlPrgCtlstatSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_prg_ctlstat::W`](W) writer structure"]
impl crate::Writable for FlctlPrgCtlstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_PRG_CTLSTAT to value 0x0c"]
impl crate::Resettable for FlctlPrgCtlstatSpec {
    const RESET_VALUE: u32 = 0x0c;
}
