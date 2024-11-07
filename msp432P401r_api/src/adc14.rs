#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc14ctl0: Adc14ctl0,
    adc14ctl1: Adc14ctl1,
    adc14lo0: Adc14lo0,
    adc14hi0: Adc14hi0,
    adc14lo1: Adc14lo1,
    adc14hi1: Adc14hi1,
    adc14mctl: [Adc14mctl; 32],
    adc14mem: [Adc14mem; 32],
    _reserved8: [u8; 0x24],
    adc14ier0: Adc14ier0,
    adc14ier1: Adc14ier1,
    adc14ifgr0: Adc14ifgr0,
    adc14ifgr1: Adc14ifgr1,
    adc14clrifgr0: Adc14clrifgr0,
    adc14clrifgr1: Adc14clrifgr1,
    adc14iv: Adc14iv,
}
impl RegisterBlock {
    #[doc = "0x00 - Control 0 Register"]
    #[inline(always)]
    pub const fn adc14ctl0(&self) -> &Adc14ctl0 {
        &self.adc14ctl0
    }
    #[doc = "0x04 - Control 1 Register"]
    #[inline(always)]
    pub const fn adc14ctl1(&self) -> &Adc14ctl1 {
        &self.adc14ctl1
    }
    #[doc = "0x08 - Window Comparator Low Threshold 0 Register"]
    #[inline(always)]
    pub const fn adc14lo0(&self) -> &Adc14lo0 {
        &self.adc14lo0
    }
    #[doc = "0x0c - Window Comparator High Threshold 0 Register"]
    #[inline(always)]
    pub const fn adc14hi0(&self) -> &Adc14hi0 {
        &self.adc14hi0
    }
    #[doc = "0x10 - Window Comparator Low Threshold 1 Register"]
    #[inline(always)]
    pub const fn adc14lo1(&self) -> &Adc14lo1 {
        &self.adc14lo1
    }
    #[doc = "0x14 - Window Comparator High Threshold 1 Register"]
    #[inline(always)]
    pub const fn adc14hi1(&self) -> &Adc14hi1 {
        &self.adc14hi1
    }
    #[doc = "0x18..0x98 - Conversion Memory Control Register"]
    #[inline(always)]
    pub const fn adc14mctl(&self, n: usize) -> &Adc14mctl {
        &self.adc14mctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x98 - Conversion Memory Control Register"]
    #[inline(always)]
    pub fn adc14mctl_iter(&self) -> impl Iterator<Item = &Adc14mctl> {
        self.adc14mctl.iter()
    }
    #[doc = "0x98..0x118 - Conversion Memory Register"]
    #[inline(always)]
    pub const fn adc14mem(&self, n: usize) -> &Adc14mem {
        &self.adc14mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x98..0x118 - Conversion Memory Register"]
    #[inline(always)]
    pub fn adc14mem_iter(&self) -> impl Iterator<Item = &Adc14mem> {
        self.adc14mem.iter()
    }
    #[doc = "0x13c - Interrupt Enable 0 Register"]
    #[inline(always)]
    pub const fn adc14ier0(&self) -> &Adc14ier0 {
        &self.adc14ier0
    }
    #[doc = "0x140 - Interrupt Enable 1 Register"]
    #[inline(always)]
    pub const fn adc14ier1(&self) -> &Adc14ier1 {
        &self.adc14ier1
    }
    #[doc = "0x144 - Interrupt Flag 0 Register"]
    #[inline(always)]
    pub const fn adc14ifgr0(&self) -> &Adc14ifgr0 {
        &self.adc14ifgr0
    }
    #[doc = "0x148 - Interrupt Flag 1 Register"]
    #[inline(always)]
    pub const fn adc14ifgr1(&self) -> &Adc14ifgr1 {
        &self.adc14ifgr1
    }
    #[doc = "0x14c - Clear Interrupt Flag 0 Register"]
    #[inline(always)]
    pub const fn adc14clrifgr0(&self) -> &Adc14clrifgr0 {
        &self.adc14clrifgr0
    }
    #[doc = "0x150 - Clear Interrupt Flag 1 Register"]
    #[inline(always)]
    pub const fn adc14clrifgr1(&self) -> &Adc14clrifgr1 {
        &self.adc14clrifgr1
    }
    #[doc = "0x154 - Interrupt Vector Register"]
    #[inline(always)]
    pub const fn adc14iv(&self) -> &Adc14iv {
        &self.adc14iv
    }
}
#[doc = "ADC14CTL0 (rw) register accessor: Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14ctl0`]
module"]
#[doc(alias = "ADC14CTL0")]
pub type Adc14ctl0 = crate::Reg<adc14ctl0::Adc14ctl0Spec>;
#[doc = "Control 0 Register"]
pub mod adc14ctl0;
#[doc = "ADC14CTL1 (rw) register accessor: Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14ctl1`]
module"]
#[doc(alias = "ADC14CTL1")]
pub type Adc14ctl1 = crate::Reg<adc14ctl1::Adc14ctl1Spec>;
#[doc = "Control 1 Register"]
pub mod adc14ctl1;
#[doc = "ADC14LO0 (rw) register accessor: Window Comparator Low Threshold 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14lo0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14lo0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14lo0`]
module"]
#[doc(alias = "ADC14LO0")]
pub type Adc14lo0 = crate::Reg<adc14lo0::Adc14lo0Spec>;
#[doc = "Window Comparator Low Threshold 0 Register"]
pub mod adc14lo0;
#[doc = "ADC14HI0 (rw) register accessor: Window Comparator High Threshold 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14hi0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14hi0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14hi0`]
module"]
#[doc(alias = "ADC14HI0")]
pub type Adc14hi0 = crate::Reg<adc14hi0::Adc14hi0Spec>;
#[doc = "Window Comparator High Threshold 0 Register"]
pub mod adc14hi0;
#[doc = "ADC14LO1 (rw) register accessor: Window Comparator Low Threshold 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14lo1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14lo1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14lo1`]
module"]
#[doc(alias = "ADC14LO1")]
pub type Adc14lo1 = crate::Reg<adc14lo1::Adc14lo1Spec>;
#[doc = "Window Comparator Low Threshold 1 Register"]
pub mod adc14lo1;
#[doc = "ADC14HI1 (rw) register accessor: Window Comparator High Threshold 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14hi1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14hi1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14hi1`]
module"]
#[doc(alias = "ADC14HI1")]
pub type Adc14hi1 = crate::Reg<adc14hi1::Adc14hi1Spec>;
#[doc = "Window Comparator High Threshold 1 Register"]
pub mod adc14hi1;
#[doc = "ADC14MCTL (rw) register accessor: Conversion Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14mctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14mctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14mctl`]
module"]
#[doc(alias = "ADC14MCTL")]
pub type Adc14mctl = crate::Reg<adc14mctl::Adc14mctlSpec>;
#[doc = "Conversion Memory Control Register"]
pub mod adc14mctl;
#[doc = "ADC14MEM (rw) register accessor: Conversion Memory Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14mem`]
module"]
#[doc(alias = "ADC14MEM")]
pub type Adc14mem = crate::Reg<adc14mem::Adc14memSpec>;
#[doc = "Conversion Memory Register"]
pub mod adc14mem;
#[doc = "ADC14IER0 (rw) register accessor: Interrupt Enable 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ier0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14ier0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14ier0`]
module"]
#[doc(alias = "ADC14IER0")]
pub type Adc14ier0 = crate::Reg<adc14ier0::Adc14ier0Spec>;
#[doc = "Interrupt Enable 0 Register"]
pub mod adc14ier0;
#[doc = "ADC14IER1 (rw) register accessor: Interrupt Enable 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14ier1`]
module"]
#[doc(alias = "ADC14IER1")]
pub type Adc14ier1 = crate::Reg<adc14ier1::Adc14ier1Spec>;
#[doc = "Interrupt Enable 1 Register"]
pub mod adc14ier1;
#[doc = "ADC14IFGR0 (r) register accessor: Interrupt Flag 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ifgr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14ifgr0`]
module"]
#[doc(alias = "ADC14IFGR0")]
pub type Adc14ifgr0 = crate::Reg<adc14ifgr0::Adc14ifgr0Spec>;
#[doc = "Interrupt Flag 0 Register"]
pub mod adc14ifgr0;
#[doc = "ADC14IFGR1 (r) register accessor: Interrupt Flag 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ifgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14ifgr1`]
module"]
#[doc(alias = "ADC14IFGR1")]
pub type Adc14ifgr1 = crate::Reg<adc14ifgr1::Adc14ifgr1Spec>;
#[doc = "Interrupt Flag 1 Register"]
pub mod adc14ifgr1;
#[doc = "ADC14CLRIFGR0 (w) register accessor: Clear Interrupt Flag 0 Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14clrifgr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14clrifgr0`]
module"]
#[doc(alias = "ADC14CLRIFGR0")]
pub type Adc14clrifgr0 = crate::Reg<adc14clrifgr0::Adc14clrifgr0Spec>;
#[doc = "Clear Interrupt Flag 0 Register"]
pub mod adc14clrifgr0;
#[doc = "ADC14CLRIFGR1 (rw) register accessor: Clear Interrupt Flag 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14clrifgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14clrifgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14clrifgr1`]
module"]
#[doc(alias = "ADC14CLRIFGR1")]
pub type Adc14clrifgr1 = crate::Reg<adc14clrifgr1::Adc14clrifgr1Spec>;
#[doc = "Clear Interrupt Flag 1 Register"]
pub mod adc14clrifgr1;
#[doc = "ADC14IV (rw) register accessor: Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14iv`]
module"]
#[doc(alias = "ADC14IV")]
pub type Adc14iv = crate::Reg<adc14iv::Adc14ivSpec>;
#[doc = "Interrupt Vector Register"]
pub mod adc14iv;
