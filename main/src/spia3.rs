use peripherals::peripherals;

pub fn spia3_init() {
    let p = peripherals();

    p.eusci_a3.ucax_ctlw0().write(|w| unsafe { w.bits(0xAD83) });
    p.eusci_a3.ucax_brw().write(|w| unsafe { w.bits(3) });
    p.eusci_a3.ucax_mctlw().write(|w| unsafe { w.bits(0) });
    p.dio.pesel0().modify(|r, w| unsafe { w.p9sel0().bits(r.p9sel0().bits() | 0b010110000) });
    p.dio.pesel1().modify(|r, w| unsafe { w.p9sel1().bits(r.p9sel1().bits() & !0b010110000) });
    p.eusci_a3.ucax_ctlw0().modify(|_, w| { w.ucswrst().clear_bit() });
    p.eusci_a3.ucax_ie().modify(|r, w| unsafe { w.bits(r.bits() & !0b1111) });
}

pub fn spia3_wait_for_tx() {
    while peripherals().eusci_a3.ucax_ifg().read().uctxifg().bit_is_clear() { }
}

pub fn spia3_wait_for_tx_rx_ready() {
    while peripherals().eusci_a3.ucax_statw().read().ucbusy().bit_is_set() { }
}

pub fn spia3_write_tx_buffer(data: u8) {
    peripherals().eusci_a3.ucax_txbuf().write(|w| unsafe { w.bits(data as u16) });
}

pub fn spia3_out_char(data: u8) {
    spia3_wait_for_tx();
    spia3_write_tx_buffer(data);
}

fn spia3_out_string(string: &str) {
    for c in string.bytes() {
        spia3_out_char(c);
    }
}