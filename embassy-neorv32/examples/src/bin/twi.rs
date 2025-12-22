#![no_std]
#![no_main]
use core::fmt::Write;
use embassy_neorv32::twi::Twi;
use embassy_neorv32::uart::UartTx;
use embassy_neorv32_examples::*;
use embassy_time::Timer;
use lm75::{Address, Lm75};

// I've tied my LM75 addr pins all to ground
// Change this depending on your configuration
const LM75_ALL_GND: u8 = 0b10011110;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = UartTx::new_blocking(p.UART0, UART_BAUD, UART_IS_SIM, false);

    // Setup TWI with frequency of 100 kHz and clock stretching enabled
    let twi = Twi::new_blocking(p.TWI, 100_000, true);

    // Setup and enable LM75 driver
    let address = Address::from(LM75_ALL_GND);
    let mut sensor = Lm75::new(twi, address);
    if sensor.enable().is_err() {
        uart.blocking_write(b"Error enabling LM75\n");
        panic!()
    }

    // Periodically read temperature from LM75 over TWI
    loop {
        if let Ok(temp) = sensor.read_temperature() {
            writeln!(&mut uart, "Temperature: {} ºC", temp).unwrap();
        } else {
            uart.blocking_write(b"Error reading from LM75\n");
        }
        Timer::after_micros(s_to_us(1)).await;
    }
}
