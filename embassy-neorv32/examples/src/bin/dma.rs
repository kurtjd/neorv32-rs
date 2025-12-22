#![no_std]
#![no_main]

use embassy_neorv32::bind_interrupts;
use embassy_neorv32::dma::{self, Dma};
use embassy_neorv32::peripherals;
use embassy_neorv32::uart::{self, UartTx};
use embassy_neorv32_examples::*;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::once_lock::OnceLock;
use embassy_time::Timer;

bind_interrupts!(struct Irqs {
    DMA => dma::InterruptHandler<peripherals::DMA>;
    UART0 => uart::InterruptHandler<peripherals::UART0>;
});

type UartMutex = Mutex<CriticalSectionRawMutex, UartTx<'static, uart::Async>>;
static UART: OnceLock<UartMutex> = OnceLock::new();

#[embassy_executor::task]
async fn dma_transfer(mut dma: Dma<'static>, uart: &'static UartMutex) {
    loop {
        let src = [42u8; 1024];
        let mut dst = [69u8; 1024];

        uart.lock().await.write(b"DMA transfer started..\n").await;
        let res = dma.copy(&src, &mut dst, false).await;
        {
            let mut uart = uart.lock().await;
            if res.is_ok() && src.iter().last() == dst.iter().last() {
                uart.write(b"DMA transfer complete\n").await;
            } else {
                uart.write(b"DMA transfer failed\n").await;
            }
        }

        Timer::after_micros(s_to_us(2)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART just for printing WDT state
    let uart = UartTx::new_async(p.UART0, UART_BAUD, UART_IS_SIM, false, Irqs);
    let uart = UART.get_or_init(|| Mutex::new(uart));

    // Setup DMA
    let dma = Dma::new(p.DMA, Irqs);
    spawner.must_spawn(dma_transfer(dma, uart));

    loop {
        uart.lock().await.write(b"Doing some work...\n").await;
        Timer::after_micros(s_to_us(1)).await;
    }
}
