#[doc = "Register `SYS_SYSTEM_STAT` reader"]
pub type R = crate::R<SysSystemStatSpec>;
#[doc = "Field `DBG_SEC_ACT` reader - Debug Security active"]
pub type DbgSecActR = crate::BitReader;
#[doc = "Field `JTAG_SWD_LOCK_ACT` reader - Indicates if JTAG and SWD Lock is active"]
pub type JtagSwdLockActR = crate::BitReader;
#[doc = "Field `IP_PROT_ACT` reader - Indicates if IP protection is active"]
pub type IpProtActR = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Debug Security active"]
    #[inline(always)]
    pub fn dbg_sec_act(&self) -> DbgSecActR {
        DbgSecActR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates if JTAG and SWD Lock is active"]
    #[inline(always)]
    pub fn jtag_swd_lock_act(&self) -> JtagSwdLockActR {
        JtagSwdLockActR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates if IP protection is active"]
    #[inline(always)]
    pub fn ip_prot_act(&self) -> IpProtActR {
        IpProtActR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "System Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_system_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSystemStatSpec;
impl crate::RegisterSpec for SysSystemStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_system_stat::R`](R) reader structure"]
impl crate::Readable for SysSystemStatSpec {}
