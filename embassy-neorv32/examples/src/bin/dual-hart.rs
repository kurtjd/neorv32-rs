//! To run this example, use:
//! `cargo run-dh-sim --release --bin dual-hart`
#![no_std]
#![no_main]

#[cfg(not(feature = "dual-hart"))]
compile_error!("The `dual-hart` feature must be supported.");

use core::fmt::Write;
use embassy_neorv32::dual_hart;
use embassy_neorv32::trng::{self, Trng};
use embassy_neorv32::uart::{self, UartTx};
use embassy_neorv32::{bind_interrupts, peripherals};
use embassy_neorv32_examples::*;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use embassy_sync::once_lock::OnceLock;
use embassy_time::Timer;

bind_interrupts!(struct Irqs {
    TRNG => trng::InterruptHandler<peripherals::TRNG>;
    UART0 => uart::InterruptHandler<peripherals::UART0>;
});

type SharedUart = Mutex<CriticalSectionRawMutex, UartTx<'static, uart::Async>>;

static UART: OnceLock<SharedUart> = OnceLock::new();
static RNG: Channel<CriticalSectionRawMutex, u32, 1> = Channel::new();

#[embassy_executor::task]
async fn hart0_task(uart: &'static SharedUart) {
    assert_eq!(riscv::register::mhartid::read(), 0);

    loop {
        let rng = RNG.receive().await;
        writeln!(&mut uart.lock().await, "Hart 0: Received RNG: 0x{rng:04X}").unwrap();
    }
}

#[embassy_executor::task]
async fn hart1_task(uart: &'static SharedUart, mut trng: Trng<'static, trng::Async>) {
    assert_eq!(riscv::register::mhartid::read(), 1);

    uart.lock()
        .await
        .write(b"Hello from hart 1!\n")
        .await
        .unwrap();
    loop {
        let mut buf = [0; 4];
        trng.read(&mut buf).await;
        RNG.send(u32::from_be_bytes(buf)).await;
        uart.lock()
            .await
            .write(b"Hart 1: Sent RNG\n")
            .await
            .unwrap();
        Timer::after_micros(ms_to_us(100)).await;
    }
}

// Dual-hart support requires a custom Embassy executor (provided by the HAL), so we just need to use it here
// We have to then also explicitly state we want to use the riscv-rt entry
#[embassy_executor::main(executor = "dual_hart::executor::Executor", entry = "riscv_rt::entry")]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    let uart = UartTx::new_async(p.UART0, UART_BAUD, UART_IS_SIM, false, Irqs)
        .expect("UART must be supported");
    let uart = UART.get_or_init(|| Mutex::new(uart));
    uart.lock()
        .await
        .write(b"Hello from hart 0!\n")
        .await
        .unwrap();

    dual_hart::hart1_start_with_executor(p.HART1, |spawner| {
        let trng = Trng::new_async(p.TRNG, Irqs).expect("TRNG must be supported");
        spawner.must_spawn(hart1_task(uart, trng));
    });

    spawner.must_spawn(hart0_task(uart));
}
