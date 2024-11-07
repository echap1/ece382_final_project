#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ucbx_ctlw0: UcbxCtlw0,
    ucbx_ctlw1: UcbxCtlw1,
    _reserved2: [u8; 0x02],
    ucbx_brw: UcbxBrw,
    ucbx_statw: UcbxStatw,
    ucbx_tbcnt: UcbxTbcnt,
    ucbx_rxbuf: UcbxRxbuf,
    ucbx_txbuf: UcbxTxbuf,
    _reserved7: [u8; 0x04],
    ucbx_i2coa0: UcbxI2coa0,
    ucbx_i2coa1: UcbxI2coa1,
    ucbx_i2coa2: UcbxI2coa2,
    ucbx_i2coa3: UcbxI2coa3,
    ucbx_addrx: UcbxAddrx,
    ucbx_addmask: UcbxAddmask,
    ucbx_i2csa: UcbxI2csa,
    _reserved14: [u8; 0x08],
    ucbx_ie: UcbxIe,
    ucbx_ifg: UcbxIfg,
    ucbx_iv: UcbxIv,
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub const fn ucbx_ctlw0(&self) -> &UcbxCtlw0 {
        &self.ucbx_ctlw0
    }
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    #[inline(always)]
    pub const fn ucbx_ctlw1(&self) -> &UcbxCtlw1 {
        &self.ucbx_ctlw1
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub const fn ucbx_brw(&self) -> &UcbxBrw {
        &self.ucbx_brw
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub const fn ucbx_statw(&self) -> &UcbxStatw {
        &self.ucbx_statw
    }
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    #[inline(always)]
    pub const fn ucbx_tbcnt(&self) -> &UcbxTbcnt {
        &self.ucbx_tbcnt
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub const fn ucbx_rxbuf(&self) -> &UcbxRxbuf {
        &self.ucbx_rxbuf
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub const fn ucbx_txbuf(&self) -> &UcbxTxbuf {
        &self.ucbx_txbuf
    }
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    #[inline(always)]
    pub const fn ucbx_i2coa0(&self) -> &UcbxI2coa0 {
        &self.ucbx_i2coa0
    }
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    #[inline(always)]
    pub const fn ucbx_i2coa1(&self) -> &UcbxI2coa1 {
        &self.ucbx_i2coa1
    }
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    #[inline(always)]
    pub const fn ucbx_i2coa2(&self) -> &UcbxI2coa2 {
        &self.ucbx_i2coa2
    }
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    #[inline(always)]
    pub const fn ucbx_i2coa3(&self) -> &UcbxI2coa3 {
        &self.ucbx_i2coa3
    }
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    #[inline(always)]
    pub const fn ucbx_addrx(&self) -> &UcbxAddrx {
        &self.ucbx_addrx
    }
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    #[inline(always)]
    pub const fn ucbx_addmask(&self) -> &UcbxAddmask {
        &self.ucbx_addmask
    }
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    #[inline(always)]
    pub const fn ucbx_i2csa(&self) -> &UcbxI2csa {
        &self.ucbx_i2csa
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucbx_ie(&self) -> &UcbxIe {
        &self.ucbx_ie
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub const fn ucbx_ifg(&self) -> &UcbxIfg {
        &self.ucbx_ifg
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucbx_iv(&self) -> &UcbxIv {
        &self.ucbx_iv
    }
}
#[doc = "UCBxCTLW0 (rw) register accessor: eUSCI_Bx Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_ctlw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_ctlw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_ctlw0`]
module"]
#[doc(alias = "UCBxCTLW0")]
pub type UcbxCtlw0 = crate::Reg<ucbx_ctlw0::UcbxCtlw0Spec>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucbx_ctlw0;
#[doc = "UCBxCTLW1 (rw) register accessor: eUSCI_Bx Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_ctlw1`]
module"]
#[doc(alias = "UCBxCTLW1")]
pub type UcbxCtlw1 = crate::Reg<ucbx_ctlw1::UcbxCtlw1Spec>;
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucbx_ctlw1;
#[doc = "UCBxBRW (rw) register accessor: eUSCI_Bx Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_brw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_brw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_brw`]
module"]
#[doc(alias = "UCBxBRW")]
pub type UcbxBrw = crate::Reg<ucbx_brw::UcbxBrwSpec>;
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucbx_brw;
#[doc = "UCBxSTATW (rw) register accessor: eUSCI_Bx Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_statw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_statw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_statw`]
module"]
#[doc(alias = "UCBxSTATW")]
pub type UcbxStatw = crate::Reg<ucbx_statw::UcbxStatwSpec>;
#[doc = "eUSCI_Bx Status Register"]
pub mod ucbx_statw;
#[doc = "UCBxTBCNT (rw) register accessor: eUSCI_Bx Byte Counter Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_tbcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_tbcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_tbcnt`]
module"]
#[doc(alias = "UCBxTBCNT")]
pub type UcbxTbcnt = crate::Reg<ucbx_tbcnt::UcbxTbcntSpec>;
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucbx_tbcnt;
#[doc = "UCBxRXBUF (r) register accessor: eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_rxbuf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_rxbuf`]
module"]
#[doc(alias = "UCBxRXBUF")]
pub type UcbxRxbuf = crate::Reg<ucbx_rxbuf::UcbxRxbufSpec>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucbx_rxbuf;
#[doc = "UCBxTXBUF (rw) register accessor: eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_txbuf`]
module"]
#[doc(alias = "UCBxTXBUF")]
pub type UcbxTxbuf = crate::Reg<ucbx_txbuf::UcbxTxbufSpec>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucbx_txbuf;
#[doc = "UCBxI2COA0 (rw) register accessor: eUSCI_Bx I2C Own Address 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_i2coa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_i2coa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_i2coa0`]
module"]
#[doc(alias = "UCBxI2COA0")]
pub type UcbxI2coa0 = crate::Reg<ucbx_i2coa0::UcbxI2coa0Spec>;
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucbx_i2coa0;
#[doc = "UCBxI2COA1 (rw) register accessor: eUSCI_Bx I2C Own Address 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_i2coa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_i2coa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_i2coa1`]
module"]
#[doc(alias = "UCBxI2COA1")]
pub type UcbxI2coa1 = crate::Reg<ucbx_i2coa1::UcbxI2coa1Spec>;
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucbx_i2coa1;
#[doc = "UCBxI2COA2 (rw) register accessor: eUSCI_Bx I2C Own Address 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_i2coa2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_i2coa2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_i2coa2`]
module"]
#[doc(alias = "UCBxI2COA2")]
pub type UcbxI2coa2 = crate::Reg<ucbx_i2coa2::UcbxI2coa2Spec>;
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucbx_i2coa2;
#[doc = "UCBxI2COA3 (rw) register accessor: eUSCI_Bx I2C Own Address 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_i2coa3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_i2coa3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_i2coa3`]
module"]
#[doc(alias = "UCBxI2COA3")]
pub type UcbxI2coa3 = crate::Reg<ucbx_i2coa3::UcbxI2coa3Spec>;
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucbx_i2coa3;
#[doc = "UCBxADDRX (r) register accessor: eUSCI_Bx I2C Received Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_addrx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_addrx`]
module"]
#[doc(alias = "UCBxADDRX")]
pub type UcbxAddrx = crate::Reg<ucbx_addrx::UcbxAddrxSpec>;
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucbx_addrx;
#[doc = "UCBxADDMASK (rw) register accessor: eUSCI_Bx I2C Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_addmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_addmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_addmask`]
module"]
#[doc(alias = "UCBxADDMASK")]
pub type UcbxAddmask = crate::Reg<ucbx_addmask::UcbxAddmaskSpec>;
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucbx_addmask;
#[doc = "UCBxI2CSA (rw) register accessor: eUSCI_Bx I2C Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_i2csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_i2csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_i2csa`]
module"]
#[doc(alias = "UCBxI2CSA")]
pub type UcbxI2csa = crate::Reg<ucbx_i2csa::UcbxI2csaSpec>;
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucbx_i2csa;
#[doc = "UCBxIE (rw) register accessor: eUSCI_Bx Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_ie`]
module"]
#[doc(alias = "UCBxIE")]
pub type UcbxIe = crate::Reg<ucbx_ie::UcbxIeSpec>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucbx_ie;
#[doc = "UCBxIFG (rw) register accessor: eUSCI_Bx Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_ifg`]
module"]
#[doc(alias = "UCBxIFG")]
pub type UcbxIfg = crate::Reg<ucbx_ifg::UcbxIfgSpec>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucbx_ifg;
#[doc = "UCBxIV (r) register accessor: eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucbx_iv`]
module"]
#[doc(alias = "UCBxIV")]
pub type UcbxIv = crate::Reg<ucbx_iv::UcbxIvSpec>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucbx_iv;
