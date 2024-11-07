#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cskey: Cskey,
    csctl0: Csctl0,
    csctl1: Csctl1,
    csctl2: Csctl2,
    csctl3: Csctl3,
    _reserved5: [u8; 0x1c],
    csclken: Csclken,
    csstat: Csstat,
    _reserved7: [u8; 0x08],
    csie: Csie,
    _reserved8: [u8; 0x04],
    csifg: Csifg,
    _reserved9: [u8; 0x04],
    csclrifg: Csclrifg,
    _reserved10: [u8; 0x04],
    cssetifg: Cssetifg,
    _reserved11: [u8; 0x04],
    csdcoercal0: Csdcoercal0,
    csdcoercal1: Csdcoercal1,
}
impl RegisterBlock {
    #[doc = "0x00 - Key Register"]
    #[inline(always)]
    pub const fn cskey(&self) -> &Cskey {
        &self.cskey
    }
    #[doc = "0x04 - Control 0 Register"]
    #[inline(always)]
    pub const fn csctl0(&self) -> &Csctl0 {
        &self.csctl0
    }
    #[doc = "0x08 - Control 1 Register"]
    #[inline(always)]
    pub const fn csctl1(&self) -> &Csctl1 {
        &self.csctl1
    }
    #[doc = "0x0c - Control 2 Register"]
    #[inline(always)]
    pub const fn csctl2(&self) -> &Csctl2 {
        &self.csctl2
    }
    #[doc = "0x10 - Control 3 Register"]
    #[inline(always)]
    pub const fn csctl3(&self) -> &Csctl3 {
        &self.csctl3
    }
    #[doc = "0x30 - Clock Enable Register"]
    #[inline(always)]
    pub const fn csclken(&self) -> &Csclken {
        &self.csclken
    }
    #[doc = "0x34 - Status Register"]
    #[inline(always)]
    pub const fn csstat(&self) -> &Csstat {
        &self.csstat
    }
    #[doc = "0x40 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn csie(&self) -> &Csie {
        &self.csie
    }
    #[doc = "0x48 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn csifg(&self) -> &Csifg {
        &self.csifg
    }
    #[doc = "0x50 - Clear Interrupt Flag Register"]
    #[inline(always)]
    pub const fn csclrifg(&self) -> &Csclrifg {
        &self.csclrifg
    }
    #[doc = "0x58 - Set Interrupt Flag Register"]
    #[inline(always)]
    pub const fn cssetifg(&self) -> &Cssetifg {
        &self.cssetifg
    }
    #[doc = "0x60 - DCO External Resistor Cailbration 0 Register"]
    #[inline(always)]
    pub const fn csdcoercal0(&self) -> &Csdcoercal0 {
        &self.csdcoercal0
    }
    #[doc = "0x64 - DCO External Resistor Calibration 1 Register"]
    #[inline(always)]
    pub const fn csdcoercal1(&self) -> &Csdcoercal1 {
        &self.csdcoercal1
    }
}
#[doc = "CSKEY (rw) register accessor: Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cskey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cskey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cskey`]
module"]
#[doc(alias = "CSKEY")]
pub type Cskey = crate::Reg<cskey::CskeySpec>;
#[doc = "Key Register"]
pub mod cskey;
#[doc = "CSCTL0 (rw) register accessor: Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl0`]
module"]
#[doc(alias = "CSCTL0")]
pub type Csctl0 = crate::Reg<csctl0::Csctl0Spec>;
#[doc = "Control 0 Register"]
pub mod csctl0;
#[doc = "CSCTL1 (rw) register accessor: Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl1`]
module"]
#[doc(alias = "CSCTL1")]
pub type Csctl1 = crate::Reg<csctl1::Csctl1Spec>;
#[doc = "Control 1 Register"]
pub mod csctl1;
#[doc = "CSCTL2 (rw) register accessor: Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl2`]
module"]
#[doc(alias = "CSCTL2")]
pub type Csctl2 = crate::Reg<csctl2::Csctl2Spec>;
#[doc = "Control 2 Register"]
pub mod csctl2;
#[doc = "CSCTL3 (rw) register accessor: Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl3`]
module"]
#[doc(alias = "CSCTL3")]
pub type Csctl3 = crate::Reg<csctl3::Csctl3Spec>;
#[doc = "Control 3 Register"]
pub mod csctl3;
#[doc = "CSCLKEN (rw) register accessor: Clock Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csclken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csclken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csclken`]
module"]
#[doc(alias = "CSCLKEN")]
pub type Csclken = crate::Reg<csclken::CsclkenSpec>;
#[doc = "Clock Enable Register"]
pub mod csclken;
#[doc = "CSSTAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csstat`]
module"]
#[doc(alias = "CSSTAT")]
pub type Csstat = crate::Reg<csstat::CsstatSpec>;
#[doc = "Status Register"]
pub mod csstat;
#[doc = "CSIE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csie`]
module"]
#[doc(alias = "CSIE")]
pub type Csie = crate::Reg<csie::CsieSpec>;
#[doc = "Interrupt Enable Register"]
pub mod csie;
#[doc = "CSIFG (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csifg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csifg`]
module"]
#[doc(alias = "CSIFG")]
pub type Csifg = crate::Reg<csifg::CsifgSpec>;
#[doc = "Interrupt Flag Register"]
pub mod csifg;
#[doc = "CSCLRIFG (w) register accessor: Clear Interrupt Flag Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csclrifg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csclrifg`]
module"]
#[doc(alias = "CSCLRIFG")]
pub type Csclrifg = crate::Reg<csclrifg::CsclrifgSpec>;
#[doc = "Clear Interrupt Flag Register"]
pub mod csclrifg;
#[doc = "CSSETIFG (w) register accessor: Set Interrupt Flag Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cssetifg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cssetifg`]
module"]
#[doc(alias = "CSSETIFG")]
pub type Cssetifg = crate::Reg<cssetifg::CssetifgSpec>;
#[doc = "Set Interrupt Flag Register"]
pub mod cssetifg;
#[doc = "CSDCOERCAL0 (rw) register accessor: DCO External Resistor Cailbration 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csdcoercal0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csdcoercal0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csdcoercal0`]
module"]
#[doc(alias = "CSDCOERCAL0")]
pub type Csdcoercal0 = crate::Reg<csdcoercal0::Csdcoercal0Spec>;
#[doc = "DCO External Resistor Cailbration 0 Register"]
pub mod csdcoercal0;
#[doc = "CSDCOERCAL1 (rw) register accessor: DCO External Resistor Calibration 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csdcoercal1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csdcoercal1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csdcoercal1`]
module"]
#[doc(alias = "CSDCOERCAL1")]
pub type Csdcoercal1 = crate::Reg<csdcoercal1::Csdcoercal1Spec>;
#[doc = "DCO External Resistor Calibration 1 Register"]
pub mod csdcoercal1;
