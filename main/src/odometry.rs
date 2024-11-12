use units::{Angle, Length, Time, Velocity};

pub struct Pose {
    pub x: Length,
    pub y: Length,
    pub theta: Angle
}

pub struct RobotState {
    pub pose: Pose,
    pub l_vel: Velocity,
    pub r_vel: Velocity,
}

static mut STATE: RobotState = RobotState {
    pose: Pose {
        x: Length::from_m(0.0),
        y: Length::from_m(0.0),
        theta: Angle::from_rad(0.0),
    },
    l_vel: Velocity::from_m_per_sec(0.0),
    r_vel: Velocity::from_m_per_sec(0.0)
};

const WHEEL_DIST: Length = Length::from_m(0.14);

pub fn odometry_init() {
    unsafe {
        STATE = RobotState {
            pose: Pose {
                x: Length::from_m(0.0),
                y: Length::from_m(0.0),
                theta: Angle::from_rad(0.0),
            },
            l_vel: Velocity::from_m_per_sec(0.0),
            r_vel: Velocity::from_m_per_sec(0.0)
        }
    }
}

pub fn odometry_update(l_dist: Length, r_dist: Length, elapsed: Time) {
    let state = unsafe { &mut STATE };

    let d_avg_m = (l_dist.as_m() + r_dist.as_m()) / 2.0;

    let delta_theta = (r_dist.as_m() - l_dist.as_m()) / WHEEL_DIST.as_m();
    *state.pose.theta.as_rad_mut() += delta_theta;
    // let (sin_theta, cos_theta) = libm::sincosf(state.pose.theta.get::<radian>());
    let (sin_theta, cos_theta) = (0.,0.);
    *state.pose.x.as_m_mut() += d_avg_m * cos_theta;
    *state.pose.y.as_m_mut() += d_avg_m * sin_theta;
    *state.l_vel.as_m_per_sec_mut() = l_dist.as_m() / elapsed.as_s();
    *state.r_vel.as_m_per_sec_mut() = r_dist.as_m() / elapsed.as_s();
}

pub fn odometry_get_state() -> &'static RobotState {
    unsafe {
        &STATE
    }
}