#![no_std]
#![no_main]
use core::fmt::Write;
use embassy_neorv32::bind_interrupts;
use embassy_neorv32::peripherals;
use embassy_neorv32::trng::{self, Trng};
use embassy_neorv32::uart::{self, UartTx};
use embassy_neorv32_examples::*;
use embassy_time::Timer;

bind_interrupts!(struct Irqs {
    TRNG => trng::InterruptHandler<peripherals::TRNG>;
    UART0 => uart::InterruptHandler<peripherals::UART0>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = UartTx::new_async(p.UART0, UART_BAUD, UART_IS_SIM, false, Irqs);

    // Setup async TRNG
    let mut trng = Trng::new_async(p.TRNG, Irqs);
    if trng.sim_mode() {
        uart.write(b"Running in simulation so PRNG is used\n").await;
    }

    loop {
        // Make buffer slightly large to ensure interrupt is triggered
        let mut buf = [0; 32];
        trng.read(&mut buf).await;

        // BE vs LE arbitrary here
        let word1 = u32::from_be_bytes(buf[0..4].try_into().unwrap());
        let word2 = u32::from_be_bytes(buf[4..8].try_into().unwrap());

        writeln!(&mut uart, "Random word1: 0x{word1:08X}").unwrap();
        writeln!(&mut uart, "Random word2: 0x{word2:08X}").unwrap();
        Timer::after_micros(s_to_us(1)).await;
    }
}
