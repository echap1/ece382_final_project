use core::cmp::min;

use peripherals::peripherals;

pub fn motor_init() {
    let p = peripherals();

    // Configure P5.4 and P5.5 for PH
    p.dio.pcsel0().modify(|r, w| unsafe { w.p5sel0().bits(r.p5sel0().bits() & !0b110000) });
    p.dio.pcsel1().modify(|r, w| unsafe { w.p5sel1().bits(r.p5sel1().bits() & !0b110000) });
    p.dio.pcdir().modify(|r, w| unsafe { w.p5dir().bits(r.p5dir().bits() | 0b110000) });
    p.dio.pcren().modify(|r, w| unsafe { w.p5ren().bits(r.p5ren().bits() | 0b110000) });
    p.dio.pcout().modify(|r, w| unsafe { w.p5out().bits(r.p5out().bits() & !0b110000) });

    // Configure P3.6 and P3.7 for SLEEP
    p.dio.pbsel0().modify(|r, w| unsafe { w.p3sel0().bits(r.p3sel0().bits() & !0b1100000) });
    p.dio.pbsel1().modify(|r, w| unsafe { w.p3sel1().bits(r.p3sel1().bits() & !0b1100000) });
    p.dio.pbdir().modify(|r, w| unsafe { w.p3dir().bits(r.p3dir().bits() | 0b1100000) });
    p.dio.pbren().modify(|r, w| unsafe { w.p3ren().bits(r.p3ren().bits() | 0b1100000) });
    p.dio.pbout().modify(|r, w| unsafe { w.p3out().bits(r.p3out().bits() & !0b1100000) });

    pwm_init();
}

pub fn motor_coast() {
    let p = peripherals();

    p.dio.pcout().modify(|r, w| unsafe { w.p5out().bits(r.p5out().bits() & !0b110000) });
    p.dio.pbout().modify(|r, w| unsafe { w.p3out().bits(r.p3out().bits() & !0b1100000) });

    pwm_duty_left(0);
    pwm_duty_right(0);
}

pub fn motor_brake() {
    let p = peripherals();

    p.dio.pcout().modify(|r, w| unsafe { w.p5out().bits(r.p5out().bits() | 0b110000) });

    pwm_duty_left(0);
    pwm_duty_right(0);
}

pub fn motor_drive(duty_left: i16, duty_right: i16) {
    let p = peripherals();

    p.dio.pbout().modify(|r, w| unsafe { w.p3out().bits(r.p3out().bits() | 0b1100000) });

    if duty_left > 0 {
        p.dio.pcout().modify(|r, w| unsafe { w.p5out().bits(r.p5out().bits() & !0b10000) });
    } else {
        p.dio.pcout().modify(|r, w| unsafe { w.p5out().bits(r.p5out().bits() | 0b10000) });
    }

    if duty_right > 0 {
        p.dio.pcout().modify(|r, w| unsafe { w.p5out().bits(r.p5out().bits() & !0b100000) });
    } else {
        p.dio.pcout().modify(|r, w| unsafe { w.p5out().bits(r.p5out().bits() | 0b100000) });
    }

    pwm_duty_left(duty_left.unsigned_abs());
    pwm_duty_right(duty_right.unsigned_abs());
}

fn pwm_init() {
    let p = peripherals();

    p.dio.pasel0().modify(|r, w| unsafe { w.p2sel0().bits(r.p2sel0().bits() | 0b11000000) });
    p.dio.pasel1().modify(|r, w| unsafe { w.p2sel1().bits(r.p2sel1().bits() & !0b11000000) });
    p.dio.padir().modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits() | 0b11000000) });

    p.timer_a0.tax_cctl(0).write(|w| unsafe { w.bits(0x80) });
    p.timer_a0.tax_ccr(0).write(|w| unsafe { w.bits(15000) });
    p.timer_a0.tax_ex0().write(|w| unsafe { w.bits(0) });
    p.timer_a0.tax_cctl(3).write(|w| unsafe { w.bits(0x40) });
    p.timer_a0.tax_ccr(3).write(|w| unsafe { w.bits(0) });
    p.timer_a0.tax_cctl(4).write(|w| unsafe { w.bits(0x40) });
    p.timer_a0.tax_ccr(4).write(|w| unsafe { w.bits(0) });
    p.timer_a0.tax_ctl().write(|w| unsafe { w.bits(0x02F0) });
}

fn pwm_duty_right(mut duty_permil: u16) {
    if duty_permil > 1000 { duty_permil = 1000; }
    let duty = min(15000, (duty_permil * 30) >> 1);
    peripherals().timer_a0.tax_ccr(3).write(|w| unsafe { w.bits(duty) });
}

fn pwm_duty_left(mut duty_permil: u16) {
    if duty_permil > 1000 { duty_permil = 1000; }
    let duty = min(15000, (duty_permil * 30) >> 1);
    peripherals().timer_a0.tax_ccr(4).write(|w| unsafe { w.bits(duty) });
}