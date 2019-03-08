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
extern crate ws2812_timer_delay as ws2812;

use hal::{
    clock::GenericClockController, delay::Delay, entry, prelude::*, timer::TimerCounter,
    CorePeripherals, Peripherals,
};
use smart_leds::{brightness, Color, SmartLedsWrite};

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

    let mut pins = hal::Pins::new(peripherals.PORT);
    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK);
    timer.start(3_000_000u32.hz());

    let mut neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixel = ws2812::Ws2812::new(timer, &mut neopixel_pin);
    let mut delay = Delay::new(core_peripherals.SYST, &mut clocks);
    let mut values = [Color::default(); NUM_LEDS];

    loop {
        for j in 0..(256 * 5) {
            for (i, value) in values.iter_mut().enumerate() {
                *value = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
            }

            neopixel
                .write(brightness(values.iter().cloned(), 32))
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
