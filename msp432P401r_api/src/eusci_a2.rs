#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ucax_ctlw0: UcaxCtlw0,
    ucax_ctlw1: UcaxCtlw1,
    _reserved2: [u8; 0x02],
    ucax_brw: UcaxBrw,
    ucax_mctlw: UcaxMctlw,
    ucax_statw: UcaxStatw,
    ucax_rxbuf: UcaxRxbuf,
    ucax_txbuf: UcaxTxbuf,
    ucax_abctl: UcaxAbctl,
    ucax_irctl: UcaxIrctl,
    _reserved9: [u8; 0x06],
    ucax_ie: UcaxIe,
    ucax_ifg: UcaxIfg,
    ucax_iv: UcaxIv,
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub const fn ucax_ctlw0(&self) -> &UcaxCtlw0 {
        &self.ucax_ctlw0
    }
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    #[inline(always)]
    pub const fn ucax_ctlw1(&self) -> &UcaxCtlw1 {
        &self.ucax_ctlw1
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub const fn ucax_brw(&self) -> &UcaxBrw {
        &self.ucax_brw
    }
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    #[inline(always)]
    pub const fn ucax_mctlw(&self) -> &UcaxMctlw {
        &self.ucax_mctlw
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub const fn ucax_statw(&self) -> &UcaxStatw {
        &self.ucax_statw
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub const fn ucax_rxbuf(&self) -> &UcaxRxbuf {
        &self.ucax_rxbuf
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub const fn ucax_txbuf(&self) -> &UcaxTxbuf {
        &self.ucax_txbuf
    }
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    #[inline(always)]
    pub const fn ucax_abctl(&self) -> &UcaxAbctl {
        &self.ucax_abctl
    }
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    #[inline(always)]
    pub const fn ucax_irctl(&self) -> &UcaxIrctl {
        &self.ucax_irctl
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucax_ie(&self) -> &UcaxIe {
        &self.ucax_ie
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub const fn ucax_ifg(&self) -> &UcaxIfg {
        &self.ucax_ifg
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucax_iv(&self) -> &UcaxIv {
        &self.ucax_iv
    }
}
#[doc = "UCAxCTLW0 (rw) register accessor: eUSCI_Ax Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_ctlw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_ctlw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_ctlw0`]
module"]
#[doc(alias = "UCAxCTLW0")]
pub type UcaxCtlw0 = crate::Reg<ucax_ctlw0::UcaxCtlw0Spec>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod ucax_ctlw0;
#[doc = "UCAxCTLW1 (rw) register accessor: eUSCI_Ax Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_ctlw1`]
module"]
#[doc(alias = "UCAxCTLW1")]
pub type UcaxCtlw1 = crate::Reg<ucax_ctlw1::UcaxCtlw1Spec>;
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod ucax_ctlw1;
#[doc = "UCAxBRW (rw) register accessor: eUSCI_Ax Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_brw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_brw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_brw`]
module"]
#[doc(alias = "UCAxBRW")]
pub type UcaxBrw = crate::Reg<ucax_brw::UcaxBrwSpec>;
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod ucax_brw;
#[doc = "UCAxMCTLW (rw) register accessor: eUSCI_Ax Modulation Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_mctlw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_mctlw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_mctlw`]
module"]
#[doc(alias = "UCAxMCTLW")]
pub type UcaxMctlw = crate::Reg<ucax_mctlw::UcaxMctlwSpec>;
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod ucax_mctlw;
#[doc = "UCAxSTATW (rw) register accessor: eUSCI_Ax Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_statw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_statw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_statw`]
module"]
#[doc(alias = "UCAxSTATW")]
pub type UcaxStatw = crate::Reg<ucax_statw::UcaxStatwSpec>;
#[doc = "eUSCI_Ax Status Register"]
pub mod ucax_statw;
#[doc = "UCAxRXBUF (r) register accessor: eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_rxbuf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_rxbuf`]
module"]
#[doc(alias = "UCAxRXBUF")]
pub type UcaxRxbuf = crate::Reg<ucax_rxbuf::UcaxRxbufSpec>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod ucax_rxbuf;
#[doc = "UCAxTXBUF (rw) register accessor: eUSCI_Ax Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_txbuf`]
module"]
#[doc(alias = "UCAxTXBUF")]
pub type UcaxTxbuf = crate::Reg<ucax_txbuf::UcaxTxbufSpec>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod ucax_txbuf;
#[doc = "UCAxABCTL (rw) register accessor: eUSCI_Ax Auto Baud Rate Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_abctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_abctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_abctl`]
module"]
#[doc(alias = "UCAxABCTL")]
pub type UcaxAbctl = crate::Reg<ucax_abctl::UcaxAbctlSpec>;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod ucax_abctl;
#[doc = "UCAxIRCTL (rw) register accessor: eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_irctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_irctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_irctl`]
module"]
#[doc(alias = "UCAxIRCTL")]
pub type UcaxIrctl = crate::Reg<ucax_irctl::UcaxIrctlSpec>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod ucax_irctl;
#[doc = "UCAxIE (rw) register accessor: eUSCI_Ax Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_ie`]
module"]
#[doc(alias = "UCAxIE")]
pub type UcaxIe = crate::Reg<ucax_ie::UcaxIeSpec>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod ucax_ie;
#[doc = "UCAxIFG (rw) register accessor: eUSCI_Ax Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_ifg`]
module"]
#[doc(alias = "UCAxIFG")]
pub type UcaxIfg = crate::Reg<ucax_ifg::UcaxIfgSpec>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod ucax_ifg;
#[doc = "UCAxIV (r) register accessor: eUSCI_Ax Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucax_iv`]
module"]
#[doc(alias = "UCAxIV")]
pub type UcaxIv = crate::Reg<ucax_iv::UcaxIvSpec>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod ucax_iv;
