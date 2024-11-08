use uom::si::f32::Length;
use uom::si::length::millimeter;
use peripherals::peripherals;
use timer_a3_input_capture::timera3_capture_init;

struct WheelState {
    prev_int_time: u16,
    current_int_time: u16,
    steps: i32,
}

static mut LEFT_STATE: WheelState = WheelState {
    prev_int_time: 0,
    current_int_time: 0,
    steps: 0,
};

static mut RIGHT_STATE: WheelState = WheelState {
    prev_int_time: 0,
    current_int_time: 0,
    steps: 0,
};

impl WheelState {
    fn update(&mut self, current_time: u16, is_forward: bool) {
        self.prev_int_time = self.current_int_time;
        self.current_int_time = current_time;
        if is_forward {
            self.steps += 1;
        } else {
            self.steps -= 1;
        }
    }
}

pub fn tachometer_init() {
    let p = peripherals();

    p.dio.pcsel0().modify(|r, w| unsafe { w.p5sel0().bits(r.p5sel0().bits() & !0x05) });
    p.dio.pcsel1().modify(|r, w| unsafe { w.p5sel1().bits(r.p5sel1().bits() & !0x05) });
    p.dio.pcdir().modify(|r, w| unsafe { w.p5dir().bits(r.p5dir().bits() & !0x05) });

    timera3_capture_init(right_interrupt, left_interrupt);
}

unsafe fn left_interrupt(current_time: u16) {
    LEFT_STATE.update(current_time, peripherals().dio.pcin().read().bits() & 4 != 0)
}

unsafe fn right_interrupt(current_time: u16) {
    RIGHT_STATE.update(current_time, peripherals().dio.pcin().read().bits() & 1 != 0)
}

pub fn get_distances_and_clear() -> (Length, Length) {
    unsafe {
        let res = (
            Length::new::<millimeter>(LEFT_STATE.steps as f32 * 220.0/360.0),
            Length::new::<millimeter>(RIGHT_STATE.steps as f32 * 220.0/360.0)
        );
        LEFT_STATE.steps = 0;
        RIGHT_STATE.steps = 0;
        return res;
    }
}