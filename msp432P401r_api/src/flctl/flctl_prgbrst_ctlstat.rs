#[doc = "Register `FLCTL_PRGBRST_CTLSTAT` reader"]
pub type R = crate::R<FlctlPrgbrstCtlstatSpec>;
#[doc = "Register `FLCTL_PRGBRST_CTLSTAT` writer"]
pub type W = crate::W<FlctlPrgbrstCtlstatSpec>;
#[doc = "Field `START` writer - Trigger start of burst program operation"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Type of memory that burst program is carried out on\n\nValue on reset: 0"]
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
#[doc = "Field `TYPE` reader - Type of memory that burst program is carried out on"]
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
#[doc = "Field `TYPE` writer - Type of memory that burst program is carried out on"]
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
#[doc = "Length of burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Len {
    #[doc = "0: No burst operation"]
    Len0 = 0,
    #[doc = "1: 1 word burst of 128 bits, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    Len1 = 1,
    #[doc = "2: 2*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    Len2 = 2,
    #[doc = "3: 3*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    Len3 = 3,
    #[doc = "4: 4*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    Len4 = 4,
}
impl From<Len> for u8 {
    #[inline(always)]
    fn from(variant: Len) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Len {
    type Ux = u8;
}
impl crate::IsEnum for Len {}
#[doc = "Field `LEN` reader - Length of burst"]
pub type LenR = crate::FieldReader<Len>;
impl LenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Len> {
        match self.bits {
            0 => Some(Len::Len0),
            1 => Some(Len::Len1),
            2 => Some(Len::Len2),
            3 => Some(Len::Len3),
            4 => Some(Len::Len4),
            _ => None,
        }
    }
    #[doc = "No burst operation"]
    #[inline(always)]
    pub fn is_len_0(&self) -> bool {
        *self == Len::Len0
    }
    #[doc = "1 word burst of 128 bits, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn is_len_1(&self) -> bool {
        *self == Len::Len1
    }
    #[doc = "2*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn is_len_2(&self) -> bool {
        *self == Len::Len2
    }
    #[doc = "3*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn is_len_3(&self) -> bool {
        *self == Len::Len3
    }
    #[doc = "4*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn is_len_4(&self) -> bool {
        *self == Len::Len4
    }
}
#[doc = "Field `LEN` writer - Length of burst"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Len>;
impl<'a, REG> LenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No burst operation"]
    #[inline(always)]
    pub fn len_0(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len0)
    }
    #[doc = "1 word burst of 128 bits, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_1(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len1)
    }
    #[doc = "2*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_2(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len2)
    }
    #[doc = "3*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_3(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len3)
    }
    #[doc = "4*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_4(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Len4)
    }
}
#[doc = "Auto-Verify operation before the Burst Program\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoPre {
    #[doc = "0: No program verify operations carried out"]
    AutoPre0 = 0,
    #[doc = "1: Causes an automatic Burst Program Verify after the Burst Program Operation"]
    AutoPre1 = 1,
}
impl From<AutoPre> for bool {
    #[inline(always)]
    fn from(variant: AutoPre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTO_PRE` reader - Auto-Verify operation before the Burst Program"]
pub type AutoPreR = crate::BitReader<AutoPre>;
impl AutoPreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoPre {
        match self.bits {
            false => AutoPre::AutoPre0,
            true => AutoPre::AutoPre1,
        }
    }
    #[doc = "No program verify operations carried out"]
    #[inline(always)]
    pub fn is_auto_pre_0(&self) -> bool {
        *self == AutoPre::AutoPre0
    }
    #[doc = "Causes an automatic Burst Program Verify after the Burst Program Operation"]
    #[inline(always)]
    pub fn is_auto_pre_1(&self) -> bool {
        *self == AutoPre::AutoPre1
    }
}
#[doc = "Field `AUTO_PRE` writer - Auto-Verify operation before the Burst Program"]
pub type AutoPreW<'a, REG> = crate::BitWriter<'a, REG, AutoPre>;
impl<'a, REG> AutoPreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No program verify operations carried out"]
    #[inline(always)]
    pub fn auto_pre_0(self) -> &'a mut crate::W<REG> {
        self.variant(AutoPre::AutoPre0)
    }
    #[doc = "Causes an automatic Burst Program Verify after the Burst Program Operation"]
    #[inline(always)]
    pub fn auto_pre_1(self) -> &'a mut crate::W<REG> {
        self.variant(AutoPre::AutoPre1)
    }
}
#[doc = "Auto-Verify operation after the Burst Program\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoPst {
    #[doc = "0: No program verify operations carried out"]
    AutoPst0 = 0,
    #[doc = "1: Causes an automatic Burst Program Verify before the Burst Program Operation"]
    AutoPst1 = 1,
}
impl From<AutoPst> for bool {
    #[inline(always)]
    fn from(variant: AutoPst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTO_PST` reader - Auto-Verify operation after the Burst Program"]
pub type AutoPstR = crate::BitReader<AutoPst>;
impl AutoPstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AutoPst {
        match self.bits {
            false => AutoPst::AutoPst0,
            true => AutoPst::AutoPst1,
        }
    }
    #[doc = "No program verify operations carried out"]
    #[inline(always)]
    pub fn is_auto_pst_0(&self) -> bool {
        *self == AutoPst::AutoPst0
    }
    #[doc = "Causes an automatic Burst Program Verify before the Burst Program Operation"]
    #[inline(always)]
    pub fn is_auto_pst_1(&self) -> bool {
        *self == AutoPst::AutoPst1
    }
}
#[doc = "Field `AUTO_PST` writer - Auto-Verify operation after the Burst Program"]
pub type AutoPstW<'a, REG> = crate::BitWriter<'a, REG, AutoPst>;
impl<'a, REG> AutoPstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No program verify operations carried out"]
    #[inline(always)]
    pub fn auto_pst_0(self) -> &'a mut crate::W<REG> {
        self.variant(AutoPst::AutoPst0)
    }
    #[doc = "Causes an automatic Burst Program Verify before the Burst Program Operation"]
    #[inline(always)]
    pub fn auto_pst_1(self) -> &'a mut crate::W<REG> {
        self.variant(AutoPst::AutoPst1)
    }
}
#[doc = "Status of a Burst Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BurstStatus {
    #[doc = "0: Idle (Burst not active)"]
    BurstStatus0 = 0,
    #[doc = "1: Burst program started but pending"]
    BurstStatus1 = 1,
    #[doc = "2: Burst active, with 1st 128 bit word being written into Flash"]
    BurstStatus2 = 2,
    #[doc = "3: Burst active, with 2nd 128 bit word being written into Flash"]
    BurstStatus3 = 3,
    #[doc = "4: Burst active, with 3rd 128 bit word being written into Flash"]
    BurstStatus4 = 4,
    #[doc = "5: Burst active, with 4th 128 bit word being written into Flash"]
    BurstStatus5 = 5,
    #[doc = "7: Burst Complete (status of completed burst remains in this state unless explicitly cleared by SW)"]
    BurstStatus7 = 7,
}
impl From<BurstStatus> for u8 {
    #[inline(always)]
    fn from(variant: BurstStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BurstStatus {
    type Ux = u8;
}
impl crate::IsEnum for BurstStatus {}
#[doc = "Field `BURST_STATUS` reader - Status of a Burst Operation"]
pub type BurstStatusR = crate::FieldReader<BurstStatus>;
impl BurstStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BurstStatus> {
        match self.bits {
            0 => Some(BurstStatus::BurstStatus0),
            1 => Some(BurstStatus::BurstStatus1),
            2 => Some(BurstStatus::BurstStatus2),
            3 => Some(BurstStatus::BurstStatus3),
            4 => Some(BurstStatus::BurstStatus4),
            5 => Some(BurstStatus::BurstStatus5),
            7 => Some(BurstStatus::BurstStatus7),
            _ => None,
        }
    }
    #[doc = "Idle (Burst not active)"]
    #[inline(always)]
    pub fn is_burst_status_0(&self) -> bool {
        *self == BurstStatus::BurstStatus0
    }
    #[doc = "Burst program started but pending"]
    #[inline(always)]
    pub fn is_burst_status_1(&self) -> bool {
        *self == BurstStatus::BurstStatus1
    }
    #[doc = "Burst active, with 1st 128 bit word being written into Flash"]
    #[inline(always)]
    pub fn is_burst_status_2(&self) -> bool {
        *self == BurstStatus::BurstStatus2
    }
    #[doc = "Burst active, with 2nd 128 bit word being written into Flash"]
    #[inline(always)]
    pub fn is_burst_status_3(&self) -> bool {
        *self == BurstStatus::BurstStatus3
    }
    #[doc = "Burst active, with 3rd 128 bit word being written into Flash"]
    #[inline(always)]
    pub fn is_burst_status_4(&self) -> bool {
        *self == BurstStatus::BurstStatus4
    }
    #[doc = "Burst active, with 4th 128 bit word being written into Flash"]
    #[inline(always)]
    pub fn is_burst_status_5(&self) -> bool {
        *self == BurstStatus::BurstStatus5
    }
    #[doc = "Burst Complete (status of completed burst remains in this state unless explicitly cleared by SW)"]
    #[inline(always)]
    pub fn is_burst_status_7(&self) -> bool {
        *self == BurstStatus::BurstStatus7
    }
}
#[doc = "Field `PRE_ERR` reader - Burst Operation encountered preprogram auto-verify errors"]
pub type PreErrR = crate::BitReader;
#[doc = "Field `PST_ERR` reader - Burst Operation encountered postprogram auto-verify errors"]
pub type PstErrR = crate::BitReader;
#[doc = "Field `ADDR_ERR` reader - Burst Operation was terminated due to attempted program of reserved memory"]
pub type AddrErrR = crate::BitReader;
#[doc = "Field `CLR_STAT` writer - Clear status bits 21-16 of this register"]
pub type ClrStatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:2 - Type of memory that burst program is carried out on"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - Length of burst"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Auto-Verify operation before the Burst Program"]
    #[inline(always)]
    pub fn auto_pre(&self) -> AutoPreR {
        AutoPreR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto-Verify operation after the Burst Program"]
    #[inline(always)]
    pub fn auto_pst(&self) -> AutoPstR {
        AutoPstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Status of a Burst Operation"]
    #[inline(always)]
    pub fn burst_status(&self) -> BurstStatusR {
        BurstStatusR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Burst Operation encountered preprogram auto-verify errors"]
    #[inline(always)]
    pub fn pre_err(&self) -> PreErrR {
        PreErrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Burst Operation encountered postprogram auto-verify errors"]
    #[inline(always)]
    pub fn pst_err(&self) -> PstErrR {
        PstErrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Burst Operation was terminated due to attempted program of reserved memory"]
    #[inline(always)]
    pub fn addr_err(&self) -> AddrErrR {
        AddrErrR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger start of burst program operation"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<FlctlPrgbrstCtlstatSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Type of memory that burst program is carried out on"]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<FlctlPrgbrstCtlstatSpec> {
        TypeW::new(self, 1)
    }
    #[doc = "Bits 3:5 - Length of burst"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<FlctlPrgbrstCtlstatSpec> {
        LenW::new(self, 3)
    }
    #[doc = "Bit 6 - Auto-Verify operation before the Burst Program"]
    #[inline(always)]
    pub fn auto_pre(&mut self) -> AutoPreW<FlctlPrgbrstCtlstatSpec> {
        AutoPreW::new(self, 6)
    }
    #[doc = "Bit 7 - Auto-Verify operation after the Burst Program"]
    #[inline(always)]
    pub fn auto_pst(&mut self) -> AutoPstW<FlctlPrgbrstCtlstatSpec> {
        AutoPstW::new(self, 7)
    }
    #[doc = "Bit 23 - Clear status bits 21-16 of this register"]
    #[inline(always)]
    pub fn clr_stat(&mut self) -> ClrStatW<FlctlPrgbrstCtlstatSpec> {
        ClrStatW::new(self, 23)
    }
}
#[doc = "Program Burst Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_ctlstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_ctlstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlPrgbrstCtlstatSpec;
impl crate::RegisterSpec for FlctlPrgbrstCtlstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_prgbrst_ctlstat::R`](R) reader structure"]
impl crate::Readable for FlctlPrgbrstCtlstatSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_prgbrst_ctlstat::W`](W) writer structure"]
impl crate::Writable for FlctlPrgbrstCtlstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_PRGBRST_CTLSTAT to value 0xc0"]
impl crate::Resettable for FlctlPrgbrstCtlstatSpec {
    const RESET_VALUE: u32 = 0xc0;
}
