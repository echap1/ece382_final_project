#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dma_device_cfg: DmaDeviceCfg,
    dma_sw_chtrig: DmaSwChtrig,
    _reserved2: [u8; 0x08],
    dma_ch_srccfg: [DmaChSrccfg; 32],
    _reserved3: [u8; 0x70],
    dma_int1_srccfg: DmaInt1Srccfg,
    dma_int2_srccfg: DmaInt2Srccfg,
    dma_int3_srccfg: DmaInt3Srccfg,
    _reserved6: [u8; 0x04],
    dma_int0_srcflg: DmaInt0Srcflg,
    dma_int0_clrflg: DmaInt0Clrflg,
    _reserved8: [u8; 0x0ee8],
    dma_stat: DmaStat,
    dma_cfg: DmaCfg,
    dma_ctlbase: DmaCtlbase,
    dma_altbase: DmaAltbase,
    dma_waitstat: DmaWaitstat,
    dma_swreq: DmaSwreq,
    dma_useburstset: DmaUseburstset,
    dma_useburstclr: DmaUseburstclr,
    dma_reqmaskset: DmaReqmaskset,
    dma_reqmaskclr: DmaReqmaskclr,
    dma_enaset: DmaEnaset,
    dma_enaclr: DmaEnaclr,
    dma_altset: DmaAltset,
    dma_altclr: DmaAltclr,
    dma_prioset: DmaPrioset,
    dma_prioclr: DmaPrioclr,
    _reserved24: [u8; 0x0c],
    dma_errclr: DmaErrclr,
}
impl RegisterBlock {
    #[doc = "0x00 - Device Configuration Status"]
    #[inline(always)]
    pub const fn dma_device_cfg(&self) -> &DmaDeviceCfg {
        &self.dma_device_cfg
    }
    #[doc = "0x04 - Software Channel Trigger Register"]
    #[inline(always)]
    pub const fn dma_sw_chtrig(&self) -> &DmaSwChtrig {
        &self.dma_sw_chtrig
    }
    #[doc = "0x10..0x90 - Channel n Source Configuration Register"]
    #[inline(always)]
    pub const fn dma_ch_srccfg(&self, n: usize) -> &DmaChSrccfg {
        &self.dma_ch_srccfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x90 - Channel n Source Configuration Register"]
    #[inline(always)]
    pub fn dma_ch_srccfg_iter(&self) -> impl Iterator<Item = &DmaChSrccfg> {
        self.dma_ch_srccfg.iter()
    }
    #[doc = "0x100 - Interrupt 1 Source Channel Configuration"]
    #[inline(always)]
    pub const fn dma_int1_srccfg(&self) -> &DmaInt1Srccfg {
        &self.dma_int1_srccfg
    }
    #[doc = "0x104 - Interrupt 2 Source Channel Configuration Register"]
    #[inline(always)]
    pub const fn dma_int2_srccfg(&self) -> &DmaInt2Srccfg {
        &self.dma_int2_srccfg
    }
    #[doc = "0x108 - Interrupt 3 Source Channel Configuration Register"]
    #[inline(always)]
    pub const fn dma_int3_srccfg(&self) -> &DmaInt3Srccfg {
        &self.dma_int3_srccfg
    }
    #[doc = "0x110 - Interrupt 0 Source Channel Flag Register"]
    #[inline(always)]
    pub const fn dma_int0_srcflg(&self) -> &DmaInt0Srcflg {
        &self.dma_int0_srcflg
    }
    #[doc = "0x114 - Interrupt 0 Source Channel Clear Flag Register"]
    #[inline(always)]
    pub const fn dma_int0_clrflg(&self) -> &DmaInt0Clrflg {
        &self.dma_int0_clrflg
    }
    #[doc = "0x1000 - Status Register"]
    #[inline(always)]
    pub const fn dma_stat(&self) -> &DmaStat {
        &self.dma_stat
    }
    #[doc = "0x1004 - Configuration Register"]
    #[inline(always)]
    pub const fn dma_cfg(&self) -> &DmaCfg {
        &self.dma_cfg
    }
    #[doc = "0x1008 - Channel Control Data Base Pointer Register"]
    #[inline(always)]
    pub const fn dma_ctlbase(&self) -> &DmaCtlbase {
        &self.dma_ctlbase
    }
    #[doc = "0x100c - Channel Alternate Control Data Base Pointer Register"]
    #[inline(always)]
    pub const fn dma_altbase(&self) -> &DmaAltbase {
        &self.dma_altbase
    }
    #[doc = "0x1010 - Channel Wait on Request Status Register"]
    #[inline(always)]
    pub const fn dma_waitstat(&self) -> &DmaWaitstat {
        &self.dma_waitstat
    }
    #[doc = "0x1014 - Channel Software Request Register"]
    #[inline(always)]
    pub const fn dma_swreq(&self) -> &DmaSwreq {
        &self.dma_swreq
    }
    #[doc = "0x1018 - Channel Useburst Set Register"]
    #[inline(always)]
    pub const fn dma_useburstset(&self) -> &DmaUseburstset {
        &self.dma_useburstset
    }
    #[doc = "0x101c - Channel Useburst Clear Register"]
    #[inline(always)]
    pub const fn dma_useburstclr(&self) -> &DmaUseburstclr {
        &self.dma_useburstclr
    }
    #[doc = "0x1020 - Channel Request Mask Set Register"]
    #[inline(always)]
    pub const fn dma_reqmaskset(&self) -> &DmaReqmaskset {
        &self.dma_reqmaskset
    }
    #[doc = "0x1024 - Channel Request Mask Clear Register"]
    #[inline(always)]
    pub const fn dma_reqmaskclr(&self) -> &DmaReqmaskclr {
        &self.dma_reqmaskclr
    }
    #[doc = "0x1028 - Channel Enable Set Register"]
    #[inline(always)]
    pub const fn dma_enaset(&self) -> &DmaEnaset {
        &self.dma_enaset
    }
    #[doc = "0x102c - Channel Enable Clear Register"]
    #[inline(always)]
    pub const fn dma_enaclr(&self) -> &DmaEnaclr {
        &self.dma_enaclr
    }
    #[doc = "0x1030 - Channel Primary-Alternate Set Register"]
    #[inline(always)]
    pub const fn dma_altset(&self) -> &DmaAltset {
        &self.dma_altset
    }
    #[doc = "0x1034 - Channel Primary-Alternate Clear Register"]
    #[inline(always)]
    pub const fn dma_altclr(&self) -> &DmaAltclr {
        &self.dma_altclr
    }
    #[doc = "0x1038 - Channel Priority Set Register"]
    #[inline(always)]
    pub const fn dma_prioset(&self) -> &DmaPrioset {
        &self.dma_prioset
    }
    #[doc = "0x103c - Channel Priority Clear Register"]
    #[inline(always)]
    pub const fn dma_prioclr(&self) -> &DmaPrioclr {
        &self.dma_prioclr
    }
    #[doc = "0x104c - Bus Error Clear Register"]
    #[inline(always)]
    pub const fn dma_errclr(&self) -> &DmaErrclr {
        &self.dma_errclr
    }
}
#[doc = "DMA_DEVICE_CFG (r) register accessor: Device Configuration Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_device_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_device_cfg`]
module"]
#[doc(alias = "DMA_DEVICE_CFG")]
pub type DmaDeviceCfg = crate::Reg<dma_device_cfg::DmaDeviceCfgSpec>;
#[doc = "Device Configuration Status"]
pub mod dma_device_cfg;
#[doc = "DMA_SW_CHTRIG (rw) register accessor: Software Channel Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sw_chtrig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sw_chtrig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sw_chtrig`]
module"]
#[doc(alias = "DMA_SW_CHTRIG")]
pub type DmaSwChtrig = crate::Reg<dma_sw_chtrig::DmaSwChtrigSpec>;
#[doc = "Software Channel Trigger Register"]
pub mod dma_sw_chtrig;
#[doc = "DMA_CH_SRCCFG (rw) register accessor: Channel n Source Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ch_srccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ch_srccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ch_srccfg`]
module"]
#[doc(alias = "DMA_CH_SRCCFG")]
pub type DmaChSrccfg = crate::Reg<dma_ch_srccfg::DmaChSrccfgSpec>;
#[doc = "Channel n Source Configuration Register"]
pub mod dma_ch_srccfg;
#[doc = "DMA_INT1_SRCCFG (rw) register accessor: Interrupt 1 Source Channel Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int1_srccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int1_srccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int1_srccfg`]
module"]
#[doc(alias = "DMA_INT1_SRCCFG")]
pub type DmaInt1Srccfg = crate::Reg<dma_int1_srccfg::DmaInt1SrccfgSpec>;
#[doc = "Interrupt 1 Source Channel Configuration"]
pub mod dma_int1_srccfg;
#[doc = "DMA_INT2_SRCCFG (rw) register accessor: Interrupt 2 Source Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int2_srccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int2_srccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int2_srccfg`]
module"]
#[doc(alias = "DMA_INT2_SRCCFG")]
pub type DmaInt2Srccfg = crate::Reg<dma_int2_srccfg::DmaInt2SrccfgSpec>;
#[doc = "Interrupt 2 Source Channel Configuration Register"]
pub mod dma_int2_srccfg;
#[doc = "DMA_INT3_SRCCFG (rw) register accessor: Interrupt 3 Source Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int3_srccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int3_srccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int3_srccfg`]
module"]
#[doc(alias = "DMA_INT3_SRCCFG")]
pub type DmaInt3Srccfg = crate::Reg<dma_int3_srccfg::DmaInt3SrccfgSpec>;
#[doc = "Interrupt 3 Source Channel Configuration Register"]
pub mod dma_int3_srccfg;
#[doc = "DMA_INT0_SRCFLG (r) register accessor: Interrupt 0 Source Channel Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int0_srcflg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int0_srcflg`]
module"]
#[doc(alias = "DMA_INT0_SRCFLG")]
pub type DmaInt0Srcflg = crate::Reg<dma_int0_srcflg::DmaInt0SrcflgSpec>;
#[doc = "Interrupt 0 Source Channel Flag Register"]
pub mod dma_int0_srcflg;
#[doc = "DMA_INT0_CLRFLG (w) register accessor: Interrupt 0 Source Channel Clear Flag Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int0_clrflg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int0_clrflg`]
module"]
#[doc(alias = "DMA_INT0_CLRFLG")]
pub type DmaInt0Clrflg = crate::Reg<dma_int0_clrflg::DmaInt0ClrflgSpec>;
#[doc = "Interrupt 0 Source Channel Clear Flag Register"]
pub mod dma_int0_clrflg;
#[doc = "DMA_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_stat`]
module"]
#[doc(alias = "DMA_STAT")]
pub type DmaStat = crate::Reg<dma_stat::DmaStatSpec>;
#[doc = "Status Register"]
pub mod dma_stat;
#[doc = "DMA_CFG (w) register accessor: Configuration Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cfg`]
module"]
#[doc(alias = "DMA_CFG")]
pub type DmaCfg = crate::Reg<dma_cfg::DmaCfgSpec>;
#[doc = "Configuration Register"]
pub mod dma_cfg;
#[doc = "DMA_CTLBASE (rw) register accessor: Channel Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ctlbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ctlbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ctlbase`]
module"]
#[doc(alias = "DMA_CTLBASE")]
pub type DmaCtlbase = crate::Reg<dma_ctlbase::DmaCtlbaseSpec>;
#[doc = "Channel Control Data Base Pointer Register"]
pub mod dma_ctlbase;
#[doc = "DMA_ALTBASE (r) register accessor: Channel Alternate Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_altbase::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_altbase`]
module"]
#[doc(alias = "DMA_ALTBASE")]
pub type DmaAltbase = crate::Reg<dma_altbase::DmaAltbaseSpec>;
#[doc = "Channel Alternate Control Data Base Pointer Register"]
pub mod dma_altbase;
#[doc = "DMA_WAITSTAT (r) register accessor: Channel Wait on Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_waitstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_waitstat`]
module"]
#[doc(alias = "DMA_WAITSTAT")]
pub type DmaWaitstat = crate::Reg<dma_waitstat::DmaWaitstatSpec>;
#[doc = "Channel Wait on Request Status Register"]
pub mod dma_waitstat;
#[doc = "DMA_SWREQ (w) register accessor: Channel Software Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_swreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_swreq`]
module"]
#[doc(alias = "DMA_SWREQ")]
pub type DmaSwreq = crate::Reg<dma_swreq::DmaSwreqSpec>;
#[doc = "Channel Software Request Register"]
pub mod dma_swreq;
#[doc = "DMA_USEBURSTSET (rw) register accessor: Channel Useburst Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_useburstset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_useburstset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_useburstset`]
module"]
#[doc(alias = "DMA_USEBURSTSET")]
pub type DmaUseburstset = crate::Reg<dma_useburstset::DmaUseburstsetSpec>;
#[doc = "Channel Useburst Set Register"]
pub mod dma_useburstset;
#[doc = "DMA_USEBURSTCLR (w) register accessor: Channel Useburst Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_useburstclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_useburstclr`]
module"]
#[doc(alias = "DMA_USEBURSTCLR")]
pub type DmaUseburstclr = crate::Reg<dma_useburstclr::DmaUseburstclrSpec>;
#[doc = "Channel Useburst Clear Register"]
pub mod dma_useburstclr;
#[doc = "DMA_REQMASKSET (rw) register accessor: Channel Request Mask Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_reqmaskset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_reqmaskset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_reqmaskset`]
module"]
#[doc(alias = "DMA_REQMASKSET")]
pub type DmaReqmaskset = crate::Reg<dma_reqmaskset::DmaReqmasksetSpec>;
#[doc = "Channel Request Mask Set Register"]
pub mod dma_reqmaskset;
#[doc = "DMA_REQMASKCLR (w) register accessor: Channel Request Mask Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_reqmaskclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_reqmaskclr`]
module"]
#[doc(alias = "DMA_REQMASKCLR")]
pub type DmaReqmaskclr = crate::Reg<dma_reqmaskclr::DmaReqmaskclrSpec>;
#[doc = "Channel Request Mask Clear Register"]
pub mod dma_reqmaskclr;
#[doc = "DMA_ENASET (rw) register accessor: Channel Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_enaset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_enaset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_enaset`]
module"]
#[doc(alias = "DMA_ENASET")]
pub type DmaEnaset = crate::Reg<dma_enaset::DmaEnasetSpec>;
#[doc = "Channel Enable Set Register"]
pub mod dma_enaset;
#[doc = "DMA_ENACLR (w) register accessor: Channel Enable Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_enaclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_enaclr`]
module"]
#[doc(alias = "DMA_ENACLR")]
pub type DmaEnaclr = crate::Reg<dma_enaclr::DmaEnaclrSpec>;
#[doc = "Channel Enable Clear Register"]
pub mod dma_enaclr;
#[doc = "DMA_ALTSET (rw) register accessor: Channel Primary-Alternate Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_altset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_altset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_altset`]
module"]
#[doc(alias = "DMA_ALTSET")]
pub type DmaAltset = crate::Reg<dma_altset::DmaAltsetSpec>;
#[doc = "Channel Primary-Alternate Set Register"]
pub mod dma_altset;
#[doc = "DMA_ALTCLR (w) register accessor: Channel Primary-Alternate Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_altclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_altclr`]
module"]
#[doc(alias = "DMA_ALTCLR")]
pub type DmaAltclr = crate::Reg<dma_altclr::DmaAltclrSpec>;
#[doc = "Channel Primary-Alternate Clear Register"]
pub mod dma_altclr;
#[doc = "DMA_PRIOSET (rw) register accessor: Channel Priority Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_prioset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_prioset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_prioset`]
module"]
#[doc(alias = "DMA_PRIOSET")]
pub type DmaPrioset = crate::Reg<dma_prioset::DmaPriosetSpec>;
#[doc = "Channel Priority Set Register"]
pub mod dma_prioset;
#[doc = "DMA_PRIOCLR (w) register accessor: Channel Priority Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_prioclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_prioclr`]
module"]
#[doc(alias = "DMA_PRIOCLR")]
pub type DmaPrioclr = crate::Reg<dma_prioclr::DmaPrioclrSpec>;
#[doc = "Channel Priority Clear Register"]
pub mod dma_prioclr;
#[doc = "DMA_ERRCLR (rw) register accessor: Bus Error Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_errclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_errclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_errclr`]
module"]
#[doc(alias = "DMA_ERRCLR")]
pub type DmaErrclr = crate::Reg<dma_errclr::DmaErrclrSpec>;
#[doc = "Bus Error Clear Register"]
pub mod dma_errclr;
