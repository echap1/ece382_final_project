use msp432P401r_api::Dio;
use peripherals::peripherals;

pub struct RGBLed<'l> {
    dio: &'l Dio
}

impl<'l> RGBLed<'l> {
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
