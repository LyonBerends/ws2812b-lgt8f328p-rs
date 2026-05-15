use crate::ws2812b::{LED, WS2812B};

pub fn animation_one<const LEDS_COUNT : usize>(counter: &mut usize, leds: &mut WS2812B<LEDS_COUNT>, segment_size : usize) {
    leds.leds = [0; LEDS_COUNT];

    for i in 0..leds.leds.len() {
        let j = i % segment_size;

        let mut distance = (j as i32) - (segment_size as i32) / 2;
        if distance < 0 {
            distance = distance * -1
        };

        let distance = distance as usize;

        let led_state_1: LED = LED { r: 50, g: 0, b: 0 };
        let led_state_2: LED = LED { r: 50, g: 0, b: 50 };

        let counter_mod = *counter % (segment_size * 2);

        let led: LED;
        if counter_mod < segment_size {
            led = if distance <= counter_mod % segment_size {
                led_state_1
            } else {
                led_state_2
            };
        } else {
            led = if distance >= counter_mod % segment_size {
                led_state_1
            } else {
                led_state_2
            };
        }

        leds.set_led(i, led.into());
    }
}
