use cortex_m::asm::delay;
use itoa::Integer;

use ascii::ASCII;
use peripherals::peripherals;
use spia3::{spia3_init, spia3_wait_for_tx, spia3_wait_for_tx_rx_ready, spia3_write_tx_buffer};

const SCREEN_W: u8 = 84;
const SCREEN_H: u8 = 48;

const MAX_X: u8 = SCREEN_W;
const MAX_Y: u8 = SCREEN_H;

const DC: *mut u8 = 0x42099058 as *mut u8;
const RESET: *mut u8 = 0x4209904C as *mut u8;


fn command_write(command: u8) {
    spia3_wait_for_tx_rx_ready();
    unsafe { *DC = 0; }
    spia3_write_tx_buffer(command);
    spia3_wait_for_tx_rx_ready();
}

fn data_write(data: u8) {
    spia3_wait_for_tx();
    unsafe { *DC = 1; }
    spia3_write_tx_buffer(data);
}

pub fn lcd_set_contrast(contrast: u8) {
    unsafe { *RESET = 0; };
    delay(100);
    unsafe { *RESET = 1; }
    command_write(0x21);
    command_write(contrast);
    command_write(0x04);
    command_write(0x14);
    command_write(0x20);
    command_write(0x0C);
}

pub fn lcd_init() {
    let p = peripherals();

    spia3_init();

    p.dio.pesel0().modify(|r, w| unsafe { w.p9sel0().bits(r.p9sel0().bits() & !0x48)});
    p.dio.pesel1().modify(|r, w| unsafe { w.p9sel1().bits(r.p9sel1().bits() & !0x48)});
    p.dio.pedir().modify(|r, w| unsafe { w.p9dir().bits(r.p9dir().bits() | 0x48)});

    lcd_set_contrast(0xB1);
    lcd_clear();
}

pub fn lcd_out_char(data: u8) {
    data_write(0x00);
    for i in 0..5 {
        data_write(ASCII[data as usize - 0x20][i]);
    }
    data_write(0x00);
}

pub fn lcd_out_string(string: &str) {
    for c in string.bytes() {
        lcd_out_char(c);
    }
}

pub fn lcd_out_number<N: Integer>(n: N, min_length: usize) {
    let mut buf = itoa::Buffer::new();
    let s = buf.format(n);
    let s_len = s.len();
    if s_len < min_length {
        for _ in 0..min_length - s_len {
            lcd_out_char(b' ');
        }
    }
    lcd_out_string(s);
}

pub fn lcd_set_cursor(x: u8, y: u8) {
    if x > 11 || y > 5 {
        panic!();
    }
    command_write(0x80 | (x * 7));
    command_write(0x40 | y);
}

pub fn lcd_clear() {
    for _ in 0..(MAX_X as u16 * MAX_Y as u16 / 8) {
        data_write(0x00);
    }
    lcd_set_cursor(0, 0);
}