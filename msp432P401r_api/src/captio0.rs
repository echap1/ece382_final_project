#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0e],
    captiox_ctl: CaptioxCtl,
}
impl RegisterBlock {
    #[doc = "0x0e - Capacitive Touch IO x Control Register"]
    #[inline(always)]
    pub const fn captiox_ctl(&self) -> &CaptioxCtl {
        &self.captiox_ctl
    }
}
#[doc = "CAPTIOxCTL (rw) register accessor: Capacitive Touch IO x Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`captiox_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`captiox_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@captiox_ctl`]
module"]
#[doc(alias = "CAPTIOxCTL")]
pub type CaptioxCtl = crate::Reg<captiox_ctl::CaptioxCtlSpec>;
#[doc = "Capacitive Touch IO x Control Register"]
pub mod captiox_ctl;
