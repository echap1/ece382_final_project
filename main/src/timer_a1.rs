use cortex_m_rt::interrupt;

use msp432P401r_api::Interrupt as interrupt;
use peripherals::{peripherals, peripherals_cortex};
use units::Time;

static mut TIMERA1_TASK: Option<unsafe fn() -> ()> = None;
static mut DIVIDER: u32 = 1;
static mut COUNTER: u32 = 0;

pub fn timera1_init(task: unsafe fn() -> (), period: Time) {
    let timer_a1 = &peripherals().timer_a1;
    let nvic = &peripherals_cortex().NVIC;

    unsafe { TIMERA1_TASK = Some(task); }

    timer_a1.tax_ctl().modify(|r, w| unsafe { w.bits(r.bits() & !0x0030) });
    timer_a1.tax_ctl().write(|w| unsafe { w.bits(0x0280) });

    timer_a1.tax_cctl(0).write(|w| unsafe { w.bits(0x10) });

    let mut period_2us;

    unsafe {
        DIVIDER = 1;
        loop {
            period_2us = period.as_us() as u32 / (DIVIDER * 2) - 1;
            if period_2us > u16::MAX as u32 {
                DIVIDER += 1;
            } else {
                break;
            }
        }
    }

    timer_a1.tax_ccr(0).write(|w| unsafe { w.bits(period_2us as u16) });

    timer_a1.tax_ex0().write(|w| unsafe { w.bits(0x5) });

    unsafe {
        nvic.ipr[10].write(0x40);
        nvic.iser[0].write(0x400);
    }

    timer_a1.tax_ctl().modify(|r, w| unsafe { w.bits(r.bits() | 0x14) });
}

#[interrupt]
unsafe fn TA1_0_IRQ() {
    peripherals().timer_a1.tax_cctl(0).modify(|_, w| { w.ccifg().clear_bit() });
    if COUNTER % DIVIDER == 0 {
        if let Some(task) = TIMERA1_TASK {
            task();
        }
    }
    COUNTER = (COUNTER + 1) % DIVIDER;
}