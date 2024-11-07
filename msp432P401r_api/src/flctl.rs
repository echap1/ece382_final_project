#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    flctl_power_stat: FlctlPowerStat,
    _reserved1: [u8; 0x0c],
    flctl_bank0_rdctl: FlctlBank0Rdctl,
    flctl_bank1_rdctl: FlctlBank1Rdctl,
    _reserved3: [u8; 0x08],
    flctl_rdbrst_ctlstat: FlctlRdbrstCtlstat,
    flctl_rdbrst_startaddr: FlctlRdbrstStartaddr,
    flctl_rdbrst_len: FlctlRdbrstLen,
    _reserved6: [u8; 0x10],
    flctl_rdbrst_failaddr: FlctlRdbrstFailaddr,
    flctl_rdbrst_failcnt: FlctlRdbrstFailcnt,
    _reserved8: [u8; 0x0c],
    flctl_prg_ctlstat: FlctlPrgCtlstat,
    flctl_prgbrst_ctlstat: FlctlPrgbrstCtlstat,
    flctl_prgbrst_startaddr: FlctlPrgbrstStartaddr,
    _reserved11: [u8; 0x04],
    flctl_prgbrst_data0_0: FlctlPrgbrstData0_0,
    flctl_prgbrst_data0_1: FlctlPrgbrstData0_1,
    flctl_prgbrst_data0_2: FlctlPrgbrstData0_2,
    flctl_prgbrst_data0_3: FlctlPrgbrstData0_3,
    flctl_prgbrst_data1_0: FlctlPrgbrstData1_0,
    flctl_prgbrst_data1_1: FlctlPrgbrstData1_1,
    flctl_prgbrst_data1_2: FlctlPrgbrstData1_2,
    flctl_prgbrst_data1_3: FlctlPrgbrstData1_3,
    flctl_prgbrst_data2_0: FlctlPrgbrstData2_0,
    flctl_prgbrst_data2_1: FlctlPrgbrstData2_1,
    flctl_prgbrst_data2_2: FlctlPrgbrstData2_2,
    flctl_prgbrst_data2_3: FlctlPrgbrstData2_3,
    flctl_prgbrst_data3_0: FlctlPrgbrstData3_0,
    flctl_prgbrst_data3_1: FlctlPrgbrstData3_1,
    flctl_prgbrst_data3_2: FlctlPrgbrstData3_2,
    flctl_prgbrst_data3_3: FlctlPrgbrstData3_3,
    flctl_erase_ctlstat: FlctlEraseCtlstat,
    flctl_erase_sectaddr: FlctlEraseSectaddr,
    _reserved29: [u8; 0x08],
    flctl_bank0_info_weprot: FlctlBank0InfoWeprot,
    flctl_bank0_main_weprot: FlctlBank0MainWeprot,
    _reserved31: [u8; 0x08],
    flctl_bank1_info_weprot: FlctlBank1InfoWeprot,
    flctl_bank1_main_weprot: FlctlBank1MainWeprot,
    _reserved33: [u8; 0x08],
    flctl_bmrk_ctlstat: FlctlBmrkCtlstat,
    flctl_bmrk_ifetch: FlctlBmrkIfetch,
    flctl_bmrk_dread: FlctlBmrkDread,
    flctl_bmrk_cmp: FlctlBmrkCmp,
    _reserved37: [u8; 0x10],
    flctl_ifg: FlctlIfg,
    flctl_ie: FlctlIe,
    flctl_clrifg: FlctlClrifg,
    flctl_setifg: FlctlSetifg,
    flctl_read_timctl: FlctlReadTimctl,
    flctl_readmargin_timctl: FlctlReadmarginTimctl,
    flctl_prgver_timctl: FlctlPrgverTimctl,
    flctl_ersver_timctl: FlctlErsverTimctl,
    flctl_lkgver_timctl: FlctlLkgverTimctl,
    flctl_program_timctl: FlctlProgramTimctl,
    flctl_erase_timctl: FlctlEraseTimctl,
    flctl_masserase_timctl: FlctlMasseraseTimctl,
    flctl_burstprg_timctl: FlctlBurstprgTimctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Power Status Register"]
    #[inline(always)]
    pub const fn flctl_power_stat(&self) -> &FlctlPowerStat {
        &self.flctl_power_stat
    }
    #[doc = "0x10 - Bank0 Read Control Register"]
    #[inline(always)]
    pub const fn flctl_bank0_rdctl(&self) -> &FlctlBank0Rdctl {
        &self.flctl_bank0_rdctl
    }
    #[doc = "0x14 - Bank1 Read Control Register"]
    #[inline(always)]
    pub const fn flctl_bank1_rdctl(&self) -> &FlctlBank1Rdctl {
        &self.flctl_bank1_rdctl
    }
    #[doc = "0x20 - Read Burst/Compare Control and Status Register"]
    #[inline(always)]
    pub const fn flctl_rdbrst_ctlstat(&self) -> &FlctlRdbrstCtlstat {
        &self.flctl_rdbrst_ctlstat
    }
    #[doc = "0x24 - Read Burst/Compare Start Address Register"]
    #[inline(always)]
    pub const fn flctl_rdbrst_startaddr(&self) -> &FlctlRdbrstStartaddr {
        &self.flctl_rdbrst_startaddr
    }
    #[doc = "0x28 - Read Burst/Compare Length Register"]
    #[inline(always)]
    pub const fn flctl_rdbrst_len(&self) -> &FlctlRdbrstLen {
        &self.flctl_rdbrst_len
    }
    #[doc = "0x3c - Read Burst/Compare Fail Address Register"]
    #[inline(always)]
    pub const fn flctl_rdbrst_failaddr(&self) -> &FlctlRdbrstFailaddr {
        &self.flctl_rdbrst_failaddr
    }
    #[doc = "0x40 - Read Burst/Compare Fail Count Register"]
    #[inline(always)]
    pub const fn flctl_rdbrst_failcnt(&self) -> &FlctlRdbrstFailcnt {
        &self.flctl_rdbrst_failcnt
    }
    #[doc = "0x50 - Program Control and Status Register"]
    #[inline(always)]
    pub const fn flctl_prg_ctlstat(&self) -> &FlctlPrgCtlstat {
        &self.flctl_prg_ctlstat
    }
    #[doc = "0x54 - Program Burst Control and Status Register"]
    #[inline(always)]
    pub const fn flctl_prgbrst_ctlstat(&self) -> &FlctlPrgbrstCtlstat {
        &self.flctl_prgbrst_ctlstat
    }
    #[doc = "0x58 - Program Burst Start Address Register"]
    #[inline(always)]
    pub const fn flctl_prgbrst_startaddr(&self) -> &FlctlPrgbrstStartaddr {
        &self.flctl_prgbrst_startaddr
    }
    #[doc = "0x60 - Program Burst Data0 Register0"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data0_0(&self) -> &FlctlPrgbrstData0_0 {
        &self.flctl_prgbrst_data0_0
    }
    #[doc = "0x64 - Program Burst Data0 Register1"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data0_1(&self) -> &FlctlPrgbrstData0_1 {
        &self.flctl_prgbrst_data0_1
    }
    #[doc = "0x68 - Program Burst Data0 Register2"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data0_2(&self) -> &FlctlPrgbrstData0_2 {
        &self.flctl_prgbrst_data0_2
    }
    #[doc = "0x6c - Program Burst Data0 Register3"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data0_3(&self) -> &FlctlPrgbrstData0_3 {
        &self.flctl_prgbrst_data0_3
    }
    #[doc = "0x70 - Program Burst Data1 Register0"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data1_0(&self) -> &FlctlPrgbrstData1_0 {
        &self.flctl_prgbrst_data1_0
    }
    #[doc = "0x74 - Program Burst Data1 Register1"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data1_1(&self) -> &FlctlPrgbrstData1_1 {
        &self.flctl_prgbrst_data1_1
    }
    #[doc = "0x78 - Program Burst Data1 Register2"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data1_2(&self) -> &FlctlPrgbrstData1_2 {
        &self.flctl_prgbrst_data1_2
    }
    #[doc = "0x7c - Program Burst Data1 Register3"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data1_3(&self) -> &FlctlPrgbrstData1_3 {
        &self.flctl_prgbrst_data1_3
    }
    #[doc = "0x80 - Program Burst Data2 Register0"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data2_0(&self) -> &FlctlPrgbrstData2_0 {
        &self.flctl_prgbrst_data2_0
    }
    #[doc = "0x84 - Program Burst Data2 Register1"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data2_1(&self) -> &FlctlPrgbrstData2_1 {
        &self.flctl_prgbrst_data2_1
    }
    #[doc = "0x88 - Program Burst Data2 Register2"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data2_2(&self) -> &FlctlPrgbrstData2_2 {
        &self.flctl_prgbrst_data2_2
    }
    #[doc = "0x8c - Program Burst Data2 Register3"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data2_3(&self) -> &FlctlPrgbrstData2_3 {
        &self.flctl_prgbrst_data2_3
    }
    #[doc = "0x90 - Program Burst Data3 Register0"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data3_0(&self) -> &FlctlPrgbrstData3_0 {
        &self.flctl_prgbrst_data3_0
    }
    #[doc = "0x94 - Program Burst Data3 Register1"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data3_1(&self) -> &FlctlPrgbrstData3_1 {
        &self.flctl_prgbrst_data3_1
    }
    #[doc = "0x98 - Program Burst Data3 Register2"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data3_2(&self) -> &FlctlPrgbrstData3_2 {
        &self.flctl_prgbrst_data3_2
    }
    #[doc = "0x9c - Program Burst Data3 Register3"]
    #[inline(always)]
    pub const fn flctl_prgbrst_data3_3(&self) -> &FlctlPrgbrstData3_3 {
        &self.flctl_prgbrst_data3_3
    }
    #[doc = "0xa0 - Erase Control and Status Register"]
    #[inline(always)]
    pub const fn flctl_erase_ctlstat(&self) -> &FlctlEraseCtlstat {
        &self.flctl_erase_ctlstat
    }
    #[doc = "0xa4 - Erase Sector Address Register"]
    #[inline(always)]
    pub const fn flctl_erase_sectaddr(&self) -> &FlctlEraseSectaddr {
        &self.flctl_erase_sectaddr
    }
    #[doc = "0xb0 - Information Memory Bank0 Write/Erase Protection Register"]
    #[inline(always)]
    pub const fn flctl_bank0_info_weprot(&self) -> &FlctlBank0InfoWeprot {
        &self.flctl_bank0_info_weprot
    }
    #[doc = "0xb4 - Main Memory Bank0 Write/Erase Protection Register"]
    #[inline(always)]
    pub const fn flctl_bank0_main_weprot(&self) -> &FlctlBank0MainWeprot {
        &self.flctl_bank0_main_weprot
    }
    #[doc = "0xc0 - Information Memory Bank1 Write/Erase Protection Register"]
    #[inline(always)]
    pub const fn flctl_bank1_info_weprot(&self) -> &FlctlBank1InfoWeprot {
        &self.flctl_bank1_info_weprot
    }
    #[doc = "0xc4 - Main Memory Bank1 Write/Erase Protection Register"]
    #[inline(always)]
    pub const fn flctl_bank1_main_weprot(&self) -> &FlctlBank1MainWeprot {
        &self.flctl_bank1_main_weprot
    }
    #[doc = "0xd0 - Benchmark Control and Status Register"]
    #[inline(always)]
    pub const fn flctl_bmrk_ctlstat(&self) -> &FlctlBmrkCtlstat {
        &self.flctl_bmrk_ctlstat
    }
    #[doc = "0xd4 - Benchmark Instruction Fetch Count Register"]
    #[inline(always)]
    pub const fn flctl_bmrk_ifetch(&self) -> &FlctlBmrkIfetch {
        &self.flctl_bmrk_ifetch
    }
    #[doc = "0xd8 - Benchmark Data Read Count Register"]
    #[inline(always)]
    pub const fn flctl_bmrk_dread(&self) -> &FlctlBmrkDread {
        &self.flctl_bmrk_dread
    }
    #[doc = "0xdc - Benchmark Count Compare Register"]
    #[inline(always)]
    pub const fn flctl_bmrk_cmp(&self) -> &FlctlBmrkCmp {
        &self.flctl_bmrk_cmp
    }
    #[doc = "0xf0 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn flctl_ifg(&self) -> &FlctlIfg {
        &self.flctl_ifg
    }
    #[doc = "0xf4 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn flctl_ie(&self) -> &FlctlIe {
        &self.flctl_ie
    }
    #[doc = "0xf8 - Clear Interrupt Flag Register"]
    #[inline(always)]
    pub const fn flctl_clrifg(&self) -> &FlctlClrifg {
        &self.flctl_clrifg
    }
    #[doc = "0xfc - Set Interrupt Flag Register"]
    #[inline(always)]
    pub const fn flctl_setifg(&self) -> &FlctlSetifg {
        &self.flctl_setifg
    }
    #[doc = "0x100 - Read Timing Control Register"]
    #[inline(always)]
    pub const fn flctl_read_timctl(&self) -> &FlctlReadTimctl {
        &self.flctl_read_timctl
    }
    #[doc = "0x104 - Read Margin Timing Control Register"]
    #[inline(always)]
    pub const fn flctl_readmargin_timctl(&self) -> &FlctlReadmarginTimctl {
        &self.flctl_readmargin_timctl
    }
    #[doc = "0x108 - Program Verify Timing Control Register"]
    #[inline(always)]
    pub const fn flctl_prgver_timctl(&self) -> &FlctlPrgverTimctl {
        &self.flctl_prgver_timctl
    }
    #[doc = "0x10c - Erase Verify Timing Control Register"]
    #[inline(always)]
    pub const fn flctl_ersver_timctl(&self) -> &FlctlErsverTimctl {
        &self.flctl_ersver_timctl
    }
    #[doc = "0x110 - Leakage Verify Timing Control Register"]
    #[inline(always)]
    pub const fn flctl_lkgver_timctl(&self) -> &FlctlLkgverTimctl {
        &self.flctl_lkgver_timctl
    }
    #[doc = "0x114 - Program Timing Control Register"]
    #[inline(always)]
    pub const fn flctl_program_timctl(&self) -> &FlctlProgramTimctl {
        &self.flctl_program_timctl
    }
    #[doc = "0x118 - Erase Timing Control Register"]
    #[inline(always)]
    pub const fn flctl_erase_timctl(&self) -> &FlctlEraseTimctl {
        &self.flctl_erase_timctl
    }
    #[doc = "0x11c - Mass Erase Timing Control Register"]
    #[inline(always)]
    pub const fn flctl_masserase_timctl(&self) -> &FlctlMasseraseTimctl {
        &self.flctl_masserase_timctl
    }
    #[doc = "0x120 - Burst Program Timing Control Register"]
    #[inline(always)]
    pub const fn flctl_burstprg_timctl(&self) -> &FlctlBurstprgTimctl {
        &self.flctl_burstprg_timctl
    }
}
#[doc = "FLCTL_POWER_STAT (r) register accessor: Power Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_power_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_power_stat`]
module"]
#[doc(alias = "FLCTL_POWER_STAT")]
pub type FlctlPowerStat = crate::Reg<flctl_power_stat::FlctlPowerStatSpec>;
#[doc = "Power Status Register"]
pub mod flctl_power_stat;
#[doc = "FLCTL_BANK0_RDCTL (rw) register accessor: Bank0 Read Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bank0_rdctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bank0_rdctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_bank0_rdctl`]
module"]
#[doc(alias = "FLCTL_BANK0_RDCTL")]
pub type FlctlBank0Rdctl = crate::Reg<flctl_bank0_rdctl::FlctlBank0RdctlSpec>;
#[doc = "Bank0 Read Control Register"]
pub mod flctl_bank0_rdctl;
#[doc = "FLCTL_BANK1_RDCTL (rw) register accessor: Bank1 Read Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bank1_rdctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bank1_rdctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_bank1_rdctl`]
module"]
#[doc(alias = "FLCTL_BANK1_RDCTL")]
pub type FlctlBank1Rdctl = crate::Reg<flctl_bank1_rdctl::FlctlBank1RdctlSpec>;
#[doc = "Bank1 Read Control Register"]
pub mod flctl_bank1_rdctl;
#[doc = "FLCTL_RDBRST_CTLSTAT (rw) register accessor: Read Burst/Compare Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_rdbrst_ctlstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_rdbrst_ctlstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_rdbrst_ctlstat`]
module"]
#[doc(alias = "FLCTL_RDBRST_CTLSTAT")]
pub type FlctlRdbrstCtlstat = crate::Reg<flctl_rdbrst_ctlstat::FlctlRdbrstCtlstatSpec>;
#[doc = "Read Burst/Compare Control and Status Register"]
pub mod flctl_rdbrst_ctlstat;
#[doc = "FLCTL_RDBRST_STARTADDR (rw) register accessor: Read Burst/Compare Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_rdbrst_startaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_rdbrst_startaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_rdbrst_startaddr`]
module"]
#[doc(alias = "FLCTL_RDBRST_STARTADDR")]
pub type FlctlRdbrstStartaddr = crate::Reg<flctl_rdbrst_startaddr::FlctlRdbrstStartaddrSpec>;
#[doc = "Read Burst/Compare Start Address Register"]
pub mod flctl_rdbrst_startaddr;
#[doc = "FLCTL_RDBRST_LEN (rw) register accessor: Read Burst/Compare Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_rdbrst_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_rdbrst_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_rdbrst_len`]
module"]
#[doc(alias = "FLCTL_RDBRST_LEN")]
pub type FlctlRdbrstLen = crate::Reg<flctl_rdbrst_len::FlctlRdbrstLenSpec>;
#[doc = "Read Burst/Compare Length Register"]
pub mod flctl_rdbrst_len;
#[doc = "FLCTL_RDBRST_FAILADDR (rw) register accessor: Read Burst/Compare Fail Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_rdbrst_failaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_rdbrst_failaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_rdbrst_failaddr`]
module"]
#[doc(alias = "FLCTL_RDBRST_FAILADDR")]
pub type FlctlRdbrstFailaddr = crate::Reg<flctl_rdbrst_failaddr::FlctlRdbrstFailaddrSpec>;
#[doc = "Read Burst/Compare Fail Address Register"]
pub mod flctl_rdbrst_failaddr;
#[doc = "FLCTL_RDBRST_FAILCNT (rw) register accessor: Read Burst/Compare Fail Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_rdbrst_failcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_rdbrst_failcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_rdbrst_failcnt`]
module"]
#[doc(alias = "FLCTL_RDBRST_FAILCNT")]
pub type FlctlRdbrstFailcnt = crate::Reg<flctl_rdbrst_failcnt::FlctlRdbrstFailcntSpec>;
#[doc = "Read Burst/Compare Fail Count Register"]
pub mod flctl_rdbrst_failcnt;
#[doc = "FLCTL_PRG_CTLSTAT (rw) register accessor: Program Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prg_ctlstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prg_ctlstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prg_ctlstat`]
module"]
#[doc(alias = "FLCTL_PRG_CTLSTAT")]
pub type FlctlPrgCtlstat = crate::Reg<flctl_prg_ctlstat::FlctlPrgCtlstatSpec>;
#[doc = "Program Control and Status Register"]
pub mod flctl_prg_ctlstat;
#[doc = "FLCTL_PRGBRST_CTLSTAT (rw) register accessor: Program Burst Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_ctlstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_ctlstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_ctlstat`]
module"]
#[doc(alias = "FLCTL_PRGBRST_CTLSTAT")]
pub type FlctlPrgbrstCtlstat = crate::Reg<flctl_prgbrst_ctlstat::FlctlPrgbrstCtlstatSpec>;
#[doc = "Program Burst Control and Status Register"]
pub mod flctl_prgbrst_ctlstat;
#[doc = "FLCTL_PRGBRST_STARTADDR (rw) register accessor: Program Burst Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_startaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_startaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_startaddr`]
module"]
#[doc(alias = "FLCTL_PRGBRST_STARTADDR")]
pub type FlctlPrgbrstStartaddr = crate::Reg<flctl_prgbrst_startaddr::FlctlPrgbrstStartaddrSpec>;
#[doc = "Program Burst Start Address Register"]
pub mod flctl_prgbrst_startaddr;
#[doc = "FLCTL_PRGBRST_DATA0_0 (rw) register accessor: Program Burst Data0 Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data0_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data0_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data0_0`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA0_0")]
pub type FlctlPrgbrstData0_0 = crate::Reg<flctl_prgbrst_data0_0::FlctlPrgbrstData0_0Spec>;
#[doc = "Program Burst Data0 Register0"]
pub mod flctl_prgbrst_data0_0;
#[doc = "FLCTL_PRGBRST_DATA0_1 (rw) register accessor: Program Burst Data0 Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data0_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data0_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data0_1`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA0_1")]
pub type FlctlPrgbrstData0_1 = crate::Reg<flctl_prgbrst_data0_1::FlctlPrgbrstData0_1Spec>;
#[doc = "Program Burst Data0 Register1"]
pub mod flctl_prgbrst_data0_1;
#[doc = "FLCTL_PRGBRST_DATA0_2 (rw) register accessor: Program Burst Data0 Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data0_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data0_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data0_2`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA0_2")]
pub type FlctlPrgbrstData0_2 = crate::Reg<flctl_prgbrst_data0_2::FlctlPrgbrstData0_2Spec>;
#[doc = "Program Burst Data0 Register2"]
pub mod flctl_prgbrst_data0_2;
#[doc = "FLCTL_PRGBRST_DATA0_3 (rw) register accessor: Program Burst Data0 Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data0_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data0_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data0_3`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA0_3")]
pub type FlctlPrgbrstData0_3 = crate::Reg<flctl_prgbrst_data0_3::FlctlPrgbrstData0_3Spec>;
#[doc = "Program Burst Data0 Register3"]
pub mod flctl_prgbrst_data0_3;
#[doc = "FLCTL_PRGBRST_DATA1_0 (rw) register accessor: Program Burst Data1 Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data1_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data1_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data1_0`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA1_0")]
pub type FlctlPrgbrstData1_0 = crate::Reg<flctl_prgbrst_data1_0::FlctlPrgbrstData1_0Spec>;
#[doc = "Program Burst Data1 Register0"]
pub mod flctl_prgbrst_data1_0;
#[doc = "FLCTL_PRGBRST_DATA1_1 (rw) register accessor: Program Burst Data1 Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data1_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data1_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data1_1`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA1_1")]
pub type FlctlPrgbrstData1_1 = crate::Reg<flctl_prgbrst_data1_1::FlctlPrgbrstData1_1Spec>;
#[doc = "Program Burst Data1 Register1"]
pub mod flctl_prgbrst_data1_1;
#[doc = "FLCTL_PRGBRST_DATA1_2 (rw) register accessor: Program Burst Data1 Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data1_2`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA1_2")]
pub type FlctlPrgbrstData1_2 = crate::Reg<flctl_prgbrst_data1_2::FlctlPrgbrstData1_2Spec>;
#[doc = "Program Burst Data1 Register2"]
pub mod flctl_prgbrst_data1_2;
#[doc = "FLCTL_PRGBRST_DATA1_3 (rw) register accessor: Program Burst Data1 Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data1_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data1_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data1_3`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA1_3")]
pub type FlctlPrgbrstData1_3 = crate::Reg<flctl_prgbrst_data1_3::FlctlPrgbrstData1_3Spec>;
#[doc = "Program Burst Data1 Register3"]
pub mod flctl_prgbrst_data1_3;
#[doc = "FLCTL_PRGBRST_DATA2_0 (rw) register accessor: Program Burst Data2 Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data2_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data2_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data2_0`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA2_0")]
pub type FlctlPrgbrstData2_0 = crate::Reg<flctl_prgbrst_data2_0::FlctlPrgbrstData2_0Spec>;
#[doc = "Program Burst Data2 Register0"]
pub mod flctl_prgbrst_data2_0;
#[doc = "FLCTL_PRGBRST_DATA2_1 (rw) register accessor: Program Burst Data2 Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data2_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data2_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data2_1`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA2_1")]
pub type FlctlPrgbrstData2_1 = crate::Reg<flctl_prgbrst_data2_1::FlctlPrgbrstData2_1Spec>;
#[doc = "Program Burst Data2 Register1"]
pub mod flctl_prgbrst_data2_1;
#[doc = "FLCTL_PRGBRST_DATA2_2 (rw) register accessor: Program Burst Data2 Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data2_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data2_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data2_2`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA2_2")]
pub type FlctlPrgbrstData2_2 = crate::Reg<flctl_prgbrst_data2_2::FlctlPrgbrstData2_2Spec>;
#[doc = "Program Burst Data2 Register2"]
pub mod flctl_prgbrst_data2_2;
#[doc = "FLCTL_PRGBRST_DATA2_3 (rw) register accessor: Program Burst Data2 Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data2_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data2_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data2_3`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA2_3")]
pub type FlctlPrgbrstData2_3 = crate::Reg<flctl_prgbrst_data2_3::FlctlPrgbrstData2_3Spec>;
#[doc = "Program Burst Data2 Register3"]
pub mod flctl_prgbrst_data2_3;
#[doc = "FLCTL_PRGBRST_DATA3_0 (rw) register accessor: Program Burst Data3 Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data3_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data3_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data3_0`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA3_0")]
pub type FlctlPrgbrstData3_0 = crate::Reg<flctl_prgbrst_data3_0::FlctlPrgbrstData3_0Spec>;
#[doc = "Program Burst Data3 Register0"]
pub mod flctl_prgbrst_data3_0;
#[doc = "FLCTL_PRGBRST_DATA3_1 (rw) register accessor: Program Burst Data3 Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data3_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data3_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data3_1`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA3_1")]
pub type FlctlPrgbrstData3_1 = crate::Reg<flctl_prgbrst_data3_1::FlctlPrgbrstData3_1Spec>;
#[doc = "Program Burst Data3 Register1"]
pub mod flctl_prgbrst_data3_1;
#[doc = "FLCTL_PRGBRST_DATA3_2 (rw) register accessor: Program Burst Data3 Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data3_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data3_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data3_2`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA3_2")]
pub type FlctlPrgbrstData3_2 = crate::Reg<flctl_prgbrst_data3_2::FlctlPrgbrstData3_2Spec>;
#[doc = "Program Burst Data3 Register2"]
pub mod flctl_prgbrst_data3_2;
#[doc = "FLCTL_PRGBRST_DATA3_3 (rw) register accessor: Program Burst Data3 Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgbrst_data3_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_prgbrst_data3_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgbrst_data3_3`]
module"]
#[doc(alias = "FLCTL_PRGBRST_DATA3_3")]
pub type FlctlPrgbrstData3_3 = crate::Reg<flctl_prgbrst_data3_3::FlctlPrgbrstData3_3Spec>;
#[doc = "Program Burst Data3 Register3"]
pub mod flctl_prgbrst_data3_3;
#[doc = "FLCTL_ERASE_CTLSTAT (rw) register accessor: Erase Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_erase_ctlstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_erase_ctlstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_erase_ctlstat`]
module"]
#[doc(alias = "FLCTL_ERASE_CTLSTAT")]
pub type FlctlEraseCtlstat = crate::Reg<flctl_erase_ctlstat::FlctlEraseCtlstatSpec>;
#[doc = "Erase Control and Status Register"]
pub mod flctl_erase_ctlstat;
#[doc = "FLCTL_ERASE_SECTADDR (rw) register accessor: Erase Sector Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_erase_sectaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_erase_sectaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_erase_sectaddr`]
module"]
#[doc(alias = "FLCTL_ERASE_SECTADDR")]
pub type FlctlEraseSectaddr = crate::Reg<flctl_erase_sectaddr::FlctlEraseSectaddrSpec>;
#[doc = "Erase Sector Address Register"]
pub mod flctl_erase_sectaddr;
#[doc = "FLCTL_BANK0_INFO_WEPROT (rw) register accessor: Information Memory Bank0 Write/Erase Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bank0_info_weprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bank0_info_weprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_bank0_info_weprot`]
module"]
#[doc(alias = "FLCTL_BANK0_INFO_WEPROT")]
pub type FlctlBank0InfoWeprot = crate::Reg<flctl_bank0_info_weprot::FlctlBank0InfoWeprotSpec>;
#[doc = "Information Memory Bank0 Write/Erase Protection Register"]
pub mod flctl_bank0_info_weprot;
#[doc = "FLCTL_BANK0_MAIN_WEPROT (rw) register accessor: Main Memory Bank0 Write/Erase Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bank0_main_weprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bank0_main_weprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_bank0_main_weprot`]
module"]
#[doc(alias = "FLCTL_BANK0_MAIN_WEPROT")]
pub type FlctlBank0MainWeprot = crate::Reg<flctl_bank0_main_weprot::FlctlBank0MainWeprotSpec>;
#[doc = "Main Memory Bank0 Write/Erase Protection Register"]
pub mod flctl_bank0_main_weprot;
#[doc = "FLCTL_BANK1_INFO_WEPROT (rw) register accessor: Information Memory Bank1 Write/Erase Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bank1_info_weprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bank1_info_weprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_bank1_info_weprot`]
module"]
#[doc(alias = "FLCTL_BANK1_INFO_WEPROT")]
pub type FlctlBank1InfoWeprot = crate::Reg<flctl_bank1_info_weprot::FlctlBank1InfoWeprotSpec>;
#[doc = "Information Memory Bank1 Write/Erase Protection Register"]
pub mod flctl_bank1_info_weprot;
#[doc = "FLCTL_BANK1_MAIN_WEPROT (rw) register accessor: Main Memory Bank1 Write/Erase Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bank1_main_weprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bank1_main_weprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_bank1_main_weprot`]
module"]
#[doc(alias = "FLCTL_BANK1_MAIN_WEPROT")]
pub type FlctlBank1MainWeprot = crate::Reg<flctl_bank1_main_weprot::FlctlBank1MainWeprotSpec>;
#[doc = "Main Memory Bank1 Write/Erase Protection Register"]
pub mod flctl_bank1_main_weprot;
#[doc = "FLCTL_BMRK_CTLSTAT (rw) register accessor: Benchmark Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bmrk_ctlstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bmrk_ctlstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_bmrk_ctlstat`]
module"]
#[doc(alias = "FLCTL_BMRK_CTLSTAT")]
pub type FlctlBmrkCtlstat = crate::Reg<flctl_bmrk_ctlstat::FlctlBmrkCtlstatSpec>;
#[doc = "Benchmark Control and Status Register"]
pub mod flctl_bmrk_ctlstat;
#[doc = "FLCTL_BMRK_IFETCH (rw) register accessor: Benchmark Instruction Fetch Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bmrk_ifetch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bmrk_ifetch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_bmrk_ifetch`]
module"]
#[doc(alias = "FLCTL_BMRK_IFETCH")]
pub type FlctlBmrkIfetch = crate::Reg<flctl_bmrk_ifetch::FlctlBmrkIfetchSpec>;
#[doc = "Benchmark Instruction Fetch Count Register"]
pub mod flctl_bmrk_ifetch;
#[doc = "FLCTL_BMRK_DREAD (rw) register accessor: Benchmark Data Read Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bmrk_dread::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bmrk_dread::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_bmrk_dread`]
module"]
#[doc(alias = "FLCTL_BMRK_DREAD")]
pub type FlctlBmrkDread = crate::Reg<flctl_bmrk_dread::FlctlBmrkDreadSpec>;
#[doc = "Benchmark Data Read Count Register"]
pub mod flctl_bmrk_dread;
#[doc = "FLCTL_BMRK_CMP (rw) register accessor: Benchmark Count Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bmrk_cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bmrk_cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_bmrk_cmp`]
module"]
#[doc(alias = "FLCTL_BMRK_CMP")]
pub type FlctlBmrkCmp = crate::Reg<flctl_bmrk_cmp::FlctlBmrkCmpSpec>;
#[doc = "Benchmark Count Compare Register"]
pub mod flctl_bmrk_cmp;
#[doc = "FLCTL_IFG (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_ifg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_ifg`]
module"]
#[doc(alias = "FLCTL_IFG")]
pub type FlctlIfg = crate::Reg<flctl_ifg::FlctlIfgSpec>;
#[doc = "Interrupt Flag Register"]
pub mod flctl_ifg;
#[doc = "FLCTL_IE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_ie`]
module"]
#[doc(alias = "FLCTL_IE")]
pub type FlctlIe = crate::Reg<flctl_ie::FlctlIeSpec>;
#[doc = "Interrupt Enable Register"]
pub mod flctl_ie;
#[doc = "FLCTL_CLRIFG (w) register accessor: Clear Interrupt Flag Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_clrifg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_clrifg`]
module"]
#[doc(alias = "FLCTL_CLRIFG")]
pub type FlctlClrifg = crate::Reg<flctl_clrifg::FlctlClrifgSpec>;
#[doc = "Clear Interrupt Flag Register"]
pub mod flctl_clrifg;
#[doc = "FLCTL_SETIFG (w) register accessor: Set Interrupt Flag Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_setifg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_setifg`]
module"]
#[doc(alias = "FLCTL_SETIFG")]
pub type FlctlSetifg = crate::Reg<flctl_setifg::FlctlSetifgSpec>;
#[doc = "Set Interrupt Flag Register"]
pub mod flctl_setifg;
#[doc = "FLCTL_READ_TIMCTL (r) register accessor: Read Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_read_timctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_read_timctl`]
module"]
#[doc(alias = "FLCTL_READ_TIMCTL")]
pub type FlctlReadTimctl = crate::Reg<flctl_read_timctl::FlctlReadTimctlSpec>;
#[doc = "Read Timing Control Register"]
pub mod flctl_read_timctl;
#[doc = "FLCTL_READMARGIN_TIMCTL (r) register accessor: Read Margin Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_readmargin_timctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_readmargin_timctl`]
module"]
#[doc(alias = "FLCTL_READMARGIN_TIMCTL")]
pub type FlctlReadmarginTimctl = crate::Reg<flctl_readmargin_timctl::FlctlReadmarginTimctlSpec>;
#[doc = "Read Margin Timing Control Register"]
pub mod flctl_readmargin_timctl;
#[doc = "FLCTL_PRGVER_TIMCTL (r) register accessor: Program Verify Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_prgver_timctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_prgver_timctl`]
module"]
#[doc(alias = "FLCTL_PRGVER_TIMCTL")]
pub type FlctlPrgverTimctl = crate::Reg<flctl_prgver_timctl::FlctlPrgverTimctlSpec>;
#[doc = "Program Verify Timing Control Register"]
pub mod flctl_prgver_timctl;
#[doc = "FLCTL_ERSVER_TIMCTL (r) register accessor: Erase Verify Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_ersver_timctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_ersver_timctl`]
module"]
#[doc(alias = "FLCTL_ERSVER_TIMCTL")]
pub type FlctlErsverTimctl = crate::Reg<flctl_ersver_timctl::FlctlErsverTimctlSpec>;
#[doc = "Erase Verify Timing Control Register"]
pub mod flctl_ersver_timctl;
#[doc = "FLCTL_LKGVER_TIMCTL (r) register accessor: Leakage Verify Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_lkgver_timctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_lkgver_timctl`]
module"]
#[doc(alias = "FLCTL_LKGVER_TIMCTL")]
pub type FlctlLkgverTimctl = crate::Reg<flctl_lkgver_timctl::FlctlLkgverTimctlSpec>;
#[doc = "Leakage Verify Timing Control Register"]
pub mod flctl_lkgver_timctl;
#[doc = "FLCTL_PROGRAM_TIMCTL (r) register accessor: Program Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_program_timctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_program_timctl`]
module"]
#[doc(alias = "FLCTL_PROGRAM_TIMCTL")]
pub type FlctlProgramTimctl = crate::Reg<flctl_program_timctl::FlctlProgramTimctlSpec>;
#[doc = "Program Timing Control Register"]
pub mod flctl_program_timctl;
#[doc = "FLCTL_ERASE_TIMCTL (r) register accessor: Erase Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_erase_timctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_erase_timctl`]
module"]
#[doc(alias = "FLCTL_ERASE_TIMCTL")]
pub type FlctlEraseTimctl = crate::Reg<flctl_erase_timctl::FlctlEraseTimctlSpec>;
#[doc = "Erase Timing Control Register"]
pub mod flctl_erase_timctl;
#[doc = "FLCTL_MASSERASE_TIMCTL (r) register accessor: Mass Erase Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_masserase_timctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_masserase_timctl`]
module"]
#[doc(alias = "FLCTL_MASSERASE_TIMCTL")]
pub type FlctlMasseraseTimctl = crate::Reg<flctl_masserase_timctl::FlctlMasseraseTimctlSpec>;
#[doc = "Mass Erase Timing Control Register"]
pub mod flctl_masserase_timctl;
#[doc = "FLCTL_BURSTPRG_TIMCTL (r) register accessor: Burst Program Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_burstprg_timctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flctl_burstprg_timctl`]
module"]
#[doc(alias = "FLCTL_BURSTPRG_TIMCTL")]
pub type FlctlBurstprgTimctl = crate::Reg<flctl_burstprg_timctl::FlctlBurstprgTimctlSpec>;
#[doc = "Burst Program Timing Control Register"]
pub mod flctl_burstprg_timctl;
