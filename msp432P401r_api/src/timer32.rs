#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    t32load1: T32load1,
    t32value1: T32value1,
    t32control1: T32control1,
    t32intclr1: T32intclr1,
    t32ris1: T32ris1,
    t32mis1: T32mis1,
    t32bgload1: T32bgload1,
    _reserved7: [u8; 0x04],
    t32load2: T32load2,
    t32value2: T32value2,
    t32control2: T32control2,
    t32intclr2: T32intclr2,
    t32ris2: T32ris2,
    t32mis2: T32mis2,
    t32bgload2: T32bgload2,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer 1 Load Register"]
    #[inline(always)]
    pub const fn t32load1(&self) -> &T32load1 {
        &self.t32load1
    }
    #[doc = "0x04 - Timer 1 Current Value Register"]
    #[inline(always)]
    pub const fn t32value1(&self) -> &T32value1 {
        &self.t32value1
    }
    #[doc = "0x08 - Timer 1 Timer Control Register"]
    #[inline(always)]
    pub const fn t32control1(&self) -> &T32control1 {
        &self.t32control1
    }
    #[doc = "0x0c - Timer 1 Interrupt Clear Register"]
    #[inline(always)]
    pub const fn t32intclr1(&self) -> &T32intclr1 {
        &self.t32intclr1
    }
    #[doc = "0x10 - Timer 1 Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn t32ris1(&self) -> &T32ris1 {
        &self.t32ris1
    }
    #[doc = "0x14 - Timer 1 Interrupt Status Register"]
    #[inline(always)]
    pub const fn t32mis1(&self) -> &T32mis1 {
        &self.t32mis1
    }
    #[doc = "0x18 - Timer 1 Background Load Register"]
    #[inline(always)]
    pub const fn t32bgload1(&self) -> &T32bgload1 {
        &self.t32bgload1
    }
    #[doc = "0x20 - Timer 2 Load Register"]
    #[inline(always)]
    pub const fn t32load2(&self) -> &T32load2 {
        &self.t32load2
    }
    #[doc = "0x24 - Timer 2 Current Value Register"]
    #[inline(always)]
    pub const fn t32value2(&self) -> &T32value2 {
        &self.t32value2
    }
    #[doc = "0x28 - Timer 2 Timer Control Register"]
    #[inline(always)]
    pub const fn t32control2(&self) -> &T32control2 {
        &self.t32control2
    }
    #[doc = "0x2c - Timer 2 Interrupt Clear Register"]
    #[inline(always)]
    pub const fn t32intclr2(&self) -> &T32intclr2 {
        &self.t32intclr2
    }
    #[doc = "0x30 - Timer 2 Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn t32ris2(&self) -> &T32ris2 {
        &self.t32ris2
    }
    #[doc = "0x34 - Timer 2 Interrupt Status Register"]
    #[inline(always)]
    pub const fn t32mis2(&self) -> &T32mis2 {
        &self.t32mis2
    }
    #[doc = "0x38 - Timer 2 Background Load Register"]
    #[inline(always)]
    pub const fn t32bgload2(&self) -> &T32bgload2 {
        &self.t32bgload2
    }
}
#[doc = "T32LOAD1 (rw) register accessor: Timer 1 Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32load1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32load1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32load1`]
module"]
#[doc(alias = "T32LOAD1")]
pub type T32load1 = crate::Reg<t32load1::T32load1Spec>;
#[doc = "Timer 1 Load Register"]
pub mod t32load1;
#[doc = "T32VALUE1 (r) register accessor: Timer 1 Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32value1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32value1`]
module"]
#[doc(alias = "T32VALUE1")]
pub type T32value1 = crate::Reg<t32value1::T32value1Spec>;
#[doc = "Timer 1 Current Value Register"]
pub mod t32value1;
#[doc = "T32CONTROL1 (rw) register accessor: Timer 1 Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32control1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32control1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32control1`]
module"]
#[doc(alias = "T32CONTROL1")]
pub type T32control1 = crate::Reg<t32control1::T32control1Spec>;
#[doc = "Timer 1 Timer Control Register"]
pub mod t32control1;
#[doc = "T32INTCLR1 (w) register accessor: Timer 1 Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32intclr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32intclr1`]
module"]
#[doc(alias = "T32INTCLR1")]
pub type T32intclr1 = crate::Reg<t32intclr1::T32intclr1Spec>;
#[doc = "Timer 1 Interrupt Clear Register"]
pub mod t32intclr1;
#[doc = "T32RIS1 (r) register accessor: Timer 1 Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32ris1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32ris1`]
module"]
#[doc(alias = "T32RIS1")]
pub type T32ris1 = crate::Reg<t32ris1::T32ris1Spec>;
#[doc = "Timer 1 Raw Interrupt Status Register"]
pub mod t32ris1;
#[doc = "T32MIS1 (r) register accessor: Timer 1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32mis1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32mis1`]
module"]
#[doc(alias = "T32MIS1")]
pub type T32mis1 = crate::Reg<t32mis1::T32mis1Spec>;
#[doc = "Timer 1 Interrupt Status Register"]
pub mod t32mis1;
#[doc = "T32BGLOAD1 (rw) register accessor: Timer 1 Background Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32bgload1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32bgload1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32bgload1`]
module"]
#[doc(alias = "T32BGLOAD1")]
pub type T32bgload1 = crate::Reg<t32bgload1::T32bgload1Spec>;
#[doc = "Timer 1 Background Load Register"]
pub mod t32bgload1;
#[doc = "T32LOAD2 (rw) register accessor: Timer 2 Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32load2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32load2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32load2`]
module"]
#[doc(alias = "T32LOAD2")]
pub type T32load2 = crate::Reg<t32load2::T32load2Spec>;
#[doc = "Timer 2 Load Register"]
pub mod t32load2;
#[doc = "T32VALUE2 (r) register accessor: Timer 2 Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32value2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32value2`]
module"]
#[doc(alias = "T32VALUE2")]
pub type T32value2 = crate::Reg<t32value2::T32value2Spec>;
#[doc = "Timer 2 Current Value Register"]
pub mod t32value2;
#[doc = "T32CONTROL2 (rw) register accessor: Timer 2 Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32control2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32control2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32control2`]
module"]
#[doc(alias = "T32CONTROL2")]
pub type T32control2 = crate::Reg<t32control2::T32control2Spec>;
#[doc = "Timer 2 Timer Control Register"]
pub mod t32control2;
#[doc = "T32INTCLR2 (w) register accessor: Timer 2 Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32intclr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32intclr2`]
module"]
#[doc(alias = "T32INTCLR2")]
pub type T32intclr2 = crate::Reg<t32intclr2::T32intclr2Spec>;
#[doc = "Timer 2 Interrupt Clear Register"]
pub mod t32intclr2;
#[doc = "T32RIS2 (r) register accessor: Timer 2 Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32ris2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32ris2`]
module"]
#[doc(alias = "T32RIS2")]
pub type T32ris2 = crate::Reg<t32ris2::T32ris2Spec>;
#[doc = "Timer 2 Raw Interrupt Status Register"]
pub mod t32ris2;
#[doc = "T32MIS2 (r) register accessor: Timer 2 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32mis2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32mis2`]
module"]
#[doc(alias = "T32MIS2")]
pub type T32mis2 = crate::Reg<t32mis2::T32mis2Spec>;
#[doc = "Timer 2 Interrupt Status Register"]
pub mod t32mis2;
#[doc = "T32BGLOAD2 (rw) register accessor: Timer 2 Background Load Register\n\nYou can [`read`](crate::Reg::read) this register and get [`t32bgload2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t32bgload2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t32bgload2`]
module"]
#[doc(alias = "T32BGLOAD2")]
pub type T32bgload2 = crate::Reg<t32bgload2::T32bgload2Spec>;
#[doc = "Timer 2 Background Load Register"]
pub mod t32bgload2;
