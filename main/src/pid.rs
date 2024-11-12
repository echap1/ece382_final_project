use units::Velocity;

const K_P: f32 = 3000.0;

pub fn pid_compute_duty(setpoint: Velocity, current: Velocity) -> i16 {
    let error_m_per_s = setpoint.as_m_per_sec() - current.as_m_per_sec();
    let duty = (error_m_per_s * K_P) as i16;
    return duty;
}