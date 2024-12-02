use crate::moving_average::MovingAverage;
use crate::peripherals::peripherals;
use crate::units::Length;

struct ADCOutput {
    ch17: u32,
    ch14: u32,
    ch16: u32,
}

pub struct IRSensorOutput {
    left: Length,
    center: Length,
    right: Length
}

static mut CH17: MovingAverage<u32, 256> = MovingAverage::new(0);
static mut CH14: MovingAverage<u32, 256> = MovingAverage::new(0);
static mut CH16: MovingAverage<u32, 256> = MovingAverage::new(0);

pub fn adc0_init() {
    let p = peripherals();
    p.adc14.adc14ctl0().modify(|r, w| unsafe { w.bits(r.bits() & !2)});
    while p.adc14.adc14ctl0().read().adc14busy().bit_is_set() { }
    p.adc14.adc14ctl0().write(|w| unsafe { w.bits(0x04220390) });
    p.adc14.adc14ctl1().write(|w| unsafe { w.bits(0x00020030) });
    p.adc14.adc14mctl(2).write(|w| unsafe { w.bits(0x00000011) });
    p.adc14.adc14mctl(3).write(|w| unsafe { w.bits(0x0000000E) });
    p.adc14.adc14mctl(4).write(|w| unsafe { w.bits(0x00000090) });
    p.adc14.adc14ier0().write(|w| unsafe { w.bits(0) });
    p.adc14.adc14ier1().write(|w| unsafe { w.bits(0) });
    p.dio.pcsel0().modify(|r, w| unsafe { w.p6sel0().bits(r.p6sel0().bits() | 2) });
    p.dio.pcsel1().modify(|r, w| unsafe { w.p6sel1().bits(r.p6sel1().bits() | 2) });
    p.dio.pesel0().modify(|r, w| unsafe { w.p9sel0().bits(r.p9sel0().bits() | 3) });
    p.dio.pesel1().modify(|r, w| unsafe { w.p9sel1().bits(r.p9sel1().bits() | 3) });
    p.adc14.adc14ctl0().modify(|r, w| unsafe { w.bits(r.bits() | 2)});
}

const MAX_DIST: u32 = 800;  // mm

#[inline(always)]
fn adc_convert(val: u32, adcmax: u32, irslope: u32, iroffset: u32, dist_offset: u32) -> Length {
    Length::from_mm(if val < adcmax {
        MAX_DIST
    } else {
        let res = dist_offset + irslope / (val - iroffset);
        if res > MAX_DIST { MAX_DIST }
        else { res }
    } as f32)
}

pub fn adc_in() -> IRSensorOutput {
    let raw = adc_in_raw();
    let left = unsafe { CH16.push_and_report(raw.ch16) };
    let center = unsafe { CH14.push_and_report(raw.ch14) };
    let right = unsafe { CH17.push_and_report(raw.ch17) };
    IRSensorOutput {
        left: adc_convert(left, 1112, 1083571, 342, 80),
        center: adc_convert(center, 1234, 1081174, 572, 70),
        right: adc_convert(right, 1367, 1121822, 478, 80),
    }
}

fn adc_in_raw() -> ADCOutput {
    let p = peripherals();
    while p.adc14.adc14ctl0().read().adc14busy().bit_is_set() { }
    p.adc14.adc14ctl0().modify(|r, w| unsafe { w.bits(r.bits() | 1)});
    while p.adc14.adc14ifgr0().read().adc14ifg4().bit_is_clear() { }
    ADCOutput {
        ch17: p.adc14.adc14mem(2).read().bits(),
        ch14: p.adc14.adc14mem(3).read().bits(),
        ch16: p.adc14.adc14mem(4).read().bits(),
    }
}