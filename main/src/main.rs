#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate msp432P401r_api;
extern crate panic_semihosting;
extern crate uom;
extern crate itoa;

use cortex_m_rt::entry;
#[allow(unused_imports)]
use panic_semihosting as _;
use uom::si::f32::Time;
use uom::si::time::second;

use clock::{clock_init48mhz, wait_for_interrupt};
use lcd::{lcd_init, lcd_out_string, lcd_set_cursor};
use motor::{motor_drive, motor_init};
use rgb_led::RGBLed;
use sys_init::system_init;
use timer_a1::timera1_init;

mod clock;
mod sys_init;
mod rgb_led;
mod timer_a1;
mod peripherals;
mod motor;
mod spia3;
mod lcd;
mod ascii;
mod tachometer;


#[entry]
unsafe fn main() -> ! {
    system_init();
    clock_init48mhz();

    RGBLed::init();

    timera1_init(task, Time::new::<second>(1.0));

    lcd_init();
    lcd_set_cursor(0, 0); lcd_out_string("Hello");
    lcd_set_cursor(0, 1); lcd_out_string("from");
    lcd_set_cursor(0, 2); lcd_out_string("Rust!!");
    lcd_set_cursor(0, 3); lcd_out_string(":)");

    // motor_init();
    // motor_drive(-1000, -1000);

    loop { wait_for_interrupt() }
}

static mut TICKER: bool = false;

unsafe fn task() {
    RGBLed::set(match TICKER {
        true => { RGBLed::RED }
        false => { RGBLed::BLUE }
    });
    TICKER = !TICKER;
}