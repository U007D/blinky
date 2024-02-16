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

    // Light up the LED

    // Wait for 1 second

    // Turn off the LED

    // Wait for 1 second

    // Loop forever
}
