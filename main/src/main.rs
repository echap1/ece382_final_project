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

use core::fmt::Write;
use cortex_m_rt::entry;
#[allow(unused_imports)]
use panic_semihosting as _;

use clock::{clock_init48mhz, delay_1ms};
use lcd::{lcd_clear, lcd_init, lcd_out_float, lcd_out_number, lcd_set_cursor};
use motor::{motor_brake, motor_drive, motor_init};
use odometry::{odometry_get_state, odometry_init, odometry_update, Pose, to_wheel_speeds};
use pid::PIDFController;
use rgb_led::RGBLed;
use sys_init::system_init;
use tachometer::{get_distances_and_clear, get_speeds, tachometer_init};
use timer_a1::timera1_init;
use trajectories::TRAJECTORY;
use units::Time;
use buffer_stack::BufferStack;

use bump::{bump_init, bump_read};
use byte_mut_writer::ByteMutWriter;
use drive_to_point::drive_to_point;
use lcd::lcd_out_string;
use math::{abs, within_range};
use odometry::RobotState;
use uart0::uart0_init;
use units::{Angle, Length, Velocity};
use crate::uart0::{uart0_out_char, uart0_out_string};

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

    POSITIONS.push((Length::from_m(0.0), Length::from_m(0.0)));

    // loop { wait_for_interrupt() }

    let mut l_sum = 0f32;
    let mut r_sum = 0f32;
    let mut l_amt = 0u32;
    let mut r_amt = 0u32;

    let mut transmitted = false;

    loop {
        let state = odometry_get_state();
        STATE_BUF.push((state.pose.x, state.pose.y));

        l_sum += state.l_vel.as_m_per_sec();
        r_sum += state.r_vel.as_m_per_sec();
        l_amt += 1;
        r_amt += 1;

        let mut buf = [0; 12];
        lcd_clear();
        match &STATE {
            Level1State::Navigating{ heading, reference_pos, robot_direction } => {
                lcd_set_cursor(0, 0);
                lcd_out_string("Navigating");

                lcd_set_cursor(0, 1);
                write!(ByteMutWriter::new(&mut buf), "{:?}", heading).unwrap();
                lcd_out_string(core::str::from_utf8_unchecked(&buf));
                buf.fill(0);

                lcd_set_cursor(0, 2);
                write!(ByteMutWriter::new(&mut buf), "{:?}", robot_direction).unwrap();
                lcd_out_string(core::str::from_utf8_unchecked(&buf));
                buf.fill(0);
            }
            Level1State::Stopped(t) => {
                lcd_set_cursor(0, 0);
                lcd_out_string("Stopped");

                lcd_set_cursor(0, 2);
                write!(ByteMutWriter::new(&mut buf), "At {:.2}", t.as_s()).unwrap();
                lcd_out_string(core::str::from_utf8_unchecked(&buf));
                buf.fill(0);
            }
            Level1State::Backtracking => {
                lcd_set_cursor(0, 0);
                lcd_out_string("Backtracking");
            }
            Level1State::Done(t) => {
                lcd_set_cursor(0, 0);
                lcd_out_string("Done");

                if !transmitted && ELAPSED.as_s() - t.as_s() > 60.0 {
                    transmitted = true;
                    let mut buf2 = [0; 100];
                    while let Some(s) = STATE_BUF.pop() {
                        writeln!(ByteMutWriter::new(&mut buf2), "{},{}", s.0.as_m(), s.1.as_m()).unwrap();
                        uart0_out_string(core::str::from_utf8_unchecked(&buf2));
                        delay_1ms(1);
                    }
                }
            }
        }
        lcd_set_cursor(0, 5);
        writeln!(ByteMutWriter::new(&mut buf), "{:.2}", ELAPSED.as_s()).unwrap();
        lcd_out_string(core::str::from_utf8_unchecked(&buf));
        uart0_out_string(core::str::from_utf8_unchecked(&buf));
        buf.fill(0);
        delay_1ms(100);
    }
}

static mut ELAPSED: Time = Time::from_s(0f32);
static mut LOOP_TIME: Time = Time::from_s(0.005);

static mut LEFT: PIDFController = PIDFController::new();
static mut RIGHT: PIDFController = PIDFController::new();

#[derive(Debug)]
enum GlobalDirection {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY
}

#[derive(Debug, Copy, Clone)]
enum RobotDirection {
    Forward,
    Backward
}

enum Level1State {
    Navigating {
        heading: GlobalDirection,
        robot_direction: RobotDirection,
        reference_pos: Length  // If Forward it's the x/y to maintain, if Backward it's the stop x/y
    },
    Stopped(Time),
    Backtracking,
    Done(Time),
}

static mut STATE: Level1State = Level1State::Navigating {
    heading: GlobalDirection::PositiveX,
    robot_direction: RobotDirection::Forward,
    reference_pos: Length::from_m(0.0)
};

static mut POSITIONS: BufferStack<(Length, Length), 100> = BufferStack::new();
static mut STATE_BUF: BufferStack<(Length, Length), 2000> = BufferStack::new();

