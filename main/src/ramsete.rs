use math::{cos, sin, sqrt};
use odometry::Pose;
use units::{AngularVelocity, Velocity};

const K_BETA: f32 = 3.0;
const K_ZETA: f32 = 0.7;

pub fn compute(current: &Pose, desired: &Pose, v_d: Velocity, omega_d: AngularVelocity) -> (Velocity, AngularVelocity) {
    // Error in global ref frame
    let x_error = desired.x.as_m() - current.x.as_m();
    let y_error = desired.y.as_m() - current.y.as_m();

    let sin_theta = sin(current.theta.as_rad());
    let cos_theta = cos(current.theta.as_rad());

    // Error in local ref frame
    let e_x = x_error * cos_theta + y_error * sin_theta;
    let e_y = y_error * cos_theta - x_error * sin_theta;
    let e_theta = desired.theta.as_rad() - current.theta.as_rad();

    let omega_d = omega_d.as_rad_per_s();
    let v_d = v_d.as_m_per_sec();
    let k = 2.0 * K_ZETA * sqrt(omega_d*omega_d + K_BETA * v_d*v_d);

    let v_out = Velocity::from_m_per_sec(v_d * cos(e_theta) + k * e_x);
    let omega_out = AngularVelocity::from_rad_per_sec(omega_d + k * e_theta + if e_theta == 0.0 { 0.0 } else { (K_BETA * v_d * sin(e_theta) * e_y) / e_theta });

    (v_out, omega_out)
}