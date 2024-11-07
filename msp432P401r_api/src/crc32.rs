#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crc32di: Crc32di,
    _reserved1: [u8; 0x02],
    crc32dirb: Crc32dirb,
    _reserved2: [u8; 0x02],
    crc32inires_lo: Crc32iniresLo,
    crc32inires_hi: Crc32iniresHi,
    crc32resr_lo: Crc32resrLo,
    crc32resr_hi: Crc32resrHi,
    crc16di: Crc16di,
    _reserved7: [u8; 0x02],
    crc16dirb: Crc16dirb,
    _reserved8: [u8; 0x02],
    crc16inires: Crc16inires,
    _reserved9: [u8; 0x04],
    crc16resr: Crc16resr,
}
impl RegisterBlock {
    #[doc = "0x00 - Data Input for CRC32 Signature Computation"]
    #[inline(always)]
    pub const fn crc32di(&self) -> &Crc32di {
        &self.crc32di
    }
    #[doc = "0x04 - Data In Reverse for CRC32 Computation"]
    #[inline(always)]
    pub const fn crc32dirb(&self) -> &Crc32dirb {
        &self.crc32dirb
    }
    #[doc = "0x08 - CRC32 Initialization and Result, lower 16 bits"]
    #[inline(always)]
    pub const fn crc32inires_lo(&self) -> &Crc32iniresLo {
        &self.crc32inires_lo
    }
    #[doc = "0x0a - CRC32 Initialization and Result, upper 16 bits"]
    #[inline(always)]
    pub const fn crc32inires_hi(&self) -> &Crc32iniresHi {
        &self.crc32inires_hi
    }
    #[doc = "0x0c - CRC32 Result Reverse, lower 16 bits"]
    #[inline(always)]
    pub const fn crc32resr_lo(&self) -> &Crc32resrLo {
        &self.crc32resr_lo
    }
    #[doc = "0x0e - CRC32 Result Reverse, Upper 16 bits"]
    #[inline(always)]
    pub const fn crc32resr_hi(&self) -> &Crc32resrHi {
        &self.crc32resr_hi
    }
    #[doc = "0x10 - Data Input for CRC16 computation"]
    #[inline(always)]
    pub const fn crc16di(&self) -> &Crc16di {
        &self.crc16di
    }
    #[doc = "0x14 - CRC16 Data In Reverse"]
    #[inline(always)]
    pub const fn crc16dirb(&self) -> &Crc16dirb {
        &self.crc16dirb
    }
    #[doc = "0x18 - CRC16 Initialization and Result register"]
    #[inline(always)]
    pub const fn crc16inires(&self) -> &Crc16inires {
        &self.crc16inires
    }
    #[doc = "0x1e - CRC16 Result Reverse"]
    #[inline(always)]
    pub const fn crc16resr(&self) -> &Crc16resr {
        &self.crc16resr
    }
}
#[doc = "CRC32DI (rw) register accessor: Data Input for CRC32 Signature Computation\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32di::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32di::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32di`]
module"]
#[doc(alias = "CRC32DI")]
pub type Crc32di = crate::Reg<crc32di::Crc32diSpec>;
#[doc = "Data Input for CRC32 Signature Computation"]
pub mod crc32di;
#[doc = "CRC32DIRB (rw) register accessor: Data In Reverse for CRC32 Computation\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32dirb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32dirb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32dirb`]
module"]
#[doc(alias = "CRC32DIRB")]
pub type Crc32dirb = crate::Reg<crc32dirb::Crc32dirbSpec>;
#[doc = "Data In Reverse for CRC32 Computation"]
pub mod crc32dirb;
#[doc = "CRC32INIRES_LO (rw) register accessor: CRC32 Initialization and Result, lower 16 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32inires_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32inires_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32inires_lo`]
module"]
#[doc(alias = "CRC32INIRES_LO")]
pub type Crc32iniresLo = crate::Reg<crc32inires_lo::Crc32iniresLoSpec>;
#[doc = "CRC32 Initialization and Result, lower 16 bits"]
pub mod crc32inires_lo;
#[doc = "CRC32INIRES_HI (rw) register accessor: CRC32 Initialization and Result, upper 16 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32inires_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32inires_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32inires_hi`]
module"]
#[doc(alias = "CRC32INIRES_HI")]
pub type Crc32iniresHi = crate::Reg<crc32inires_hi::Crc32iniresHiSpec>;
#[doc = "CRC32 Initialization and Result, upper 16 bits"]
pub mod crc32inires_hi;
#[doc = "CRC32RESR_LO (rw) register accessor: CRC32 Result Reverse, lower 16 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32resr_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32resr_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32resr_lo`]
module"]
#[doc(alias = "CRC32RESR_LO")]
pub type Crc32resrLo = crate::Reg<crc32resr_lo::Crc32resrLoSpec>;
#[doc = "CRC32 Result Reverse, lower 16 bits"]
pub mod crc32resr_lo;
#[doc = "CRC32RESR_HI (rw) register accessor: CRC32 Result Reverse, Upper 16 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`crc32resr_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc32resr_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc32resr_hi`]
module"]
#[doc(alias = "CRC32RESR_HI")]
pub type Crc32resrHi = crate::Reg<crc32resr_hi::Crc32resrHiSpec>;
#[doc = "CRC32 Result Reverse, Upper 16 bits"]
pub mod crc32resr_hi;
#[doc = "CRC16DI (rw) register accessor: Data Input for CRC16 computation\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16di::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16di::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc16di`]
module"]
#[doc(alias = "CRC16DI")]
pub type Crc16di = crate::Reg<crc16di::Crc16diSpec>;
#[doc = "Data Input for CRC16 computation"]
pub mod crc16di;
#[doc = "CRC16DIRB (rw) register accessor: CRC16 Data In Reverse\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16dirb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16dirb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc16dirb`]
module"]
#[doc(alias = "CRC16DIRB")]
pub type Crc16dirb = crate::Reg<crc16dirb::Crc16dirbSpec>;
#[doc = "CRC16 Data In Reverse"]
pub mod crc16dirb;
#[doc = "CRC16INIRES (rw) register accessor: CRC16 Initialization and Result register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16inires::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16inires::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc16inires`]
module"]
#[doc(alias = "CRC16INIRES")]
pub type Crc16inires = crate::Reg<crc16inires::Crc16iniresSpec>;
#[doc = "CRC16 Initialization and Result register"]
pub mod crc16inires;
#[doc = "CRC16RESR (rw) register accessor: CRC16 Result Reverse\n\nYou can [`read`](crate::Reg::read) this register and get [`crc16resr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc16resr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc16resr`]
module"]
#[doc(alias = "CRC16RESR")]
pub type Crc16resr = crate::Reg<crc16resr::Crc16resrSpec>;
#[doc = "CRC16 Result Reverse"]
pub mod crc16resr;
