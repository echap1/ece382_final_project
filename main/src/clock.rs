use core::arch::asm;

use crate::peripherals::peripherals;

pub fn clock_init48mhz() {
    let p = peripherals();

    let mut prewait: u32 = 0;       // loops between BSP_Clock_InitFastest() called and PCM idle (expect 0)
    let mut cpmwait: u32 = 0;       // loops between Power Active Mode Request and Current Power Mode matching requested mode (expect small)
    let mut postwait: u32 = 0;      // loops between Current Power Mode matching requested mode and PCM module idle (expect about 0)
    let mut crystalstable: u32 = 0; // loops before the crystal stabilizes (expect small)

    // wait for the PCMCTL0 and Clock System to be write-able by waiting
    // for Power Control Manager to be idle
    while p.pcm.pcmctl1().read().pmr_busy().bit_is_set() {
        prewait += 1;
        if prewait >= 100000 { panic!(); }
    }

    // request power active mode LDO VCORE1 to support the 48 MHz frequency
    // clear PCMKEY bit field and AMR bit field
    // write the proper PCM key to unlock write access
    // request power active mode LDO VCORE1
    p.pcm.pcmctl0().modify(|_, w| unsafe { w.
        pcmkey().bits(0x695A)
        .amr().amr_1()
    });

    // check if the transition is invalid (see Figure 7-3 on p344 of datasheet)
    if p.pcm.pcmifg().read().am_invalid_tr_ifg().bit_is_set() {
        p.pcm.pcmclrifg().write(|w| w.clr_am_invalid_tr_ifg().set_bit());
        panic!();
    }

    // wait for the CPM (Current Power Mode) bit field to reflect a change to active mode LDO VCORE1
    while !p.pcm.pcmctl0().read().cpm().is_cpm_1() {
        cpmwait += 1;
        if cpmwait >= 500000 { panic!() }
    }

    // wait for the PCMCTL0 and Clock System to be write-able by waiting for Power Control Manager to be idle
    while p.pcm.pcmctl1().read().pmr_busy().bit_is_set() {
        postwait += 1;
        if postwait >= 100000 { panic!(); }
    }

    // initialize PJ.3 and PJ.2 and make them HFXT (PJ.3 built-in 48 MHz crystal out; PJ.2 built-in 48 MHz crystal in)
    p.dio.pjsel0().modify(|r, w| unsafe { w.bits(r.bits() | 0x0C) });
    p.dio.pjsel1().modify(|r, w| unsafe { w.bits(r.bits() & !0x0C) });

    // configure built-in 48 MHz crystal for HFXT operation
    p.cs.cskey().write(|w| unsafe { w.bits(0x695A) });  // unlock CS module for register access
    p.cs.csctl2().modify(|_, w| { w
        .hfxtfreq().hfxtfreq_6()
        .hfxtdrive().hfxtdrive_1()
        .hfxt_en().hfxt_en_1()
    });
    p.cs.csctl2().modify(|_, w| { w
        .hfxtbypass().hfxtbypass_0()
    });

    while p.cs.csifg().read().hfxtifg().bit_is_set() {
        p.cs.csclrifg().write(|w| { w.clr_hfxtifg().set_bit() });
        crystalstable += 1;
        if crystalstable >= 100000 { panic!() }
    }

    p.flctl.flctl_bank0_rdctl().modify(|_, w| { w
        .wait().wait_2()
    });
    p.flctl.flctl_bank1_rdctl().modify(|_, w| { w
        .wait().wait_2()
    });

    p.cs.csctl1().write(|w| {w
        .divs().divs_2()
        .divhs().divhs_1()
        .sela().sela_1()
        .sels().sels_5()
        .selm().selm_5()
    });
    p.cs.cskey().write(|w| unsafe { w.bits(0) });
}

pub fn delay_1ms(mut n: u32) {
    while n > 0 {
        delay(48_000_000/9162);   // 1 msec, tuned at 48 MHz
        n -= 1;
    }
}


pub extern "C" fn wait_for_interrupt(){
    unsafe { asm!("wfi") }
}

extern "C" fn delay(count: u32){
    unsafe {
        asm!(
            "2:  subs   {0}, #1",
            "    bne    2b",
            inout(reg) count => _,
            options(nomem, nostack)
        );
    }
}
