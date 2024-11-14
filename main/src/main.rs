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
use motor::{motor_brake, motor_drive, motor_init};
use odometry::{odometry_get_state, odometry_init, odometry_update, Pose, to_wheel_speeds};
use pid::PIController;
use rgb_led::RGBLed;
use sys_init::system_init;
use tachometer::{get_distances_and_clear, get_speeds, tachometer_init};
use timer_a1::timera1_init;
use trajectories::TRAJECTORY;
use units::{Angle, AngularVelocity, Time, Velocity};

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

    // loop { wait_for_interrupt() }

    unsafe { intrinsics::sinf32(1.0); }

    loop {
        let state = odometry_get_state();
        lcd_set_cursor(0, 0);
        lcd_out_number(state.pose.x.as_mm() as i32, 5);
        lcd_set_cursor(0, 1);
        lcd_out_number(state.pose.y.as_mm() as i32, 5);
        lcd_set_cursor(0, 2);
        lcd_out_number(state.pose.theta.as_deg() as i32, 5);
        lcd_set_cursor(0, 3);
        lcd_out_number(state.l_vel.as_mm_per_sec() as i32, 5);
        lcd_set_cursor(0, 4);
        lcd_out_number(state.r_vel.as_mm_per_sec() as i32, 5);
    }
}

static mut ELAPSED: Time = Time::from_s(0f32);
static mut LOOP_TIME: Time = Time::from_s(0.005);

static mut LEFT: PIController = PIController::new();
static mut RIGHT: PIController = PIController::new();

unsafe fn task() {
    let (l, r) = get_distances_and_clear();
    let (vl, vr) = get_speeds();
    odometry_update(l, r, vl, vr, LOOP_TIME);
    
    if ELAPSED.as_s() > TRAJECTORY.total_time {
        motor_brake();
    } else {
        let p = unsafe { TRAJECTORY.linear_interp(ELAPSED) };

        let state = odometry_get_state();

        let trajectory_out = ramsete::compute(&state.pose, &Pose {
            x: p.0,
            y: p.1,
            theta: p.2
        }, p.3, p.4);

        let (l_speed, r_speed) = to_wheel_speeds(trajectory_out.0, trajectory_out.1);

        // let l_speed = Velocity::from_m_per_sec(0.5);
        // let r_speed = Velocity::from_m_per_sec(0.5);

        let l = unsafe { LEFT.compute(l_speed.as_m_per_sec(), state.l_vel.as_m_per_sec()) };
        let r = unsafe { RIGHT.compute(r_speed.as_m_per_sec(), state.l_vel.as_m_per_sec()) };

        motor_drive(l as i16, r as i16);
    }
    
    *ELAPSED.as_s_mut() += LOOP_TIME.as_s();
}