use crate::peripherals::peripherals;

pub struct RGBLed;

impl RGBLed {
    pub const OFF: u8 = 0b000;
    pub const RED: u8 = 0b001;
    pub const GREEN: u8 = 0b010;
    pub const BLUE: u8 = 0b100;

    pub fn init() {
        peripherals().dio.padir().modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits() | 0b111) });
    }

    pub fn set(color: u8) {
        peripherals().dio.paout().modify(|r, w| unsafe { w.p2out().bits((r.p2out().bits() & !0b111) | color) });
    }
}
