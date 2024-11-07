#[doc = "Register `PCMCTL1` reader"]
pub type R = crate::R<Pcmctl1Spec>;
#[doc = "Register `PCMCTL1` writer"]
pub type W = crate::W<Pcmctl1Spec>;
#[doc = "Lock LPM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locklpm5 {
    #[doc = "0: LPMx.5 configuration defaults to reset condition"]
    Locklpm5_0 = 0,
    #[doc = "1: LPMx.5 configuration remains locked during LPMx.5 entry and exit"]
    Locklpm5_1 = 1,
}
impl From<Locklpm5> for bool {
    #[inline(always)]
    fn from(variant: Locklpm5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKLPM5` reader - Lock LPM5"]
pub type Locklpm5R = crate::BitReader<Locklpm5>;
impl Locklpm5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locklpm5 {
        match self.bits {
            false => Locklpm5::Locklpm5_0,
            true => Locklpm5::Locklpm5_1,
        }
    }
    #[doc = "LPMx.5 configuration defaults to reset condition"]
    #[inline(always)]
    pub fn is_locklpm5_0(&self) -> bool {
        *self == Locklpm5::Locklpm5_0
    }
    #[doc = "LPMx.5 configuration remains locked during LPMx.5 entry and exit"]
    #[inline(always)]
    pub fn is_locklpm5_1(&self) -> bool {
        *self == Locklpm5::Locklpm5_1
    }
}
#[doc = "Field `LOCKLPM5` writer - Lock LPM5"]
pub type Locklpm5W<'a, REG> = crate::BitWriter<'a, REG, Locklpm5>;
impl<'a, REG> Locklpm5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPMx.5 configuration defaults to reset condition"]
    #[inline(always)]
    pub fn locklpm5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Locklpm5::Locklpm5_0)
    }
    #[doc = "LPMx.5 configuration remains locked during LPMx.5 entry and exit"]
    #[inline(always)]
    pub fn locklpm5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Locklpm5::Locklpm5_1)
    }
}
#[doc = "Lock Backup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockbkup {
    #[doc = "0: Backup domain configuration defaults to reset condition"]
    Lockbkup0 = 0,
    #[doc = "1: Backup domain configuration remains locked during LPM3.5 entry and exit"]
    Lockbkup1 = 1,
}
impl From<Lockbkup> for bool {
    #[inline(always)]
    fn from(variant: Lockbkup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKBKUP` reader - Lock Backup"]
pub type LockbkupR = crate::BitReader<Lockbkup>;
impl LockbkupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockbkup {
        match self.bits {
            false => Lockbkup::Lockbkup0,
            true => Lockbkup::Lockbkup1,
        }
    }
    #[doc = "Backup domain configuration defaults to reset condition"]
    #[inline(always)]
    pub fn is_lockbkup_0(&self) -> bool {
        *self == Lockbkup::Lockbkup0
    }
    #[doc = "Backup domain configuration remains locked during LPM3.5 entry and exit"]
    #[inline(always)]
    pub fn is_lockbkup_1(&self) -> bool {
        *self == Lockbkup::Lockbkup1
    }
}
#[doc = "Field `LOCKBKUP` writer - Lock Backup"]
pub type LockbkupW<'a, REG> = crate::BitWriter<'a, REG, Lockbkup>;
impl<'a, REG> LockbkupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Backup domain configuration defaults to reset condition"]
    #[inline(always)]
    pub fn lockbkup_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lockbkup::Lockbkup0)
    }
    #[doc = "Backup domain configuration remains locked during LPM3.5 entry and exit"]
    #[inline(always)]
    pub fn lockbkup_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lockbkup::Lockbkup1)
    }
}
#[doc = "Force LPM entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceLpmEntry {
    #[doc = "0: PCM aborts LPM3/LPMx.5 transition if the active clock configuration does not meet the LPM3/LPMx.5 entry criteria. PCM generates the LPM_INVALID_CLK flag on abort to LPM3/LPMx.5 entry."]
    ForceLpmEntry0 = 0,
    #[doc = "1: PCM enters LPM3/LPMx.5 after shuting off the clocks forcefully. Application needs to ensure RTC and WDT are clocked using BCLK tree to keep these modules alive in LPM3/LPM3.5. In LPM4.5 all clocks are forcefully shutoff and the core voltage is turned off."]
    ForceLpmEntry1 = 1,
}
impl From<ForceLpmEntry> for bool {
    #[inline(always)]
    fn from(variant: ForceLpmEntry) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_LPM_ENTRY` reader - Force LPM entry"]
pub type ForceLpmEntryR = crate::BitReader<ForceLpmEntry>;
impl ForceLpmEntryR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceLpmEntry {
        match self.bits {
            false => ForceLpmEntry::ForceLpmEntry0,
            true => ForceLpmEntry::ForceLpmEntry1,
        }
    }
    #[doc = "PCM aborts LPM3/LPMx.5 transition if the active clock configuration does not meet the LPM3/LPMx.5 entry criteria. PCM generates the LPM_INVALID_CLK flag on abort to LPM3/LPMx.5 entry."]
    #[inline(always)]
    pub fn is_force_lpm_entry_0(&self) -> bool {
        *self == ForceLpmEntry::ForceLpmEntry0
    }
    #[doc = "PCM enters LPM3/LPMx.5 after shuting off the clocks forcefully. Application needs to ensure RTC and WDT are clocked using BCLK tree to keep these modules alive in LPM3/LPM3.5. In LPM4.5 all clocks are forcefully shutoff and the core voltage is turned off."]
    #[inline(always)]
    pub fn is_force_lpm_entry_1(&self) -> bool {
        *self == ForceLpmEntry::ForceLpmEntry1
    }
}
#[doc = "Field `FORCE_LPM_ENTRY` writer - Force LPM entry"]
pub type ForceLpmEntryW<'a, REG> = crate::BitWriter<'a, REG, ForceLpmEntry>;
impl<'a, REG> ForceLpmEntryW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PCM aborts LPM3/LPMx.5 transition if the active clock configuration does not meet the LPM3/LPMx.5 entry criteria. PCM generates the LPM_INVALID_CLK flag on abort to LPM3/LPMx.5 entry."]
    #[inline(always)]
    pub fn force_lpm_entry_0(self) -> &'a mut crate::W<REG> {
        self.variant(ForceLpmEntry::ForceLpmEntry0)
    }
    #[doc = "PCM enters LPM3/LPMx.5 after shuting off the clocks forcefully. Application needs to ensure RTC and WDT are clocked using BCLK tree to keep these modules alive in LPM3/LPM3.5. In LPM4.5 all clocks are forcefully shutoff and the core voltage is turned off."]
    #[inline(always)]
    pub fn force_lpm_entry_1(self) -> &'a mut crate::W<REG> {
        self.variant(ForceLpmEntry::ForceLpmEntry1)
    }
}
#[doc = "Field `PMR_BUSY` reader - Power mode request busy flag"]
pub type PmrBusyR = crate::BitReader;
#[doc = "Field `PMR_BUSY` writer - Power mode request busy flag"]
pub type PmrBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCMKEY` reader - PCM key"]
pub type PcmkeyR = crate::FieldReader<u16>;
#[doc = "Field `PCMKEY` writer - PCM key"]
pub type PcmkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Lock LPM5"]
    #[inline(always)]
    pub fn locklpm5(&self) -> Locklpm5R {
        Locklpm5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Backup"]
    #[inline(always)]
    pub fn lockbkup(&self) -> LockbkupR {
        LockbkupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force LPM entry"]
    #[inline(always)]
    pub fn force_lpm_entry(&self) -> ForceLpmEntryR {
        ForceLpmEntryR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Power mode request busy flag"]
    #[inline(always)]
    pub fn pmr_busy(&self) -> PmrBusyR {
        PmrBusyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&self) -> PcmkeyR {
        PcmkeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Lock LPM5"]
    #[inline(always)]
    pub fn locklpm5(&mut self) -> Locklpm5W<Pcmctl1Spec> {
        Locklpm5W::new(self, 0)
    }
    #[doc = "Bit 1 - Lock Backup"]
    #[inline(always)]
    pub fn lockbkup(&mut self) -> LockbkupW<Pcmctl1Spec> {
        LockbkupW::new(self, 1)
    }
    #[doc = "Bit 2 - Force LPM entry"]
    #[inline(always)]
    pub fn force_lpm_entry(&mut self) -> ForceLpmEntryW<Pcmctl1Spec> {
        ForceLpmEntryW::new(self, 2)
    }
    #[doc = "Bit 8 - Power mode request busy flag"]
    #[inline(always)]
    pub fn pmr_busy(&mut self) -> PmrBusyW<Pcmctl1Spec> {
        PmrBusyW::new(self, 8)
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&mut self) -> PcmkeyW<Pcmctl1Spec> {
        PcmkeyW::new(self, 16)
    }
}
#[doc = "Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcmctl1Spec;
impl crate::RegisterSpec for Pcmctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcmctl1::R`](R) reader structure"]
impl crate::Readable for Pcmctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcmctl1::W`](W) writer structure"]
impl crate::Writable for Pcmctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCMCTL1 to value 0xa596_0000"]
impl crate::Resettable for Pcmctl1Spec {
    const RESET_VALUE: u32 = 0xa596_0000;
}