unsafe fn task() {
    let (l, r) = get_distances_and_clear();
    let (vl, vr) = get_speeds();
    odometry_update(l, r, vl, vr);
    
    let state = odometry_get_state();


    let error_margin_m = 0.02;
    let backup_dist_m = 0.11;
    let pos_lookahead_m = 1.0;

    let goal_x_m = 0.0;
    let goal_y_m = 0.2;
    
    let goal_x_max_m = 0.05;
    let goal_y_min_m = 0.1;
    let goal_y_max_m = 0.5;

    let stop_time_s = 5.0;
    
    // State transitions
    match &STATE {
        Level1State::Navigating { heading, reference_pos, robot_direction } => {
            match robot_direction {
                RobotDirection::Forward => {
                    if bump_read() != 0 {
                        let (heading, reference_pos) = match heading {
                            GlobalDirection::PositiveX => { (GlobalDirection::NegativeX, state.pose.x.as_m() - backup_dist_m) }
                            GlobalDirection::NegativeX => { (GlobalDirection::PositiveX, state.pose.x.as_m() + backup_dist_m)}
                            GlobalDirection::PositiveY => { (GlobalDirection::NegativeY, state.pose.y.as_m() - backup_dist_m)}
                            GlobalDirection::NegativeY => { (GlobalDirection::PositiveY, state.pose.y.as_m() + backup_dist_m)}
                        };
                        STATE = Level1State::Navigating {
                            heading,
                            robot_direction: RobotDirection::Backward,
                            reference_pos: Length::from_m(reference_pos),
                        }
                    } else if within_range(state.pose.y.as_m(), goal_y_min_m, goal_y_max_m) && state.pose.x.as_m() < goal_x_max_m {
                        STATE = Level1State::Stopped(ELAPSED);
                    }
                }
                RobotDirection::Backward => {
                    if match heading {
                        GlobalDirection::PositiveX => { state.pose.x.as_m() > reference_pos.as_m() - error_margin_m }
                        GlobalDirection::NegativeX => { state.pose.x.as_m() < reference_pos.as_m() + error_margin_m }
                        GlobalDirection::PositiveY => { state.pose.y.as_m() > reference_pos.as_m() - error_margin_m }
                        GlobalDirection::NegativeY => { state.pose.y.as_m() < reference_pos.as_m() + error_margin_m }
                    } {
                        let (heading, reference_pos) = match heading {
                            GlobalDirection::PositiveX | GlobalDirection::NegativeX => {
                                (
                                    if goal_y_m < state.pose.y.as_m() {
                                        GlobalDirection::NegativeY
                                    } else {
                                        GlobalDirection::PositiveY
                                    },
                                    state.pose.x.as_m()
                                )
                            }
                            GlobalDirection::PositiveY | GlobalDirection::NegativeY => {
                                (
                                    if goal_x_m < state.pose.x.as_m() {
                                        GlobalDirection::NegativeX
                                    } else {
                                        GlobalDirection::PositiveX
                                    },
                                    state.pose.y.as_m()
                                )
                            }
                        };
                        POSITIONS.push((state.pose.x, state.pose.y));
                        STATE = Level1State::Navigating {
                            heading,
                            robot_direction: RobotDirection::Forward,
                            reference_pos: Length::from_m(reference_pos),
                        }
                    }
                }
            }
        }
        Level1State::Stopped(t_start) => {
            if ELAPSED.as_s() - t_start.as_s() > stop_time_s {
                RGBLed::set(RGBLed::OFF);
                STATE = Level1State::Backtracking;
            }
        }
        Level1State::Backtracking => {
            if let Some(p) = POSITIONS.peek() {
                if abs(p.0.as_m() - state.pose.x.as_m()) < error_margin_m && abs(p.1.as_m() - state.pose.y.as_m()) < error_margin_m {
                    POSITIONS.pop();
                }
            } else {
                STATE = Level1State::Done(ELAPSED);
            }
        }
        Level1State::Done(_) => {}
    }

    // State execution
    match &STATE {
        Level1State::Navigating { heading, reference_pos, robot_direction } => {
            let (cx, cy) = (state.pose.x.as_m(), state.pose.y.as_m());
            let (x, y) = match robot_direction {
                RobotDirection::Forward => {
                    match heading {
                        GlobalDirection::PositiveX => { (cx + pos_lookahead_m, reference_pos.as_m()) }
                        GlobalDirection::NegativeX => { (cx - pos_lookahead_m, reference_pos.as_m()) }
                        GlobalDirection::PositiveY => { (reference_pos.as_m(), cy + pos_lookahead_m) }
                        GlobalDirection::NegativeY => { (reference_pos.as_m(), cy - pos_lookahead_m) }
                    }
                }
                RobotDirection::Backward => {
                    match heading {
                        GlobalDirection::PositiveX | GlobalDirection::NegativeX => { (reference_pos.as_m(), cy) }
                        GlobalDirection::PositiveY | GlobalDirection::NegativeY => { (cx, reference_pos.as_m()) }
                    }
                }
            };
            drive_to_point(Length::from_m(x), Length::from_m(y), Angle::from_deg(25.0), Angle::from_deg(10.0), *robot_direction, state);
        }
        Level1State::Stopped(t) => {
            motor_brake();
            if (ELAPSED.as_s() - t.as_s()) % 1.0 < 0.5 {
                RGBLed::set(RGBLed::RED);
            } else {
                RGBLed::set(RGBLed::BLUE);
            }
        }
        Level1State::Backtracking => {
            if let Some(p) = POSITIONS.peek() {
                drive_to_point(p.0, p.1, Angle::from_deg(25.0), Angle::from_deg(10.0), RobotDirection::Forward, state);
            } else {
                motor_brake();
            }
        }
        Level1State::Done(_) => {}
    }
    
    *ELAPSED.as_s_mut() += LOOP_TIME.as_s();
}


fn drive_motor(l: Velocity, r: Velocity, state: &RobotState) {
    let l = unsafe { LEFT.compute(l.as_m_per_sec(), state.l_vel.as_m_per_sec()) };
    let r = unsafe { RIGHT.compute(r.as_m_per_sec(), state.l_vel.as_m_per_sec()) };

    motor_drive(l as i16, r as i16);
}

#[allow(dead_code)]
unsafe fn follow_trajectory() {
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

        drive_motor(l_speed, r_speed, state);
    }
}