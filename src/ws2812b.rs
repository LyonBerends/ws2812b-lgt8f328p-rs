use core::{arch::asm, ptr::{read_volatile, write_volatile}};

use arduino_hal::delay_us;

use crate::lgt8f328p::registers::*;

const PWM_VAL_0: u8 = 0; // ~0.4us high
const PWM_VAL_1: u8 = 6; // ~0.8us high

pub struct LED {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Into<u32> for LED {
    fn into(self) -> u32 {
        ((self.g as u32) << 16) | ((self.r as u32) << 8) | (self.b as u32)
    }
}


pub struct WS2812B<const LED_COUNT: usize> {
    leds: [u32; LED_COUNT],
}

impl<const LED_COUNT: usize> WS2812B<LED_COUNT> {
    pub fn new() -> Self {
        WS2812B {
            leds: [0; LED_COUNT],
        }
    }

    pub fn set_led(&mut self, index: usize, value: u32) {
        self.leds[index] = value;
    }

    pub fn update(&self) {
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

    pub fn reset(&self) {
        unsafe {
            while read_volatile(TIFR0) & TOV0_BIT == 0 {}

            let tccr = read_volatile(TCCR0A);
            write_volatile(TCCR0A, tccr & !COM0B1_BIT);

            let portd = read_volatile(PORTD);
            write_volatile(PORTD, portd & !PIN5);
        }
        delay_us(80);
    }

    pub fn clear(&mut self) {
        self.leds = [0; LED_COUNT];
    }

    pub fn leds_len(&self) -> usize {
        self.leds.len()
    }
}
