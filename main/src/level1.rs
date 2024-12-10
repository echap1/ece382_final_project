use core::fmt::Write;

use crate::buffer_stack::BufferStack;
use crate::bump::bump_read;
use crate::byte_mut_writer::ByteMutWriter;
use crate::clock::delay_1ms;
use crate::drive_to_point::drive_to_point;
use crate::ELAPSED;
use crate::lcd::{lcd_clear, lcd_out_string, lcd_set_cursor};
use crate::math::{abs, within_range};
use crate::motor::motor_brake;
use crate::odometry::{GlobalDirection, odometry_get_state, RobotDirection};
use crate::rgb_led::RGBLed;
use crate::uart0::uart0_out_string;
use crate::units::{Angle, Length, Time};


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

/// Positions where the Robot turned. Used for backtracking.
static mut POSITIONS: BufferStack<(Length, Length), 100> = BufferStack::new();

/// Position buffer that is sent over UART after maze navigation. It's only a stack because I didn't want to implement a new data type.
static mut STATE_BUF: BufferStack<(Length, Length), 2000> = BufferStack::new();

pub unsafe fn level1_main() -> ! {
    // Push the start position as the last position to return to when backtracking.
    POSITIONS.push((Length::from_m(0.0), Length::from_m(0.0)));

    let mut transmitted = false;

    loop {
        // Push coordinate data every ~100ms
        let state = odometry_get_state();
        STATE_BUF.push((state.pose.x, state.pose.y));

        // Print our state
        let mut buf = [0; 12];
        lcd_clear();
        match &STATE {
            Level1State::Navigating{ heading, robot_direction , ..} => {
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

                // Transmit the data 60 seconds after completion
                if !transmitted && ELAPSED.as_s() - t.as_s() > 60.0 {
                    transmitted = true;
                    let mut buf2 = [0; 100];
                    while let Some(s) = STATE_BUF.pop() {
                        // I love the rust write macro it makes my life soo easy!
                        writeln!(ByteMutWriter::new(&mut buf2), "{},{}", s.0.as_m(), s.1.as_m()).unwrap();
                        uart0_out_string(core::str::from_utf8_unchecked(&buf2));
                        delay_1ms(1);  // Tried to make data transmission more reliable (it didn't work)
                    }
                }
            }
        }
        lcd_set_cursor(0, 5);
        writeln!(ByteMutWriter::new(&mut buf), "{:.2}", ELAPSED.as_s()).unwrap();
        lcd_out_string(core::str::from_utf8_unchecked(&buf));
        uart0_out_string(core::str::from_utf8_unchecked(&buf));
        buf.fill(0);
        delay_1ms(100);  // Please don't kill me for using delay :)
    }
}

pub unsafe fn level1_periodic() {
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
}