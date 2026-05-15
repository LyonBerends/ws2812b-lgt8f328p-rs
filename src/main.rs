#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::{
    ptr::{read_volatile, write_volatile},
};

use arduino_hal::{delay_ms};
use panic_halt as _;
use ws2812b::{animations::{animation_one, animation_two}, lgt8f328p::{pin_toggle, registers::{self, PIN6}}, ws2812b::WS2812B};

fn setup_gpio() {
    unsafe {
        let mut ddrd_val = read_volatile(registers::DDRD);
        ddrd_val |= registers::PIN6;
        ddrd_val |= registers::PIN5;
        write_volatile(registers::DDRD, ddrd_val);

        let tccr0a_val = registers::WGM01_BIT | registers::WGM00_BIT;
        write_volatile(registers::TCCR0A, tccr0a_val);

        let mut tccr0b_val = registers::CS00_BIT | registers::WGM02_BIT;
        tccr0b_val = tccr0b_val & !registers::OC0AS_BIT;
        write_volatile(registers::TCCR0B, tccr0b_val);

        
        write_volatile(registers::OCR0A, 39);
    }
}


const SEGMENT_SIZE: usize = 8;
pub const LEDS_COUNT: usize = 64;

#[arduino_hal::entry]
fn main() -> ! {
    let mut counter: usize = 0;

    setup_gpio();

    let mut leds = WS2812B::<LEDS_COUNT>::new();

    loop {
        animation_two(&mut counter, &mut leds, SEGMENT_SIZE);
        leds.update();
        leds.reset();

        pin_toggle(PIN6);
        delay_ms(20);

        counter += 1;
    }
}
