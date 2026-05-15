pub mod registers;

use core::ptr::{read_volatile, write_volatile};

use crate::lgt8f328p::registers::*;

pub fn pin_off(pin: u8) {
    unsafe {
        let portd_val = read_volatile(registers::PORTD);
        write_volatile(PORTD, portd_val & !pin);
    }
}

pub fn pin_toggle(pin: u8) {
    unsafe {
        let portd_val = read_volatile(PORTD);
        write_volatile(PORTD, portd_val ^ pin);
    }
}

pub fn pin_on(pin: u8) {
    unsafe {
        let portd_val = read_volatile(PORTD);
        write_volatile(PORTD, portd_val | pin);
    }
}
