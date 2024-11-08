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
use uom::si::angle::degree;
use uom::si::f32::{Time, Velocity};
use uom::si::length::millimeter;
use uom::si::time::{millisecond, second};
use uom::si::velocity::centimeter_per_second;

use clock::{clock_init48mhz, wait_for_interrupt};
use lcd::{lcd_init, lcd_out_number, lcd_out_string, lcd_set_cursor};
use motor::{motor_drive, motor_init};
use odometry::odometry_update;
use rgb_led::RGBLed;
use sys_init::system_init;
use tachometer::{get_distances_and_clear, tachometer_init};
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
mod timer_a3_input_capture;
mod odometry;
mod pid;


#[entry]
unsafe fn main() -> ! {
    system_init();
    clock_init48mhz();

    RGBLed::init();

    timera1_init(task, Time::new::<millisecond>(20.0));

    lcd_init();
    // lcd_set_cursor(0, 0); lcd_out_string("Hello");
    // lcd_set_cursor(0, 1); lcd_out_string("from");
    // lcd_set_cursor(0, 2); lcd_out_string("Rust!!");
    // lcd_set_cursor(0, 3); lcd_out_string(":)");

    motor_init();

    // loop { wait_for_interrupt() }


    tachometer_init();

    loop {
        if let Some(state) = unsafe { &odometry::STATE } {
            lcd_set_cursor(0, 0);
            lcd_out_number(state.x.get::<millimeter>() as i32, 5);
            lcd_set_cursor(0, 1);
            lcd_out_number(state.y.get::<millimeter>() as i32, 5);
            lcd_set_cursor(0, 2);
            lcd_out_number(state.theta.get::<degree>() as i32, 5);
            lcd_set_cursor(0, 3);
            lcd_out_number(state.l_vel.get::<centimeter_per_second>() as i32, 5);
            lcd_set_cursor(0, 4);
            lcd_out_number(state.r_vel.get::<centimeter_per_second>() as i32, 5);
        }
    }
}

unsafe fn task() {
    let (l, r) = get_distances_and_clear();
    odometry_update(l, r, Time::new::<millisecond>(20.0));

    let kP = 30f32;
    let setpoint = Velocity::new::<centimeter_per_second>(30.0);
    let current = odometry::STATE.as_ref().unwrap_unchecked().l_vel;
    let error = setpoint - current;
    let duty = (error.get::<centimeter_per_second>() as f32 * kP) as i16;
    motor_drive(duty, 0);
}