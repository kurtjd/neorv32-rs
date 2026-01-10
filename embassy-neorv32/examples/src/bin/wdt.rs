#![no_std]
#![no_main]

use core::fmt::Write;
use embassy_neorv32::uart::UartTx;
use embassy_neorv32::wdt::{ResetCause, Wdt};
use embassy_neorv32_examples::*;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART just for printing WDT state
    let mut uart = UartTx::new_blocking(p.UART0, UART_BAUD, UART_IS_SIM, false)
        .expect("UART must be supported");

    // Setup WDT with timeout of 1ms and enable it then lock it
    let wdt = Wdt::new(p.WDT).expect("WDT must be supported");
    wdt.set_timeout_ms(1);
    wdt.enable();
    let wdt = wdt.lock();

    // Print the last reset cause
    let reset_cause = wdt.reset_cause();
    writeln!(&mut uart, "Last hardware reset cause: {:?}", reset_cause).unwrap();

    // On first reset, let's see if illegal access triggers a reset
    if matches!(reset_cause, ResetCause::External) {
        uart.blocking_write(b"Forcing HW reset...\n");
        wdt.force_hw_reset();
    }

    // On subsequent resets we feed a few times then wait for timeout reset to trigger
    for _ in 0..5 {
        uart.blocking_write(b"Feeding watchdog...\n");
        wdt.feed();
        Timer::after_micros(ms_to_us(1)).await;
    }
    uart.blocking_write(b"Waiting for watchdog timeout...\n");
}
