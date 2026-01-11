#![no_std]
#![no_main]

use embassy_neorv32::bind_interrupts;
use embassy_neorv32::dma::{self, Dma};
use embassy_neorv32::peripherals;
use embassy_neorv32::uart::UartTx;
use embassy_neorv32_examples::*;

bind_interrupts!(struct Irqs {
    DMA => dma::InterruptHandler<peripherals::DMA>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    let mut uart = UartTx::new_blocking(p.UART0, UART_BAUD, UART_IS_SIM, false)
        .expect("UART must be supported");

    let mut dma = Dma::new(p.DMA, Irqs).expect("DMA must be supported");

    let src = [0xAAu8; 1024];
    let mut dst = [0xFFu8; 1024];

    let res = dma.copy(&src, &mut dst, false).await;
    match res {
        Ok(()) if src == dst => uart.blocking_write(b"DMA transfer succeeded\n"),
        Err(_) => uart.blocking_write(b"DMA transfer encountered an error\n"),
        _ => uart.blocking_write(b"DMA transfer failed\n"),
    }
}
