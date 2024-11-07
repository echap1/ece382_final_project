#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ictr: Ictr,
    actlr: Actlr,
}
impl RegisterBlock {
    #[doc = "0x04 - Interrupt Control Type Register"]
    #[inline(always)]
    pub const fn ictr(&self) -> &Ictr {
        &self.ictr
    }
    #[doc = "0x08 - Auxiliary Control Register"]
    #[inline(always)]
    pub const fn actlr(&self) -> &Actlr {
        &self.actlr
    }
}
#[doc = "ICTR (r) register accessor: Interrupt Control Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ictr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ictr`]
module"]
#[doc(alias = "ICTR")]
pub type Ictr = crate::Reg<ictr::IctrSpec>;
#[doc = "Interrupt Control Type Register"]
pub mod ictr;
#[doc = "ACTLR (rw) register accessor: Auxiliary Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`actlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actlr`]
module"]
#[doc(alias = "ACTLR")]
pub type Actlr = crate::Reg<actlr::ActlrSpec>;
#[doc = "Auxiliary Control Register"]
pub mod actlr;
