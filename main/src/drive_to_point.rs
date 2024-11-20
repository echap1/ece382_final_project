use crate::drive_motor;
use crate::math::{atan2, clamp, normalize_angle_rad, sqrt};
use crate::odometry::{RobotState, to_wheel_speeds};
use crate::units::{Angle, AngularVelocity, Length, Velocity};

const K_P_OMEGA: f32 = 1.0;
const MAX_OMEGA: f32 = 1.0;

const K_P_VEL: f32 = 1.0;
const MAX_VEL: f32 = 1.0;

pub fn drive_to_point(x: Length, y: Length, min_d_theta_to_drive: Angle, state: &RobotState) {
    let dx = x.as_m() - state.pose.x.as_m();
    let dy = y.as_m() - state.pose.y.as_m();
    
    // The global reference frame angle from robot to target
    let global_d_theta = atan2(dy, dx);
    
    // Error in the robot's heading
    let local_d_theta = normalize_angle_rad(global_d_theta - state.pose.theta.as_rad());
    
    // Calculate angular velocity based on theta error
    let omega = AngularVelocity::from_rad_per_sec(
        clamp(K_P_OMEGA * local_d_theta, MAX_OMEGA)
    );
    
    // Calculate velocity based on distance left to target
    let v = Velocity::from_m_per_sec(
        if local_d_theta > min_d_theta_to_drive.as_rad() { 0.0 } else {
            clamp(K_P_VEL * sqrt(dx*dx + dy*dy), MAX_VEL)
        }
    );
    
    // Drive the motors
    let (vl, vr) = to_wheel_speeds(v, omega);
    drive_motor(vl, vr, state);
}
