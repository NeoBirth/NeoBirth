//! Acid house music program for the [Adafruit NeoTrellis M4], powered by the
//! [PureZen] music synthesis engine (a [Pure Data] engine targeting embedded
//! Rust devices).
//!
//! [Adafruit NeoTrellis M4]: https://learn.adafruit.com/adafruit-neotrellis-m4
//! [PureZen]: https://github.com/NeoBirth/PureZen
//! [Pure Data]: https://puredata.info/

#![no_std]
#![no_main]
#![deny(
    warnings,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]

extern crate panic_halt;
extern crate trellis_m4 as hal;
extern crate ws2812_nop_samd51 as ws2812;

use hal::prelude::*;
use hal::{clock::GenericClockController, delay::Delay, entry, CorePeripherals, Peripherals};
use smart_leds::{brightness, colors, Color, SmartLedsWrite};

/// Total number of LEDs on the NeoTrellis M4
const NUM_LEDS: usize = 32;

/// Main entrypoint
#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core_peripherals = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core_peripherals.SYST, &mut clocks);

    let hal::pins::Sets {
        neopixel,
        keypad: keypad_pins,
        mut port,
        ..
    } = hal::Pins::new(peripherals.PORT).split();

    // neopixels
    let neopixel_pin = neopixel.into_push_pull_output(&mut port);
    let mut neopixel = ws2812::Ws2812::new(neopixel_pin);
    let mut color_values = [Color::default(); NUM_LEDS];

    // keypad
    let keypad = hal::Keypad::new(keypad_pins, &mut port);
    let keypad_inputs = keypad.decompose();
    let mut keypad_state = [false; NUM_LEDS];
    let mut toggle_values = [false; NUM_LEDS];

    loop {
        for j in 0..(256 * 5) {
            for (i, value) in color_values.iter_mut().enumerate() {
                let keypad_column = i % 8;
                let keypad_row = i / 8;
                let keypad_button = &keypad_inputs[keypad_row][keypad_column];

                if keypad_button.is_high() {
                    keypad_state[i] = true;
                } else {
                    // toggle event
                    if keypad_state[i] {
                        keypad_state[i] = false;
                        toggle_values[i] = !toggle_values[i];
                    }
                }

                *value = if toggle_values[i] {
                    wheel((((i * 256) as u16 / NUM_LEDS as u16 + j) & 255) as u8)
                } else {
                    colors::DEEP_SKY_BLUE
                };
            }

            neopixel
                .write(brightness(color_values.iter().cloned(), 32))
                .unwrap();

            delay.delay_ms(5u8);
        }
    }
}

/// Color wheel
fn wheel(mut wheel_pos: u8) -> Color {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
