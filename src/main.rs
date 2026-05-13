#![no_std]
#![feature(asm_experimental_arch)]
#![no_main]

use core::{arch::asm, ptr::{read_volatile, write_volatile}};

use arduino_hal::{delay_ms, delay_us};
use panic_halt as _;

const DDRD : *mut u8 = 0x2A as *mut u8; // PORT D DIRECTION REGISTER
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

const OCR0B : *mut u8 = 0x48 as *mut u8; // TC0 Output compare register B

const PIN5 : u8 = 1 << 5;
const PIN6 : u8 = 1 << 6;
const TOV0_BIT : u8 = 1 << 0;
const CS00_BIT : u8 = 1 << 0;

const TCNT0 : *mut u8 = 0x46 as *mut u8; // TC0 count value register

// 32.768KHz
// f oc0xfpwm = f sys / (N * (1 + TOP))
// Where N represents the prescaler factor (1, 8, 64, 256, or 1024)

// Clock = 32.768KHz
// 1.25 uS = 41 cycles
// 0.8 uS = 26 cycles
// 0.4 uS = 13 cycles

const PWM_VAL_0: u8 = 0; // ~0.4us high
const PWM_VAL_1: u8 = 6; // ~0.8us high

fn pin_off(pin : u8) {
    unsafe {
        let portd_val = read_volatile(PORTD);
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

struct WS2812B<const LED_COUNT : usize> {
    leds : [u32; LED_COUNT],
}

struct LED {
    r : u8,
    g : u8,
    b : u8
}

impl Into<u32> for LED {
    fn into(self) -> u32 {
        ((self.g as u32) << 16) | ((self.r as u32) << 8) | (self.b as u32)
    }
}
impl <const LED_COUNT : usize> WS2812B<LED_COUNT> {
    fn new() -> Self {
        WS2812B {leds : [0; LED_COUNT]}
    }

    fn set_led(&mut self, index : usize, value : u32) {
        self.leds[index] = value;
    }

    fn update(&self) {
        let pwm_vals = [PWM_VAL_0, PWM_VAL_1];
        unsafe {
            asm!("cli");
            write_volatile(PORTD, read_volatile(PORTD) & !PIN5);
            write_volatile(TCCR0A, read_volatile(TCCR0A) & !COM0B1_BIT);
            write_volatile(OCR0A, 43); // TOP VALUE

            write_volatile(TCNT0, 0);

            write_volatile(TIFR0, TOV0_BIT); 

            write_volatile(TCCR0A, read_volatile(TCCR0A) | COM0B1_BIT);
            for i in 0..LED_COUNT {
                let mut val = self.leds[i];
                for _ in 0..24 {
                    let pwm_val = (val & 0b10000000000000000000000 != 0) as usize;
                    val <<= 1;
                    
                    while read_volatile(TIFR0) & TOV0_BIT == 0 {}
                    write_volatile(TIFR0, TOV0_BIT);

                    write_volatile(OCR0B, pwm_vals[pwm_val]);

                }
            }

            asm!("sei");
        }
    }

    fn reset(&self) {
        unsafe {
            while read_volatile(TIFR0) & TOV0_BIT == 0 {}

            let tccr = read_volatile(TCCR0A);
            write_volatile(TCCR0A, tccr & !COM0B1_BIT);

            let portd = read_volatile(PORTD);
            write_volatile(PORTD, portd & !PIN5);
        }
        delay_us(80);
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

const LEDS_COUNT : usize = 64;

#[arduino_hal::entry]
fn main() -> ! {
    let mut counter : usize = 0;

    setup_gpio();

    let mut leds  = WS2812B::<LEDS_COUNT>::new();

    let mut counter = 0;
    loop {
        let led : LED;
        leds.leds = [0; LEDS_COUNT];
        // for i in 0..LEDS_COUNT {
        //     leds.leds[i] = 0;
        // }
        if counter < LEDS_COUNT {
            led = LED { r: 0b10, g: 0, b: 0 };
        }
        else if counter < LEDS_COUNT * 2 {
            led = LED { r: 0, g: 0b10, b: 0 };
        } else if counter < LEDS_COUNT * 3 {
            led = LED { r: 0, g: 0, b: 0b10 };
        } else {
            led = LED { r: 0b10, g: 0, b: 0 };
            counter = 0;
        }
        leds.set_led(counter % LEDS_COUNT, led.into());
        leds.update();
        leds.reset();


        pin_toggle(PIN6);
        delay_ms(10);

        counter += 1;
    }
}
