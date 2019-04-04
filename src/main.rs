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

mod colors;

#[allow(unused_imports)]
use panic_halt;
use trellis_m4 as hal;
use ws2812_nop_samd51 as ws2812;

use hal::adxl343::accelerometer;
use hal::prelude::*;
use hal::{clock::GenericClockController, delay::Delay, entry, CorePeripherals, Peripherals};
use smart_leds::{Color, SmartLedsWrite};

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

    let mut pins = hal::Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core_peripherals.SYST, &mut clocks);

    // neopixels
    let neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixel = ws2812::Ws2812::new(neopixel_pin);
    let mut pixels = [Color::default(); NUM_LEDS];

    for pixel in pixels.iter_mut().take(8) {
        *pixel = colors::WHITE;
    }

    for pixel in pixels.iter_mut().take(16).skip(8) {
        *pixel = colors::YELLOW;
    }

    for pixel in pixels.iter_mut().take(24).skip(16) {
        *pixel = colors::ORANGE;
    }

    for pixel in pixels.iter_mut().skip(24) {
        *pixel = colors::RED;
    }

    // accelerometer
    let adxl343 = pins
        .accel
        .open(
            &mut clocks,
            peripherals.SERCOM2,
            &mut peripherals.MCLK,
            &mut pins.port,
        )
        .unwrap();

    let mut accel_tracker = adxl343.try_into_tracker().unwrap();
    let mut reversed = false;

    loop {
        match accel_tracker.orientation().unwrap() {
            accelerometer::Orientation::LandscapeUp => reversed = false,
            accelerometer::Orientation::LandscapeDown => reversed = true,
            _ => (),
        }

        if reversed {
            neopixel.write(pixels.iter().rev().cloned()).unwrap();
        } else {
            neopixel.write(pixels.iter().cloned()).unwrap();
        }

        delay.delay_ms(1u8);
    }
}
