use core::f32::consts::PI;
use crate::{drive_motor, RobotDirection};
use crate::math::{abs, atan2, clamp, normalize_angle_rad, sqrt};
use crate::odometry::{RobotState, to_wheel_speeds};
use crate::units::{Angle, AngularVelocity, Length, Velocity};

const K_P_OMEGA: f32 = 3.0;
const MAX_OMEGA: f32 = 2.0;

const K_P_VEL: f32 = 10.0;
const MAX_VEL: f32 = 0.15;

pub fn drive_to_point(x: Length, y: Length, min_d_theta_to_drive: Angle, max_speed_theta: Angle, direction: RobotDirection, state: &RobotState) {
    let dx = x.as_m() - state.pose.x.as_m();
    let dy = y.as_m() - state.pose.y.as_m();
    
    // The global reference frame angle from robot to target
    let global_d_theta = atan2(dy, dx);
    
    // Error in the robot's heading
    let local_d_theta = normalize_angle_rad(match direction {
        RobotDirection::Forward => {
            global_d_theta - state.pose.theta.as_rad()
        }
        RobotDirection::Backward => {
            global_d_theta - state.pose.theta.as_rad() + PI  // 180 degree difference
        }
    });
    
    // Calculate angular velocity based on theta error
    let omega = AngularVelocity::from_rad_per_sec(
        clamp(K_P_OMEGA * local_d_theta, MAX_OMEGA)
    );
    
    // Calculate velocity based on distance left to target
    let v = Velocity::from_m_per_sec(
        if local_d_theta > min_d_theta_to_drive.as_rad() { 0.0 } else {
            let abs_theta = abs(local_d_theta);
            
            clamp(match direction {
                RobotDirection::Forward => { K_P_VEL }
                RobotDirection::Backward => { -K_P_VEL }
            } * sqrt(dx*dx + dy*dy), MAX_VEL) * if abs_theta <= max_speed_theta.as_rad() { 1.0 } else {
                max_speed_theta.as_rad() / abs_theta
            }
        }
    );
    
    // Drive the motors
    let (vl, vr) = to_wheel_speeds(v, omega);
    drive_motor(vl, vr, state);
}
