//! This example toggles RP Pico/W GPIO  pin 0--connect a resistor and LED and it will blink..

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Configure GPIO pin 0 as an output
    let mut led = Output::new(p.PIN_0, Level::Low);

    loop {
        // Light up the LED
        led.set_high();

        // Wait for 1 second
        Timer::after_secs(1).await;

        // Turn off the LED
        led.set_low();

        // Wait for 1 second
        Timer::after_secs(1).await;

    // Loop forever
    }
}
