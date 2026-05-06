#![no_std]
#![no_main]

use core::{ptr::{read_volatile, write_volatile}};

use arduino_hal::{delay_ms, delay_us};
use panic_halt as _;

const DDRD : *mut u8 = 0x2A as *mut u8; // PORT D DIRECTION REGISTER
const PIND : *mut u8 = 0x29 as *mut u8; // PORT D INPUT DATA REGISTER
const PORTD : *mut u8 = 0x2B as *mut u8; // PORT D OUTPUT DATA REGISTER
const TIFR0 : *mut u8 = 0x35 as *mut u8; // TIMER COUNTER 0 INTERRUPT FLAG REGISTER
const OCR0A : *mut u8 = 0x47 as *mut u8; // OUTPUT COMPARE REIGSTER A

const OC0AS_BIT : u8 = 1 << 5;

const TCCR0A : *mut u8 = 0x44 as *mut u8; // TC0 CONTROL REGISTER A
const COM0B1_BIT : u8 = 1 << 5;
const WGM00_BIT : u8 = 1 << 0;
const WGM01_BIT : u8 = 1 << 1;

const TCCR0B : *mut u8 = 0x45 as *mut u8; // TC0 CONTROL REGISTER B
const WGM02_BIT : u8 = 1 << 3;

const PMX0 : *mut u8 = 0xEE as *mut u8; // Port Multiplexed Control Register
const C0AC0_BIT : u8 = 1 << 3;

const OCR0B : *mut u8 = 0x48 as *mut u8; // TC0 Output compare register B

const COM0A1_BIT : u8 = 1 << 7;
const PIN5 : u8 = 1 << 5;
const PIN6 : u8 = 1 << 6;
const TOV0_BIT : u8 = 1 << 0;
const CS00_BIT : u8 = 1 << 0;
const CS01_BIT : u8 = 1 << 1;

const TCNT0 : *mut u8 = 0x46 as *mut u8; // TC0 count value register

// 32.768KHz
// f oc0xfpwm = f sys / (N * (1 + TOP))
// Where N represents the prescaler factor (1, 8, 64, 256, or 1024)

// Clock = 32.768KHz
// 1.25 uS = 41 cycles
// 0.8 uS = 26 cycles
// 0.4 uS = 13 cycles

const PWM_VAL_0: u8 = 0; // ~0.4us high
const PWM_VAL_1: u8 = 4; // ~0.8us high

fn pin_off(pin : u8) {
    unsafe {
        let mut portd_val = read_volatile(PORTD);
        write_volatile(PORTD, portd_val & !pin);
    }
}

fn pin_toggle(pin : u8) {
    unsafe {
        let portd_val = read_volatile(PORTD);
        write_volatile(PORTD, portd_val ^ pin);
    }
}

fn pin_on(pin : u8) {
    unsafe {    
        let portd_val = read_volatile(PORTD);
        write_volatile(PORTD, portd_val | pin);
    }
}

struct WS2812B<const BUF_SIZE: usize> {
    buf : [u8; BUF_SIZE]
}

impl<const BUF_SIZE : usize> WS2812B<BUF_SIZE> {
    fn new() -> Self {
        WS2812B {buf : [PWM_VAL_0; BUF_SIZE]}
    }

    fn set_led(&mut self, index: usize, r : u8, g : u8, b : u8) {
        let bits = ((g as u32) << 16) | ((r as u32) << 8) | (b as u32);
        for i in 0..23 {
            let bit = (bits >> (23 - i)) & 1; 
            if bit == 1 {
                self.buf[index * 24 + i] = PWM_VAL_1;
            } else {
                self.buf[index * 24 + i] = PWM_VAL_0;
            }
        }
    }

    fn update(&self) {
        unsafe {
            write_volatile(PORTD, read_volatile(PORTD) & !PIN5);
            write_volatile(TCCR0A, read_volatile(TCCR0A) & !COM0B1_BIT);
            write_volatile(OCR0A, 39);

            write_volatile(OCR0B, PWM_VAL_0);

            write_volatile(TCNT0, 0);

            write_volatile(TIFR0, TOV0_BIT); 
            write_volatile(TCCR0A, read_volatile(TCCR0A) | COM0B1_BIT);

            for i in 0..24*64 {
                let val = self.buf[i];

                write_volatile(OCR0B, val);
                while read_volatile(TIFR0) & TOV0_BIT == 0 {}
                write_volatile(TIFR0, TOV0_BIT);
            }
        }
    }

    fn reset(&self) {
        unsafe {
            while read_volatile(TIFR0) & TOV0_BIT == 0 {}

            let mut tccr = read_volatile(TCCR0A);
            write_volatile(TCCR0A, tccr & !COM0B1_BIT);

            let mut portd = read_volatile(PORTD);
            write_volatile(PORTD, portd & !PIN5);
        }
        
        delay_us(60);
    }
}

fn setup_gpio() {
    unsafe {
    let mut ddrd_val = read_volatile(DDRD);
    ddrd_val |= PIN6;
    ddrd_val |= PIN5;
    write_volatile(DDRD, ddrd_val);

    let tccr0a_val = WGM01_BIT | WGM00_BIT;
    write_volatile(TCCR0A, tccr0a_val);

    let mut tccr0b_val = CS00_BIT | WGM02_BIT;
    tccr0b_val = tccr0b_val & !OC0AS_BIT;
    write_volatile(TCCR0B, tccr0b_val);

    write_volatile(OCR0A, 39);
    }
}

#[macro_export]
macro_rules! new_ws2812b {
    ($num_leds: expr) => {
        WS2812B::<{$num_leds * 24}>::new();
    };
}

#[arduino_hal::entry]
fn main() -> ! {
    static mut TEST : usize = 0;

    setup_gpio();

    let mut ws2812b = WS2812B::<1536>::new();
    loop {
        for i in 0..64 {
            let offset = *TEST % 2;
            if (i + offset) % 2 == 0 {
                ws2812b.set_led(i, 20, 0, 0);
            } else {
                ws2812b.set_led(i, 0, 0, 20);
            }
        }

        ws2812b.update();
        ws2812b.reset();
        
        delay_ms(50);

        pin_toggle(PIN6);
        
        *TEST += 1;
    }
}
