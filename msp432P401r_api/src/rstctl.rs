#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rstctl_reset_req: RstctlResetReq,
    rstctl_hardreset_stat: RstctlHardresetStat,
    rstctl_hardreset_clr: RstctlHardresetClr,
    rstctl_hardreset_set: RstctlHardresetSet,
    rstctl_softreset_stat: RstctlSoftresetStat,
    rstctl_softreset_clr: RstctlSoftresetClr,
    rstctl_softreset_set: RstctlSoftresetSet,
    _reserved7: [u8; 0xe4],
    rstctl_pssreset_stat: RstctlPssresetStat,
    rstctl_pssreset_clr: RstctlPssresetClr,
    rstctl_pcmreset_stat: RstctlPcmresetStat,
    rstctl_pcmreset_clr: RstctlPcmresetClr,
    rstctl_pinreset_stat: RstctlPinresetStat,
    rstctl_pinreset_clr: RstctlPinresetClr,
    rstctl_rebootreset_stat: RstctlRebootresetStat,
    rstctl_rebootreset_clr: RstctlRebootresetClr,
    rstctl_csreset_stat: RstctlCsresetStat,
    rstctl_csreset_clr: RstctlCsresetClr,
}
impl RegisterBlock {
    #[doc = "0x00 - Reset Request Register"]
    #[inline(always)]
    pub const fn rstctl_reset_req(&self) -> &RstctlResetReq {
        &self.rstctl_reset_req
    }
    #[doc = "0x04 - Hard Reset Status Register"]
    #[inline(always)]
    pub const fn rstctl_hardreset_stat(&self) -> &RstctlHardresetStat {
        &self.rstctl_hardreset_stat
    }
    #[doc = "0x08 - Hard Reset Status Clear Register"]
    #[inline(always)]
    pub const fn rstctl_hardreset_clr(&self) -> &RstctlHardresetClr {
        &self.rstctl_hardreset_clr
    }
    #[doc = "0x0c - Hard Reset Status Set Register"]
    #[inline(always)]
    pub const fn rstctl_hardreset_set(&self) -> &RstctlHardresetSet {
        &self.rstctl_hardreset_set
    }
    #[doc = "0x10 - Soft Reset Status Register"]
    #[inline(always)]
    pub const fn rstctl_softreset_stat(&self) -> &RstctlSoftresetStat {
        &self.rstctl_softreset_stat
    }
    #[doc = "0x14 - Soft Reset Status Clear Register"]
    #[inline(always)]
    pub const fn rstctl_softreset_clr(&self) -> &RstctlSoftresetClr {
        &self.rstctl_softreset_clr
    }
    #[doc = "0x18 - Soft Reset Status Set Register"]
    #[inline(always)]
    pub const fn rstctl_softreset_set(&self) -> &RstctlSoftresetSet {
        &self.rstctl_softreset_set
    }
    #[doc = "0x100 - PSS Reset Status Register"]
    #[inline(always)]
    pub const fn rstctl_pssreset_stat(&self) -> &RstctlPssresetStat {
        &self.rstctl_pssreset_stat
    }
    #[doc = "0x104 - PSS Reset Status Clear Register"]
    #[inline(always)]
    pub const fn rstctl_pssreset_clr(&self) -> &RstctlPssresetClr {
        &self.rstctl_pssreset_clr
    }
    #[doc = "0x108 - PCM Reset Status Register"]
    #[inline(always)]
    pub const fn rstctl_pcmreset_stat(&self) -> &RstctlPcmresetStat {
        &self.rstctl_pcmreset_stat
    }
    #[doc = "0x10c - PCM Reset Status Clear Register"]
    #[inline(always)]
    pub const fn rstctl_pcmreset_clr(&self) -> &RstctlPcmresetClr {
        &self.rstctl_pcmreset_clr
    }
    #[doc = "0x110 - Pin Reset Status Register"]
    #[inline(always)]
    pub const fn rstctl_pinreset_stat(&self) -> &RstctlPinresetStat {
        &self.rstctl_pinreset_stat
    }
    #[doc = "0x114 - Pin Reset Status Clear Register"]
    #[inline(always)]
    pub const fn rstctl_pinreset_clr(&self) -> &RstctlPinresetClr {
        &self.rstctl_pinreset_clr
    }
    #[doc = "0x118 - Reboot Reset Status Register"]
    #[inline(always)]
    pub const fn rstctl_rebootreset_stat(&self) -> &RstctlRebootresetStat {
        &self.rstctl_rebootreset_stat
    }
    #[doc = "0x11c - Reboot Reset Status Clear Register"]
    #[inline(always)]
    pub const fn rstctl_rebootreset_clr(&self) -> &RstctlRebootresetClr {
        &self.rstctl_rebootreset_clr
    }
    #[doc = "0x120 - CS Reset Status Register"]
    #[inline(always)]
    pub const fn rstctl_csreset_stat(&self) -> &RstctlCsresetStat {
        &self.rstctl_csreset_stat
    }
    #[doc = "0x124 - CS Reset Status Clear Register"]
    #[inline(always)]
    pub const fn rstctl_csreset_clr(&self) -> &RstctlCsresetClr {
        &self.rstctl_csreset_clr
    }
}
#[doc = "RSTCTL_RESET_REQ (rw) register accessor: Reset Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_reset_req::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_reset_req::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_reset_req`]
module"]
#[doc(alias = "RSTCTL_RESET_REQ")]
pub type RstctlResetReq = crate::Reg<rstctl_reset_req::RstctlResetReqSpec>;
#[doc = "Reset Request Register"]
pub mod rstctl_reset_req;
#[doc = "RSTCTL_HARDRESET_STAT (r) register accessor: Hard Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_hardreset_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_hardreset_stat`]
module"]
#[doc(alias = "RSTCTL_HARDRESET_STAT")]
pub type RstctlHardresetStat = crate::Reg<rstctl_hardreset_stat::RstctlHardresetStatSpec>;
#[doc = "Hard Reset Status Register"]
pub mod rstctl_hardreset_stat;
#[doc = "RSTCTL_HARDRESET_CLR (rw) register accessor: Hard Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_hardreset_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_hardreset_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_hardreset_clr`]
module"]
#[doc(alias = "RSTCTL_HARDRESET_CLR")]
pub type RstctlHardresetClr = crate::Reg<rstctl_hardreset_clr::RstctlHardresetClrSpec>;
#[doc = "Hard Reset Status Clear Register"]
pub mod rstctl_hardreset_clr;
#[doc = "RSTCTL_HARDRESET_SET (rw) register accessor: Hard Reset Status Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_hardreset_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_hardreset_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_hardreset_set`]
module"]
#[doc(alias = "RSTCTL_HARDRESET_SET")]
pub type RstctlHardresetSet = crate::Reg<rstctl_hardreset_set::RstctlHardresetSetSpec>;
#[doc = "Hard Reset Status Set Register"]
pub mod rstctl_hardreset_set;
#[doc = "RSTCTL_SOFTRESET_STAT (r) register accessor: Soft Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_softreset_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_softreset_stat`]
module"]
#[doc(alias = "RSTCTL_SOFTRESET_STAT")]
pub type RstctlSoftresetStat = crate::Reg<rstctl_softreset_stat::RstctlSoftresetStatSpec>;
#[doc = "Soft Reset Status Register"]
pub mod rstctl_softreset_stat;
#[doc = "RSTCTL_SOFTRESET_CLR (rw) register accessor: Soft Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_softreset_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_softreset_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_softreset_clr`]
module"]
#[doc(alias = "RSTCTL_SOFTRESET_CLR")]
pub type RstctlSoftresetClr = crate::Reg<rstctl_softreset_clr::RstctlSoftresetClrSpec>;
#[doc = "Soft Reset Status Clear Register"]
pub mod rstctl_softreset_clr;
#[doc = "RSTCTL_SOFTRESET_SET (rw) register accessor: Soft Reset Status Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_softreset_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_softreset_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_softreset_set`]
module"]
#[doc(alias = "RSTCTL_SOFTRESET_SET")]
pub type RstctlSoftresetSet = crate::Reg<rstctl_softreset_set::RstctlSoftresetSetSpec>;
#[doc = "Soft Reset Status Set Register"]
pub mod rstctl_softreset_set;
#[doc = "RSTCTL_PSSRESET_STAT (r) register accessor: PSS Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pssreset_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_pssreset_stat`]
module"]
#[doc(alias = "RSTCTL_PSSRESET_STAT")]
pub type RstctlPssresetStat = crate::Reg<rstctl_pssreset_stat::RstctlPssresetStatSpec>;
#[doc = "PSS Reset Status Register"]
pub mod rstctl_pssreset_stat;
#[doc = "RSTCTL_PSSRESET_CLR (rw) register accessor: PSS Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pssreset_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_pssreset_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_pssreset_clr`]
module"]
#[doc(alias = "RSTCTL_PSSRESET_CLR")]
pub type RstctlPssresetClr = crate::Reg<rstctl_pssreset_clr::RstctlPssresetClrSpec>;
#[doc = "PSS Reset Status Clear Register"]
pub mod rstctl_pssreset_clr;
#[doc = "RSTCTL_PCMRESET_STAT (r) register accessor: PCM Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pcmreset_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_pcmreset_stat`]
module"]
#[doc(alias = "RSTCTL_PCMRESET_STAT")]
pub type RstctlPcmresetStat = crate::Reg<rstctl_pcmreset_stat::RstctlPcmresetStatSpec>;
#[doc = "PCM Reset Status Register"]
pub mod rstctl_pcmreset_stat;
#[doc = "RSTCTL_PCMRESET_CLR (rw) register accessor: PCM Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pcmreset_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_pcmreset_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_pcmreset_clr`]
module"]
#[doc(alias = "RSTCTL_PCMRESET_CLR")]
pub type RstctlPcmresetClr = crate::Reg<rstctl_pcmreset_clr::RstctlPcmresetClrSpec>;
#[doc = "PCM Reset Status Clear Register"]
pub mod rstctl_pcmreset_clr;
#[doc = "RSTCTL_PINRESET_STAT (r) register accessor: Pin Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pinreset_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_pinreset_stat`]
module"]
#[doc(alias = "RSTCTL_PINRESET_STAT")]
pub type RstctlPinresetStat = crate::Reg<rstctl_pinreset_stat::RstctlPinresetStatSpec>;
#[doc = "Pin Reset Status Register"]
pub mod rstctl_pinreset_stat;
#[doc = "RSTCTL_PINRESET_CLR (rw) register accessor: Pin Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_pinreset_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_pinreset_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_pinreset_clr`]
module"]
#[doc(alias = "RSTCTL_PINRESET_CLR")]
pub type RstctlPinresetClr = crate::Reg<rstctl_pinreset_clr::RstctlPinresetClrSpec>;
#[doc = "Pin Reset Status Clear Register"]
pub mod rstctl_pinreset_clr;
#[doc = "RSTCTL_REBOOTRESET_STAT (r) register accessor: Reboot Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_rebootreset_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_rebootreset_stat`]
module"]
#[doc(alias = "RSTCTL_REBOOTRESET_STAT")]
pub type RstctlRebootresetStat = crate::Reg<rstctl_rebootreset_stat::RstctlRebootresetStatSpec>;
#[doc = "Reboot Reset Status Register"]
pub mod rstctl_rebootreset_stat;
#[doc = "RSTCTL_REBOOTRESET_CLR (rw) register accessor: Reboot Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_rebootreset_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_rebootreset_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_rebootreset_clr`]
module"]
#[doc(alias = "RSTCTL_REBOOTRESET_CLR")]
pub type RstctlRebootresetClr = crate::Reg<rstctl_rebootreset_clr::RstctlRebootresetClrSpec>;
#[doc = "Reboot Reset Status Clear Register"]
pub mod rstctl_rebootreset_clr;
#[doc = "RSTCTL_CSRESET_STAT (r) register accessor: CS Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_csreset_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_csreset_stat`]
module"]
#[doc(alias = "RSTCTL_CSRESET_STAT")]
pub type RstctlCsresetStat = crate::Reg<rstctl_csreset_stat::RstctlCsresetStatSpec>;
#[doc = "CS Reset Status Register"]
pub mod rstctl_csreset_stat;
#[doc = "RSTCTL_CSRESET_CLR (rw) register accessor: CS Reset Status Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_csreset_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_csreset_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl_csreset_clr`]
module"]
#[doc(alias = "RSTCTL_CSRESET_CLR")]
pub type RstctlCsresetClr = crate::Reg<rstctl_csreset_clr::RstctlCsresetClrSpec>;
#[doc = "CS Reset Status Clear Register"]
pub mod rstctl_csreset_clr;
