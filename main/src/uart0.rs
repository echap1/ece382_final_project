use crate::peripherals::peripherals;
use crate::spia3::spia3_out_char;

pub fn uart0_init() {
    let p = peripherals();

    p.eusci_a0.ucax_ctlw0().write(|w| unsafe { w.bits(0b0000000011000001) });
    p.eusci_a0.ucax_brw().write(|w| unsafe { w.bits(104) });
    p.eusci_a0.ucax_mctlw().write(|w| unsafe { w.bits(0) });
    p.dio.pasel0().modify(|r, w| unsafe { w.p1sel0().bits(r.p1sel0().bits() | 0b00001100) });
    p.dio.pasel1().modify(|r, w| unsafe { w.p1sel1().bits(r.p1sel1().bits() & !0b00001100) });
    p.eusci_a0.ucax_ctlw0().modify(|r, w| unsafe { w.bits(r.bits() & !1) });
    p.eusci_a0.ucax_ie().modify(|r, w| unsafe { w.bits(r.bits() & !0b1111) });
}

pub fn uart0_out_char(data: u8) {
    let p = peripherals();
    while p.eusci_a0.ucax_ifg().read().uctxifg().bit_is_clear() { }
    p.eusci_a0.ucax_txbuf().write(|w| unsafe { w.bits(data as u16) });
}

pub fn uart0_out_string(string: &str) {
    for c in string.bytes() {
        if c == 0 { return }
        uart0_out_char(c);
    }
}
