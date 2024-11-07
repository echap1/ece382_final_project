#[doc = "Register `UCBxCTLW0` reader"]
pub type R = crate::R<UcbxCtlw0Spec>;
#[doc = "Register `UCBxCTLW0` writer"]
pub type W = crate::W<UcbxCtlw0Spec>;
#[doc = "Software reset enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucswrst {
    #[doc = "0: Disabled. eUSCI_B reset released for operation"]
    Ucswrst0 = 0,
    #[doc = "1: Enabled. eUSCI_B logic held in reset state"]
    Ucswrst1 = 1,
}
impl From<Ucswrst> for bool {
    #[inline(always)]
    fn from(variant: Ucswrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSWRST` reader - Software reset enable"]
pub type UcswrstR = crate::BitReader<Ucswrst>;
impl UcswrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucswrst {
        match self.bits {
            false => Ucswrst::Ucswrst0,
            true => Ucswrst::Ucswrst1,
        }
    }
    #[doc = "Disabled. eUSCI_B reset released for operation"]
    #[inline(always)]
    pub fn is_ucswrst_0(&self) -> bool {
        *self == Ucswrst::Ucswrst0
    }
    #[doc = "Enabled. eUSCI_B logic held in reset state"]
    #[inline(always)]
    pub fn is_ucswrst_1(&self) -> bool {
        *self == Ucswrst::Ucswrst1
    }
}
#[doc = "Field `UCSWRST` writer - Software reset enable"]
pub type UcswrstW<'a, REG> = crate::BitWriter<'a, REG, Ucswrst>;
impl<'a, REG> UcswrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. eUSCI_B reset released for operation"]
    #[inline(always)]
    pub fn ucswrst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucswrst::Ucswrst0)
    }
    #[doc = "Enabled. eUSCI_B logic held in reset state"]
    #[inline(always)]
    pub fn ucswrst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucswrst::Ucswrst1)
    }
}
#[doc = "Transmit START condition in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxstt {
    #[doc = "0: Do not generate START condition"]
    Uctxstt0 = 0,
    #[doc = "1: Generate START condition"]
    Uctxstt1 = 1,
}
impl From<Uctxstt> for bool {
    #[inline(always)]
    fn from(variant: Uctxstt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXSTT` reader - Transmit START condition in master mode"]
pub type UctxsttR = crate::BitReader<Uctxstt>;
impl UctxsttR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxstt {
        match self.bits {
            false => Uctxstt::Uctxstt0,
            true => Uctxstt::Uctxstt1,
        }
    }
    #[doc = "Do not generate START condition"]
    #[inline(always)]
    pub fn is_uctxstt_0(&self) -> bool {
        *self == Uctxstt::Uctxstt0
    }
    #[doc = "Generate START condition"]
    #[inline(always)]
    pub fn is_uctxstt_1(&self) -> bool {
        *self == Uctxstt::Uctxstt1
    }
}
#[doc = "Field `UCTXSTT` writer - Transmit START condition in master mode"]
pub type UctxsttW<'a, REG> = crate::BitWriter<'a, REG, Uctxstt>;
impl<'a, REG> UctxsttW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not generate START condition"]
    #[inline(always)]
    pub fn uctxstt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxstt::Uctxstt0)
    }
    #[doc = "Generate START condition"]
    #[inline(always)]
    pub fn uctxstt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxstt::Uctxstt1)
    }
}
#[doc = "Transmit STOP condition in master mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxstp {
    #[doc = "0: No STOP generated"]
    Uctxstp0 = 0,
    #[doc = "1: Generate STOP"]
    Uctxstp1 = 1,
}
impl From<Uctxstp> for bool {
    #[inline(always)]
    fn from(variant: Uctxstp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXSTP` reader - Transmit STOP condition in master mode"]
pub type UctxstpR = crate::BitReader<Uctxstp>;
impl UctxstpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxstp {
        match self.bits {
            false => Uctxstp::Uctxstp0,
            true => Uctxstp::Uctxstp1,
        }
    }
    #[doc = "No STOP generated"]
    #[inline(always)]
    pub fn is_uctxstp_0(&self) -> bool {
        *self == Uctxstp::Uctxstp0
    }
    #[doc = "Generate STOP"]
    #[inline(always)]
    pub fn is_uctxstp_1(&self) -> bool {
        *self == Uctxstp::Uctxstp1
    }
}
#[doc = "Field `UCTXSTP` writer - Transmit STOP condition in master mode"]
pub type UctxstpW<'a, REG> = crate::BitWriter<'a, REG, Uctxstp>;
impl<'a, REG> UctxstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No STOP generated"]
    #[inline(always)]
    pub fn uctxstp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxstp::Uctxstp0)
    }
    #[doc = "Generate STOP"]
    #[inline(always)]
    pub fn uctxstp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxstp::Uctxstp1)
    }
}
#[doc = "Transmit a NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxnack {
    #[doc = "0: Acknowledge normally"]
    Uctxnack0 = 0,
    #[doc = "1: Generate NACK"]
    Uctxnack1 = 1,
}
impl From<Uctxnack> for bool {
    #[inline(always)]
    fn from(variant: Uctxnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXNACK` reader - Transmit a NACK"]
pub type UctxnackR = crate::BitReader<Uctxnack>;
impl UctxnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxnack {
        match self.bits {
            false => Uctxnack::Uctxnack0,
            true => Uctxnack::Uctxnack1,
        }
    }
    #[doc = "Acknowledge normally"]
    #[inline(always)]
    pub fn is_uctxnack_0(&self) -> bool {
        *self == Uctxnack::Uctxnack0
    }
    #[doc = "Generate NACK"]
    #[inline(always)]
    pub fn is_uctxnack_1(&self) -> bool {
        *self == Uctxnack::Uctxnack1
    }
}
#[doc = "Field `UCTXNACK` writer - Transmit a NACK"]
pub type UctxnackW<'a, REG> = crate::BitWriter<'a, REG, Uctxnack>;
impl<'a, REG> UctxnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Acknowledge normally"]
    #[inline(always)]
    pub fn uctxnack_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxnack::Uctxnack0)
    }
    #[doc = "Generate NACK"]
    #[inline(always)]
    pub fn uctxnack_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxnack::Uctxnack1)
    }
}
#[doc = "Transmitter/receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctr {
    #[doc = "0: Receiver"]
    Uctr0 = 0,
    #[doc = "1: Transmitter"]
    Uctr1 = 1,
}
impl From<Uctr> for bool {
    #[inline(always)]
    fn from(variant: Uctr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTR` reader - Transmitter/receiver"]
pub type UctrR = crate::BitReader<Uctr>;
impl UctrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctr {
        match self.bits {
            false => Uctr::Uctr0,
            true => Uctr::Uctr1,
        }
    }
    #[doc = "Receiver"]
    #[inline(always)]
    pub fn is_uctr_0(&self) -> bool {
        *self == Uctr::Uctr0
    }
    #[doc = "Transmitter"]
    #[inline(always)]
    pub fn is_uctr_1(&self) -> bool {
        *self == Uctr::Uctr1
    }
}
#[doc = "Field `UCTR` writer - Transmitter/receiver"]
pub type UctrW<'a, REG> = crate::BitWriter<'a, REG, Uctr>;
impl<'a, REG> UctrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver"]
    #[inline(always)]
    pub fn uctr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctr::Uctr0)
    }
    #[doc = "Transmitter"]
    #[inline(always)]
    pub fn uctr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctr::Uctr1)
    }
}
#[doc = "Transmit ACK condition in slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxack {
    #[doc = "0: Do not acknowledge the slave address"]
    Uctxack0 = 0,
    #[doc = "1: Acknowledge the slave address"]
    Uctxack1 = 1,
}
impl From<Uctxack> for bool {
    #[inline(always)]
    fn from(variant: Uctxack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXACK` reader - Transmit ACK condition in slave mode"]
pub type UctxackR = crate::BitReader<Uctxack>;
impl UctxackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxack {
        match self.bits {
            false => Uctxack::Uctxack0,
            true => Uctxack::Uctxack1,
        }
    }
    #[doc = "Do not acknowledge the slave address"]
    #[inline(always)]
    pub fn is_uctxack_0(&self) -> bool {
        *self == Uctxack::Uctxack0
    }
    #[doc = "Acknowledge the slave address"]
    #[inline(always)]
    pub fn is_uctxack_1(&self) -> bool {
        *self == Uctxack::Uctxack1
    }
}
#[doc = "Field `UCTXACK` writer - Transmit ACK condition in slave mode"]
pub type UctxackW<'a, REG> = crate::BitWriter<'a, REG, Uctxack>;
impl<'a, REG> UctxackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not acknowledge the slave address"]
    #[inline(always)]
    pub fn uctxack_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxack::Uctxack0)
    }
    #[doc = "Acknowledge the slave address"]
    #[inline(always)]
    pub fn uctxack_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxack::Uctxack1)
    }
}
#[doc = "eUSCI_B clock source select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucssel {
    #[doc = "0: UCLKI"]
    Ucssel0 = 0,
    #[doc = "1: ACLK"]
    Ucssel1 = 1,
    #[doc = "2: SMCLK"]
    Ucssel2 = 2,
    #[doc = "3: SMCLK"]
    Ucssel3 = 3,
}
impl From<Ucssel> for u8 {
    #[inline(always)]
    fn from(variant: Ucssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucssel {
    type Ux = u8;
}
impl crate::IsEnum for Ucssel {}
#[doc = "Field `UCSSEL` reader - eUSCI_B clock source select"]
pub type UcsselR = crate::FieldReader<Ucssel>;
impl UcsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucssel {
        match self.bits {
            0 => Ucssel::Ucssel0,
            1 => Ucssel::Ucssel1,
            2 => Ucssel::Ucssel2,
            3 => Ucssel::Ucssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "UCLKI"]
    #[inline(always)]
    pub fn is_ucssel_0(&self) -> bool {
        *self == Ucssel::Ucssel0
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_ucssel_1(&self) -> bool {
        *self == Ucssel::Ucssel1
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_ucssel_2(&self) -> bool {
        *self == Ucssel::Ucssel2
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        *self == Ucssel::Ucssel3
    }
}
#[doc = "Field `UCSSEL` writer - eUSCI_B clock source select"]
pub type UcsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucssel, crate::Safe>;
impl<'a, REG> UcsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UCLKI"]
    #[inline(always)]
    pub fn ucssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn ucssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel1)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn ucssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel2)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn ucssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel3)
    }
}
#[doc = "Synchronous mode enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucsync {
    #[doc = "0: Asynchronous mode"]
    Ucsync0 = 0,
    #[doc = "1: Synchronous mode"]
    Ucsync1 = 1,
}
impl From<Ucsync> for bool {
    #[inline(always)]
    fn from(variant: Ucsync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSYNC` reader - Synchronous mode enable"]
pub type UcsyncR = crate::BitReader<Ucsync>;
impl UcsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucsync {
        match self.bits {
            false => Ucsync::Ucsync0,
            true => Ucsync::Ucsync1,
        }
    }
    #[doc = "Asynchronous mode"]
    #[inline(always)]
    pub fn is_ucsync_0(&self) -> bool {
        *self == Ucsync::Ucsync0
    }
    #[doc = "Synchronous mode"]
    #[inline(always)]
    pub fn is_ucsync_1(&self) -> bool {
        *self == Ucsync::Ucsync1
    }
}
#[doc = "Field `UCSYNC` writer - Synchronous mode enable"]
pub type UcsyncW<'a, REG> = crate::BitWriter<'a, REG, Ucsync>;
impl<'a, REG> UcsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous mode"]
    #[inline(always)]
    pub fn ucsync_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsync::Ucsync0)
    }
    #[doc = "Synchronous mode"]
    #[inline(always)]
    pub fn ucsync_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsync::Ucsync1)
    }
}
#[doc = "eUSCI_B mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucmode {
    #[doc = "0: 3-pin SPI"]
    Ucmode0 = 0,
    #[doc = "1: 4-pin SPI (master or slave enabled if STE = 1)"]
    Ucmode1 = 1,
    #[doc = "2: 4-pin SPI (master or slave enabled if STE = 0)"]
    Ucmode2 = 2,
    #[doc = "3: I2C mode"]
    Ucmode3 = 3,
}
impl From<Ucmode> for u8 {
    #[inline(always)]
    fn from(variant: Ucmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucmode {
    type Ux = u8;
}
impl crate::IsEnum for Ucmode {}
#[doc = "Field `UCMODE` reader - eUSCI_B mode"]
pub type UcmodeR = crate::FieldReader<Ucmode>;
impl UcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucmode {
        match self.bits {
            0 => Ucmode::Ucmode0,
            1 => Ucmode::Ucmode1,
            2 => Ucmode::Ucmode2,
            3 => Ucmode::Ucmode3,
            _ => unreachable!(),
        }
    }
    #[doc = "3-pin SPI"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        *self == Ucmode::Ucmode0
    }
    #[doc = "4-pin SPI (master or slave enabled if STE = 1)"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        *self == Ucmode::Ucmode1
    }
    #[doc = "4-pin SPI (master or slave enabled if STE = 0)"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        *self == Ucmode::Ucmode2
    }
    #[doc = "I2C mode"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        *self == Ucmode::Ucmode3
    }
}
#[doc = "Field `UCMODE` writer - eUSCI_B mode"]
pub type UcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucmode, crate::Safe>;
impl<'a, REG> UcmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3-pin SPI"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode0)
    }
    #[doc = "4-pin SPI (master or slave enabled if STE = 1)"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode1)
    }
    #[doc = "4-pin SPI (master or slave enabled if STE = 0)"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode2)
    }
    #[doc = "I2C mode"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode3)
    }
}
#[doc = "Master mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucmst {
    #[doc = "0: Slave mode"]
    Ucmst0 = 0,
    #[doc = "1: Master mode"]
    Ucmst1 = 1,
}
impl From<Ucmst> for bool {
    #[inline(always)]
    fn from(variant: Ucmst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCMST` reader - Master mode select"]
pub type UcmstR = crate::BitReader<Ucmst>;
impl UcmstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucmst {
        match self.bits {
            false => Ucmst::Ucmst0,
            true => Ucmst::Ucmst1,
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn is_ucmst_0(&self) -> bool {
        *self == Ucmst::Ucmst0
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn is_ucmst_1(&self) -> bool {
        *self == Ucmst::Ucmst1
    }
}
#[doc = "Field `UCMST` writer - Master mode select"]
pub type UcmstW<'a, REG> = crate::BitWriter<'a, REG, Ucmst>;
impl<'a, REG> UcmstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn ucmst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmst::Ucmst0)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn ucmst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmst::Ucmst1)
    }
}
#[doc = "Multi-master environment select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucmm {
    #[doc = "0: Single master environment. There is no other master in the system. The address compare unit is disabled."]
    Ucmm0 = 0,
    #[doc = "1: Multi-master environment"]
    Ucmm1 = 1,
}
impl From<Ucmm> for bool {
    #[inline(always)]
    fn from(variant: Ucmm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCMM` reader - Multi-master environment select"]
pub type UcmmR = crate::BitReader<Ucmm>;
impl UcmmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucmm {
        match self.bits {
            false => Ucmm::Ucmm0,
            true => Ucmm::Ucmm1,
        }
    }
    #[doc = "Single master environment. There is no other master in the system. The address compare unit is disabled."]
    #[inline(always)]
    pub fn is_ucmm_0(&self) -> bool {
        *self == Ucmm::Ucmm0
    }
    #[doc = "Multi-master environment"]
    #[inline(always)]
    pub fn is_ucmm_1(&self) -> bool {
        *self == Ucmm::Ucmm1
    }
}
#[doc = "Field `UCMM` writer - Multi-master environment select"]
pub type UcmmW<'a, REG> = crate::BitWriter<'a, REG, Ucmm>;
impl<'a, REG> UcmmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single master environment. There is no other master in the system. The address compare unit is disabled."]
    #[inline(always)]
    pub fn ucmm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmm::Ucmm0)
    }
    #[doc = "Multi-master environment"]
    #[inline(always)]
    pub fn ucmm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmm::Ucmm1)
    }
}
#[doc = "Slave addressing mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucsla10 {
    #[doc = "0: Address slave with 7-bit address"]
    Ucsla10_0 = 0,
    #[doc = "1: Address slave with 10-bit address"]
    Ucsla10_1 = 1,
}
impl From<Ucsla10> for bool {
    #[inline(always)]
    fn from(variant: Ucsla10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSLA10` reader - Slave addressing mode select"]
pub type Ucsla10R = crate::BitReader<Ucsla10>;
impl Ucsla10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucsla10 {
        match self.bits {
            false => Ucsla10::Ucsla10_0,
            true => Ucsla10::Ucsla10_1,
        }
    }
    #[doc = "Address slave with 7-bit address"]
    #[inline(always)]
    pub fn is_ucsla10_0(&self) -> bool {
        *self == Ucsla10::Ucsla10_0
    }
    #[doc = "Address slave with 10-bit address"]
    #[inline(always)]
    pub fn is_ucsla10_1(&self) -> bool {
        *self == Ucsla10::Ucsla10_1
    }
}
#[doc = "Field `UCSLA10` writer - Slave addressing mode select"]
pub type Ucsla10W<'a, REG> = crate::BitWriter<'a, REG, Ucsla10>;
impl<'a, REG> Ucsla10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address slave with 7-bit address"]
    #[inline(always)]
    pub fn ucsla10_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsla10::Ucsla10_0)
    }
    #[doc = "Address slave with 10-bit address"]
    #[inline(always)]
    pub fn ucsla10_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsla10::Ucsla10_1)
    }
}
#[doc = "Own addressing mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uca10 {
    #[doc = "0: Own address is a 7-bit address"]
    Uca10_0 = 0,
    #[doc = "1: Own address is a 10-bit address"]
    Uca10_1 = 1,
}
impl From<Uca10> for bool {
    #[inline(always)]
    fn from(variant: Uca10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCA10` reader - Own addressing mode select"]
pub type Uca10R = crate::BitReader<Uca10>;
impl Uca10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uca10 {
        match self.bits {
            false => Uca10::Uca10_0,
            true => Uca10::Uca10_1,
        }
    }
    #[doc = "Own address is a 7-bit address"]
    #[inline(always)]
    pub fn is_uca10_0(&self) -> bool {
        *self == Uca10::Uca10_0
    }
    #[doc = "Own address is a 10-bit address"]
    #[inline(always)]
    pub fn is_uca10_1(&self) -> bool {
        *self == Uca10::Uca10_1
    }
}
#[doc = "Field `UCA10` writer - Own addressing mode select"]
pub type Uca10W<'a, REG> = crate::BitWriter<'a, REG, Uca10>;
impl<'a, REG> Uca10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Own address is a 7-bit address"]
    #[inline(always)]
    pub fn uca10_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uca10::Uca10_0)
    }
    #[doc = "Own address is a 10-bit address"]
    #[inline(always)]
    pub fn uca10_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uca10::Uca10_1)
    }
}
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UcswrstR {
        UcswrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit START condition in master mode"]
    #[inline(always)]
    pub fn uctxstt(&self) -> UctxsttR {
        UctxsttR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit STOP condition in master mode"]
    #[inline(always)]
    pub fn uctxstp(&self) -> UctxstpR {
        UctxstpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit a NACK"]
    #[inline(always)]
    pub fn uctxnack(&self) -> UctxnackR {
        UctxnackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter/receiver"]
    #[inline(always)]
    pub fn uctr(&self) -> UctrR {
        UctrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit ACK condition in slave mode"]
    #[inline(always)]
    pub fn uctxack(&self) -> UctxackR {
        UctxackR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - eUSCI_B clock source select"]
    #[inline(always)]
    pub fn ucssel(&self) -> UcsselR {
        UcsselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&self) -> UcsyncR {
        UcsyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - eUSCI_B mode"]
    #[inline(always)]
    pub fn ucmode(&self) -> UcmodeR {
        UcmodeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UcmstR {
        UcmstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Multi-master environment select"]
    #[inline(always)]
    pub fn ucmm(&self) -> UcmmR {
        UcmmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Slave addressing mode select"]
    #[inline(always)]
    pub fn ucsla10(&self) -> Ucsla10R {
        Ucsla10R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Own addressing mode select"]
    #[inline(always)]
    pub fn uca10(&self) -> Uca10R {
        Uca10R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UcswrstW<UcbxCtlw0Spec> {
        UcswrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit START condition in master mode"]
    #[inline(always)]
    pub fn uctxstt(&mut self) -> UctxsttW<UcbxCtlw0Spec> {
        UctxsttW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit STOP condition in master mode"]
    #[inline(always)]
    pub fn uctxstp(&mut self) -> UctxstpW<UcbxCtlw0Spec> {
        UctxstpW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit a NACK"]
    #[inline(always)]
    pub fn uctxnack(&mut self) -> UctxnackW<UcbxCtlw0Spec> {
        UctxnackW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmitter/receiver"]
    #[inline(always)]
    pub fn uctr(&mut self) -> UctrW<UcbxCtlw0Spec> {
        UctrW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit ACK condition in slave mode"]
    #[inline(always)]
    pub fn uctxack(&mut self) -> UctxackW<UcbxCtlw0Spec> {
        UctxackW::new(self, 5)
    }
    #[doc = "Bits 6:7 - eUSCI_B clock source select"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UcsselW<UcbxCtlw0Spec> {
        UcsselW::new(self, 6)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UcsyncW<UcbxCtlw0Spec> {
        UcsyncW::new(self, 8)
    }
    #[doc = "Bits 9:10 - eUSCI_B mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UcmodeW<UcbxCtlw0Spec> {
        UcmodeW::new(self, 9)
    }
    #[doc = "Bit 11 - Master mode select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UcmstW<UcbxCtlw0Spec> {
        UcmstW::new(self, 11)
    }
    #[doc = "Bit 13 - Multi-master environment select"]
    #[inline(always)]
    pub fn ucmm(&mut self) -> UcmmW<UcbxCtlw0Spec> {
        UcmmW::new(self, 13)
    }
    #[doc = "Bit 14 - Slave addressing mode select"]
    #[inline(always)]
    pub fn ucsla10(&mut self) -> Ucsla10W<UcbxCtlw0Spec> {
        Ucsla10W::new(self, 14)
    }
    #[doc = "Bit 15 - Own addressing mode select"]
    #[inline(always)]
    pub fn uca10(&mut self) -> Uca10W<UcbxCtlw0Spec> {
        Uca10W::new(self, 15)
    }
}
#[doc = "eUSCI_Bx Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_ctlw0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_ctlw0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcbxCtlw0Spec;
impl crate::RegisterSpec for UcbxCtlw0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucbx_ctlw0::R`](R) reader structure"]
impl crate::Readable for UcbxCtlw0Spec {}
#[doc = "`write(|w| ..)` method takes [`ucbx_ctlw0::W`](W) writer structure"]
impl crate::Writable for UcbxCtlw0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCBxCTLW0 to value 0x01c1"]
impl crate::Resettable for UcbxCtlw0Spec {
    const RESET_VALUE: u16 = 0x01c1;
}
