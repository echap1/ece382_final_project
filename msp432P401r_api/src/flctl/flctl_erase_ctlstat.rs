#[doc = "Register `FLCTL_ERASE_CTLSTAT` reader"]
pub type R = crate::R<FlctlEraseCtlstatSpec>;
#[doc = "Register `FLCTL_ERASE_CTLSTAT` writer"]
pub type W = crate::W<FlctlEraseCtlstatSpec>;
#[doc = "Field `START` writer - Start of Erase operation"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Erase mode selected by application\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Sector Erase (controlled by FLTCTL_ERASE_SECTADDR)"]
    Mode0 = 0,
    #[doc = "1: Mass Erase (includes all Main and Information memory sectors that don't have corresponding WE bits set)"]
    Mode1 = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Erase mode selected by application"]
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
    #[doc = "Sector Erase (controlled by FLTCTL_ERASE_SECTADDR)"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == Mode::Mode0
    }
    #[doc = "Mass Erase (includes all Main and Information memory sectors that don't have corresponding WE bits set)"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == Mode::Mode1
    }
}
#[doc = "Field `MODE` writer - Erase mode selected by application"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sector Erase (controlled by FLTCTL_ERASE_SECTADDR)"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode0)
    }
    #[doc = "Mass Erase (includes all Main and Information memory sectors that don't have corresponding WE bits set)"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode1)
    }
}
#[doc = "Type of memory that erase operation is carried out on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    #[doc = "0: Main Memory"]
    Type0 = 0,
    #[doc = "1: Information Memory"]
    Type1 = 1,
    #[doc = "3: Engineering Memory"]
    Type3 = 3,
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(variant: Type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Type {
    type Ux = u8;
}
impl crate::IsEnum for Type {}
#[doc = "Field `TYPE` reader - Type of memory that erase operation is carried out on"]
pub type TypeR = crate::FieldReader<Type>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Type> {
        match self.bits {
            0 => Some(Type::Type0),
            1 => Some(Type::Type1),
            3 => Some(Type::Type3),
            _ => None,
        }
    }
    #[doc = "Main Memory"]
    #[inline(always)]
    pub fn is_type_0(&self) -> bool {
        *self == Type::Type0
    }
    #[doc = "Information Memory"]
    #[inline(always)]
    pub fn is_type_1(&self) -> bool {
        *self == Type::Type1
    }
    #[doc = "Engineering Memory"]
    #[inline(always)]
    pub fn is_type_3(&self) -> bool {
        *self == Type::Type3
    }
}
#[doc = "Field `TYPE` writer - Type of memory that erase operation is carried out on"]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Type>;
impl<'a, REG> TypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main Memory"]
    #[inline(always)]
    pub fn type_0(self) -> &'a mut crate::W<REG> {
        self.variant(Type::Type0)
    }
    #[doc = "Information Memory"]
    #[inline(always)]
    pub fn type_1(self) -> &'a mut crate::W<REG> {
        self.variant(Type::Type1)
    }
    #[doc = "Engineering Memory"]
    #[inline(always)]
    pub fn type_3(self) -> &'a mut crate::W<REG> {
        self.variant(Type::Type3)
    }
}
#[doc = "Status of erase operations in the Flash memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    #[doc = "0: Idle (no program operation currently active)"]
    Status0 = 0,
    #[doc = "1: Erase operation triggered to START but pending"]
    Status1 = 1,
    #[doc = "2: Erase operation in progress"]
    Status2 = 2,
    #[doc = "3: Erase operation completed (status of completed erase remains in this state unless explicitly cleared by SW)"]
    Status3 = 3,
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
#[doc = "Field `STATUS` reader - Status of erase operations in the Flash memory"]
pub type StatusR = crate::FieldReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            0 => Status::Status0,
            1 => Status::Status1,
            2 => Status::Status2,
            3 => Status::Status3,
            _ => unreachable!(),
        }
    }
    #[doc = "Idle (no program operation currently active)"]
    #[inline(always)]
    pub fn is_status_0(&self) -> bool {
        *self == Status::Status0
    }
    #[doc = "Erase operation triggered to START but pending"]
    #[inline(always)]
    pub fn is_status_1(&self) -> bool {
        *self == Status::Status1
    }
    #[doc = "Erase operation in progress"]
    #[inline(always)]
    pub fn is_status_2(&self) -> bool {
        *self == Status::Status2
    }
    #[doc = "Erase operation completed (status of completed erase remains in this state unless explicitly cleared by SW)"]
    #[inline(always)]
    pub fn is_status_3(&self) -> bool {
        *self == Status::Status3
    }
}
#[doc = "Field `ADDR_ERR` reader - Erase Operation was terminated due to attempted erase of reserved memory address"]
pub type AddrErrR = crate::BitReader;
#[doc = "Field `CLR_STAT` writer - Clear status bits 18-16 of this register"]
pub type ClrStatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Erase mode selected by application"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Type of memory that erase operation is carried out on"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Status of erase operations in the Flash memory"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Erase Operation was terminated due to attempted erase of reserved memory address"]
    #[inline(always)]
    pub fn addr_err(&self) -> AddrErrR {
        AddrErrR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Erase operation"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<FlctlEraseCtlstatSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Erase mode selected by application"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<FlctlEraseCtlstatSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Type of memory that erase operation is carried out on"]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<FlctlEraseCtlstatSpec> {
        TypeW::new(self, 2)
    }
    #[doc = "Bit 19 - Clear status bits 18-16 of this register"]
    #[inline(always)]
    pub fn clr_stat(&mut self) -> ClrStatW<FlctlEraseCtlstatSpec> {
        ClrStatW::new(self, 19)
    }
}
#[doc = "Erase Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_erase_ctlstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_erase_ctlstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlEraseCtlstatSpec;
impl crate::RegisterSpec for FlctlEraseCtlstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_erase_ctlstat::R`](R) reader structure"]
impl crate::Readable for FlctlEraseCtlstatSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_erase_ctlstat::W`](W) writer structure"]
impl crate::Writable for FlctlEraseCtlstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_ERASE_CTLSTAT to value 0"]
impl crate::Resettable for FlctlEraseCtlstatSpec {
    const RESET_VALUE: u32 = 0;
}
