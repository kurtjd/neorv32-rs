#![no_std]
#![no_main]
use embassy_neorv32::gptmr::{Gptmr, Prescaler};
use embassy_neorv32::pac;
use embassy_neorv32::uart::UartTx;
use embassy_neorv32_examples::*;

#[riscv_rt::core_interrupt(pac::interrupt::CoreInterrupt::GPTMR)]
fn gptmr_handler() {
    // SAFETY: This is the only place calling `irq_clear` and it is expected
    unsafe { Gptmr::irq_clear() }

    // SAFETY: The ISR has exclusive access to the UART by the time it gets triggered
    let uart = unsafe { &*pac::Uart0::ptr() };
    for c in b"!\n" {
        // SAFETY: We are writing a valid byte
        // Assumes FIFO >= 2
        uart.data().write(|w| unsafe { w.bits(*c as u32) });
    }
}

#[riscv_rt::entry]
fn main() -> ! {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes (unsafely write to it in ISR)
    let _ = UartTx::new_blocking(p.UART0, UART_BAUD, UART_IS_SIM, false);

    // Setup GPTMR
    // SAFETY: Enabling the GPTMR interrupt here is valid
    unsafe { riscv::interrupt::enable_interrupt(pac::interrupt::CoreInterrupt::GPTMR) }
    let mut gptmr = Gptmr::new(p.GPTMR, Prescaler::Psc64);
    gptmr.set_threshold(1_000_000);
    gptmr.enable();

    // Do nothing unless gptmr interrupt triggers
    loop {
        riscv::asm::wfi();
    }
}
