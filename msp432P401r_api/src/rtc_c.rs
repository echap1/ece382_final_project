#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtcctl0: Rtcctl0,
    rtcctl13: Rtcctl13,
    rtcocal: Rtcocal,
    rtctcmp: Rtctcmp,
    rtcps0ctl: Rtcps0ctl,
    rtcps1ctl: Rtcps1ctl,
    rtcps: Rtcps,
    rtciv: Rtciv,
    rtctim0: Rtctim0,
    rtctim1: Rtctim1,
    rtcdate: Rtcdate,
    rtcyear: Rtcyear,
    rtcaminhr: Rtcaminhr,
    rtcadowday: Rtcadowday,
    rtcbin2bcd: Rtcbin2bcd,
    rtcbcd2bin: Rtcbcd2bin,
}
impl RegisterBlock {
    #[doc = "0x00 - RTCCTL0 Register"]
    #[inline(always)]
    pub const fn rtcctl0(&self) -> &Rtcctl0 {
        &self.rtcctl0
    }
    #[doc = "0x02 - RTCCTL13 Register"]
    #[inline(always)]
    pub const fn rtcctl13(&self) -> &Rtcctl13 {
        &self.rtcctl13
    }
    #[doc = "0x04 - RTCOCAL Register"]
    #[inline(always)]
    pub const fn rtcocal(&self) -> &Rtcocal {
        &self.rtcocal
    }
    #[doc = "0x06 - RTCTCMP Register"]
    #[inline(always)]
    pub const fn rtctcmp(&self) -> &Rtctcmp {
        &self.rtctcmp
    }
    #[doc = "0x08 - Real-Time Clock Prescale Timer 0 Control Register"]
    #[inline(always)]
    pub const fn rtcps0ctl(&self) -> &Rtcps0ctl {
        &self.rtcps0ctl
    }
    #[doc = "0x0a - Real-Time Clock Prescale Timer 1 Control Register"]
    #[inline(always)]
    pub const fn rtcps1ctl(&self) -> &Rtcps1ctl {
        &self.rtcps1ctl
    }
    #[doc = "0x0c - Real-Time Clock Prescale Timer Counter Register"]
    #[inline(always)]
    pub const fn rtcps(&self) -> &Rtcps {
        &self.rtcps
    }
    #[doc = "0x0e - Real-Time Clock Interrupt Vector Register"]
    #[inline(always)]
    pub const fn rtciv(&self) -> &Rtciv {
        &self.rtciv
    }
    #[doc = "0x10 - RTCTIM0 Register Hexadecimal Format"]
    #[inline(always)]
    pub const fn rtctim0(&self) -> &Rtctim0 {
        &self.rtctim0
    }
    #[doc = "0x12 - Real-Time Clock Hour, Day of Week"]
    #[inline(always)]
    pub const fn rtctim1(&self) -> &Rtctim1 {
        &self.rtctim1
    }
    #[doc = "0x14 - RTCDATE - Hexadecimal Format"]
    #[inline(always)]
    pub const fn rtcdate(&self) -> &Rtcdate {
        &self.rtcdate
    }
    #[doc = "0x16 - RTCYEAR Register Hexadecimal Format"]
    #[inline(always)]
    pub const fn rtcyear(&self) -> &Rtcyear {
        &self.rtcyear
    }
    #[doc = "0x18 - RTCMINHR - Hexadecimal Format"]
    #[inline(always)]
    pub const fn rtcaminhr(&self) -> &Rtcaminhr {
        &self.rtcaminhr
    }
    #[doc = "0x1a - RTCADOWDAY - Hexadecimal Format"]
    #[inline(always)]
    pub const fn rtcadowday(&self) -> &Rtcadowday {
        &self.rtcadowday
    }
    #[doc = "0x1c - Binary-to-BCD Conversion Register"]
    #[inline(always)]
    pub const fn rtcbin2bcd(&self) -> &Rtcbin2bcd {
        &self.rtcbin2bcd
    }
    #[doc = "0x1e - BCD-to-Binary Conversion Register"]
    #[inline(always)]
    pub const fn rtcbcd2bin(&self) -> &Rtcbcd2bin {
        &self.rtcbcd2bin
    }
}
#[doc = "RTCCTL0 (rw) register accessor: RTCCTL0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcctl0`]
module"]
#[doc(alias = "RTCCTL0")]
pub type Rtcctl0 = crate::Reg<rtcctl0::Rtcctl0Spec>;
#[doc = "RTCCTL0 Register"]
pub mod rtcctl0;
#[doc = "RTCCTL13 (rw) register accessor: RTCCTL13 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcctl13`]
module"]
#[doc(alias = "RTCCTL13")]
pub type Rtcctl13 = crate::Reg<rtcctl13::Rtcctl13Spec>;
#[doc = "RTCCTL13 Register"]
pub mod rtcctl13;
#[doc = "RTCOCAL (rw) register accessor: RTCOCAL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcocal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcocal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcocal`]
module"]
#[doc(alias = "RTCOCAL")]
pub type Rtcocal = crate::Reg<rtcocal::RtcocalSpec>;
#[doc = "RTCOCAL Register"]
pub mod rtcocal;
#[doc = "RTCTCMP (rw) register accessor: RTCTCMP Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctcmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctcmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtctcmp`]
module"]
#[doc(alias = "RTCTCMP")]
pub type Rtctcmp = crate::Reg<rtctcmp::RtctcmpSpec>;
#[doc = "RTCTCMP Register"]
pub mod rtctcmp;
#[doc = "RTCPS0CTL (rw) register accessor: Real-Time Clock Prescale Timer 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcps0ctl`]
module"]
#[doc(alias = "RTCPS0CTL")]
pub type Rtcps0ctl = crate::Reg<rtcps0ctl::Rtcps0ctlSpec>;
#[doc = "Real-Time Clock Prescale Timer 0 Control Register"]
pub mod rtcps0ctl;
#[doc = "RTCPS1CTL (rw) register accessor: Real-Time Clock Prescale Timer 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcps1ctl`]
module"]
#[doc(alias = "RTCPS1CTL")]
pub type Rtcps1ctl = crate::Reg<rtcps1ctl::Rtcps1ctlSpec>;
#[doc = "Real-Time Clock Prescale Timer 1 Control Register"]
pub mod rtcps1ctl;
#[doc = "RTCPS (rw) register accessor: Real-Time Clock Prescale Timer Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcps`]
module"]
#[doc(alias = "RTCPS")]
pub type Rtcps = crate::Reg<rtcps::RtcpsSpec>;
#[doc = "Real-Time Clock Prescale Timer Counter Register"]
pub mod rtcps;
#[doc = "RTCIV (r) register accessor: Real-Time Clock Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtciv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtciv`]
module"]
#[doc(alias = "RTCIV")]
pub type Rtciv = crate::Reg<rtciv::RtcivSpec>;
#[doc = "Real-Time Clock Interrupt Vector Register"]
pub mod rtciv;
#[doc = "RTCTIM0 (rw) register accessor: RTCTIM0 Register Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtctim0`]
module"]
#[doc(alias = "RTCTIM0")]
pub type Rtctim0 = crate::Reg<rtctim0::Rtctim0Spec>;
#[doc = "RTCTIM0 Register Hexadecimal Format"]
pub mod rtctim0;
#[doc = "RTCTIM1 (rw) register accessor: Real-Time Clock Hour, Day of Week\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtctim1`]
module"]
#[doc(alias = "RTCTIM1")]
pub type Rtctim1 = crate::Reg<rtctim1::Rtctim1Spec>;
#[doc = "Real-Time Clock Hour, Day of Week"]
pub mod rtctim1;
#[doc = "RTCDATE (rw) register accessor: RTCDATE - Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcdate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcdate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcdate`]
module"]
#[doc(alias = "RTCDATE")]
pub type Rtcdate = crate::Reg<rtcdate::RtcdateSpec>;
#[doc = "RTCDATE - Hexadecimal Format"]
pub mod rtcdate;
#[doc = "RTCYEAR (rw) register accessor: RTCYEAR Register Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcyear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcyear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcyear`]
module"]
#[doc(alias = "RTCYEAR")]
pub type Rtcyear = crate::Reg<rtcyear::RtcyearSpec>;
#[doc = "RTCYEAR Register Hexadecimal Format"]
pub mod rtcyear;
#[doc = "RTCAMINHR (rw) register accessor: RTCMINHR - Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcaminhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcaminhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcaminhr`]
module"]
#[doc(alias = "RTCAMINHR")]
pub type Rtcaminhr = crate::Reg<rtcaminhr::RtcaminhrSpec>;
#[doc = "RTCMINHR - Hexadecimal Format"]
pub mod rtcaminhr;
#[doc = "RTCADOWDAY (rw) register accessor: RTCADOWDAY - Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcadowday::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcadowday::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcadowday`]
module"]
#[doc(alias = "RTCADOWDAY")]
pub type Rtcadowday = crate::Reg<rtcadowday::RtcadowdaySpec>;
#[doc = "RTCADOWDAY - Hexadecimal Format"]
pub mod rtcadowday;
#[doc = "RTCBIN2BCD (rw) register accessor: Binary-to-BCD Conversion Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcbin2bcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcbin2bcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcbin2bcd`]
module"]
#[doc(alias = "RTCBIN2BCD")]
pub type Rtcbin2bcd = crate::Reg<rtcbin2bcd::Rtcbin2bcdSpec>;
#[doc = "Binary-to-BCD Conversion Register"]
pub mod rtcbin2bcd;
#[doc = "RTCBCD2BIN (rw) register accessor: BCD-to-Binary Conversion Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcbcd2bin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcbcd2bin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcbcd2bin`]
module"]
#[doc(alias = "RTCBCD2BIN")]
pub type Rtcbcd2bin = crate::Reg<rtcbcd2bin::Rtcbcd2binSpec>;
#[doc = "BCD-to-Binary Conversion Register"]
pub mod rtcbcd2bin;
