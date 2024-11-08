use uom::si::angle::radian;
use uom::si::f32::{Angle, Length, Time, Velocity};
use uom::si::length::centimeter;

pub struct RobotState {
    pub x: Length,
    pub y: Length,
    pub theta: Angle,
    pub l_vel: Velocity,
    pub r_vel: Velocity,
}

pub static mut STATE: Option<RobotState> = None;

pub fn odometry_update(l_dist: Length, r_dist: Length, elapsed: Time) {
    let state;
    unsafe {
        if STATE.is_none() {
            STATE = Some(RobotState {
                x: Default::default(),
                y: Default::default(),
                theta: Default::default(),
                l_vel: Default::default(),
                r_vel: Default::default(),
            })
        }
        state = STATE.as_mut().unwrap_unchecked();
    }

    let wheel_dist = Length::new::<centimeter>(14.0);

    let d_avg = (l_dist + r_dist) / 2.0;

    let delta_theta = (r_dist - l_dist) / wheel_dist;
    state.theta += delta_theta.into();
    state.x += d_avg * libm::cosf(state.theta.get::<radian>());
    state.y += d_avg * libm::sinf(state.theta.get::<radian>());
    state.l_vel = l_dist / elapsed;
    state.r_vel = r_dist / elapsed;
}