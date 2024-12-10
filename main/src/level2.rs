use core::fmt::Write;

use crate::adc14::{adc_get, adc_update};
use crate::byte_mut_writer::ByteMutWriter;
use crate::clock::delay_1ms;
use crate::ELAPSED;
use crate::lcd::{lcd_clear, lcd_out_string, lcd_set_cursor};
use crate::motor::{max_drive, motor_brake, motor_drive};
use crate::odometry::{odometry_get_state, odometry_update, to_wheel_speeds};
use crate::pid::drive_motor;
use crate::rgb_led::RGBLed;
use crate::tachometer::{get_distances_and_clear, get_speeds};
use crate::units::{AngularVelocity, Velocity};

pub unsafe fn level2_main() -> ! {
    // loop {
    //     let adc = adc_in();
    // 
    //     lcd_clear();
    //     let mut buf = [0; 12];
    // 
    //     lcd_set_cursor(0, 0);
    //     write!(ByteMutWriter::new(&mut buf), "L: {:.2}", adc.left.as_mm()).unwrap();
    //     lcd_out_string(core::str::from_utf8_unchecked(&buf));
    //     buf.fill(0);
    // 
    //     lcd_set_cursor(0, 1);
    //     write!(ByteMutWriter::new(&mut buf), "C: {:.2}", adc.center.as_mm()).unwrap();
    //     lcd_out_string(core::str::from_utf8_unchecked(&buf));
    //     buf.fill(0);
    // 
    //     lcd_set_cursor(0, 2);
    //     write!(ByteMutWriter::new(&mut buf), "R: {:.2}", adc.right.as_mm()).unwrap();
    //     lcd_out_string(core::str::from_utf8_unchecked(&buf));
    //     buf.fill(0);
    // 
    //     delay_1ms(300);
    // }
    loop {
        // let b = 1.1 - adc.center.as_m();
        // max_drive(adc.left.as_m() - adc.right.as_m(), -(b * b * 2000.0));
        
        lcd_clear();
        let mut buf = [0; 12];
        
        // Horizontal position error
        lcd_set_cursor(0, 0);
        write!(ByteMutWriter::new(&mut buf), "Error: {:.2}", error).unwrap();
        lcd_out_string(core::str::from_utf8_unchecked(&buf));
        buf.fill(0);

        // Target left velocity
        lcd_set_cursor(0, 1);
        write!(ByteMutWriter::new(&mut buf), "TVL: {:.2}", tvl.as_m_per_sec()).unwrap();
        lcd_out_string(core::str::from_utf8_unchecked(&buf));
        buf.fill(0);

        // Target right velocity
        lcd_set_cursor(0, 2);
        write!(ByteMutWriter::new(&mut buf), "TVR: {:.2}", tvr.as_m_per_sec()).unwrap();
        lcd_out_string(core::str::from_utf8_unchecked(&buf));
        buf.fill(0);

        // Actual left velocity
        lcd_set_cursor(0, 3);
        write!(ByteMutWriter::new(&mut buf), "VL: {:.2}", odometry_get_state().l_vel.as_m_per_sec()).unwrap();
        lcd_out_string(core::str::from_utf8_unchecked(&buf));
        buf.fill(0);
        
        // Actual right velocity
        lcd_set_cursor(0, 4);
        write!(ByteMutWriter::new(&mut buf), "VR: {:.2}", odometry_get_state().r_vel.as_m_per_sec()).unwrap();
        lcd_out_string(core::str::from_utf8_unchecked(&buf));
        buf.fill(0);

        // Target angular velocity
        lcd_set_cursor(0, 5);
        write!(ByteMutWriter::new(&mut buf), "Omega: {:.2}", omega.as_rad_per_s()).unwrap();
        lcd_out_string(core::str::from_utf8_unchecked(&buf));
        buf.fill(0);
        
        // Janky adc updating this should have made the measurements better but it didn't
        for _ in 0..10 {
            adc_update();
            delay_1ms(20);
        }
    }
}

static mut error: f32 = 0.0;
static mut omega: AngularVelocity = AngularVelocity::from_rad_per_sec(0.0);
static mut tvl: Velocity = Velocity::from_m_per_sec(0.0);
static mut tvr: Velocity = Velocity::from_m_per_sec(0.0);

static mut done_steps: u32 = 0;
static mut done: bool = false;

pub unsafe fn level2_periodic() {
    let adc = adc_get();

    let b = 1.3 - adc.center.as_m();
    let k_p = b * b * 12.0;

    let mut left_m = adc.left.as_m();
    let mut right_m = adc.right.as_m();

    if adc.center.as_m() > 0.45 {
        if left_m > 0.3 { left_m = 0.3; }
        if right_m > 0.3 { right_m = 0.3; }
    } else if left_m > 0.3 && right_m > 0.3 {
        left_m = 0.5;
    }
    
    if adc.center.as_m() < 0.2 && left_m < 0.3 && right_m < 0.3 {
        done_steps += 1;
        if done_steps > 10 {
            done = true;
        }
    } else {
        done_steps = 0;
    }
    
    let v = Velocity::from_m_per_sec(0.8);
    error = left_m - right_m;
    omega = AngularVelocity::from_rad_per_sec(k_p * error);

    if done {
        motor_brake();
        if ELAPSED.as_s() % 1.0 < 0.5 {
            RGBLed::set(RGBLed::RED);
        } else {
            RGBLed::set(RGBLed::BLUE);
        }
    } else {
        (tvl, tvr) = to_wheel_speeds(v, omega);
        drive_motor(tvl, tvr, odometry_get_state());
    }
}