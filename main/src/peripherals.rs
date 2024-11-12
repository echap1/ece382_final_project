use msp432P401r_api::Peripherals;

static mut PERIPHERALS: Option<Peripherals> = None;
static mut PERIPHERALS_CORTEX: Option<cortex_m::Peripherals> = None;

#[allow(static_mut_refs)]
pub fn peripherals_cortex<'l>() -> &'l cortex_m::Peripherals {
    unsafe {
        match &PERIPHERALS_CORTEX {
            None => {
                PERIPHERALS_CORTEX = Some(cortex_m::Peripherals::steal());
                PERIPHERALS_CORTEX.as_ref().unwrap_unchecked()
            }
            Some(p) => {
                return &p;
            }
        }
    }
}

#[allow(static_mut_refs)]
pub fn peripherals<'l>() -> &'l Peripherals {
    unsafe {
        match &PERIPHERALS {
            None => {
                PERIPHERALS = Some(Peripherals::steal());
                PERIPHERALS.as_ref().unwrap_unchecked()
            }
            Some(p) => {
                return &p;
            }
        }
    }
}