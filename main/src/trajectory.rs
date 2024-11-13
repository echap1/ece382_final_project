use math::floor;
use units::{Angle, AngularVelocity, Length, Time, Velocity};

pub struct Trajectory<const N: usize> {
    pub total_time: f32,
    pub step_time: f32,
    pub points: [(f32, f32, f32, f32, f32); N]
}

#[inline(always)]
fn interp(a: f32, b: f32, a_f: f32) -> f32 {
    let b_f = 1.0 - a_f;
    a * a_f + b * b_f
}

impl<const N: usize> Trajectory<N> {
    pub fn linear_interp(&self, t: Time) -> (Length, Length, Angle, Velocity, AngularVelocity) {
        let left_idx = (floor(t.as_s() / self.step_time).max(0.0) as usize).min(N - 1);
        let right_idx = (left_idx + 1).min(N - 1);
        
        let left_bias = (t.as_s() % self.step_time) / self.step_time;

        let left = self.points[left_idx];
        let right = self.points[right_idx];
        
        (
            Length::from_m(interp(left.0, right.0, left_bias)),
            Length::from_m(interp(left.1, right.1, left_bias)),
            Angle::from_rad(interp(left.2, right.2, left_bias)),
            Velocity::from_m_per_sec(interp(left.3, right.3, left_bias)),
            AngularVelocity::from_rad_per_sec(interp(left.4, right.4, left_bias)),
        )
    }
}