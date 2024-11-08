use uom::si::f32::Time;
use uom::si::time::microsecond;
use msp432P401r_api::Interrupt as interrupt;
use cortex_m_rt::interrupt;
use peripherals::{peripherals, peripherals_cortex};

type CaptureTask = unsafe fn(time: u16) -> ();
static mut CAPTURE_TASK_0: Option<CaptureTask> = None;
static mut CAPTURE_TASK_1: Option<CaptureTask> = None;

pub fn timera3_capture_init(task0: CaptureTask, task1: CaptureTask) {
    let p = peripherals();
    let nvic = &peripherals_cortex().NVIC;

    unsafe {
        CAPTURE_TASK_0 = Some(task0);
        CAPTURE_TASK_1 = Some(task1);
    }

    p.dio.pesel0().modify(|r, w| unsafe { w.p10sel0().bits(r.p10sel0().bits() | 0x30) });
    p.dio.pesel1().modify(|r, w| unsafe { w.p10sel1().bits(r.p10sel1().bits() & !0x30) });

    p.timer_a3.tax_ctl().modify(|r, w| unsafe { w.bits(r.bits() & !0x0030) });
    p.timer_a3.tax_ctl().write(|w| unsafe { w.bits(0x0200) });
    p.timer_a3.tax_ex0().write(|w| unsafe { w.bits(0x7) });
    p.timer_a3.tax_cctl(0).write(|w| unsafe { w.bits(0x4910) });
    p.timer_a3.tax_cctl(1).write(|w| unsafe { w.bits(0x4910) });

    unsafe {
        nvic.ipr[14].write(2 << 5);
        nvic.ipr[15].write(2 << 5);
        nvic.iser[0].write(0b11 << 14);
    }

    p.timer_a3.tax_ctl().modify(|r, w| unsafe { w.bits(r.bits() | 0x24) });
}

#[interrupt]
unsafe fn TA3_0_IRQ() {
    let timer_a3 = &peripherals().timer_a3;
    timer_a3.tax_cctl(0).modify(|_, w| { w.ccifg().clear_bit() });
    if let Some(task) = CAPTURE_TASK_0 {
        task(timer_a3.tax_ccr(0).read().bits());
    }
}

#[interrupt]
unsafe fn TA3_N_IRQ() {
    let timer_a3 = &peripherals().timer_a3;
    timer_a3.tax_cctl(1).modify(|_, w| { w.ccifg().clear_bit() });
    if let Some(task) = CAPTURE_TASK_1 {
        task(timer_a3.tax_ccr(1).read().bits());
    }
}