#![no_std]
#![no_main]

extern crate msp432P401r_api;
extern crate cortex_m_rt;
extern crate panic_halt;

use core::arch::asm;
use cortex_m_rt::entry;

use msp432P401r_api::Dio;

#[allow(unused_imports)]
use panic_halt as _;


#[entry]
unsafe fn main() -> ! {
    let p = msp432P401r_api::Peripherals::steal();

    p.wdt_a.wdtctl().write( |w| { w.bits(0x5A80) });
    
    let dio = p.dio;
    
    init_led(&dio);
    
    loop {
        set_led(&dio, RED);

        delay(1000000);

        set_led(&dio, BLUE);

        delay(1000000);
    }
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


extern "C" fn delay(count: u64){
    unsafe {
        asm!( "2:  subs    r0, #1",
            "    bne    2b");
    }
}