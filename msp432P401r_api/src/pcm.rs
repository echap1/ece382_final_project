#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcmctl0: Pcmctl0,
    pcmctl1: Pcmctl1,
    pcmie: Pcmie,
    pcmifg: Pcmifg,
    pcmclrifg: Pcmclrifg,
}
impl RegisterBlock {
    #[doc = "0x00 - Control 0 Register"]
    #[inline(always)]
    pub const fn pcmctl0(&self) -> &Pcmctl0 {
        &self.pcmctl0
    }
    #[doc = "0x04 - Control 1 Register"]
    #[inline(always)]
    pub const fn pcmctl1(&self) -> &Pcmctl1 {
        &self.pcmctl1
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pcmie(&self) -> &Pcmie {
        &self.pcmie
    }
    #[doc = "0x0c - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn pcmifg(&self) -> &Pcmifg {
        &self.pcmifg
    }
    #[doc = "0x10 - Clear Interrupt Flag Register"]
    #[inline(always)]
    pub const fn pcmclrifg(&self) -> &Pcmclrifg {
        &self.pcmclrifg
    }
}
#[doc = "PCMCTL0 (rw) register accessor: Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmctl0`]
module"]
#[doc(alias = "PCMCTL0")]
pub type Pcmctl0 = crate::Reg<pcmctl0::Pcmctl0Spec>;
#[doc = "Control 0 Register"]
pub mod pcmctl0;
#[doc = "PCMCTL1 (rw) register accessor: Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmctl1`]
module"]
#[doc(alias = "PCMCTL1")]
pub type Pcmctl1 = crate::Reg<pcmctl1::Pcmctl1Spec>;
#[doc = "Control 1 Register"]
pub mod pcmctl1;
#[doc = "PCMIE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmie`]
module"]
#[doc(alias = "PCMIE")]
pub type Pcmie = crate::Reg<pcmie::PcmieSpec>;
#[doc = "Interrupt Enable Register"]
pub mod pcmie;
#[doc = "PCMIFG (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmifg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmifg`]
module"]
#[doc(alias = "PCMIFG")]
pub type Pcmifg = crate::Reg<pcmifg::PcmifgSpec>;
#[doc = "Interrupt Flag Register"]
pub mod pcmifg;
#[doc = "PCMCLRIFG (w) register accessor: Clear Interrupt Flag Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmclrifg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmclrifg`]
module"]
#[doc(alias = "PCMCLRIFG")]
pub type Pcmclrifg = crate::Reg<pcmclrifg::PcmclrifgSpec>;
#[doc = "Clear Interrupt Flag Register"]
pub mod pcmclrifg;
