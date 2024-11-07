#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    psskey: Psskey,
    pssctl0: Pssctl0,
    _reserved2: [u8; 0x2c],
    pssie: Pssie,
    pssifg: Pssifg,
    pssclrifg: Pssclrifg,
}
impl RegisterBlock {
    #[doc = "0x00 - Key Register"]
    #[inline(always)]
    pub const fn psskey(&self) -> &Psskey {
        &self.psskey
    }
    #[doc = "0x04 - Control 0 Register"]
    #[inline(always)]
    pub const fn pssctl0(&self) -> &Pssctl0 {
        &self.pssctl0
    }
    #[doc = "0x34 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pssie(&self) -> &Pssie {
        &self.pssie
    }
    #[doc = "0x38 - Interrupt Flag Register"]
    #[inline(always)]
    pub const fn pssifg(&self) -> &Pssifg {
        &self.pssifg
    }
    #[doc = "0x3c - Clear Interrupt Flag Register"]
    #[inline(always)]
    pub const fn pssclrifg(&self) -> &Pssclrifg {
        &self.pssclrifg
    }
}
#[doc = "PSSKEY (rw) register accessor: Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psskey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psskey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psskey`]
module"]
#[doc(alias = "PSSKEY")]
pub type Psskey = crate::Reg<psskey::PsskeySpec>;
#[doc = "Key Register"]
pub mod psskey;
#[doc = "PSSCTL0 (rw) register accessor: Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pssctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pssctl0`]
module"]
#[doc(alias = "PSSCTL0")]
pub type Pssctl0 = crate::Reg<pssctl0::Pssctl0Spec>;
#[doc = "Control 0 Register"]
pub mod pssctl0;
#[doc = "PSSIE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pssie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pssie`]
module"]
#[doc(alias = "PSSIE")]
pub type Pssie = crate::Reg<pssie::PssieSpec>;
#[doc = "Interrupt Enable Register"]
pub mod pssie;
#[doc = "PSSIFG (r) register accessor: Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pssifg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pssifg`]
module"]
#[doc(alias = "PSSIFG")]
pub type Pssifg = crate::Reg<pssifg::PssifgSpec>;
#[doc = "Interrupt Flag Register"]
pub mod pssifg;
#[doc = "PSSCLRIFG (rw) register accessor: Clear Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pssclrifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pssclrifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pssclrifg`]
module"]
#[doc(alias = "PSSCLRIFG")]
pub type Pssclrifg = crate::Reg<pssclrifg::PssclrifgSpec>;
#[doc = "Clear Interrupt Flag Register"]
pub mod pssclrifg;
