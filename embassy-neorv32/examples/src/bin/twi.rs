#![no_std]
#![no_main]

#[cfg(feature = "sim")]
compile_error!("TWI example not available in simulation.");

use core::fmt::Write;
use embassy_neorv32::twi::Twi;
use embassy_neorv32::uart::UartTx;
use embassy_neorv32_examples::*;
use embassy_time::Timer;
use tmp108::Tmp108;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = UartTx::new_blocking(p.UART0, UART_BAUD, UART_IS_SIM, false);

    // Setup TWI with frequency of 100 kHz and clock stretching enabled
    // **Note**: For hardware reasons, unfortuantely an async TWI driver is not available
    let twi = Twi::new_blocking(p.TWI, 100_000, true);

    // Setup and enable TMP108 driver
    // Note: The constructor changes depending on your A0 config
    let mut sensor = Tmp108::new_with_a0_gnd(twi);

    // Periodically read temperature from TMP108 over TWI
    loop {
        match sensor.temperature() {
            Ok(temp) => writeln!(&mut uart, "Temperature: {} ºC", temp).unwrap(),
            Err(e) => writeln!(&mut uart, "TMP108 error: {e:?}").unwrap(),
        }

        Timer::after_secs(1).await;
    }
}
