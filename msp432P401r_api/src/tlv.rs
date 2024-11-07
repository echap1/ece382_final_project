#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tlv_checksum: TlvChecksum,
    device_info_tag: DeviceInfoTag,
    device_info_len: DeviceInfoLen,
    device_id: DeviceId,
    hwrev: Hwrev,
    bcrev: Bcrev,
    rom_drvlib_rev: RomDrvlibRev,
    die_rec_tag: DieRecTag,
    die_rec_len: DieRecLen,
    die_xpos: DieXpos,
    die_ypos: DieYpos,
    wafer_id: WaferId,
    lot_id: LotId,
    reserved0: Reserved0,
    reserved1: Reserved1,
    reserved2: Reserved2,
    test_results: TestResults,
    cs_cal_tag: CsCalTag,
    cs_cal_len: CsCalLen,
    dcoir_fcal_rsel04: DcoirFcalRsel04,
    dcoir_fcal_rsel5: DcoirFcalRsel5,
    reserved3: Reserved3,
    reserved4: Reserved4,
    reserved5: Reserved5,
    reserved6: Reserved6,
    dcoir_constk_rsel04: DcoirConstkRsel04,
    dcoir_constk_rsel5: DcoirConstkRsel5,
    dcoer_fcal_rsel04: DcoerFcalRsel04,
    dcoer_fcal_rsel5: DcoerFcalRsel5,
    reserved7: Reserved7,
    reserved8: Reserved8,
    reserved9: Reserved9,
    reserved10: Reserved10,
    dcoer_constk_rsel04: DcoerConstkRsel04,
    dcoer_constk_rsel5: DcoerConstkRsel5,
    adc14_cal_tag: Adc14CalTag,
    adc14_cal_len: Adc14CalLen,
    adc_gain_factor: AdcGainFactor,
    adc_offset: AdcOffset,
    reserved11: Reserved11,
    reserved12: Reserved12,
    reserved13: Reserved13,
    reserved14: Reserved14,
    reserved15: Reserved15,
    reserved16: Reserved16,
    reserved17: Reserved17,
    reserved18: Reserved18,
    reserved19: Reserved19,
    reserved20: Reserved20,
    reserved21: Reserved21,
    reserved22: Reserved22,
    reserved23: Reserved23,
    reserved24: Reserved24,
    reserved25: Reserved25,
    reserved26: Reserved26,
    adc14_ref1p2v_ts30c: Adc14Ref1p2vTs30c,
    adc14_ref1p2v_ts85c: Adc14Ref1p2vTs85c,
    adc14_ref1p45v_ts30c: Adc14Ref1p45vTs30c,
    adc14_ref1p45v_ts85c: Adc14Ref1p45vTs85c,
    adc14_ref2p5v_ts30c: Adc14Ref2p5vTs30c,
    adc14_ref2p5v_ts85c: Adc14Ref2p5vTs85c,
    ref_cal_tag: RefCalTag,
    ref_cal_len: RefCalLen,
    ref_1p2v: Ref1p2v,
    ref_1p45v: Ref1p45v,
    ref_2p5v: Ref2p5v,
    flash_info_tag: FlashInfoTag,
    flash_info_len: FlashInfoLen,
    flash_max_prog_pulses: FlashMaxProgPulses,
    flash_max_erase_pulses: FlashMaxErasePulses,
    random_num_tag: RandomNumTag,
    random_num_len: RandomNumLen,
    random_num_1: RandomNum1,
    random_num_2: RandomNum2,
    random_num_3: RandomNum3,
    random_num_4: RandomNum4,
    bsl_cfg_tag: BslCfgTag,
    bsl_cfg_len: BslCfgLen,
    bsl_periphif_sel: BslPeriphifSel,
    bsl_portif_cfg_uart: BslPortifCfgUart,
    bsl_portif_cfg_spi: BslPortifCfgSpi,
    bsl_portif_cfg_i2c: BslPortifCfgI2c,
    tlv_end: TlvEnd,
}
impl RegisterBlock {
    #[doc = "0x00 - TLV Checksum"]
    #[inline(always)]
    pub const fn tlv_checksum(&self) -> &TlvChecksum {
        &self.tlv_checksum
    }
    #[doc = "0x04 - Device Info Tag"]
    #[inline(always)]
    pub const fn device_info_tag(&self) -> &DeviceInfoTag {
        &self.device_info_tag
    }
    #[doc = "0x08 - Device Info Length"]
    #[inline(always)]
    pub const fn device_info_len(&self) -> &DeviceInfoLen {
        &self.device_info_len
    }
    #[doc = "0x0c - Device ID"]
    #[inline(always)]
    pub const fn device_id(&self) -> &DeviceId {
        &self.device_id
    }
    #[doc = "0x10 - HW Revision"]
    #[inline(always)]
    pub const fn hwrev(&self) -> &Hwrev {
        &self.hwrev
    }
    #[doc = "0x14 - Boot Code Revision"]
    #[inline(always)]
    pub const fn bcrev(&self) -> &Bcrev {
        &self.bcrev
    }
    #[doc = "0x18 - ROM Driver Library Revision"]
    #[inline(always)]
    pub const fn rom_drvlib_rev(&self) -> &RomDrvlibRev {
        &self.rom_drvlib_rev
    }
    #[doc = "0x1c - Die Record Tag"]
    #[inline(always)]
    pub const fn die_rec_tag(&self) -> &DieRecTag {
        &self.die_rec_tag
    }
    #[doc = "0x20 - Die Record Length"]
    #[inline(always)]
    pub const fn die_rec_len(&self) -> &DieRecLen {
        &self.die_rec_len
    }
    #[doc = "0x24 - Die X-Position"]
    #[inline(always)]
    pub const fn die_xpos(&self) -> &DieXpos {
        &self.die_xpos
    }
    #[doc = "0x28 - Die Y-Position"]
    #[inline(always)]
    pub const fn die_ypos(&self) -> &DieYpos {
        &self.die_ypos
    }
    #[doc = "0x2c - Wafer ID"]
    #[inline(always)]
    pub const fn wafer_id(&self) -> &WaferId {
        &self.wafer_id
    }
    #[doc = "0x30 - Lot ID"]
    #[inline(always)]
    pub const fn lot_id(&self) -> &LotId {
        &self.lot_id
    }
    #[doc = "0x34 - Reserved"]
    #[inline(always)]
    pub const fn reserved0(&self) -> &Reserved0 {
        &self.reserved0
    }
    #[doc = "0x38 - Reserved"]
    #[inline(always)]
    pub const fn reserved1(&self) -> &Reserved1 {
        &self.reserved1
    }
    #[doc = "0x3c - Reserved"]
    #[inline(always)]
    pub const fn reserved2(&self) -> &Reserved2 {
        &self.reserved2
    }
    #[doc = "0x40 - Test Results"]
    #[inline(always)]
    pub const fn test_results(&self) -> &TestResults {
        &self.test_results
    }
    #[doc = "0x44 - Clock System Calibration Tag"]
    #[inline(always)]
    pub const fn cs_cal_tag(&self) -> &CsCalTag {
        &self.cs_cal_tag
    }
    #[doc = "0x48 - Clock System Calibration Length"]
    #[inline(always)]
    pub const fn cs_cal_len(&self) -> &CsCalLen {
        &self.cs_cal_len
    }
    #[doc = "0x4c - DCO IR mode: Frequency calibration for DCORSEL 0 to 4"]
    #[inline(always)]
    pub const fn dcoir_fcal_rsel04(&self) -> &DcoirFcalRsel04 {
        &self.dcoir_fcal_rsel04
    }
    #[doc = "0x50 - DCO IR mode: Frequency calibration for DCORSEL 5"]
    #[inline(always)]
    pub const fn dcoir_fcal_rsel5(&self) -> &DcoirFcalRsel5 {
        &self.dcoir_fcal_rsel5
    }
    #[doc = "0x54 - Reserved"]
    #[inline(always)]
    pub const fn reserved3(&self) -> &Reserved3 {
        &self.reserved3
    }
    #[doc = "0x58 - Reserved"]
    #[inline(always)]
    pub const fn reserved4(&self) -> &Reserved4 {
        &self.reserved4
    }
    #[doc = "0x5c - Reserved"]
    #[inline(always)]
    pub const fn reserved5(&self) -> &Reserved5 {
        &self.reserved5
    }
    #[doc = "0x60 - Reserved"]
    #[inline(always)]
    pub const fn reserved6(&self) -> &Reserved6 {
        &self.reserved6
    }
    #[doc = "0x64 - DCO IR mode: DCO Constant (K) for DCORSEL 0 to 4"]
    #[inline(always)]
    pub const fn dcoir_constk_rsel04(&self) -> &DcoirConstkRsel04 {
        &self.dcoir_constk_rsel04
    }
    #[doc = "0x68 - DCO IR mode: DCO Constant (K) for DCORSEL 5"]
    #[inline(always)]
    pub const fn dcoir_constk_rsel5(&self) -> &DcoirConstkRsel5 {
        &self.dcoir_constk_rsel5
    }
    #[doc = "0x6c - DCO ER mode: Frequency calibration for DCORSEL 0 to 4"]
    #[inline(always)]
    pub const fn dcoer_fcal_rsel04(&self) -> &DcoerFcalRsel04 {
        &self.dcoer_fcal_rsel04
    }
    #[doc = "0x70 - DCO ER mode: Frequency calibration for DCORSEL 5"]
    #[inline(always)]
    pub const fn dcoer_fcal_rsel5(&self) -> &DcoerFcalRsel5 {
        &self.dcoer_fcal_rsel5
    }
    #[doc = "0x74 - Reserved"]
    #[inline(always)]
    pub const fn reserved7(&self) -> &Reserved7 {
        &self.reserved7
    }
    #[doc = "0x78 - Reserved"]
    #[inline(always)]
    pub const fn reserved8(&self) -> &Reserved8 {
        &self.reserved8
    }
    #[doc = "0x7c - Reserved"]
    #[inline(always)]
    pub const fn reserved9(&self) -> &Reserved9 {
        &self.reserved9
    }
    #[doc = "0x80 - Reserved"]
    #[inline(always)]
    pub const fn reserved10(&self) -> &Reserved10 {
        &self.reserved10
    }
    #[doc = "0x84 - DCO ER mode: DCO Constant (K) for DCORSEL 0 to 4"]
    #[inline(always)]
    pub const fn dcoer_constk_rsel04(&self) -> &DcoerConstkRsel04 {
        &self.dcoer_constk_rsel04
    }
    #[doc = "0x88 - DCO ER mode: DCO Constant (K) for DCORSEL 5"]
    #[inline(always)]
    pub const fn dcoer_constk_rsel5(&self) -> &DcoerConstkRsel5 {
        &self.dcoer_constk_rsel5
    }
    #[doc = "0x8c - ADC14 Calibration Tag"]
    #[inline(always)]
    pub const fn adc14_cal_tag(&self) -> &Adc14CalTag {
        &self.adc14_cal_tag
    }
    #[doc = "0x90 - ADC14 Calibration Length"]
    #[inline(always)]
    pub const fn adc14_cal_len(&self) -> &Adc14CalLen {
        &self.adc14_cal_len
    }
    #[doc = "0x94 - ADC Gain Factor"]
    #[inline(always)]
    pub const fn adc_gain_factor(&self) -> &AdcGainFactor {
        &self.adc_gain_factor
    }
    #[doc = "0x98 - ADC Offset"]
    #[inline(always)]
    pub const fn adc_offset(&self) -> &AdcOffset {
        &self.adc_offset
    }
    #[doc = "0x9c - Reserved"]
    #[inline(always)]
    pub const fn reserved11(&self) -> &Reserved11 {
        &self.reserved11
    }
    #[doc = "0xa0 - Reserved"]
    #[inline(always)]
    pub const fn reserved12(&self) -> &Reserved12 {
        &self.reserved12
    }
    #[doc = "0xa4 - Reserved"]
    #[inline(always)]
    pub const fn reserved13(&self) -> &Reserved13 {
        &self.reserved13
    }
    #[doc = "0xa8 - Reserved"]
    #[inline(always)]
    pub const fn reserved14(&self) -> &Reserved14 {
        &self.reserved14
    }
    #[doc = "0xac - Reserved"]
    #[inline(always)]
    pub const fn reserved15(&self) -> &Reserved15 {
        &self.reserved15
    }
    #[doc = "0xb0 - Reserved"]
    #[inline(always)]
    pub const fn reserved16(&self) -> &Reserved16 {
        &self.reserved16
    }
    #[doc = "0xb4 - Reserved"]
    #[inline(always)]
    pub const fn reserved17(&self) -> &Reserved17 {
        &self.reserved17
    }
    #[doc = "0xb8 - Reserved"]
    #[inline(always)]
    pub const fn reserved18(&self) -> &Reserved18 {
        &self.reserved18
    }
    #[doc = "0xbc - Reserved"]
    #[inline(always)]
    pub const fn reserved19(&self) -> &Reserved19 {
        &self.reserved19
    }
    #[doc = "0xc0 - Reserved"]
    #[inline(always)]
    pub const fn reserved20(&self) -> &Reserved20 {
        &self.reserved20
    }
    #[doc = "0xc4 - Reserved"]
    #[inline(always)]
    pub const fn reserved21(&self) -> &Reserved21 {
        &self.reserved21
    }
    #[doc = "0xc8 - Reserved"]
    #[inline(always)]
    pub const fn reserved22(&self) -> &Reserved22 {
        &self.reserved22
    }
    #[doc = "0xcc - Reserved"]
    #[inline(always)]
    pub const fn reserved23(&self) -> &Reserved23 {
        &self.reserved23
    }
    #[doc = "0xd0 - Reserved"]
    #[inline(always)]
    pub const fn reserved24(&self) -> &Reserved24 {
        &self.reserved24
    }
    #[doc = "0xd4 - Reserved"]
    #[inline(always)]
    pub const fn reserved25(&self) -> &Reserved25 {
        &self.reserved25
    }
    #[doc = "0xd8 - Reserved"]
    #[inline(always)]
    pub const fn reserved26(&self) -> &Reserved26 {
        &self.reserved26
    }
    #[doc = "0xdc - ADC14 1.2V Reference Temp. Sensor 30C"]
    #[inline(always)]
    pub const fn adc14_ref1p2v_ts30c(&self) -> &Adc14Ref1p2vTs30c {
        &self.adc14_ref1p2v_ts30c
    }
    #[doc = "0xe0 - ADC14 1.2V Reference Temp. Sensor 85C"]
    #[inline(always)]
    pub const fn adc14_ref1p2v_ts85c(&self) -> &Adc14Ref1p2vTs85c {
        &self.adc14_ref1p2v_ts85c
    }
    #[doc = "0xe4 - ADC14 1.45V Reference Temp. Sensor 30C"]
    #[inline(always)]
    pub const fn adc14_ref1p45v_ts30c(&self) -> &Adc14Ref1p45vTs30c {
        &self.adc14_ref1p45v_ts30c
    }
    #[doc = "0xe8 - ADC14 1.45V Reference Temp. Sensor 85C"]
    #[inline(always)]
    pub const fn adc14_ref1p45v_ts85c(&self) -> &Adc14Ref1p45vTs85c {
        &self.adc14_ref1p45v_ts85c
    }
    #[doc = "0xec - ADC14 2.5V Reference Temp. Sensor 30C"]
    #[inline(always)]
    pub const fn adc14_ref2p5v_ts30c(&self) -> &Adc14Ref2p5vTs30c {
        &self.adc14_ref2p5v_ts30c
    }
    #[doc = "0xf0 - ADC14 2.5V Reference Temp. Sensor 85C"]
    #[inline(always)]
    pub const fn adc14_ref2p5v_ts85c(&self) -> &Adc14Ref2p5vTs85c {
        &self.adc14_ref2p5v_ts85c
    }
    #[doc = "0xf4 - REF Calibration Tag"]
    #[inline(always)]
    pub const fn ref_cal_tag(&self) -> &RefCalTag {
        &self.ref_cal_tag
    }
    #[doc = "0xf8 - REF Calibration Length"]
    #[inline(always)]
    pub const fn ref_cal_len(&self) -> &RefCalLen {
        &self.ref_cal_len
    }
    #[doc = "0xfc - REF 1.2V Reference"]
    #[inline(always)]
    pub const fn ref_1p2v(&self) -> &Ref1p2v {
        &self.ref_1p2v
    }
    #[doc = "0x100 - REF 1.45V Reference"]
    #[inline(always)]
    pub const fn ref_1p45v(&self) -> &Ref1p45v {
        &self.ref_1p45v
    }
    #[doc = "0x104 - REF 2.5V Reference"]
    #[inline(always)]
    pub const fn ref_2p5v(&self) -> &Ref2p5v {
        &self.ref_2p5v
    }
    #[doc = "0x108 - Flash Info Tag"]
    #[inline(always)]
    pub const fn flash_info_tag(&self) -> &FlashInfoTag {
        &self.flash_info_tag
    }
    #[doc = "0x10c - Flash Info Length"]
    #[inline(always)]
    pub const fn flash_info_len(&self) -> &FlashInfoLen {
        &self.flash_info_len
    }
    #[doc = "0x110 - Flash Maximum Programming Pulses"]
    #[inline(always)]
    pub const fn flash_max_prog_pulses(&self) -> &FlashMaxProgPulses {
        &self.flash_max_prog_pulses
    }
    #[doc = "0x114 - Flash Maximum Erase Pulses"]
    #[inline(always)]
    pub const fn flash_max_erase_pulses(&self) -> &FlashMaxErasePulses {
        &self.flash_max_erase_pulses
    }
    #[doc = "0x118 - 128-bit Random Number Tag"]
    #[inline(always)]
    pub const fn random_num_tag(&self) -> &RandomNumTag {
        &self.random_num_tag
    }
    #[doc = "0x11c - 128-bit Random Number Length"]
    #[inline(always)]
    pub const fn random_num_len(&self) -> &RandomNumLen {
        &self.random_num_len
    }
    #[doc = "0x120 - 32-bit Random Number 1"]
    #[inline(always)]
    pub const fn random_num_1(&self) -> &RandomNum1 {
        &self.random_num_1
    }
    #[doc = "0x124 - 32-bit Random Number 2"]
    #[inline(always)]
    pub const fn random_num_2(&self) -> &RandomNum2 {
        &self.random_num_2
    }
    #[doc = "0x128 - 32-bit Random Number 3"]
    #[inline(always)]
    pub const fn random_num_3(&self) -> &RandomNum3 {
        &self.random_num_3
    }
    #[doc = "0x12c - 32-bit Random Number 4"]
    #[inline(always)]
    pub const fn random_num_4(&self) -> &RandomNum4 {
        &self.random_num_4
    }
    #[doc = "0x130 - BSL Configuration Tag"]
    #[inline(always)]
    pub const fn bsl_cfg_tag(&self) -> &BslCfgTag {
        &self.bsl_cfg_tag
    }
    #[doc = "0x134 - BSL Configuration Length"]
    #[inline(always)]
    pub const fn bsl_cfg_len(&self) -> &BslCfgLen {
        &self.bsl_cfg_len
    }
    #[doc = "0x138 - BSL Peripheral Interface Selection"]
    #[inline(always)]
    pub const fn bsl_periphif_sel(&self) -> &BslPeriphifSel {
        &self.bsl_periphif_sel
    }
    #[doc = "0x13c - BSL Port Interface Configuration for UART"]
    #[inline(always)]
    pub const fn bsl_portif_cfg_uart(&self) -> &BslPortifCfgUart {
        &self.bsl_portif_cfg_uart
    }
    #[doc = "0x140 - BSL Port Interface Configuration for SPI"]
    #[inline(always)]
    pub const fn bsl_portif_cfg_spi(&self) -> &BslPortifCfgSpi {
        &self.bsl_portif_cfg_spi
    }
    #[doc = "0x144 - BSL Port Interface Configuration for I2C"]
    #[inline(always)]
    pub const fn bsl_portif_cfg_i2c(&self) -> &BslPortifCfgI2c {
        &self.bsl_portif_cfg_i2c
    }
    #[doc = "0x148 - TLV End Word"]
    #[inline(always)]
    pub const fn tlv_end(&self) -> &TlvEnd {
        &self.tlv_end
    }
}
#[doc = "TLV_CHECKSUM (r) register accessor: TLV Checksum\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_checksum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_checksum`]
module"]
#[doc(alias = "TLV_CHECKSUM")]
pub type TlvChecksum = crate::Reg<tlv_checksum::TlvChecksumSpec>;
#[doc = "TLV Checksum"]
pub mod tlv_checksum;
#[doc = "DEVICE_INFO_TAG (r) register accessor: Device Info Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`device_info_tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_info_tag`]
module"]
#[doc(alias = "DEVICE_INFO_TAG")]
pub type DeviceInfoTag = crate::Reg<device_info_tag::DeviceInfoTagSpec>;
#[doc = "Device Info Tag"]
pub mod device_info_tag;
#[doc = "DEVICE_INFO_LEN (r) register accessor: Device Info Length\n\nYou can [`read`](crate::Reg::read) this register and get [`device_info_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_info_len`]
module"]
#[doc(alias = "DEVICE_INFO_LEN")]
pub type DeviceInfoLen = crate::Reg<device_info_len::DeviceInfoLenSpec>;
#[doc = "Device Info Length"]
pub mod device_info_len;
#[doc = "DEVICE_ID (r) register accessor: Device ID\n\nYou can [`read`](crate::Reg::read) this register and get [`device_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_id`]
module"]
#[doc(alias = "DEVICE_ID")]
pub type DeviceId = crate::Reg<device_id::DeviceIdSpec>;
#[doc = "Device ID"]
pub mod device_id;
#[doc = "HWREV (r) register accessor: HW Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`hwrev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwrev`]
module"]
#[doc(alias = "HWREV")]
pub type Hwrev = crate::Reg<hwrev::HwrevSpec>;
#[doc = "HW Revision"]
pub mod hwrev;
#[doc = "BCREV (r) register accessor: Boot Code Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`bcrev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcrev`]
module"]
#[doc(alias = "BCREV")]
pub type Bcrev = crate::Reg<bcrev::BcrevSpec>;
#[doc = "Boot Code Revision"]
pub mod bcrev;
#[doc = "ROM_DRVLIB_REV (r) register accessor: ROM Driver Library Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_drvlib_rev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_drvlib_rev`]
module"]
#[doc(alias = "ROM_DRVLIB_REV")]
pub type RomDrvlibRev = crate::Reg<rom_drvlib_rev::RomDrvlibRevSpec>;
#[doc = "ROM Driver Library Revision"]
pub mod rom_drvlib_rev;
#[doc = "DIE_REC_TAG (r) register accessor: Die Record Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`die_rec_tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@die_rec_tag`]
module"]
#[doc(alias = "DIE_REC_TAG")]
pub type DieRecTag = crate::Reg<die_rec_tag::DieRecTagSpec>;
#[doc = "Die Record Tag"]
pub mod die_rec_tag;
#[doc = "DIE_REC_LEN (r) register accessor: Die Record Length\n\nYou can [`read`](crate::Reg::read) this register and get [`die_rec_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@die_rec_len`]
module"]
#[doc(alias = "DIE_REC_LEN")]
pub type DieRecLen = crate::Reg<die_rec_len::DieRecLenSpec>;
#[doc = "Die Record Length"]
pub mod die_rec_len;
#[doc = "DIE_XPOS (r) register accessor: Die X-Position\n\nYou can [`read`](crate::Reg::read) this register and get [`die_xpos::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@die_xpos`]
module"]
#[doc(alias = "DIE_XPOS")]
pub type DieXpos = crate::Reg<die_xpos::DieXposSpec>;
#[doc = "Die X-Position"]
pub mod die_xpos;
#[doc = "DIE_YPOS (r) register accessor: Die Y-Position\n\nYou can [`read`](crate::Reg::read) this register and get [`die_ypos::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@die_ypos`]
module"]
#[doc(alias = "DIE_YPOS")]
pub type DieYpos = crate::Reg<die_ypos::DieYposSpec>;
#[doc = "Die Y-Position"]
pub mod die_ypos;
#[doc = "WAFER_ID (r) register accessor: Wafer ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wafer_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wafer_id`]
module"]
#[doc(alias = "WAFER_ID")]
pub type WaferId = crate::Reg<wafer_id::WaferIdSpec>;
#[doc = "Wafer ID"]
pub mod wafer_id;
#[doc = "LOT_ID (r) register accessor: Lot ID\n\nYou can [`read`](crate::Reg::read) this register and get [`lot_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lot_id`]
module"]
#[doc(alias = "LOT_ID")]
pub type LotId = crate::Reg<lot_id::LotIdSpec>;
#[doc = "Lot ID"]
pub mod lot_id;
#[doc = "RESERVED0 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved0`]
module"]
#[doc(alias = "RESERVED0")]
pub type Reserved0 = crate::Reg<reserved0::Reserved0Spec>;
#[doc = "Reserved"]
pub mod reserved0;
#[doc = "RESERVED1 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved1`]
module"]
#[doc(alias = "RESERVED1")]
pub type Reserved1 = crate::Reg<reserved1::Reserved1Spec>;
#[doc = "Reserved"]
pub mod reserved1;
#[doc = "RESERVED2 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved2`]
module"]
#[doc(alias = "RESERVED2")]
pub type Reserved2 = crate::Reg<reserved2::Reserved2Spec>;
#[doc = "Reserved"]
pub mod reserved2;
#[doc = "TEST_RESULTS (r) register accessor: Test Results\n\nYou can [`read`](crate::Reg::read) this register and get [`test_results::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test_results`]
module"]
#[doc(alias = "TEST_RESULTS")]
pub type TestResults = crate::Reg<test_results::TestResultsSpec>;
#[doc = "Test Results"]
pub mod test_results;
#[doc = "CS_CAL_TAG (r) register accessor: Clock System Calibration Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`cs_cal_tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs_cal_tag`]
module"]
#[doc(alias = "CS_CAL_TAG")]
pub type CsCalTag = crate::Reg<cs_cal_tag::CsCalTagSpec>;
#[doc = "Clock System Calibration Tag"]
pub mod cs_cal_tag;
#[doc = "CS_CAL_LEN (r) register accessor: Clock System Calibration Length\n\nYou can [`read`](crate::Reg::read) this register and get [`cs_cal_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs_cal_len`]
module"]
#[doc(alias = "CS_CAL_LEN")]
pub type CsCalLen = crate::Reg<cs_cal_len::CsCalLenSpec>;
#[doc = "Clock System Calibration Length"]
pub mod cs_cal_len;
#[doc = "DCOIR_FCAL_RSEL04 (r) register accessor: DCO IR mode: Frequency calibration for DCORSEL 0 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoir_fcal_rsel04::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcoir_fcal_rsel04`]
module"]
#[doc(alias = "DCOIR_FCAL_RSEL04")]
pub type DcoirFcalRsel04 = crate::Reg<dcoir_fcal_rsel04::DcoirFcalRsel04Spec>;
#[doc = "DCO IR mode: Frequency calibration for DCORSEL 0 to 4"]
pub mod dcoir_fcal_rsel04;
#[doc = "DCOIR_FCAL_RSEL5 (r) register accessor: DCO IR mode: Frequency calibration for DCORSEL 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoir_fcal_rsel5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcoir_fcal_rsel5`]
module"]
#[doc(alias = "DCOIR_FCAL_RSEL5")]
pub type DcoirFcalRsel5 = crate::Reg<dcoir_fcal_rsel5::DcoirFcalRsel5Spec>;
#[doc = "DCO IR mode: Frequency calibration for DCORSEL 5"]
pub mod dcoir_fcal_rsel5;
#[doc = "RESERVED3 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved3`]
module"]
#[doc(alias = "RESERVED3")]
pub type Reserved3 = crate::Reg<reserved3::Reserved3Spec>;
#[doc = "Reserved"]
pub mod reserved3;
#[doc = "RESERVED4 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved4`]
module"]
#[doc(alias = "RESERVED4")]
pub type Reserved4 = crate::Reg<reserved4::Reserved4Spec>;
#[doc = "Reserved"]
pub mod reserved4;
#[doc = "RESERVED5 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved5`]
module"]
#[doc(alias = "RESERVED5")]
pub type Reserved5 = crate::Reg<reserved5::Reserved5Spec>;
#[doc = "Reserved"]
pub mod reserved5;
#[doc = "RESERVED6 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved6`]
module"]
#[doc(alias = "RESERVED6")]
pub type Reserved6 = crate::Reg<reserved6::Reserved6Spec>;
#[doc = "Reserved"]
pub mod reserved6;
#[doc = "DCOIR_CONSTK_RSEL04 (r) register accessor: DCO IR mode: DCO Constant (K) for DCORSEL 0 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoir_constk_rsel04::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcoir_constk_rsel04`]
module"]
#[doc(alias = "DCOIR_CONSTK_RSEL04")]
pub type DcoirConstkRsel04 = crate::Reg<dcoir_constk_rsel04::DcoirConstkRsel04Spec>;
#[doc = "DCO IR mode: DCO Constant (K) for DCORSEL 0 to 4"]
pub mod dcoir_constk_rsel04;
#[doc = "DCOIR_CONSTK_RSEL5 (r) register accessor: DCO IR mode: DCO Constant (K) for DCORSEL 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoir_constk_rsel5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcoir_constk_rsel5`]
module"]
#[doc(alias = "DCOIR_CONSTK_RSEL5")]
pub type DcoirConstkRsel5 = crate::Reg<dcoir_constk_rsel5::DcoirConstkRsel5Spec>;
#[doc = "DCO IR mode: DCO Constant (K) for DCORSEL 5"]
pub mod dcoir_constk_rsel5;
#[doc = "DCOER_FCAL_RSEL04 (r) register accessor: DCO ER mode: Frequency calibration for DCORSEL 0 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoer_fcal_rsel04::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcoer_fcal_rsel04`]
module"]
#[doc(alias = "DCOER_FCAL_RSEL04")]
pub type DcoerFcalRsel04 = crate::Reg<dcoer_fcal_rsel04::DcoerFcalRsel04Spec>;
#[doc = "DCO ER mode: Frequency calibration for DCORSEL 0 to 4"]
pub mod dcoer_fcal_rsel04;
#[doc = "DCOER_FCAL_RSEL5 (r) register accessor: DCO ER mode: Frequency calibration for DCORSEL 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoer_fcal_rsel5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcoer_fcal_rsel5`]
module"]
#[doc(alias = "DCOER_FCAL_RSEL5")]
pub type DcoerFcalRsel5 = crate::Reg<dcoer_fcal_rsel5::DcoerFcalRsel5Spec>;
#[doc = "DCO ER mode: Frequency calibration for DCORSEL 5"]
pub mod dcoer_fcal_rsel5;
#[doc = "RESERVED7 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved7`]
module"]
#[doc(alias = "RESERVED7")]
pub type Reserved7 = crate::Reg<reserved7::Reserved7Spec>;
#[doc = "Reserved"]
pub mod reserved7;
#[doc = "RESERVED8 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved8`]
module"]
#[doc(alias = "RESERVED8")]
pub type Reserved8 = crate::Reg<reserved8::Reserved8Spec>;
#[doc = "Reserved"]
pub mod reserved8;
#[doc = "RESERVED9 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved9`]
module"]
#[doc(alias = "RESERVED9")]
pub type Reserved9 = crate::Reg<reserved9::Reserved9Spec>;
#[doc = "Reserved"]
pub mod reserved9;
#[doc = "RESERVED10 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved10`]
module"]
#[doc(alias = "RESERVED10")]
pub type Reserved10 = crate::Reg<reserved10::Reserved10Spec>;
#[doc = "Reserved"]
pub mod reserved10;
#[doc = "DCOER_CONSTK_RSEL04 (r) register accessor: DCO ER mode: DCO Constant (K) for DCORSEL 0 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoer_constk_rsel04::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcoer_constk_rsel04`]
module"]
#[doc(alias = "DCOER_CONSTK_RSEL04")]
pub type DcoerConstkRsel04 = crate::Reg<dcoer_constk_rsel04::DcoerConstkRsel04Spec>;
#[doc = "DCO ER mode: DCO Constant (K) for DCORSEL 0 to 4"]
pub mod dcoer_constk_rsel04;
#[doc = "DCOER_CONSTK_RSEL5 (r) register accessor: DCO ER mode: DCO Constant (K) for DCORSEL 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dcoer_constk_rsel5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcoer_constk_rsel5`]
module"]
#[doc(alias = "DCOER_CONSTK_RSEL5")]
pub type DcoerConstkRsel5 = crate::Reg<dcoer_constk_rsel5::DcoerConstkRsel5Spec>;
#[doc = "DCO ER mode: DCO Constant (K) for DCORSEL 5"]
pub mod dcoer_constk_rsel5;
#[doc = "ADC14_CAL_TAG (r) register accessor: ADC14 Calibration Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14_cal_tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14_cal_tag`]
module"]
#[doc(alias = "ADC14_CAL_TAG")]
pub type Adc14CalTag = crate::Reg<adc14_cal_tag::Adc14CalTagSpec>;
#[doc = "ADC14 Calibration Tag"]
pub mod adc14_cal_tag;
#[doc = "ADC14_CAL_LEN (r) register accessor: ADC14 Calibration Length\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14_cal_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14_cal_len`]
module"]
#[doc(alias = "ADC14_CAL_LEN")]
pub type Adc14CalLen = crate::Reg<adc14_cal_len::Adc14CalLenSpec>;
#[doc = "ADC14 Calibration Length"]
pub mod adc14_cal_len;
#[doc = "ADC_GAIN_FACTOR (r) register accessor: ADC Gain Factor\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_gain_factor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_gain_factor`]
module"]
#[doc(alias = "ADC_GAIN_FACTOR")]
pub type AdcGainFactor = crate::Reg<adc_gain_factor::AdcGainFactorSpec>;
#[doc = "ADC Gain Factor"]
pub mod adc_gain_factor;
#[doc = "ADC_OFFSET (r) register accessor: ADC Offset\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_offset::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_offset`]
module"]
#[doc(alias = "ADC_OFFSET")]
pub type AdcOffset = crate::Reg<adc_offset::AdcOffsetSpec>;
#[doc = "ADC Offset"]
pub mod adc_offset;
#[doc = "RESERVED11 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved11`]
module"]
#[doc(alias = "RESERVED11")]
pub type Reserved11 = crate::Reg<reserved11::Reserved11Spec>;
#[doc = "Reserved"]
pub mod reserved11;
#[doc = "RESERVED12 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved12`]
module"]
#[doc(alias = "RESERVED12")]
pub type Reserved12 = crate::Reg<reserved12::Reserved12Spec>;
#[doc = "Reserved"]
pub mod reserved12;
#[doc = "RESERVED13 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved13`]
module"]
#[doc(alias = "RESERVED13")]
pub type Reserved13 = crate::Reg<reserved13::Reserved13Spec>;
#[doc = "Reserved"]
pub mod reserved13;
#[doc = "RESERVED14 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved14`]
module"]
#[doc(alias = "RESERVED14")]
pub type Reserved14 = crate::Reg<reserved14::Reserved14Spec>;
#[doc = "Reserved"]
pub mod reserved14;
#[doc = "RESERVED15 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved15`]
module"]
#[doc(alias = "RESERVED15")]
pub type Reserved15 = crate::Reg<reserved15::Reserved15Spec>;
#[doc = "Reserved"]
pub mod reserved15;
#[doc = "RESERVED16 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved16`]
module"]
#[doc(alias = "RESERVED16")]
pub type Reserved16 = crate::Reg<reserved16::Reserved16Spec>;
#[doc = "Reserved"]
pub mod reserved16;
#[doc = "RESERVED17 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved17`]
module"]
#[doc(alias = "RESERVED17")]
pub type Reserved17 = crate::Reg<reserved17::Reserved17Spec>;
#[doc = "Reserved"]
pub mod reserved17;
#[doc = "RESERVED18 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved18`]
module"]
#[doc(alias = "RESERVED18")]
pub type Reserved18 = crate::Reg<reserved18::Reserved18Spec>;
#[doc = "Reserved"]
pub mod reserved18;
#[doc = "RESERVED19 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved19`]
module"]
#[doc(alias = "RESERVED19")]
pub type Reserved19 = crate::Reg<reserved19::Reserved19Spec>;
#[doc = "Reserved"]
pub mod reserved19;
#[doc = "RESERVED20 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved20`]
module"]
#[doc(alias = "RESERVED20")]
pub type Reserved20 = crate::Reg<reserved20::Reserved20Spec>;
#[doc = "Reserved"]
pub mod reserved20;
#[doc = "RESERVED21 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved21`]
module"]
#[doc(alias = "RESERVED21")]
pub type Reserved21 = crate::Reg<reserved21::Reserved21Spec>;
#[doc = "Reserved"]
pub mod reserved21;
#[doc = "RESERVED22 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved22`]
module"]
#[doc(alias = "RESERVED22")]
pub type Reserved22 = crate::Reg<reserved22::Reserved22Spec>;
#[doc = "Reserved"]
pub mod reserved22;
#[doc = "RESERVED23 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved23`]
module"]
#[doc(alias = "RESERVED23")]
pub type Reserved23 = crate::Reg<reserved23::Reserved23Spec>;
#[doc = "Reserved"]
pub mod reserved23;
#[doc = "RESERVED24 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved24`]
module"]
#[doc(alias = "RESERVED24")]
pub type Reserved24 = crate::Reg<reserved24::Reserved24Spec>;
#[doc = "Reserved"]
pub mod reserved24;
#[doc = "RESERVED25 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved25`]
module"]
#[doc(alias = "RESERVED25")]
pub type Reserved25 = crate::Reg<reserved25::Reserved25Spec>;
#[doc = "Reserved"]
pub mod reserved25;
#[doc = "RESERVED26 (r) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved26`]
module"]
#[doc(alias = "RESERVED26")]
pub type Reserved26 = crate::Reg<reserved26::Reserved26Spec>;
#[doc = "Reserved"]
pub mod reserved26;
#[doc = "ADC14_REF1P2V_TS30C (r) register accessor: ADC14 1.2V Reference Temp. Sensor 30C\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14_ref1p2v_ts30c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14_ref1p2v_ts30c`]
module"]
#[doc(alias = "ADC14_REF1P2V_TS30C")]
pub type Adc14Ref1p2vTs30c = crate::Reg<adc14_ref1p2v_ts30c::Adc14Ref1p2vTs30cSpec>;
#[doc = "ADC14 1.2V Reference Temp. Sensor 30C"]
pub mod adc14_ref1p2v_ts30c;
#[doc = "ADC14_REF1P2V_TS85C (r) register accessor: ADC14 1.2V Reference Temp. Sensor 85C\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14_ref1p2v_ts85c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14_ref1p2v_ts85c`]
module"]
#[doc(alias = "ADC14_REF1P2V_TS85C")]
pub type Adc14Ref1p2vTs85c = crate::Reg<adc14_ref1p2v_ts85c::Adc14Ref1p2vTs85cSpec>;
#[doc = "ADC14 1.2V Reference Temp. Sensor 85C"]
pub mod adc14_ref1p2v_ts85c;
#[doc = "ADC14_REF1P45V_TS30C (r) register accessor: ADC14 1.45V Reference Temp. Sensor 30C\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14_ref1p45v_ts30c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14_ref1p45v_ts30c`]
module"]
#[doc(alias = "ADC14_REF1P45V_TS30C")]
pub type Adc14Ref1p45vTs30c = crate::Reg<adc14_ref1p45v_ts30c::Adc14Ref1p45vTs30cSpec>;
#[doc = "ADC14 1.45V Reference Temp. Sensor 30C"]
pub mod adc14_ref1p45v_ts30c;
#[doc = "ADC14_REF1P45V_TS85C (r) register accessor: ADC14 1.45V Reference Temp. Sensor 85C\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14_ref1p45v_ts85c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14_ref1p45v_ts85c`]
module"]
#[doc(alias = "ADC14_REF1P45V_TS85C")]
pub type Adc14Ref1p45vTs85c = crate::Reg<adc14_ref1p45v_ts85c::Adc14Ref1p45vTs85cSpec>;
#[doc = "ADC14 1.45V Reference Temp. Sensor 85C"]
pub mod adc14_ref1p45v_ts85c;
#[doc = "ADC14_REF2P5V_TS30C (r) register accessor: ADC14 2.5V Reference Temp. Sensor 30C\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14_ref2p5v_ts30c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14_ref2p5v_ts30c`]
module"]
#[doc(alias = "ADC14_REF2P5V_TS30C")]
pub type Adc14Ref2p5vTs30c = crate::Reg<adc14_ref2p5v_ts30c::Adc14Ref2p5vTs30cSpec>;
#[doc = "ADC14 2.5V Reference Temp. Sensor 30C"]
pub mod adc14_ref2p5v_ts30c;
#[doc = "ADC14_REF2P5V_TS85C (r) register accessor: ADC14 2.5V Reference Temp. Sensor 85C\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14_ref2p5v_ts85c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14_ref2p5v_ts85c`]
module"]
#[doc(alias = "ADC14_REF2P5V_TS85C")]
pub type Adc14Ref2p5vTs85c = crate::Reg<adc14_ref2p5v_ts85c::Adc14Ref2p5vTs85cSpec>;
#[doc = "ADC14 2.5V Reference Temp. Sensor 85C"]
pub mod adc14_ref2p5v_ts85c;
#[doc = "REF_CAL_TAG (r) register accessor: REF Calibration Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_cal_tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_cal_tag`]
module"]
#[doc(alias = "REF_CAL_TAG")]
pub type RefCalTag = crate::Reg<ref_cal_tag::RefCalTagSpec>;
#[doc = "REF Calibration Tag"]
pub mod ref_cal_tag;
#[doc = "REF_CAL_LEN (r) register accessor: REF Calibration Length\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_cal_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_cal_len`]
module"]
#[doc(alias = "REF_CAL_LEN")]
pub type RefCalLen = crate::Reg<ref_cal_len::RefCalLenSpec>;
#[doc = "REF Calibration Length"]
pub mod ref_cal_len;
#[doc = "REF_1P2V (r) register accessor: REF 1.2V Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_1p2v::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_1p2v`]
module"]
#[doc(alias = "REF_1P2V")]
pub type Ref1p2v = crate::Reg<ref_1p2v::Ref1p2vSpec>;
#[doc = "REF 1.2V Reference"]
pub mod ref_1p2v;
#[doc = "REF_1P45V (r) register accessor: REF 1.45V Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_1p45v::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_1p45v`]
module"]
#[doc(alias = "REF_1P45V")]
pub type Ref1p45v = crate::Reg<ref_1p45v::Ref1p45vSpec>;
#[doc = "REF 1.45V Reference"]
pub mod ref_1p45v;
#[doc = "REF_2P5V (r) register accessor: REF 2.5V Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_2p5v::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_2p5v`]
module"]
#[doc(alias = "REF_2P5V")]
pub type Ref2p5v = crate::Reg<ref_2p5v::Ref2p5vSpec>;
#[doc = "REF 2.5V Reference"]
pub mod ref_2p5v;
#[doc = "FLASH_INFO_TAG (r) register accessor: Flash Info Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_info_tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_info_tag`]
module"]
#[doc(alias = "FLASH_INFO_TAG")]
pub type FlashInfoTag = crate::Reg<flash_info_tag::FlashInfoTagSpec>;
#[doc = "Flash Info Tag"]
pub mod flash_info_tag;
#[doc = "FLASH_INFO_LEN (r) register accessor: Flash Info Length\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_info_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_info_len`]
module"]
#[doc(alias = "FLASH_INFO_LEN")]
pub type FlashInfoLen = crate::Reg<flash_info_len::FlashInfoLenSpec>;
#[doc = "Flash Info Length"]
pub mod flash_info_len;
#[doc = "FLASH_MAX_PROG_PULSES (r) register accessor: Flash Maximum Programming Pulses\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_max_prog_pulses::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_max_prog_pulses`]
module"]
#[doc(alias = "FLASH_MAX_PROG_PULSES")]
pub type FlashMaxProgPulses = crate::Reg<flash_max_prog_pulses::FlashMaxProgPulsesSpec>;
#[doc = "Flash Maximum Programming Pulses"]
pub mod flash_max_prog_pulses;
#[doc = "FLASH_MAX_ERASE_PULSES (r) register accessor: Flash Maximum Erase Pulses\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_max_erase_pulses::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_max_erase_pulses`]
module"]
#[doc(alias = "FLASH_MAX_ERASE_PULSES")]
pub type FlashMaxErasePulses = crate::Reg<flash_max_erase_pulses::FlashMaxErasePulsesSpec>;
#[doc = "Flash Maximum Erase Pulses"]
pub mod flash_max_erase_pulses;
#[doc = "RANDOM_NUM_TAG (r) register accessor: 128-bit Random Number Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`random_num_tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@random_num_tag`]
module"]
#[doc(alias = "RANDOM_NUM_TAG")]
pub type RandomNumTag = crate::Reg<random_num_tag::RandomNumTagSpec>;
#[doc = "128-bit Random Number Tag"]
pub mod random_num_tag;
#[doc = "RANDOM_NUM_LEN (r) register accessor: 128-bit Random Number Length\n\nYou can [`read`](crate::Reg::read) this register and get [`random_num_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@random_num_len`]
module"]
#[doc(alias = "RANDOM_NUM_LEN")]
pub type RandomNumLen = crate::Reg<random_num_len::RandomNumLenSpec>;
#[doc = "128-bit Random Number Length"]
pub mod random_num_len;
#[doc = "RANDOM_NUM_1 (r) register accessor: 32-bit Random Number 1\n\nYou can [`read`](crate::Reg::read) this register and get [`random_num_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@random_num_1`]
module"]
#[doc(alias = "RANDOM_NUM_1")]
pub type RandomNum1 = crate::Reg<random_num_1::RandomNum1Spec>;
#[doc = "32-bit Random Number 1"]
pub mod random_num_1;
#[doc = "RANDOM_NUM_2 (r) register accessor: 32-bit Random Number 2\n\nYou can [`read`](crate::Reg::read) this register and get [`random_num_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@random_num_2`]
module"]
#[doc(alias = "RANDOM_NUM_2")]
pub type RandomNum2 = crate::Reg<random_num_2::RandomNum2Spec>;
#[doc = "32-bit Random Number 2"]
pub mod random_num_2;
#[doc = "RANDOM_NUM_3 (r) register accessor: 32-bit Random Number 3\n\nYou can [`read`](crate::Reg::read) this register and get [`random_num_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@random_num_3`]
module"]
#[doc(alias = "RANDOM_NUM_3")]
pub type RandomNum3 = crate::Reg<random_num_3::RandomNum3Spec>;
#[doc = "32-bit Random Number 3"]
pub mod random_num_3;
#[doc = "RANDOM_NUM_4 (r) register accessor: 32-bit Random Number 4\n\nYou can [`read`](crate::Reg::read) this register and get [`random_num_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@random_num_4`]
module"]
#[doc(alias = "RANDOM_NUM_4")]
pub type RandomNum4 = crate::Reg<random_num_4::RandomNum4Spec>;
#[doc = "32-bit Random Number 4"]
pub mod random_num_4;
#[doc = "BSL_CFG_TAG (r) register accessor: BSL Configuration Tag\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_cfg_tag::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsl_cfg_tag`]
module"]
#[doc(alias = "BSL_CFG_TAG")]
pub type BslCfgTag = crate::Reg<bsl_cfg_tag::BslCfgTagSpec>;
#[doc = "BSL Configuration Tag"]
pub mod bsl_cfg_tag;
#[doc = "BSL_CFG_LEN (r) register accessor: BSL Configuration Length\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_cfg_len::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsl_cfg_len`]
module"]
#[doc(alias = "BSL_CFG_LEN")]
pub type BslCfgLen = crate::Reg<bsl_cfg_len::BslCfgLenSpec>;
#[doc = "BSL Configuration Length"]
pub mod bsl_cfg_len;
#[doc = "BSL_PERIPHIF_SEL (r) register accessor: BSL Peripheral Interface Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_periphif_sel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsl_periphif_sel`]
module"]
#[doc(alias = "BSL_PERIPHIF_SEL")]
pub type BslPeriphifSel = crate::Reg<bsl_periphif_sel::BslPeriphifSelSpec>;
#[doc = "BSL Peripheral Interface Selection"]
pub mod bsl_periphif_sel;
#[doc = "BSL_PORTIF_CFG_UART (r) register accessor: BSL Port Interface Configuration for UART\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_portif_cfg_uart::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsl_portif_cfg_uart`]
module"]
#[doc(alias = "BSL_PORTIF_CFG_UART")]
pub type BslPortifCfgUart = crate::Reg<bsl_portif_cfg_uart::BslPortifCfgUartSpec>;
#[doc = "BSL Port Interface Configuration for UART"]
pub mod bsl_portif_cfg_uart;
#[doc = "BSL_PORTIF_CFG_SPI (r) register accessor: BSL Port Interface Configuration for SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_portif_cfg_spi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsl_portif_cfg_spi`]
module"]
#[doc(alias = "BSL_PORTIF_CFG_SPI")]
pub type BslPortifCfgSpi = crate::Reg<bsl_portif_cfg_spi::BslPortifCfgSpiSpec>;
#[doc = "BSL Port Interface Configuration for SPI"]
pub mod bsl_portif_cfg_spi;
#[doc = "BSL_PORTIF_CFG_I2C (r) register accessor: BSL Port Interface Configuration for I2C\n\nYou can [`read`](crate::Reg::read) this register and get [`bsl_portif_cfg_i2c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsl_portif_cfg_i2c`]
module"]
#[doc(alias = "BSL_PORTIF_CFG_I2C")]
pub type BslPortifCfgI2c = crate::Reg<bsl_portif_cfg_i2c::BslPortifCfgI2cSpec>;
#[doc = "BSL Port Interface Configuration for I2C"]
pub mod bsl_portif_cfg_i2c;
#[doc = "TLV_END (r) register accessor: TLV End Word\n\nYou can [`read`](crate::Reg::read) this register and get [`tlv_end::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tlv_end`]
module"]
#[doc(alias = "TLV_END")]
pub type TlvEnd = crate::Reg<tlv_end::TlvEndSpec>;
#[doc = "TLV End Word"]
pub mod tlv_end;
