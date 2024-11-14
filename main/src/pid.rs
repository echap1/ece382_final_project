const K_P: f32 = 2000.0;
const K_I: f32 = 50.0;
const K_D: f32 = 0.0;

const PREV_WINDOW: usize = 3;
const WINDOW_GAP: usize = 2;
const CURRENT_WINDOW: usize = 3;
const BUF_SIZE: usize = PREV_WINDOW + WINDOW_GAP + CURRENT_WINDOW;


pub struct PIController {
    accum_error: f32,
    d_buf: [f32; BUF_SIZE],
    buf_idx: usize,
    prev_sum: f32,
    current_sum: f32,
}

impl PIController {
    pub const fn new() -> Self {
        Self {
            accum_error: 0.0,
            d_buf: [0.0; BUF_SIZE],
            buf_idx: 0,
            prev_sum: 0.0,
            current_sum: 0.0,
        }
    }

    pub fn compute(&mut self, setpoint: f32, current: f32) -> f32 {
        let error = setpoint - current;

        self.buf_idx = (self.buf_idx + 1) % BUF_SIZE;
        self.prev_sum += self.d_buf[(self.buf_idx + PREV_WINDOW) % BUF_SIZE] - self.d_buf[self.buf_idx];
        self.current_sum += error - self.d_buf[(self.buf_idx + (BUF_SIZE - CURRENT_WINDOW)) % BUF_SIZE];
        self.d_buf[self.buf_idx] = error;

        let d_gain = ((self.current_sum / CURRENT_WINDOW as f32) - (self.prev_sum / PREV_WINDOW as f32)) / (WINDOW_GAP as f32 + 0.5 * (CURRENT_WINDOW + PREV_WINDOW) as f32);

        let i_val = if error > 0.2 {
            0.0
        } else {
            self.accum_error += error;
            self.accum_error * K_I
        };

        error * K_P + i_val + d_gain * K_D
    }
}