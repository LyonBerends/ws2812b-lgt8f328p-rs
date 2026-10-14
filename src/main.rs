#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::ptr::{read_volatile, write_volatile};

use arduino_hal::{delay_ms};
use panic_halt as _;
use ws2812b::{animations::animation_three, lgt8f328p::{pin_read, pin_toggle, registers::{self}}, ws2812b::WS2812B};

fn setup_gpio() {
    unsafe {
        let mut ddrd_val = read_volatile(registers::DDRD);
        ddrd_val = ddrd_val & !registers::PIN7;
        ddrd_val = ddrd_val & !registers::PIN2;
        ddrd_val = ddrd_val & !registers::PIN4;
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
const CLKPR : *mut u8 = 0x61 as *mut u8;
const WCE : u8 = 1 << 7;

#[arduino_hal::entry]
fn main() -> ! {
    // let dp = arduino_hal::Peripherals::take().unwrap();
    // let pins = arduino_hal::pins!(dp);
    // let mut serial = default_serial!(dp,pins,57600);

    // unsafe {
    //     let clkpr = WCE;
    //     write_volatile(CLKPR, clkpr);
    //     let clkpr = 0b1;
    //     write_volatile(CLKPR, clkpr);
    // }

    let mut counter: usize = 0;

    setup_gpio();

    let mut leds = WS2812B::<LEDS_COUNT>::new();

    let brightness = 10;
    loop {
        let r = pin_read(registers::PIN2);
        let g = pin_read(registers::PIN7);
        let b = pin_read(registers::PIN4);
        // ufmt::uwriteln!(&mut serial, "test").unwrap_infallible();

        animation_three(&mut leds, r, g, b, brightness);
        leds.update();
        leds.reset();

        pin_toggle(registers::PIN6);
        delay_ms(10);

        counter = counter % 60 + 1;
    }
}
