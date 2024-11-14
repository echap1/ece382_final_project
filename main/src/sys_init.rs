use crate::peripherals::peripherals;

pub fn system_init() {
    let p = peripherals();

    // Halt the WDT
    p.wdt_a.wdtctl().write( |w| unsafe { w.bits(0x5A80) });

    // Enable all SRAM banks
    p.sysctl.sys_sram_banken().write(|w| { w.bnk7_en().set_bit() });

    // DCO = 3 MHz; MCLK = source
    p.cs.cskey().write(|w| unsafe { w.cskey().bits(0x695A) });  // Unlock CS module for register access
    p.cs.csctl0().write(|w| { w.dcorsel().dcorsel_0() });  // Set DCO to 1.5MHz
    p.cs.csctl1().modify(|r, w| unsafe { w.bits((r.bits() & !0x70007) | 0x3) });  // Select MCLK as DCO source
    p.cs.cskey().write(|w| unsafe { w.cskey().bits(0) });

    // Set Flash Bank read buffering
    p.flctl.flctl_bank0_rdctl().modify(|_, w| { w
        .bufd().clear_bit()
        .bufi().clear_bit()
    });
    p.flctl.flctl_bank1_rdctl().modify(|_, w| { w
        .bufd().clear_bit()
        .bufi().clear_bit()
    });
}
