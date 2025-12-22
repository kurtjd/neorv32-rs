#![no_std]
pub mod dma;
#[cfg(feature = "dual-core")]
pub mod dualcore;
pub mod gpio;
pub mod gptmr;
pub mod interrupts;
pub mod pwm;
pub mod spi;
pub mod sysinfo;
#[cfg(feature = "time-driver")]
mod time_driver;
pub mod trng;
pub mod twi;
pub mod uart;
pub mod wdt;

// Peripherals and interrupts supported by the NEORV32 chip
mod chip {
    pub use neorv32_pac as pac;
    embassy_hal_internal::peripherals!(
        HART1, CLINT, WDT, UART0, UART1, GPTMR, TRNG, DMA, GPIO, PORT0, PORT1, PORT2, PORT3, PORT4,
        PORT5, PORT6, PORT7, PORT8, PORT9, PORT10, PORT11, PORT12, PORT13, PORT14, PORT15, PORT16,
        PORT17, PORT18, PORT19, PORT20, PORT21, PORT22, PORT23, PORT24, PORT25, PORT26, PORT27,
        PORT28, PORT29, PORT30, PORT31, PWM0, PWM1, PWM2, PWM3, PWM4, PWM5, PWM6, PWM7, PWM8, PWM9,
        PWM10, PWM11, PWM12, PWM13, PWM14, PWM15, SPI, TWI,
    );
    pub mod interrupts {
        crate::interrupt_mod!(UART0, UART1, TRNG, DMA, GPIO, SPI);
    }
}

pub use chip::pac;
pub use chip::{Peripherals, interrupts::*, peripherals};

/// Initialize the HAL. This must only be called from hart 0.
///
/// # Panics
///
/// Panics if this has already been called once before or not called from hart 0.
pub fn init() -> Peripherals {
    assert_eq!(riscv::register::mhartid::read(), 0);

    // Attempt to take first so we panic before doing anything else
    let p = Peripherals::take();

    // In dual-core, global interrupts are enabled in hart_main()
    // So for single-core just enable them now
    #[cfg(feature = "single-core")]
    // SAFETY: We're not worried about breaking any critical sections here
    unsafe {
        riscv::interrupt::enable()
    }

    time_driver::init();
    p
}

/// The motivation for this macro is that due to neorv32 constraints, several peripherals need
/// to disable the peripheral interrupt in their IRQ handler for proper async behavior.
///
/// The driver then needs to re-enable the interrupt on wake. The HALs are designed to not require
/// an Instance generic as part of the struct for ergonomic purposes, so we need to explicitly list
/// the peripheral name.
///
/// Currently this is fine since the neorv32 only supports single instances of
/// the peripherals where this is used, but may need revisiting if that ever changes in the future.
macro_rules! enable_periph_irq {
    ($periph:ident) => {{ <$crate::peripherals::$periph as Instance>::Interrupt::enable() }};
}
pub(crate) use enable_periph_irq;
