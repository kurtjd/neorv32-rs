#![no_std]
#![no_main]

#[cfg(feature = "sim")]
compile_error!("SPI example not available in simulation.");

use embassy_neorv32::bind_interrupts;
use embassy_neorv32::dma::{self, Dma};
use embassy_neorv32::gpio::Gpio;
use embassy_neorv32::peripherals;
use embassy_neorv32::spi::{self, MODE_0, Spi};
use embassy_neorv32_examples as _;
use embassy_time::{Delay, Timer};
use embedded_hal_bus::spi::ExclusiveDevice;
use is31fl3743b_driver::{CSy, Is31fl3743b, SWx};

bind_interrupts!(struct Irqs {
    DMA => dma::InterruptHandler<peripherals::DMA>;
    SPI => spi::InterruptHandler<peripherals::SPI>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup DMA for SPI transfers
    let dma = Dma::new(p.DMA, Irqs);

    // Initialize SPI driver and give it the DMA controller
    let mut spi = Spi::new_async(p.SPI, 1_000_000, MODE_0, Irqs);
    spi.give_dma(dma);

    // Setup CS pin and create an exclusive SPI device with it
    let cs = Gpio::new_blocking(p.GPIO).new_output(p.PORT7);
    let spi_dev = ExclusiveDevice::new(spi, cs, Delay).unwrap();

    // Initialize led matrix controller driver
    let mut driver = Is31fl3743b::new(spi_dev).await.unwrap();
    driver.enable_phase_delay().await.unwrap();
    driver.set_global_current(50).await.unwrap();

    // Set peak current of all LEDs
    let mut buf = [100_u8; 11 * 18];
    driver
        .set_led_peak_current_bulk(SWx::SW1, CSy::CS1, &buf)
        .await
        .unwrap();

    // Create a white breathing effect
    loop {
        for brightness in (0..=255_u8).chain((0..=255).rev()) {
            buf.fill(brightness);
            driver
                .set_led_brightness_bulk(SWx::SW1, CSy::CS1, &buf)
                .await
                .unwrap();
            Timer::after_micros(1).await;
        }
    }
}
