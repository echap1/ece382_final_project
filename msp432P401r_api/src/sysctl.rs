#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sys_reboot_ctl: SysRebootCtl,
    sys_nmi_ctlstat: SysNmiCtlstat,
    sys_wdtreset_ctl: SysWdtresetCtl,
    sys_perihalt_ctl: SysPerihaltCtl,
    sys_sram_size: SysSramSize,
    sys_sram_banken: SysSramBanken,
    sys_sram_bankret: SysSramBankret,
    _reserved7: [u8; 0x04],
    sys_flash_size: SysFlashSize,
    _reserved8: [u8; 0x0c],
    sys_dio_gltflt_ctl: SysDioGltfltCtl,
    _reserved9: [u8; 0x0c],
    sys_secdata_unlock: SysSecdataUnlock,
    _reserved10: [u8; 0x0fbc],
    sys_master_unlock: SysMasterUnlock,
    sys_bootover_req: [SysBootoverReq; 2],
    sys_bootover_ack: SysBootoverAck,
    sys_reset_req: SysResetReq,
    sys_reset_statover: SysResetStatover,
    _reserved15: [u8; 0x08],
    sys_system_stat: SysSystemStat,
}
impl RegisterBlock {
    #[doc = "0x00 - Reboot Control Register"]
    #[inline(always)]
    pub const fn sys_reboot_ctl(&self) -> &SysRebootCtl {
        &self.sys_reboot_ctl
    }
    #[doc = "0x04 - NMI Control and Status Register"]
    #[inline(always)]
    pub const fn sys_nmi_ctlstat(&self) -> &SysNmiCtlstat {
        &self.sys_nmi_ctlstat
    }
    #[doc = "0x08 - Watchdog Reset Control Register"]
    #[inline(always)]
    pub const fn sys_wdtreset_ctl(&self) -> &SysWdtresetCtl {
        &self.sys_wdtreset_ctl
    }
    #[doc = "0x0c - Peripheral Halt Control Register"]
    #[inline(always)]
    pub const fn sys_perihalt_ctl(&self) -> &SysPerihaltCtl {
        &self.sys_perihalt_ctl
    }
    #[doc = "0x10 - SRAM Size Register"]
    #[inline(always)]
    pub const fn sys_sram_size(&self) -> &SysSramSize {
        &self.sys_sram_size
    }
    #[doc = "0x14 - SRAM Bank Enable Register"]
    #[inline(always)]
    pub const fn sys_sram_banken(&self) -> &SysSramBanken {
        &self.sys_sram_banken
    }
    #[doc = "0x18 - SRAM Bank Retention Control Register"]
    #[inline(always)]
    pub const fn sys_sram_bankret(&self) -> &SysSramBankret {
        &self.sys_sram_bankret
    }
    #[doc = "0x20 - Flash Size Register"]
    #[inline(always)]
    pub const fn sys_flash_size(&self) -> &SysFlashSize {
        &self.sys_flash_size
    }
    #[doc = "0x30 - Digital I/O Glitch Filter Control Register"]
    #[inline(always)]
    pub const fn sys_dio_gltflt_ctl(&self) -> &SysDioGltfltCtl {
        &self.sys_dio_gltflt_ctl
    }
    #[doc = "0x40 - IP Protected Secure Zone Data Access Unlock Register"]
    #[inline(always)]
    pub const fn sys_secdata_unlock(&self) -> &SysSecdataUnlock {
        &self.sys_secdata_unlock
    }
    #[doc = "0x1000 - Master Unlock Register"]
    #[inline(always)]
    pub const fn sys_master_unlock(&self) -> &SysMasterUnlock {
        &self.sys_master_unlock
    }
    #[doc = "0x1004..0x100c - Boot Override Request Register"]
    #[inline(always)]
    pub const fn sys_bootover_req(&self, n: usize) -> &SysBootoverReq {
        &self.sys_bootover_req[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1004..0x100c - Boot Override Request Register"]
    #[inline(always)]
    pub fn sys_bootover_req_iter(&self) -> impl Iterator<Item = &SysBootoverReq> {
        self.sys_bootover_req.iter()
    }
    #[doc = "0x100c - Boot Override Acknowledge Register"]
    #[inline(always)]
    pub const fn sys_bootover_ack(&self) -> &SysBootoverAck {
        &self.sys_bootover_ack
    }
    #[doc = "0x1010 - Reset Request Register"]
    #[inline(always)]
    pub const fn sys_reset_req(&self) -> &SysResetReq {
        &self.sys_reset_req
    }
    #[doc = "0x1014 - Reset Status and Override Register"]
    #[inline(always)]
    pub const fn sys_reset_statover(&self) -> &SysResetStatover {
        &self.sys_reset_statover
    }
    #[doc = "0x1020 - System Status Register"]
    #[inline(always)]
    pub const fn sys_system_stat(&self) -> &SysSystemStat {
        &self.sys_system_stat
    }
}
#[doc = "SYS_REBOOT_CTL (rw) register accessor: Reboot Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_reboot_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_reboot_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_reboot_ctl`]
module"]
#[doc(alias = "SYS_REBOOT_CTL")]
pub type SysRebootCtl = crate::Reg<sys_reboot_ctl::SysRebootCtlSpec>;
#[doc = "Reboot Control Register"]
pub mod sys_reboot_ctl;
#[doc = "SYS_NMI_CTLSTAT (rw) register accessor: NMI Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_nmi_ctlstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_nmi_ctlstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_nmi_ctlstat`]
module"]
#[doc(alias = "SYS_NMI_CTLSTAT")]
pub type SysNmiCtlstat = crate::Reg<sys_nmi_ctlstat::SysNmiCtlstatSpec>;
#[doc = "NMI Control and Status Register"]
pub mod sys_nmi_ctlstat;
#[doc = "SYS_WDTRESET_CTL (rw) register accessor: Watchdog Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_wdtreset_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_wdtreset_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_wdtreset_ctl`]
module"]
#[doc(alias = "SYS_WDTRESET_CTL")]
pub type SysWdtresetCtl = crate::Reg<sys_wdtreset_ctl::SysWdtresetCtlSpec>;
#[doc = "Watchdog Reset Control Register"]
pub mod sys_wdtreset_ctl;
#[doc = "SYS_PERIHALT_CTL (rw) register accessor: Peripheral Halt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_perihalt_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_perihalt_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_perihalt_ctl`]
module"]
#[doc(alias = "SYS_PERIHALT_CTL")]
pub type SysPerihaltCtl = crate::Reg<sys_perihalt_ctl::SysPerihaltCtlSpec>;
#[doc = "Peripheral Halt Control Register"]
pub mod sys_perihalt_ctl;
#[doc = "SYS_SRAM_SIZE (r) register accessor: SRAM Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sram_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sram_size`]
module"]
#[doc(alias = "SYS_SRAM_SIZE")]
pub type SysSramSize = crate::Reg<sys_sram_size::SysSramSizeSpec>;
#[doc = "SRAM Size Register"]
pub mod sys_sram_size;
#[doc = "SYS_SRAM_BANKEN (rw) register accessor: SRAM Bank Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sram_banken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_sram_banken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sram_banken`]
module"]
#[doc(alias = "SYS_SRAM_BANKEN")]
pub type SysSramBanken = crate::Reg<sys_sram_banken::SysSramBankenSpec>;
#[doc = "SRAM Bank Enable Register"]
pub mod sys_sram_banken;
#[doc = "SYS_SRAM_BANKRET (rw) register accessor: SRAM Bank Retention Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sram_bankret::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_sram_bankret::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_sram_bankret`]
module"]
#[doc(alias = "SYS_SRAM_BANKRET")]
pub type SysSramBankret = crate::Reg<sys_sram_bankret::SysSramBankretSpec>;
#[doc = "SRAM Bank Retention Control Register"]
pub mod sys_sram_bankret;
#[doc = "SYS_FLASH_SIZE (r) register accessor: Flash Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_flash_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_flash_size`]
module"]
#[doc(alias = "SYS_FLASH_SIZE")]
pub type SysFlashSize = crate::Reg<sys_flash_size::SysFlashSizeSpec>;
#[doc = "Flash Size Register"]
pub mod sys_flash_size;
#[doc = "SYS_DIO_GLTFLT_CTL (rw) register accessor: Digital I/O Glitch Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_dio_gltflt_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_dio_gltflt_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_dio_gltflt_ctl`]
module"]
#[doc(alias = "SYS_DIO_GLTFLT_CTL")]
pub type SysDioGltfltCtl = crate::Reg<sys_dio_gltflt_ctl::SysDioGltfltCtlSpec>;
#[doc = "Digital I/O Glitch Filter Control Register"]
pub mod sys_dio_gltflt_ctl;
#[doc = "SYS_SECDATA_UNLOCK (rw) register accessor: IP Protected Secure Zone Data Access Unlock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_secdata_unlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_secdata_unlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_secdata_unlock`]
module"]
#[doc(alias = "SYS_SECDATA_UNLOCK")]
pub type SysSecdataUnlock = crate::Reg<sys_secdata_unlock::SysSecdataUnlockSpec>;
#[doc = "IP Protected Secure Zone Data Access Unlock Register"]
pub mod sys_secdata_unlock;
#[doc = "SYS_MASTER_UNLOCK (rw) register accessor: Master Unlock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_master_unlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_master_unlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_master_unlock`]
module"]
#[doc(alias = "SYS_MASTER_UNLOCK")]
pub type SysMasterUnlock = crate::Reg<sys_master_unlock::SysMasterUnlockSpec>;
#[doc = "Master Unlock Register"]
pub mod sys_master_unlock;
#[doc = "SYS_BOOTOVER_REQ (rw) register accessor: Boot Override Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_bootover_req::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_bootover_req::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_bootover_req`]
module"]
#[doc(alias = "SYS_BOOTOVER_REQ")]
pub type SysBootoverReq = crate::Reg<sys_bootover_req::SysBootoverReqSpec>;
#[doc = "Boot Override Request Register"]
pub mod sys_bootover_req;
#[doc = "SYS_BOOTOVER_ACK (rw) register accessor: Boot Override Acknowledge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_bootover_ack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_bootover_ack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_bootover_ack`]
module"]
#[doc(alias = "SYS_BOOTOVER_ACK")]
pub type SysBootoverAck = crate::Reg<sys_bootover_ack::SysBootoverAckSpec>;
#[doc = "Boot Override Acknowledge Register"]
pub mod sys_bootover_ack;
#[doc = "SYS_RESET_REQ (rw) register accessor: Reset Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_reset_req::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_reset_req::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_reset_req`]
module"]
#[doc(alias = "SYS_RESET_REQ")]
pub type SysResetReq = crate::Reg<sys_reset_req::SysResetReqSpec>;
#[doc = "Reset Request Register"]
pub mod sys_reset_req;
#[doc = "SYS_RESET_STATOVER (rw) register accessor: Reset Status and Override Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_reset_statover::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_reset_statover::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_reset_statover`]
module"]
#[doc(alias = "SYS_RESET_STATOVER")]
pub type SysResetStatover = crate::Reg<sys_reset_statover::SysResetStatoverSpec>;
#[doc = "Reset Status and Override Register"]
pub mod sys_reset_statover;
#[doc = "SYS_SYSTEM_STAT (r) register accessor: System Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_system_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_system_stat`]
module"]
#[doc(alias = "SYS_SYSTEM_STAT")]
pub type SysSystemStat = crate::Reg<sys_system_stat::SysSystemStatSpec>;
#[doc = "System Status Register"]
pub mod sys_system_stat;
