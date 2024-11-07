#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pmapkeyid: Pmapkeyid,
    pmapctl: Pmapctl,
    _reserved2: [u8; 0x04],
    p1map01: P1map01,
    p1map23: P1map23,
    p1map45: P1map45,
    p1map67: P1map67,
    p2map01: P2map01,
    p2map23: P2map23,
    p2map45: P2map45,
    p2map67: P2map67,
    p3map01: P3map01,
    p3map23: P3map23,
    p3map45: P3map45,
    p3map67: P3map67,
    p4map01: P4map01,
    p4map23: P4map23,
    p4map45: P4map45,
    p4map67: P4map67,
    p5map01: P5map01,
    p5map23: P5map23,
    p5map45: P5map45,
    p5map67: P5map67,
    p6map01: P6map01,
    p6map23: P6map23,
    p6map45: P6map45,
    p6map67: P6map67,
    p7map01: P7map01,
    p7map23: P7map23,
    p7map45: P7map45,
    p7map67: P7map67,
}
impl RegisterBlock {
    #[doc = "0x00 - Port Mapping Key Register"]
    #[inline(always)]
    pub const fn pmapkeyid(&self) -> &Pmapkeyid {
        &self.pmapkeyid
    }
    #[doc = "0x02 - Port Mapping Control Register"]
    #[inline(always)]
    pub const fn pmapctl(&self) -> &Pmapctl {
        &self.pmapctl
    }
    #[doc = "0x08 - Port mapping register, P1.0 and P1.1"]
    #[inline(always)]
    pub const fn p1map01(&self) -> &P1map01 {
        &self.p1map01
    }
    #[doc = "0x0a - Port mapping register, P1.2 and P1.3"]
    #[inline(always)]
    pub const fn p1map23(&self) -> &P1map23 {
        &self.p1map23
    }
    #[doc = "0x0c - Port mapping register, P1.4 and P1.5"]
    #[inline(always)]
    pub const fn p1map45(&self) -> &P1map45 {
        &self.p1map45
    }
    #[doc = "0x0e - Port mapping register, P1.6 and P1.7"]
    #[inline(always)]
    pub const fn p1map67(&self) -> &P1map67 {
        &self.p1map67
    }
    #[doc = "0x10 - Port mapping register, P2.0 and P2.1"]
    #[inline(always)]
    pub const fn p2map01(&self) -> &P2map01 {
        &self.p2map01
    }
    #[doc = "0x12 - Port mapping register, P2.2 and P2.3"]
    #[inline(always)]
    pub const fn p2map23(&self) -> &P2map23 {
        &self.p2map23
    }
    #[doc = "0x14 - Port mapping register, P2.4 and P2.5"]
    #[inline(always)]
    pub const fn p2map45(&self) -> &P2map45 {
        &self.p2map45
    }
    #[doc = "0x16 - Port mapping register, P2.6 and P2.7"]
    #[inline(always)]
    pub const fn p2map67(&self) -> &P2map67 {
        &self.p2map67
    }
    #[doc = "0x18 - Port mapping register, P3.0 and P3.1"]
    #[inline(always)]
    pub const fn p3map01(&self) -> &P3map01 {
        &self.p3map01
    }
    #[doc = "0x1a - Port mapping register, P3.2 and P3.3"]
    #[inline(always)]
    pub const fn p3map23(&self) -> &P3map23 {
        &self.p3map23
    }
    #[doc = "0x1c - Port mapping register, P3.4 and P3.5"]
    #[inline(always)]
    pub const fn p3map45(&self) -> &P3map45 {
        &self.p3map45
    }
    #[doc = "0x1e - Port mapping register, P3.6 and P3.7"]
    #[inline(always)]
    pub const fn p3map67(&self) -> &P3map67 {
        &self.p3map67
    }
    #[doc = "0x20 - Port mapping register, P4.0 and P4.1"]
    #[inline(always)]
    pub const fn p4map01(&self) -> &P4map01 {
        &self.p4map01
    }
    #[doc = "0x22 - Port mapping register, P4.2 and P4.3"]
    #[inline(always)]
    pub const fn p4map23(&self) -> &P4map23 {
        &self.p4map23
    }
    #[doc = "0x24 - Port mapping register, P4.4 and P4.5"]
    #[inline(always)]
    pub const fn p4map45(&self) -> &P4map45 {
        &self.p4map45
    }
    #[doc = "0x26 - Port mapping register, P4.6 and P4.7"]
    #[inline(always)]
    pub const fn p4map67(&self) -> &P4map67 {
        &self.p4map67
    }
    #[doc = "0x28 - Port mapping register, P5.0 and P5.1"]
    #[inline(always)]
    pub const fn p5map01(&self) -> &P5map01 {
        &self.p5map01
    }
    #[doc = "0x2a - Port mapping register, P5.2 and P5.3"]
    #[inline(always)]
    pub const fn p5map23(&self) -> &P5map23 {
        &self.p5map23
    }
    #[doc = "0x2c - Port mapping register, P5.4 and P5.5"]
    #[inline(always)]
    pub const fn p5map45(&self) -> &P5map45 {
        &self.p5map45
    }
    #[doc = "0x2e - Port mapping register, P5.6 and P5.7"]
    #[inline(always)]
    pub const fn p5map67(&self) -> &P5map67 {
        &self.p5map67
    }
    #[doc = "0x30 - Port mapping register, P6.0 and P6.1"]
    #[inline(always)]
    pub const fn p6map01(&self) -> &P6map01 {
        &self.p6map01
    }
    #[doc = "0x32 - Port mapping register, P6.2 and P6.3"]
    #[inline(always)]
    pub const fn p6map23(&self) -> &P6map23 {
        &self.p6map23
    }
    #[doc = "0x34 - Port mapping register, P6.4 and P6.5"]
    #[inline(always)]
    pub const fn p6map45(&self) -> &P6map45 {
        &self.p6map45
    }
    #[doc = "0x36 - Port mapping register, P6.6 and P6.7"]
    #[inline(always)]
    pub const fn p6map67(&self) -> &P6map67 {
        &self.p6map67
    }
    #[doc = "0x38 - Port mapping register, P7.0 and P7.1"]
    #[inline(always)]
    pub const fn p7map01(&self) -> &P7map01 {
        &self.p7map01
    }
    #[doc = "0x3a - Port mapping register, P7.2 and P7.3"]
    #[inline(always)]
    pub const fn p7map23(&self) -> &P7map23 {
        &self.p7map23
    }
    #[doc = "0x3c - Port mapping register, P7.4 and P7.5"]
    #[inline(always)]
    pub const fn p7map45(&self) -> &P7map45 {
        &self.p7map45
    }
    #[doc = "0x3e - Port mapping register, P7.6 and P7.7"]
    #[inline(always)]
    pub const fn p7map67(&self) -> &P7map67 {
        &self.p7map67
    }
}
#[doc = "PMAPKEYID (rw) register accessor: Port Mapping Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmapkeyid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmapkeyid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmapkeyid`]
module"]
#[doc(alias = "PMAPKEYID")]
pub type Pmapkeyid = crate::Reg<pmapkeyid::PmapkeyidSpec>;
#[doc = "Port Mapping Key Register"]
pub mod pmapkeyid;
#[doc = "PMAPCTL (rw) register accessor: Port Mapping Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmapctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmapctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmapctl`]
module"]
#[doc(alias = "PMAPCTL")]
pub type Pmapctl = crate::Reg<pmapctl::PmapctlSpec>;
#[doc = "Port Mapping Control Register"]
pub mod pmapctl;
#[doc = "P1MAP01 (rw) register accessor: Port mapping register, P1.0 and P1.1\n\nYou can [`read`](crate::Reg::read) this register and get [`p1map01::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1map01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1map01`]
module"]
#[doc(alias = "P1MAP01")]
pub type P1map01 = crate::Reg<p1map01::P1map01Spec>;
#[doc = "Port mapping register, P1.0 and P1.1"]
pub mod p1map01;
#[doc = "P1MAP23 (rw) register accessor: Port mapping register, P1.2 and P1.3\n\nYou can [`read`](crate::Reg::read) this register and get [`p1map23::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1map23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1map23`]
module"]
#[doc(alias = "P1MAP23")]
pub type P1map23 = crate::Reg<p1map23::P1map23Spec>;
#[doc = "Port mapping register, P1.2 and P1.3"]
pub mod p1map23;
#[doc = "P1MAP45 (rw) register accessor: Port mapping register, P1.4 and P1.5\n\nYou can [`read`](crate::Reg::read) this register and get [`p1map45::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1map45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1map45`]
module"]
#[doc(alias = "P1MAP45")]
pub type P1map45 = crate::Reg<p1map45::P1map45Spec>;
#[doc = "Port mapping register, P1.4 and P1.5"]
pub mod p1map45;
#[doc = "P1MAP67 (rw) register accessor: Port mapping register, P1.6 and P1.7\n\nYou can [`read`](crate::Reg::read) this register and get [`p1map67::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1map67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1map67`]
module"]
#[doc(alias = "P1MAP67")]
pub type P1map67 = crate::Reg<p1map67::P1map67Spec>;
#[doc = "Port mapping register, P1.6 and P1.7"]
pub mod p1map67;
#[doc = "P2MAP01 (rw) register accessor: Port mapping register, P2.0 and P2.1\n\nYou can [`read`](crate::Reg::read) this register and get [`p2map01::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2map01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2map01`]
module"]
#[doc(alias = "P2MAP01")]
pub type P2map01 = crate::Reg<p2map01::P2map01Spec>;
#[doc = "Port mapping register, P2.0 and P2.1"]
pub mod p2map01;
#[doc = "P2MAP23 (rw) register accessor: Port mapping register, P2.2 and P2.3\n\nYou can [`read`](crate::Reg::read) this register and get [`p2map23::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2map23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2map23`]
module"]
#[doc(alias = "P2MAP23")]
pub type P2map23 = crate::Reg<p2map23::P2map23Spec>;
#[doc = "Port mapping register, P2.2 and P2.3"]
pub mod p2map23;
#[doc = "P2MAP45 (rw) register accessor: Port mapping register, P2.4 and P2.5\n\nYou can [`read`](crate::Reg::read) this register and get [`p2map45::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2map45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2map45`]
module"]
#[doc(alias = "P2MAP45")]
pub type P2map45 = crate::Reg<p2map45::P2map45Spec>;
#[doc = "Port mapping register, P2.4 and P2.5"]
pub mod p2map45;
#[doc = "P2MAP67 (rw) register accessor: Port mapping register, P2.6 and P2.7\n\nYou can [`read`](crate::Reg::read) this register and get [`p2map67::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2map67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2map67`]
module"]
#[doc(alias = "P2MAP67")]
pub type P2map67 = crate::Reg<p2map67::P2map67Spec>;
#[doc = "Port mapping register, P2.6 and P2.7"]
pub mod p2map67;
#[doc = "P3MAP01 (rw) register accessor: Port mapping register, P3.0 and P3.1\n\nYou can [`read`](crate::Reg::read) this register and get [`p3map01::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3map01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3map01`]
module"]
#[doc(alias = "P3MAP01")]
pub type P3map01 = crate::Reg<p3map01::P3map01Spec>;
#[doc = "Port mapping register, P3.0 and P3.1"]
pub mod p3map01;
#[doc = "P3MAP23 (rw) register accessor: Port mapping register, P3.2 and P3.3\n\nYou can [`read`](crate::Reg::read) this register and get [`p3map23::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3map23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3map23`]
module"]
#[doc(alias = "P3MAP23")]
pub type P3map23 = crate::Reg<p3map23::P3map23Spec>;
#[doc = "Port mapping register, P3.2 and P3.3"]
pub mod p3map23;
#[doc = "P3MAP45 (rw) register accessor: Port mapping register, P3.4 and P3.5\n\nYou can [`read`](crate::Reg::read) this register and get [`p3map45::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3map45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3map45`]
module"]
#[doc(alias = "P3MAP45")]
pub type P3map45 = crate::Reg<p3map45::P3map45Spec>;
#[doc = "Port mapping register, P3.4 and P3.5"]
pub mod p3map45;
#[doc = "P3MAP67 (rw) register accessor: Port mapping register, P3.6 and P3.7\n\nYou can [`read`](crate::Reg::read) this register and get [`p3map67::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3map67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3map67`]
module"]
#[doc(alias = "P3MAP67")]
pub type P3map67 = crate::Reg<p3map67::P3map67Spec>;
#[doc = "Port mapping register, P3.6 and P3.7"]
pub mod p3map67;
#[doc = "P4MAP01 (rw) register accessor: Port mapping register, P4.0 and P4.1\n\nYou can [`read`](crate::Reg::read) this register and get [`p4map01::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4map01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map01`]
module"]
#[doc(alias = "P4MAP01")]
pub type P4map01 = crate::Reg<p4map01::P4map01Spec>;
#[doc = "Port mapping register, P4.0 and P4.1"]
pub mod p4map01;
#[doc = "P4MAP23 (rw) register accessor: Port mapping register, P4.2 and P4.3\n\nYou can [`read`](crate::Reg::read) this register and get [`p4map23::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4map23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map23`]
module"]
#[doc(alias = "P4MAP23")]
pub type P4map23 = crate::Reg<p4map23::P4map23Spec>;
#[doc = "Port mapping register, P4.2 and P4.3"]
pub mod p4map23;
#[doc = "P4MAP45 (rw) register accessor: Port mapping register, P4.4 and P4.5\n\nYou can [`read`](crate::Reg::read) this register and get [`p4map45::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4map45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map45`]
module"]
#[doc(alias = "P4MAP45")]
pub type P4map45 = crate::Reg<p4map45::P4map45Spec>;
#[doc = "Port mapping register, P4.4 and P4.5"]
pub mod p4map45;
#[doc = "P4MAP67 (rw) register accessor: Port mapping register, P4.6 and P4.7\n\nYou can [`read`](crate::Reg::read) this register and get [`p4map67::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4map67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4map67`]
module"]
#[doc(alias = "P4MAP67")]
pub type P4map67 = crate::Reg<p4map67::P4map67Spec>;
#[doc = "Port mapping register, P4.6 and P4.7"]
pub mod p4map67;
#[doc = "P5MAP01 (rw) register accessor: Port mapping register, P5.0 and P5.1\n\nYou can [`read`](crate::Reg::read) this register and get [`p5map01::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5map01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5map01`]
module"]
#[doc(alias = "P5MAP01")]
pub type P5map01 = crate::Reg<p5map01::P5map01Spec>;
#[doc = "Port mapping register, P5.0 and P5.1"]
pub mod p5map01;
#[doc = "P5MAP23 (rw) register accessor: Port mapping register, P5.2 and P5.3\n\nYou can [`read`](crate::Reg::read) this register and get [`p5map23::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5map23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5map23`]
module"]
#[doc(alias = "P5MAP23")]
pub type P5map23 = crate::Reg<p5map23::P5map23Spec>;
#[doc = "Port mapping register, P5.2 and P5.3"]
pub mod p5map23;
#[doc = "P5MAP45 (rw) register accessor: Port mapping register, P5.4 and P5.5\n\nYou can [`read`](crate::Reg::read) this register and get [`p5map45::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5map45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5map45`]
module"]
#[doc(alias = "P5MAP45")]
pub type P5map45 = crate::Reg<p5map45::P5map45Spec>;
#[doc = "Port mapping register, P5.4 and P5.5"]
pub mod p5map45;
#[doc = "P5MAP67 (rw) register accessor: Port mapping register, P5.6 and P5.7\n\nYou can [`read`](crate::Reg::read) this register and get [`p5map67::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5map67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5map67`]
module"]
#[doc(alias = "P5MAP67")]
pub type P5map67 = crate::Reg<p5map67::P5map67Spec>;
#[doc = "Port mapping register, P5.6 and P5.7"]
pub mod p5map67;
#[doc = "P6MAP01 (rw) register accessor: Port mapping register, P6.0 and P6.1\n\nYou can [`read`](crate::Reg::read) this register and get [`p6map01::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6map01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6map01`]
module"]
#[doc(alias = "P6MAP01")]
pub type P6map01 = crate::Reg<p6map01::P6map01Spec>;
#[doc = "Port mapping register, P6.0 and P6.1"]
pub mod p6map01;
#[doc = "P6MAP23 (rw) register accessor: Port mapping register, P6.2 and P6.3\n\nYou can [`read`](crate::Reg::read) this register and get [`p6map23::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6map23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6map23`]
module"]
#[doc(alias = "P6MAP23")]
pub type P6map23 = crate::Reg<p6map23::P6map23Spec>;
#[doc = "Port mapping register, P6.2 and P6.3"]
pub mod p6map23;
#[doc = "P6MAP45 (rw) register accessor: Port mapping register, P6.4 and P6.5\n\nYou can [`read`](crate::Reg::read) this register and get [`p6map45::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6map45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6map45`]
module"]
#[doc(alias = "P6MAP45")]
pub type P6map45 = crate::Reg<p6map45::P6map45Spec>;
#[doc = "Port mapping register, P6.4 and P6.5"]
pub mod p6map45;
#[doc = "P6MAP67 (rw) register accessor: Port mapping register, P6.6 and P6.7\n\nYou can [`read`](crate::Reg::read) this register and get [`p6map67::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6map67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6map67`]
module"]
#[doc(alias = "P6MAP67")]
pub type P6map67 = crate::Reg<p6map67::P6map67Spec>;
#[doc = "Port mapping register, P6.6 and P6.7"]
pub mod p6map67;
#[doc = "P7MAP01 (rw) register accessor: Port mapping register, P7.0 and P7.1\n\nYou can [`read`](crate::Reg::read) this register and get [`p7map01::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7map01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7map01`]
module"]
#[doc(alias = "P7MAP01")]
pub type P7map01 = crate::Reg<p7map01::P7map01Spec>;
#[doc = "Port mapping register, P7.0 and P7.1"]
pub mod p7map01;
#[doc = "P7MAP23 (rw) register accessor: Port mapping register, P7.2 and P7.3\n\nYou can [`read`](crate::Reg::read) this register and get [`p7map23::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7map23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7map23`]
module"]
#[doc(alias = "P7MAP23")]
pub type P7map23 = crate::Reg<p7map23::P7map23Spec>;
#[doc = "Port mapping register, P7.2 and P7.3"]
pub mod p7map23;
#[doc = "P7MAP45 (rw) register accessor: Port mapping register, P7.4 and P7.5\n\nYou can [`read`](crate::Reg::read) this register and get [`p7map45::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7map45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7map45`]
module"]
#[doc(alias = "P7MAP45")]
pub type P7map45 = crate::Reg<p7map45::P7map45Spec>;
#[doc = "Port mapping register, P7.4 and P7.5"]
pub mod p7map45;
#[doc = "P7MAP67 (rw) register accessor: Port mapping register, P7.6 and P7.7\n\nYou can [`read`](crate::Reg::read) this register and get [`p7map67::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7map67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7map67`]
module"]
#[doc(alias = "P7MAP67")]
pub type P7map67 = crate::Reg<p7map67::P7map67Spec>;
#[doc = "Port mapping register, P7.6 and P7.7"]
pub mod p7map67;
