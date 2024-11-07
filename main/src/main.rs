#![no_std]
#![no_main]

extern crate msp432P401r_api;
extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::entry;

use msp432P401r_api::Dio;

#[allow(unused_imports)]
use panic_halt as _;


#[entry]
unsafe fn main() -> ! {
    let p = msp432P401r_api::Peripherals::steal();
    
    let dio = p.dio;
    
    init_led(&dio);
    
    set_led(&dio, GREEN | BLUE);
    
    loop {}
}

fn init_led(dio: &Dio) {
    dio.padir().modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits() | 0b111) });
}

const RED: u8 = 0b001;
const GREEN: u8 = 0b010;
const BLUE: u8 = 0b100;

fn set_led(dio: &Dio, color: u8) {
    dio.paout().modify(|r, w| unsafe { w.p2out().bits((r.p2out().bits() & !0b111) | color) });
}
