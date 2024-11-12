#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(core_intrinsics)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate itoa;
extern crate msp432P401r_api;
extern crate panic_semihosting;

use core::intrinsics;

use cortex_m_rt::entry;
#[allow(unused_imports)]
use panic_semihosting as _;

use clock::clock_init48mhz;
use lcd::{lcd_init, lcd_out_number, lcd_set_cursor};
use motor::{motor_drive, motor_init};
use odometry::{odometry_get_state, odometry_init, odometry_update};
use pid::pid_compute_duty;
use rgb_led::RGBLed;
use sys_init::system_init;
use tachometer::{get_distances_and_clear, tachometer_init};
use timer_a1::timera1_init;
use units::{AngularVelocity, Time, Velocity};

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


#[entry]
unsafe fn main() -> ! {
    system_init();
    clock_init48mhz();

    RGBLed::init();
    timera1_init(task, Time::from_ms(20.0));

    lcd_init();
    motor_init();
    tachometer_init();
    odometry_init();

    // loop { wait_for_interrupt() }

    unsafe { intrinsics::sinf32(1.0); }

    loop {
        let state = odometry_get_state();
        lcd_set_cursor(0, 0);
        lcd_out_number(state.pose.x.as_m() as i32, 5);
        lcd_set_cursor(0, 1);
        lcd_out_number(state.pose.y.as_m() as i32, 5);
        lcd_set_cursor(0, 2);
        lcd_out_number(state.pose.theta.as_deg() as i32, 5);
        lcd_set_cursor(0, 3);
        lcd_out_number(state.l_vel.as_mm_per_sec() as i32, 5);
        lcd_set_cursor(0, 4);
        lcd_out_number(state.r_vel.as_mm_per_sec() as i32, 5);
    }
}

unsafe fn task() {
    let (l, r) = get_distances_and_clear();
    odometry_update(l, r, Time::from_ms(20.0));

    let setpoint = Velocity::from_mm_per_sec(300.0);

    let state = odometry_get_state();
    
    let a = ramsete::compute(&state.pose, &state.pose, setpoint, AngularVelocity::from_rad_per_s(0.0));
    
    motor_drive(pid_compute_duty(setpoint, state.l_vel), pid_compute_duty(setpoint, state.r_vel));
}