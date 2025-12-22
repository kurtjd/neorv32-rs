#![no_std]
#![no_main]
use embassy_neorv32::bind_interrupts;
use embassy_neorv32::gpio::{self, Gpio};
use embassy_neorv32::peripherals;
use embassy_neorv32::uart::{self, UartTx};
use embassy_neorv32_examples::*;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::once_lock::OnceLock;
use embassy_time::Timer;

type UartMutex = Mutex<CriticalSectionRawMutex, UartTx<'static, uart::Async>>;

bind_interrupts!(struct Irqs {
    GPIO => gpio::InterruptHandler<peripherals::GPIO>;
    UART0 => uart::InterruptHandler<peripherals::UART0>;
});

#[embassy_executor::task(pool_size = 2)]
async fn input_task(
    mut input_pin: gpio::Input<'static, gpio::Async>,
    uart: &'static UartMutex,
    msg: &'static [u8],
) {
    loop {
        input_pin.wait_for_any_edge().await;
        uart.lock().await.write(msg).await;
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn output_task(mut output_pin: gpio::Output<'static>, ms: u64) {
    loop {
        output_pin.toggle();
        Timer::after_micros(ms_to_us(ms)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    static UART: OnceLock<UartMutex> = OnceLock::new();
    let uart = UartTx::new_async(p.UART0, UART_BAUD, UART_IS_SIM, false, Irqs);
    let uart = UART.get_or_init(|| Mutex::new(uart));

    let gpio = Gpio::new_async(p.GPIO, Irqs);

    // Port 0
    let (input0, output0) = gpio.new_port(p.PORT0).split();
    spawner.must_spawn(input_task(input0, uart, b"Switch 0 toggled!\n"));
    spawner.must_spawn(output_task(output0, 1000));

    // Port 1
    let (input1, output1) = gpio.new_port(p.PORT1).split();
    spawner.must_spawn(input_task(input1, uart, b"Switch 1 toggled!\n"));
    spawner.must_spawn(output_task(output1, 200));
}
