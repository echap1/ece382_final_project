#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pain: Pain,
    paout: Paout,
    padir: Padir,
    paren: Paren,
    pads: Pads,
    pasel0: Pasel0,
    pasel1: Pasel1,
    p1iv: P1iv,
    _reserved8: [u8; 0x06],
    paselc: Paselc,
    paies: Paies,
    paie: Paie,
    paifg: Paifg,
    p2iv: P2iv,
    pbin: Pbin,
    pbout: Pbout,
    pbdir: Pbdir,
    pbren: Pbren,
    pbds: Pbds,
    pbsel0: Pbsel0,
    pbsel1: Pbsel1,
    p3iv: P3iv,
    _reserved21: [u8; 0x06],
    pbselc: Pbselc,
    pbies: Pbies,
    pbie: Pbie,
    pbifg: Pbifg,
    p4iv: P4iv,
    pcin: Pcin,
    pcout: Pcout,
    pcdir: Pcdir,
    pcren: Pcren,
    pcds: Pcds,
    pcsel0: Pcsel0,
    pcsel1: Pcsel1,
    p5iv: P5iv,
    _reserved34: [u8; 0x06],
    pcselc: Pcselc,
    pcies: Pcies,
    pcie: Pcie,
    pcifg: Pcifg,
    p6iv: P6iv,
    pdin: Pdin,
    pdout: Pdout,
    pddir: Pddir,
    pdren: Pdren,
    pdds: Pdds,
    pdsel0: Pdsel0,
    pdsel1: Pdsel1,
    p7iv: P7iv,
    _reserved47: [u8; 0x06],
    pdselc: Pdselc,
    pdies: Pdies,
    pdie: Pdie,
    pdifg: Pdifg,
    p8iv: P8iv,
    pein: Pein,
    peout: Peout,
    pedir: Pedir,
    peren: Peren,
    peds: Peds,
    pesel0: Pesel0,
    pesel1: Pesel1,
    p9iv: P9iv,
    _reserved60: [u8; 0x06],
    peselc: Peselc,
    peies: Peies,
    peie: Peie,
    peifg: Peifg,
    p10iv: P10iv,
    _reserved65: [u8; 0x80],
    pjin: Pjin,
    pjout: Pjout,
    pjdir: Pjdir,
    pjren: Pjren,
    pjds: Pjds,
    pjsel0: Pjsel0,
    pjsel1: Pjsel1,
    _reserved72: [u8; 0x08],
    pjselc: Pjselc,
}
impl RegisterBlock {
    #[doc = "0x00 - Port A Input"]
    #[inline(always)]
    pub const fn pain(&self) -> &Pain {
        &self.pain
    }
    #[doc = "0x02 - Port A Output"]
    #[inline(always)]
    pub const fn paout(&self) -> &Paout {
        &self.paout
    }
    #[doc = "0x04 - Port A Direction"]
    #[inline(always)]
    pub const fn padir(&self) -> &Padir {
        &self.padir
    }
    #[doc = "0x06 - Port A Resistor Enable"]
    #[inline(always)]
    pub const fn paren(&self) -> &Paren {
        &self.paren
    }
    #[doc = "0x08 - Port A Drive Strength"]
    #[inline(always)]
    pub const fn pads(&self) -> &Pads {
        &self.pads
    }
    #[doc = "0x0a - Port A Select 0"]
    #[inline(always)]
    pub const fn pasel0(&self) -> &Pasel0 {
        &self.pasel0
    }
    #[doc = "0x0c - Port A Select 1"]
    #[inline(always)]
    pub const fn pasel1(&self) -> &Pasel1 {
        &self.pasel1
    }
    #[doc = "0x0e - Port 1 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p1iv(&self) -> &P1iv {
        &self.p1iv
    }
    #[doc = "0x16 - Port A Complement Select"]
    #[inline(always)]
    pub const fn paselc(&self) -> &Paselc {
        &self.paselc
    }
    #[doc = "0x18 - Port A Interrupt Edge Select"]
    #[inline(always)]
    pub const fn paies(&self) -> &Paies {
        &self.paies
    }
    #[doc = "0x1a - Port A Interrupt Enable"]
    #[inline(always)]
    pub const fn paie(&self) -> &Paie {
        &self.paie
    }
    #[doc = "0x1c - Port A Interrupt Flag"]
    #[inline(always)]
    pub const fn paifg(&self) -> &Paifg {
        &self.paifg
    }
    #[doc = "0x1e - Port 2 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p2iv(&self) -> &P2iv {
        &self.p2iv
    }
    #[doc = "0x20 - Port B Input"]
    #[inline(always)]
    pub const fn pbin(&self) -> &Pbin {
        &self.pbin
    }
    #[doc = "0x22 - Port B Output"]
    #[inline(always)]
    pub const fn pbout(&self) -> &Pbout {
        &self.pbout
    }
    #[doc = "0x24 - Port B Direction"]
    #[inline(always)]
    pub const fn pbdir(&self) -> &Pbdir {
        &self.pbdir
    }
    #[doc = "0x26 - Port B Resistor Enable"]
    #[inline(always)]
    pub const fn pbren(&self) -> &Pbren {
        &self.pbren
    }
    #[doc = "0x28 - Port B Drive Strength"]
    #[inline(always)]
    pub const fn pbds(&self) -> &Pbds {
        &self.pbds
    }
    #[doc = "0x2a - Port B Select 0"]
    #[inline(always)]
    pub const fn pbsel0(&self) -> &Pbsel0 {
        &self.pbsel0
    }
    #[doc = "0x2c - Port B Select 1"]
    #[inline(always)]
    pub const fn pbsel1(&self) -> &Pbsel1 {
        &self.pbsel1
    }
    #[doc = "0x2e - Port 3 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p3iv(&self) -> &P3iv {
        &self.p3iv
    }
    #[doc = "0x36 - Port B Complement Select"]
    #[inline(always)]
    pub const fn pbselc(&self) -> &Pbselc {
        &self.pbselc
    }
    #[doc = "0x38 - Port B Interrupt Edge Select"]
    #[inline(always)]
    pub const fn pbies(&self) -> &Pbies {
        &self.pbies
    }
    #[doc = "0x3a - Port B Interrupt Enable"]
    #[inline(always)]
    pub const fn pbie(&self) -> &Pbie {
        &self.pbie
    }
    #[doc = "0x3c - Port B Interrupt Flag"]
    #[inline(always)]
    pub const fn pbifg(&self) -> &Pbifg {
        &self.pbifg
    }
    #[doc = "0x3e - Port 4 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p4iv(&self) -> &P4iv {
        &self.p4iv
    }
    #[doc = "0x40 - Port C Input"]
    #[inline(always)]
    pub const fn pcin(&self) -> &Pcin {
        &self.pcin
    }
    #[doc = "0x42 - Port C Output"]
    #[inline(always)]
    pub const fn pcout(&self) -> &Pcout {
        &self.pcout
    }
    #[doc = "0x44 - Port C Direction"]
    #[inline(always)]
    pub const fn pcdir(&self) -> &Pcdir {
        &self.pcdir
    }
    #[doc = "0x46 - Port C Resistor Enable"]
    #[inline(always)]
    pub const fn pcren(&self) -> &Pcren {
        &self.pcren
    }
    #[doc = "0x48 - Port C Drive Strength"]
    #[inline(always)]
    pub const fn pcds(&self) -> &Pcds {
        &self.pcds
    }
    #[doc = "0x4a - Port C Select 0"]
    #[inline(always)]
    pub const fn pcsel0(&self) -> &Pcsel0 {
        &self.pcsel0
    }
    #[doc = "0x4c - Port C Select 1"]
    #[inline(always)]
    pub const fn pcsel1(&self) -> &Pcsel1 {
        &self.pcsel1
    }
    #[doc = "0x4e - Port 5 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p5iv(&self) -> &P5iv {
        &self.p5iv
    }
    #[doc = "0x56 - Port C Complement Select"]
    #[inline(always)]
    pub const fn pcselc(&self) -> &Pcselc {
        &self.pcselc
    }
    #[doc = "0x58 - Port C Interrupt Edge Select"]
    #[inline(always)]
    pub const fn pcies(&self) -> &Pcies {
        &self.pcies
    }
    #[doc = "0x5a - Port C Interrupt Enable"]
    #[inline(always)]
    pub const fn pcie(&self) -> &Pcie {
        &self.pcie
    }
    #[doc = "0x5c - Port C Interrupt Flag"]
    #[inline(always)]
    pub const fn pcifg(&self) -> &Pcifg {
        &self.pcifg
    }
    #[doc = "0x5e - Port 6 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p6iv(&self) -> &P6iv {
        &self.p6iv
    }
    #[doc = "0x60 - Port D Input"]
    #[inline(always)]
    pub const fn pdin(&self) -> &Pdin {
        &self.pdin
    }
    #[doc = "0x62 - Port D Output"]
    #[inline(always)]
    pub const fn pdout(&self) -> &Pdout {
        &self.pdout
    }
    #[doc = "0x64 - Port D Direction"]
    #[inline(always)]
    pub const fn pddir(&self) -> &Pddir {
        &self.pddir
    }
    #[doc = "0x66 - Port D Resistor Enable"]
    #[inline(always)]
    pub const fn pdren(&self) -> &Pdren {
        &self.pdren
    }
    #[doc = "0x68 - Port D Drive Strength"]
    #[inline(always)]
    pub const fn pdds(&self) -> &Pdds {
        &self.pdds
    }
    #[doc = "0x6a - Port D Select 0"]
    #[inline(always)]
    pub const fn pdsel0(&self) -> &Pdsel0 {
        &self.pdsel0
    }
    #[doc = "0x6c - Port D Select 1"]
    #[inline(always)]
    pub const fn pdsel1(&self) -> &Pdsel1 {
        &self.pdsel1
    }
    #[doc = "0x6e - Port 7 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p7iv(&self) -> &P7iv {
        &self.p7iv
    }
    #[doc = "0x76 - Port D Complement Select"]
    #[inline(always)]
    pub const fn pdselc(&self) -> &Pdselc {
        &self.pdselc
    }
    #[doc = "0x78 - Port D Interrupt Edge Select"]
    #[inline(always)]
    pub const fn pdies(&self) -> &Pdies {
        &self.pdies
    }
    #[doc = "0x7a - Port D Interrupt Enable"]
    #[inline(always)]
    pub const fn pdie(&self) -> &Pdie {
        &self.pdie
    }
    #[doc = "0x7c - Port D Interrupt Flag"]
    #[inline(always)]
    pub const fn pdifg(&self) -> &Pdifg {
        &self.pdifg
    }
    #[doc = "0x7e - Port 8 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p8iv(&self) -> &P8iv {
        &self.p8iv
    }
    #[doc = "0x80 - Port E Input"]
    #[inline(always)]
    pub const fn pein(&self) -> &Pein {
        &self.pein
    }
    #[doc = "0x82 - Port E Output"]
    #[inline(always)]
    pub const fn peout(&self) -> &Peout {
        &self.peout
    }
    #[doc = "0x84 - Port E Direction"]
    #[inline(always)]
    pub const fn pedir(&self) -> &Pedir {
        &self.pedir
    }
    #[doc = "0x86 - Port E Resistor Enable"]
    #[inline(always)]
    pub const fn peren(&self) -> &Peren {
        &self.peren
    }
    #[doc = "0x88 - Port E Drive Strength"]
    #[inline(always)]
    pub const fn peds(&self) -> &Peds {
        &self.peds
    }
    #[doc = "0x8a - Port E Select 0"]
    #[inline(always)]
    pub const fn pesel0(&self) -> &Pesel0 {
        &self.pesel0
    }
    #[doc = "0x8c - Port E Select 1"]
    #[inline(always)]
    pub const fn pesel1(&self) -> &Pesel1 {
        &self.pesel1
    }
    #[doc = "0x8e - Port 9 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p9iv(&self) -> &P9iv {
        &self.p9iv
    }
    #[doc = "0x96 - Port E Complement Select"]
    #[inline(always)]
    pub const fn peselc(&self) -> &Peselc {
        &self.peselc
    }
    #[doc = "0x98 - Port E Interrupt Edge Select"]
    #[inline(always)]
    pub const fn peies(&self) -> &Peies {
        &self.peies
    }
    #[doc = "0x9a - Port E Interrupt Enable"]
    #[inline(always)]
    pub const fn peie(&self) -> &Peie {
        &self.peie
    }
    #[doc = "0x9c - Port E Interrupt Flag"]
    #[inline(always)]
    pub const fn peifg(&self) -> &Peifg {
        &self.peifg
    }
    #[doc = "0x9e - Port 10 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p10iv(&self) -> &P10iv {
        &self.p10iv
    }
    #[doc = "0x120 - Port J Input"]
    #[inline(always)]
    pub const fn pjin(&self) -> &Pjin {
        &self.pjin
    }
    #[doc = "0x122 - Port J Output"]
    #[inline(always)]
    pub const fn pjout(&self) -> &Pjout {
        &self.pjout
    }
    #[doc = "0x124 - Port J Direction"]
    #[inline(always)]
    pub const fn pjdir(&self) -> &Pjdir {
        &self.pjdir
    }
    #[doc = "0x126 - Port J Resistor Enable"]
    #[inline(always)]
    pub const fn pjren(&self) -> &Pjren {
        &self.pjren
    }
    #[doc = "0x128 - Port J Drive Strength"]
    #[inline(always)]
    pub const fn pjds(&self) -> &Pjds {
        &self.pjds
    }
    #[doc = "0x12a - Port J Select 0"]
    #[inline(always)]
    pub const fn pjsel0(&self) -> &Pjsel0 {
        &self.pjsel0
    }
    #[doc = "0x12c - Port J Select 1"]
    #[inline(always)]
    pub const fn pjsel1(&self) -> &Pjsel1 {
        &self.pjsel1
    }
    #[doc = "0x136 - Port J Complement Select"]
    #[inline(always)]
    pub const fn pjselc(&self) -> &Pjselc {
        &self.pjselc
    }
}
#[doc = "PAIN (r) register accessor: Port A Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pain::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pain`]
module"]
#[doc(alias = "PAIN")]
pub type Pain = crate::Reg<pain::PainSpec>;
#[doc = "Port A Input"]
pub mod pain;
#[doc = "PAOUT (rw) register accessor: Port A Output\n\nYou can [`read`](crate::Reg::read) this register and get [`paout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paout`]
module"]
#[doc(alias = "PAOUT")]
pub type Paout = crate::Reg<paout::PaoutSpec>;
#[doc = "Port A Output"]
pub mod paout;
#[doc = "PADIR (rw) register accessor: Port A Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`padir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padir`]
module"]
#[doc(alias = "PADIR")]
pub type Padir = crate::Reg<padir::PadirSpec>;
#[doc = "Port A Direction"]
pub mod padir;
#[doc = "PAREN (rw) register accessor: Port A Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`paren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paren`]
module"]
#[doc(alias = "PAREN")]
pub type Paren = crate::Reg<paren::ParenSpec>;
#[doc = "Port A Resistor Enable"]
pub mod paren;
#[doc = "PADS (rw) register accessor: Port A Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`pads::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pads::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pads`]
module"]
#[doc(alias = "PADS")]
pub type Pads = crate::Reg<pads::PadsSpec>;
#[doc = "Port A Drive Strength"]
pub mod pads;
#[doc = "PASEL0 (rw) register accessor: Port A Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pasel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pasel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pasel0`]
module"]
#[doc(alias = "PASEL0")]
pub type Pasel0 = crate::Reg<pasel0::Pasel0Spec>;
#[doc = "Port A Select 0"]
pub mod pasel0;
#[doc = "PASEL1 (rw) register accessor: Port A Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pasel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pasel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pasel1`]
module"]
#[doc(alias = "PASEL1")]
pub type Pasel1 = crate::Reg<pasel1::Pasel1Spec>;
#[doc = "Port A Select 1"]
pub mod pasel1;
#[doc = "P1IV (r) register accessor: Port 1 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p1iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p1iv`]
module"]
#[doc(alias = "P1IV")]
pub type P1iv = crate::Reg<p1iv::P1ivSpec>;
#[doc = "Port 1 Interrupt Vector Register"]
pub mod p1iv;
#[doc = "PASELC (rw) register accessor: Port A Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`paselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paselc`]
module"]
#[doc(alias = "PASELC")]
pub type Paselc = crate::Reg<paselc::PaselcSpec>;
#[doc = "Port A Complement Select"]
pub mod paselc;
#[doc = "PAIES (rw) register accessor: Port A Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`paies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paies`]
module"]
#[doc(alias = "PAIES")]
pub type Paies = crate::Reg<paies::PaiesSpec>;
#[doc = "Port A Interrupt Edge Select"]
pub mod paies;
#[doc = "PAIE (rw) register accessor: Port A Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`paie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paie`]
module"]
#[doc(alias = "PAIE")]
pub type Paie = crate::Reg<paie::PaieSpec>;
#[doc = "Port A Interrupt Enable"]
pub mod paie;
#[doc = "PAIFG (rw) register accessor: Port A Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`paifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paifg`]
module"]
#[doc(alias = "PAIFG")]
pub type Paifg = crate::Reg<paifg::PaifgSpec>;
#[doc = "Port A Interrupt Flag"]
pub mod paifg;
#[doc = "P2IV (r) register accessor: Port 2 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p2iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p2iv`]
module"]
#[doc(alias = "P2IV")]
pub type P2iv = crate::Reg<p2iv::P2ivSpec>;
#[doc = "Port 2 Interrupt Vector Register"]
pub mod p2iv;
#[doc = "PBIN (r) register accessor: Port B Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pbin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbin`]
module"]
#[doc(alias = "PBIN")]
pub type Pbin = crate::Reg<pbin::PbinSpec>;
#[doc = "Port B Input"]
pub mod pbin;
#[doc = "PBOUT (rw) register accessor: Port B Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pbout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbout`]
module"]
#[doc(alias = "PBOUT")]
pub type Pbout = crate::Reg<pbout::PboutSpec>;
#[doc = "Port B Output"]
pub mod pbout;
#[doc = "PBDIR (rw) register accessor: Port B Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pbdir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbdir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbdir`]
module"]
#[doc(alias = "PBDIR")]
pub type Pbdir = crate::Reg<pbdir::PbdirSpec>;
#[doc = "Port B Direction"]
pub mod pbdir;
#[doc = "PBREN (rw) register accessor: Port B Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pbren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbren`]
module"]
#[doc(alias = "PBREN")]
pub type Pbren = crate::Reg<pbren::PbrenSpec>;
#[doc = "Port B Resistor Enable"]
pub mod pbren;
#[doc = "PBDS (rw) register accessor: Port B Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`pbds::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbds`]
module"]
#[doc(alias = "PBDS")]
pub type Pbds = crate::Reg<pbds::PbdsSpec>;
#[doc = "Port B Drive Strength"]
pub mod pbds;
#[doc = "PBSEL0 (rw) register accessor: Port B Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbsel0`]
module"]
#[doc(alias = "PBSEL0")]
pub type Pbsel0 = crate::Reg<pbsel0::Pbsel0Spec>;
#[doc = "Port B Select 0"]
pub mod pbsel0;
#[doc = "PBSEL1 (rw) register accessor: Port B Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbsel1`]
module"]
#[doc(alias = "PBSEL1")]
pub type Pbsel1 = crate::Reg<pbsel1::Pbsel1Spec>;
#[doc = "Port B Select 1"]
pub mod pbsel1;
#[doc = "P3IV (r) register accessor: Port 3 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p3iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3iv`]
module"]
#[doc(alias = "P3IV")]
pub type P3iv = crate::Reg<p3iv::P3ivSpec>;
#[doc = "Port 3 Interrupt Vector Register"]
pub mod p3iv;
#[doc = "PBSELC (rw) register accessor: Port B Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pbselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbselc`]
module"]
#[doc(alias = "PBSELC")]
pub type Pbselc = crate::Reg<pbselc::PbselcSpec>;
#[doc = "Port B Complement Select"]
pub mod pbselc;
#[doc = "PBIES (rw) register accessor: Port B Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pbies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbies`]
module"]
#[doc(alias = "PBIES")]
pub type Pbies = crate::Reg<pbies::PbiesSpec>;
#[doc = "Port B Interrupt Edge Select"]
pub mod pbies;
#[doc = "PBIE (rw) register accessor: Port B Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pbie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbie`]
module"]
#[doc(alias = "PBIE")]
pub type Pbie = crate::Reg<pbie::PbieSpec>;
#[doc = "Port B Interrupt Enable"]
pub mod pbie;
#[doc = "PBIFG (rw) register accessor: Port B Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pbifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbifg`]
module"]
#[doc(alias = "PBIFG")]
pub type Pbifg = crate::Reg<pbifg::PbifgSpec>;
#[doc = "Port B Interrupt Flag"]
pub mod pbifg;
#[doc = "P4IV (r) register accessor: Port 4 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p4iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4iv`]
module"]
#[doc(alias = "P4IV")]
pub type P4iv = crate::Reg<p4iv::P4ivSpec>;
#[doc = "Port 4 Interrupt Vector Register"]
pub mod p4iv;
#[doc = "PCIN (r) register accessor: Port C Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pcin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcin`]
module"]
#[doc(alias = "PCIN")]
pub type Pcin = crate::Reg<pcin::PcinSpec>;
#[doc = "Port C Input"]
pub mod pcin;
#[doc = "PCOUT (rw) register accessor: Port C Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pcout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcout`]
module"]
#[doc(alias = "PCOUT")]
pub type Pcout = crate::Reg<pcout::PcoutSpec>;
#[doc = "Port C Output"]
pub mod pcout;
#[doc = "PCDIR (rw) register accessor: Port C Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pcdir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcdir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcdir`]
module"]
#[doc(alias = "PCDIR")]
pub type Pcdir = crate::Reg<pcdir::PcdirSpec>;
#[doc = "Port C Direction"]
pub mod pcdir;
#[doc = "PCREN (rw) register accessor: Port C Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pcren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcren`]
module"]
#[doc(alias = "PCREN")]
pub type Pcren = crate::Reg<pcren::PcrenSpec>;
#[doc = "Port C Resistor Enable"]
pub mod pcren;
#[doc = "PCDS (rw) register accessor: Port C Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`pcds::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcds`]
module"]
#[doc(alias = "PCDS")]
pub type Pcds = crate::Reg<pcds::PcdsSpec>;
#[doc = "Port C Drive Strength"]
pub mod pcds;
#[doc = "PCSEL0 (rw) register accessor: Port C Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcsel0`]
module"]
#[doc(alias = "PCSEL0")]
pub type Pcsel0 = crate::Reg<pcsel0::Pcsel0Spec>;
#[doc = "Port C Select 0"]
pub mod pcsel0;
#[doc = "PCSEL1 (rw) register accessor: Port C Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcsel1`]
module"]
#[doc(alias = "PCSEL1")]
pub type Pcsel1 = crate::Reg<pcsel1::Pcsel1Spec>;
#[doc = "Port C Select 1"]
pub mod pcsel1;
#[doc = "P5IV (r) register accessor: Port 5 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p5iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p5iv`]
module"]
#[doc(alias = "P5IV")]
pub type P5iv = crate::Reg<p5iv::P5ivSpec>;
#[doc = "Port 5 Interrupt Vector Register"]
pub mod p5iv;
#[doc = "PCSELC (rw) register accessor: Port C Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pcselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcselc`]
module"]
#[doc(alias = "PCSELC")]
pub type Pcselc = crate::Reg<pcselc::PcselcSpec>;
#[doc = "Port C Complement Select"]
pub mod pcselc;
#[doc = "PCIES (rw) register accessor: Port C Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pcies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcies`]
module"]
#[doc(alias = "PCIES")]
pub type Pcies = crate::Reg<pcies::PciesSpec>;
#[doc = "Port C Interrupt Edge Select"]
pub mod pcies;
#[doc = "PCIE (rw) register accessor: Port C Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie`]
module"]
#[doc(alias = "PCIE")]
pub type Pcie = crate::Reg<pcie::PcieSpec>;
#[doc = "Port C Interrupt Enable"]
pub mod pcie;
#[doc = "PCIFG (rw) register accessor: Port C Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pcifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcifg`]
module"]
#[doc(alias = "PCIFG")]
pub type Pcifg = crate::Reg<pcifg::PcifgSpec>;
#[doc = "Port C Interrupt Flag"]
pub mod pcifg;
#[doc = "P6IV (r) register accessor: Port 6 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p6iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6iv`]
module"]
#[doc(alias = "P6IV")]
pub type P6iv = crate::Reg<p6iv::P6ivSpec>;
#[doc = "Port 6 Interrupt Vector Register"]
pub mod p6iv;
#[doc = "PDIN (r) register accessor: Port D Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pdin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdin`]
module"]
#[doc(alias = "PDIN")]
pub type Pdin = crate::Reg<pdin::PdinSpec>;
#[doc = "Port D Input"]
pub mod pdin;
#[doc = "PDOUT (rw) register accessor: Port D Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pdout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdout`]
module"]
#[doc(alias = "PDOUT")]
pub type Pdout = crate::Reg<pdout::PdoutSpec>;
#[doc = "Port D Output"]
pub mod pdout;
#[doc = "PDDIR (rw) register accessor: Port D Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pddir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pddir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pddir`]
module"]
#[doc(alias = "PDDIR")]
pub type Pddir = crate::Reg<pddir::PddirSpec>;
#[doc = "Port D Direction"]
pub mod pddir;
#[doc = "PDREN (rw) register accessor: Port D Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pdren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdren`]
module"]
#[doc(alias = "PDREN")]
pub type Pdren = crate::Reg<pdren::PdrenSpec>;
#[doc = "Port D Resistor Enable"]
pub mod pdren;
#[doc = "PDDS (rw) register accessor: Port D Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`pdds::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdds`]
module"]
#[doc(alias = "PDDS")]
pub type Pdds = crate::Reg<pdds::PddsSpec>;
#[doc = "Port D Drive Strength"]
pub mod pdds;
#[doc = "PDSEL0 (rw) register accessor: Port D Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsel0`]
module"]
#[doc(alias = "PDSEL0")]
pub type Pdsel0 = crate::Reg<pdsel0::Pdsel0Spec>;
#[doc = "Port D Select 0"]
pub mod pdsel0;
#[doc = "PDSEL1 (rw) register accessor: Port D Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsel1`]
module"]
#[doc(alias = "PDSEL1")]
pub type Pdsel1 = crate::Reg<pdsel1::Pdsel1Spec>;
#[doc = "Port D Select 1"]
pub mod pdsel1;
#[doc = "P7IV (r) register accessor: Port 7 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p7iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p7iv`]
module"]
#[doc(alias = "P7IV")]
pub type P7iv = crate::Reg<p7iv::P7ivSpec>;
#[doc = "Port 7 Interrupt Vector Register"]
pub mod p7iv;
#[doc = "PDSELC (rw) register accessor: Port D Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pdselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdselc`]
module"]
#[doc(alias = "PDSELC")]
pub type Pdselc = crate::Reg<pdselc::PdselcSpec>;
#[doc = "Port D Complement Select"]
pub mod pdselc;
#[doc = "PDIES (rw) register accessor: Port D Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pdies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdies`]
module"]
#[doc(alias = "PDIES")]
pub type Pdies = crate::Reg<pdies::PdiesSpec>;
#[doc = "Port D Interrupt Edge Select"]
pub mod pdies;
#[doc = "PDIE (rw) register accessor: Port D Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pdie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdie`]
module"]
#[doc(alias = "PDIE")]
pub type Pdie = crate::Reg<pdie::PdieSpec>;
#[doc = "Port D Interrupt Enable"]
pub mod pdie;
#[doc = "PDIFG (rw) register accessor: Port D Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pdifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdifg`]
module"]
#[doc(alias = "PDIFG")]
pub type Pdifg = crate::Reg<pdifg::PdifgSpec>;
#[doc = "Port D Interrupt Flag"]
pub mod pdifg;
#[doc = "P8IV (r) register accessor: Port 8 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p8iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p8iv`]
module"]
#[doc(alias = "P8IV")]
pub type P8iv = crate::Reg<p8iv::P8ivSpec>;
#[doc = "Port 8 Interrupt Vector Register"]
pub mod p8iv;
#[doc = "PEIN (r) register accessor: Port E Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pein::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pein`]
module"]
#[doc(alias = "PEIN")]
pub type Pein = crate::Reg<pein::PeinSpec>;
#[doc = "Port E Input"]
pub mod pein;
#[doc = "PEOUT (rw) register accessor: Port E Output\n\nYou can [`read`](crate::Reg::read) this register and get [`peout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peout`]
module"]
#[doc(alias = "PEOUT")]
pub type Peout = crate::Reg<peout::PeoutSpec>;
#[doc = "Port E Output"]
pub mod peout;
#[doc = "PEDIR (rw) register accessor: Port E Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pedir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pedir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pedir`]
module"]
#[doc(alias = "PEDIR")]
pub type Pedir = crate::Reg<pedir::PedirSpec>;
#[doc = "Port E Direction"]
pub mod pedir;
#[doc = "PEREN (rw) register accessor: Port E Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`peren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peren`]
module"]
#[doc(alias = "PEREN")]
pub type Peren = crate::Reg<peren::PerenSpec>;
#[doc = "Port E Resistor Enable"]
pub mod peren;
#[doc = "PEDS (rw) register accessor: Port E Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`peds::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peds`]
module"]
#[doc(alias = "PEDS")]
pub type Peds = crate::Reg<peds::PedsSpec>;
#[doc = "Port E Drive Strength"]
pub mod peds;
#[doc = "PESEL0 (rw) register accessor: Port E Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pesel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pesel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pesel0`]
module"]
#[doc(alias = "PESEL0")]
pub type Pesel0 = crate::Reg<pesel0::Pesel0Spec>;
#[doc = "Port E Select 0"]
pub mod pesel0;
#[doc = "PESEL1 (rw) register accessor: Port E Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pesel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pesel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pesel1`]
module"]
#[doc(alias = "PESEL1")]
pub type Pesel1 = crate::Reg<pesel1::Pesel1Spec>;
#[doc = "Port E Select 1"]
pub mod pesel1;
#[doc = "P9IV (r) register accessor: Port 9 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p9iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p9iv`]
module"]
#[doc(alias = "P9IV")]
pub type P9iv = crate::Reg<p9iv::P9ivSpec>;
#[doc = "Port 9 Interrupt Vector Register"]
pub mod p9iv;
#[doc = "PESELC (rw) register accessor: Port E Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`peselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peselc`]
module"]
#[doc(alias = "PESELC")]
pub type Peselc = crate::Reg<peselc::PeselcSpec>;
#[doc = "Port E Complement Select"]
pub mod peselc;
#[doc = "PEIES (rw) register accessor: Port E Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`peies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peies`]
module"]
#[doc(alias = "PEIES")]
pub type Peies = crate::Reg<peies::PeiesSpec>;
#[doc = "Port E Interrupt Edge Select"]
pub mod peies;
#[doc = "PEIE (rw) register accessor: Port E Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`peie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peie`]
module"]
#[doc(alias = "PEIE")]
pub type Peie = crate::Reg<peie::PeieSpec>;
#[doc = "Port E Interrupt Enable"]
pub mod peie;
#[doc = "PEIFG (rw) register accessor: Port E Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`peifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peifg`]
module"]
#[doc(alias = "PEIFG")]
pub type Peifg = crate::Reg<peifg::PeifgSpec>;
#[doc = "Port E Interrupt Flag"]
pub mod peifg;
#[doc = "P10IV (r) register accessor: Port 10 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p10iv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p10iv`]
module"]
#[doc(alias = "P10IV")]
pub type P10iv = crate::Reg<p10iv::P10ivSpec>;
#[doc = "Port 10 Interrupt Vector Register"]
pub mod p10iv;
#[doc = "PJIN (r) register accessor: Port J Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pjin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjin`]
module"]
#[doc(alias = "PJIN")]
pub type Pjin = crate::Reg<pjin::PjinSpec>;
#[doc = "Port J Input"]
pub mod pjin;
#[doc = "PJOUT (rw) register accessor: Port J Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pjout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjout`]
module"]
#[doc(alias = "PJOUT")]
pub type Pjout = crate::Reg<pjout::PjoutSpec>;
#[doc = "Port J Output"]
pub mod pjout;
#[doc = "PJDIR (rw) register accessor: Port J Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pjdir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjdir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjdir`]
module"]
#[doc(alias = "PJDIR")]
pub type Pjdir = crate::Reg<pjdir::PjdirSpec>;
#[doc = "Port J Direction"]
pub mod pjdir;
#[doc = "PJREN (rw) register accessor: Port J Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pjren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjren`]
module"]
#[doc(alias = "PJREN")]
pub type Pjren = crate::Reg<pjren::PjrenSpec>;
#[doc = "Port J Resistor Enable"]
pub mod pjren;
#[doc = "PJDS (rw) register accessor: Port J Drive Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`pjds::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjds`]
module"]
#[doc(alias = "PJDS")]
pub type Pjds = crate::Reg<pjds::PjdsSpec>;
#[doc = "Port J Drive Strength"]
pub mod pjds;
#[doc = "PJSEL0 (rw) register accessor: Port J Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pjsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjsel0`]
module"]
#[doc(alias = "PJSEL0")]
pub type Pjsel0 = crate::Reg<pjsel0::Pjsel0Spec>;
#[doc = "Port J Select 0"]
pub mod pjsel0;
#[doc = "PJSEL1 (rw) register accessor: Port J Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pjsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjsel1`]
module"]
#[doc(alias = "PJSEL1")]
pub type Pjsel1 = crate::Reg<pjsel1::Pjsel1Spec>;
#[doc = "Port J Select 1"]
pub mod pjsel1;
#[doc = "PJSELC (rw) register accessor: Port J Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pjselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pjselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pjselc`]
module"]
#[doc(alias = "PJSELC")]
pub type Pjselc = crate::Reg<pjselc::PjselcSpec>;
#[doc = "Port J Complement Select"]
pub mod pjselc;
