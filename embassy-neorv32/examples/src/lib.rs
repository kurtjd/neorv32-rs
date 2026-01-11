#![no_std]

#[cfg(all(feature = "sim", feature = "fpga"))]
compile_error!("Only one of `sim` or `fpga` features must be enabled.");

#[cfg(not(any(feature = "sim", feature = "fpga")))]
compile_error!("At least one of `sim` or `fpga` features must be enabled.");

#[cfg(all(feature = "single-hart", feature = "dual-hart"))]
compile_error!("Only one of `single-hart` or `dual-hart` features must be enabled.");

#[cfg(not(any(feature = "single-hart", feature = "dual-hart")))]
compile_error!("At least one of `single-hart` or `dual-hart` features must be enabled.");

/// Baud rate UART host expects.
pub const UART_BAUD: u32 = 19200;

/// Represents if the UART peripheral should enter simulation mode.
#[cfg(feature = "sim")]
pub const UART_IS_SIM: bool = true;
#[cfg(feature = "fpga")]
pub const UART_IS_SIM: bool = false;

/// Time is much slower in simulation so this is just a rough scaling to try and get simulation
/// to match our perception.
///
/// Revisit: This made sense in earlier versions of NEORV32 but as of v1.12.6 simulation output
/// doesn't seem to flush at all until stop-time is reached. Might be a regression, so keep this
/// in the case it gets fixed.
#[cfg(feature = "sim")]
pub const US_PER_SEC: u64 = 50;
#[cfg(feature = "fpga")]
pub const US_PER_SEC: u64 = 1000000;

/// Helper for examples to convert miliseconds to microseconds.
pub fn ms_to_us(ms: u64) -> u64 {
    ((ms * US_PER_SEC) / 1000).max(1)
}

/// Helper for examples to convert seconds to microseconds.
pub fn s_to_us(s: u64) -> u64 {
    s * US_PER_SEC
}

// A helpful custom panic handler for printing panic message over UART
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;

    let hart = riscv::register::mhartid::read();
    // SAFETY: Don't have a choice if we want to display the panic message,
    // but worst that can happen is the UART output gets corrupted
    let p = unsafe { embassy_neorv32::Peripherals::steal() };
    if let Ok(mut uart) =
        embassy_neorv32::uart::UartTx::new_blocking(p.UART0, UART_BAUD, UART_IS_SIM, false)
    {
        writeln!(
            &mut uart,
            "\n\nHART {} PANIC: {} at {}",
            hart,
            info.message(),
            info.location().unwrap()
        )
        .unwrap();
    }

    loop {
        riscv::asm::wfi();
    }
}
