#![no_std]
#![no_main]
use core::fmt::Write;
use embassy_neorv32::dualcore;
use embassy_neorv32::trng::{self, Trng};
use embassy_neorv32::uart::{self, UartTx};
use embassy_neorv32::{bind_interrupts, peripherals};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use embassy_sync::once_lock::OnceLock;
use embassy_time::Timer;

#[cfg(not(feature = "dual-core"))]
compile_error!("The `dual-core` feature must be enabled.");

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

    uart.lock().await.write(b"Hello from hart 1!\n").await;
    loop {
        let mut buf = [0; 4];
        trng.read(&mut buf).await;
        RNG.send(u32::from_be_bytes(buf)).await;
        uart.lock().await.write(b"Hart 1: Sent RNG\n").await;
        Timer::after_micros(100).await;
    }
}

#[embassy_executor::main(executor = "dualcore::executor::Executor", entry = "riscv_rt::entry")]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    let uart = UartTx::new_async(p.UART0, 19200, true, false, Irqs);
    let uart = UART.get_or_init(|| Mutex::new(uart));
    uart.lock().await.write(b"Hello from hart 0!\n").await;

    dualcore::hart1_start_with_executor(p.HART1, |spawner| {
        let trng = Trng::new_async(p.TRNG, Irqs);
        spawner.must_spawn(hart1_task(uart, trng));
    });

    spawner.must_spawn(hart0_task(uart));
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let hart = riscv::register::mhartid::read();
    let p = unsafe { embassy_neorv32::Peripherals::steal() };
    let mut uart = UartTx::new_blocking(p.UART0, 19200, true, false);

    writeln!(
        &mut uart,
        "\n\nHART {} PANIC: {} at {}",
        hart,
        info.message(),
        info.location().unwrap()
    )
    .unwrap();

    loop {
        riscv::asm::wfi();
    }
}
