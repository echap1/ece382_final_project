use crate::peripherals::peripherals;

pub fn bump_init() {
    let p = peripherals();
    p.dio.pbsel0().modify(|r, w| unsafe { w.p4sel0().bits(r.p4sel0().bits() & !0b11101101)});
    p.dio.pbsel1().modify(|r, w| unsafe { w.p4sel1().bits(r.p4sel1().bits() & !0b11101101)});
    p.dio.pbdir().modify(|r, w| unsafe { w.p4dir().bits(r.p4dir().bits() & !0b11101101)});
    p.dio.pbren().modify(|r, w| unsafe { w.p4ren().bits(r.p4ren().bits() | 0b11101101)});
    p.dio.pbout().modify(|r, w| unsafe { w.p4out().bits(r.p4out().bits() | 0b11101101)});
}

pub fn bump_read() -> u8 {
    let v = !peripherals().dio.pbin().read().p4in().bits();
    (v & 1) + ((v & 0b00001100) >> 1) + ((v & 0b11100000) >> 2)
}