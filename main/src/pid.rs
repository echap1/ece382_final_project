use crate::{ELAPSED, LEFT, ramsete, RIGHT};
use crate::math::abs;
use crate::motor::{motor_brake, motor_drive};
use crate::odometry::{odometry_get_state, Pose, RobotState, to_wheel_speeds};
use crate::trajectories::TRAJECTORY;
use crate::units::Velocity;

const K_P: f32 = 1000.0;
const K_I: f32 = 0.0;
const K_D: f32 = 0.0;
const K_F: f32 = 1070.0;

const PREV_WINDOW: usize = 3;
const WINDOW_GAP: usize = 2;
const CURRENT_WINDOW: usize = 3;
const BUF_SIZE: usize = PREV_WINDOW + WINDOW_GAP + CURRENT_WINDOW;


pub struct PIDFController {
    accum_error: f32,
    d_buf: [f32; BUF_SIZE],
    buf_idx: usize,
    prev_sum: f32,
    current_sum: f32,
}

impl PIDFController {
    pub const fn new() -> Self {
        Self {
            accum_error: 0.0,
            d_buf: [0.0; BUF_SIZE],
            buf_idx: 0,
            prev_sum: 0.0,
            current_sum: 0.0,
        }
    }

    pub fn compute(&mut self, setpoint: f32, current: f32) -> f32 {
        let error = setpoint - current;

        // Compute D gain
        self.buf_idx = (self.buf_idx + 1) % BUF_SIZE;
        self.prev_sum += self.d_buf[(self.buf_idx + PREV_WINDOW) % BUF_SIZE] - self.d_buf[self.buf_idx];
        self.current_sum += error - self.d_buf[(self.buf_idx + (BUF_SIZE - CURRENT_WINDOW)) % BUF_SIZE];
        self.d_buf[self.buf_idx] = error;
        let d_gain = ((self.current_sum / CURRENT_WINDOW as f32) - (self.prev_sum / PREV_WINDOW as f32)) / (WINDOW_GAP as f32 + 0.5 * (CURRENT_WINDOW + PREV_WINDOW) as f32);

        // Zero out I gain if the error is too high
        let i_val = if abs(error) > 0.2 {
            self.accum_error = 0.0;
            0.0
        } else {
            self.accum_error += error;
            self.accum_error * K_I
        };

        error * K_P + i_val + d_gain * K_D + setpoint * K_F
    }
}

pub fn drive_motor(l: Velocity, r: Velocity, state: &RobotState) {
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
