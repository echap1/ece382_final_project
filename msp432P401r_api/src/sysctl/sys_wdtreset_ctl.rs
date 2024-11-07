#[doc = "Register `SYS_WDTRESET_CTL` reader"]
pub type R = crate::R<SysWdtresetCtlSpec>;
#[doc = "Register `SYS_WDTRESET_CTL` writer"]
pub type W = crate::W<SysWdtresetCtlSpec>;
#[doc = "WDT timeout reset type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeout {
    #[doc = "0: WDT timeout event generates Soft reset"]
    Timeout0 = 0,
    #[doc = "1: WDT timeout event generates Hard reset"]
    Timeout1 = 1,
}
impl From<Timeout> for bool {
    #[inline(always)]
    fn from(variant: Timeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` reader - WDT timeout reset type"]
pub type TimeoutR = crate::BitReader<Timeout>;
impl TimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeout {
        match self.bits {
            false => Timeout::Timeout0,
            true => Timeout::Timeout1,
        }
    }
    #[doc = "WDT timeout event generates Soft reset"]
    #[inline(always)]
    pub fn is_timeout_0(&self) -> bool {
        *self == Timeout::Timeout0
    }
    #[doc = "WDT timeout event generates Hard reset"]
    #[inline(always)]
    pub fn is_timeout_1(&self) -> bool {
        *self == Timeout::Timeout1
    }
}
#[doc = "Field `TIMEOUT` writer - WDT timeout reset type"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG, Timeout>;
impl<'a, REG> TimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDT timeout event generates Soft reset"]
    #[inline(always)]
    pub fn timeout_0(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::Timeout0)
    }
    #[doc = "WDT timeout event generates Hard reset"]
    #[inline(always)]
    pub fn timeout_1(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::Timeout1)
    }
}
#[doc = "WDT password violation reset type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Violation {
    #[doc = "0: WDT password violation event generates Soft reset"]
    Violation0 = 0,
    #[doc = "1: WDT password violation event generates Hard reset"]
    Violation1 = 1,
}
impl From<Violation> for bool {
    #[inline(always)]
    fn from(variant: Violation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIOLATION` reader - WDT password violation reset type"]
pub type ViolationR = crate::BitReader<Violation>;
impl ViolationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Violation {
        match self.bits {
            false => Violation::Violation0,
            true => Violation::Violation1,
        }
    }
    #[doc = "WDT password violation event generates Soft reset"]
    #[inline(always)]
    pub fn is_violation_0(&self) -> bool {
        *self == Violation::Violation0
    }
    #[doc = "WDT password violation event generates Hard reset"]
    #[inline(always)]
    pub fn is_violation_1(&self) -> bool {
        *self == Violation::Violation1
    }
}
#[doc = "Field `VIOLATION` writer - WDT password violation reset type"]
pub type ViolationW<'a, REG> = crate::BitWriter<'a, REG, Violation>;
impl<'a, REG> ViolationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDT password violation event generates Soft reset"]
    #[inline(always)]
    pub fn violation_0(self) -> &'a mut crate::W<REG> {
        self.variant(Violation::Violation0)
    }
    #[doc = "WDT password violation event generates Hard reset"]
    #[inline(always)]
    pub fn violation_1(self) -> &'a mut crate::W<REG> {
        self.variant(Violation::Violation1)
    }
}
impl R {
    #[doc = "Bit 0 - WDT timeout reset type"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT password violation reset type"]
    #[inline(always)]
    pub fn violation(&self) -> ViolationR {
        ViolationR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT timeout reset type"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<SysWdtresetCtlSpec> {
        TimeoutW::new(self, 0)
    }
    #[doc = "Bit 1 - WDT password violation reset type"]
    #[inline(always)]
    pub fn violation(&mut self) -> ViolationW<SysWdtresetCtlSpec> {
        ViolationW::new(self, 1)
    }
}
#[doc = "Watchdog Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_wdtreset_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_wdtreset_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysWdtresetCtlSpec;
impl crate::RegisterSpec for SysWdtresetCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_wdtreset_ctl::R`](R) reader structure"]
impl crate::Readable for SysWdtresetCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_wdtreset_ctl::W`](W) writer structure"]
impl crate::Writable for SysWdtresetCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_WDTRESET_CTL to value 0x03"]
impl crate::Resettable for SysWdtresetCtlSpec {
    const RESET_VALUE: u32 = 0x03;
}
