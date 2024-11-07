#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tax_ctl: TaxCtl,
    tax_cctl: [TaxCctl; 5],
    _reserved2: [u8; 0x04],
    tax_r: TaxR,
    tax_ccr: [TaxCcr; 5],
    _reserved4: [u8; 0x04],
    tax_ex0: TaxEx0,
    _reserved5: [u8; 0x0c],
    tax_iv: TaxIv,
}
impl RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    #[inline(always)]
    pub const fn tax_ctl(&self) -> &TaxCtl {
        &self.tax_ctl
    }
    #[doc = "0x02..0x0c - Timer_A Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tax_cctl(&self, n: usize) -> &TaxCctl {
        &self.tax_cctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x02..0x0c - Timer_A Capture/Compare Control Register"]
    #[inline(always)]
    pub fn tax_cctl_iter(&self) -> impl Iterator<Item = &TaxCctl> {
        self.tax_cctl.iter()
    }
    #[doc = "0x10 - TimerA register"]
    #[inline(always)]
    pub const fn tax_r(&self) -> &TaxR {
        &self.tax_r
    }
    #[doc = "0x12..0x1c - Timer_A Capture/Compare Register"]
    #[inline(always)]
    pub const fn tax_ccr(&self, n: usize) -> &TaxCcr {
        &self.tax_ccr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12..0x1c - Timer_A Capture/Compare Register"]
    #[inline(always)]
    pub fn tax_ccr_iter(&self) -> impl Iterator<Item = &TaxCcr> {
        self.tax_ccr.iter()
    }
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    #[inline(always)]
    pub const fn tax_ex0(&self) -> &TaxEx0 {
        &self.tax_ex0
    }
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn tax_iv(&self) -> &TaxIv {
        &self.tax_iv
    }
}
#[doc = "TAxCTL (rw) register accessor: TimerAx Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tax_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tax_ctl`]
module"]
#[doc(alias = "TAxCTL")]
pub type TaxCtl = crate::Reg<tax_ctl::TaxCtlSpec>;
#[doc = "TimerAx Control Register"]
pub mod tax_ctl;
#[doc = "TAxCCTL (rw) register accessor: Timer_A Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_cctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tax_cctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tax_cctl`]
module"]
#[doc(alias = "TAxCCTL")]
pub type TaxCctl = crate::Reg<tax_cctl::TaxCctlSpec>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod tax_cctl;
#[doc = "TAxR (rw) register accessor: TimerA register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tax_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tax_r`]
module"]
#[doc(alias = "TAxR")]
pub type TaxR = crate::Reg<tax_r::TaxRSpec>;
#[doc = "TimerA register"]
pub mod tax_r;
#[doc = "TAxCCR (rw) register accessor: Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tax_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tax_ccr`]
module"]
#[doc(alias = "TAxCCR")]
pub type TaxCcr = crate::Reg<tax_ccr::TaxCcrSpec>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod tax_ccr;
#[doc = "TAxEX0 (rw) register accessor: TimerAx Expansion 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_ex0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tax_ex0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tax_ex0`]
module"]
#[doc(alias = "TAxEX0")]
pub type TaxEx0 = crate::Reg<tax_ex0::TaxEx0Spec>;
#[doc = "TimerAx Expansion 0 Register"]
pub mod tax_ex0;
#[doc = "TAxIV (r) register accessor: TimerAx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tax_iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tax_iv`]
module"]
#[doc(alias = "TAxIV")]
pub type TaxIv = crate::Reg<tax_iv::TaxIvSpec>;
#[doc = "TimerAx Interrupt Vector Register"]
pub mod tax_iv;
