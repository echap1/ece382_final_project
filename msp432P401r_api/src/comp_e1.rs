#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cex_ctl0: CexCtl0,
    cex_ctl1: CexCtl1,
    cex_ctl2: CexCtl2,
    cex_ctl3: CexCtl3,
    _reserved4: [u8; 0x04],
    cex_int: CexInt,
    cex_iv: CexIv,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    #[inline(always)]
    pub const fn cex_ctl0(&self) -> &CexCtl0 {
        &self.cex_ctl0
    }
    #[doc = "0x02 - Comparator Control Register 1"]
    #[inline(always)]
    pub const fn cex_ctl1(&self) -> &CexCtl1 {
        &self.cex_ctl1
    }
    #[doc = "0x04 - Comparator Control Register 2"]
    #[inline(always)]
    pub const fn cex_ctl2(&self) -> &CexCtl2 {
        &self.cex_ctl2
    }
    #[doc = "0x06 - Comparator Control Register 3"]
    #[inline(always)]
    pub const fn cex_ctl3(&self) -> &CexCtl3 {
        &self.cex_ctl3
    }
    #[doc = "0x0c - Comparator Interrupt Control Register"]
    #[inline(always)]
    pub const fn cex_int(&self) -> &CexInt {
        &self.cex_int
    }
    #[doc = "0x0e - Comparator Interrupt Vector Word Register"]
    #[inline(always)]
    pub const fn cex_iv(&self) -> &CexIv {
        &self.cex_iv
    }
}
#[doc = "CExCTL0 (rw) register accessor: Comparator Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cex_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cex_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cex_ctl0`]
module"]
#[doc(alias = "CExCTL0")]
pub type CexCtl0 = crate::Reg<cex_ctl0::CexCtl0Spec>;
#[doc = "Comparator Control Register 0"]
pub mod cex_ctl0;
#[doc = "CExCTL1 (rw) register accessor: Comparator Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cex_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cex_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cex_ctl1`]
module"]
#[doc(alias = "CExCTL1")]
pub type CexCtl1 = crate::Reg<cex_ctl1::CexCtl1Spec>;
#[doc = "Comparator Control Register 1"]
pub mod cex_ctl1;
#[doc = "CExCTL2 (rw) register accessor: Comparator Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cex_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cex_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cex_ctl2`]
module"]
#[doc(alias = "CExCTL2")]
pub type CexCtl2 = crate::Reg<cex_ctl2::CexCtl2Spec>;
#[doc = "Comparator Control Register 2"]
pub mod cex_ctl2;
#[doc = "CExCTL3 (rw) register accessor: Comparator Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cex_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cex_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cex_ctl3`]
module"]
#[doc(alias = "CExCTL3")]
pub type CexCtl3 = crate::Reg<cex_ctl3::CexCtl3Spec>;
#[doc = "Comparator Control Register 3"]
pub mod cex_ctl3;
#[doc = "CExINT (rw) register accessor: Comparator Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cex_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cex_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cex_int`]
module"]
#[doc(alias = "CExINT")]
pub type CexInt = crate::Reg<cex_int::CexIntSpec>;
#[doc = "Comparator Interrupt Control Register"]
pub mod cex_int;
#[doc = "CExIV (r) register accessor: Comparator Interrupt Vector Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cex_iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cex_iv`]
module"]
#[doc(alias = "CExIV")]
pub type CexIv = crate::Reg<cex_iv::CexIvSpec>;
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cex_iv;
