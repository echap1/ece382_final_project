use crate::adc14::adc_in;
use crate::clock::wait_for_interrupt;
use crate::odometry::odometry_get_state;

pub unsafe fn level2_main() -> ! {
    loop {
        wait_for_interrupt();
    }
}

pub unsafe fn level2_periodic() {
    let state = odometry_get_state();
    
    let adc_values = adc_in();
}