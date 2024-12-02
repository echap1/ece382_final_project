#![no_std]
#![no_main]
#![allow(internal_features)]
#![allow(static_mut_refs)]
#![feature(core_intrinsics)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate itoa;
extern crate msp432P401r_api;
extern crate panic_semihosting;
extern crate ryu;

use cortex_m_rt::entry;
#[allow(unused_imports)]
use panic_semihosting as _;

use bump::bump_init;
use clock::clock_init48mhz;
use lcd::lcd_init;
use motor::motor_init;
use odometry::{odometry_init, odometry_update};
use pid::PIDFController;
use rgb_led::RGBLed;
use sys_init::system_init;
use tachometer::{get_distances_and_clear, get_speeds, tachometer_init};
use timer_a1::timera1_init;
use uart0::uart0_init;
use units::Time;

use crate::level1::{level1_main, level1_periodic};
use crate::level2::{level2_main, level2_periodic};
use crate::odometry::RobotDirection;

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
mod ramsete;
mod units;
mod math;
mod trajectory;
mod trajectories;
mod drive_to_point;
mod bump;
mod buffer_stack;
mod uart0;
mod byte_mut_writer;
mod level1;
mod level2;
mod adc14;
mod moving_average;

#[entry]
unsafe fn main() -> ! {
    system_init();
    clock_init48mhz();

    RGBLed::init();
    timera1_init(task, LOOP_TIME);

    lcd_init();
    motor_init();
    tachometer_init();
    odometry_init();
    bump_init();
    uart0_init();

    // level1_main();
    level2_main();
}

static mut ELAPSED: Time = Time::from_s(0f32);
static mut LOOP_TIME: Time = Time::from_s(0.005);

static mut LEFT: PIDFController = PIDFController::new();
static mut RIGHT: PIDFController = PIDFController::new();

unsafe fn task() {
    let (l, r) = get_distances_and_clear();
    let (vl, vr) = get_speeds();
    odometry_update(l, r, vl, vr);
    
    // level1_periodic();
    level2_periodic();
    
    *ELAPSED.as_s_mut() += LOOP_TIME.as_s();
}
