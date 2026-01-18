#![doc = include_str!("../README.md")]
#![no_std]
pub mod dma;
#[cfg(feature = "dual-hart")]
pub mod dual_hart;
pub mod gpio;
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
    #[rustfmt::skip]
    embassy_hal_internal::peripherals!(
        HART1,
        WDT,
        UART0, UART1,
        TRNG,
        DMA,
        SPI,
        TWI,
        GPIO,
        PORT0, PORT1, PORT2, PORT3, PORT4, PORT5, PORT6, PORT7,
        PORT8, PORT9, PORT10, PORT11, PORT12, PORT13, PORT14, PORT15,
        PORT16, PORT17, PORT18, PORT19, PORT20, PORT21, PORT22, PORT23,
        PORT24, PORT25, PORT26, PORT27, PORT28, PORT29, PORT30, PORT31,
        PWM,
        PWMCHAN0, PWMCHAN1, PWMCHAN2, PWMCHAN3, PWMCHAN4, PWMCHAN5, PWMCHAN6, PWMCHAN7,
        PWMCHAN8, PWMCHAN9, PWMCHAN10, PWMCHAN11, PWMCHAN12, PWMCHAN13, PWMCHAN14, PWMCHAN15,
        PWMCHAN16, PWMCHAN17, PWMCHAN18, PWMCHAN19, PWMCHAN20, PWMCHAN21, PWMCHAN22, PWMCHAN23,
        PWMCHAN24, PWMCHAN25, PWMCHAN26, PWMCHAN27, PWMCHAN28, PWMCHAN29, PWMCHAN30, PWMCHAN31,
    );
    pub mod interrupts {
        crate::interrupt_mod!(UART0, UART1, TRNG, DMA, GPIO, SPI);
    }
}

pub use chip::{Peripherals, interrupts::*, peripherals};
pub use neorv32_pac as pac;

/// Initialize the HAL. This must only be called from hart 0.
///
/// # Panics
///
/// Panics if this has already been called once before or not called from hart 0.
///
/// Panics if `time-driver` feature is enabled but `CLINT` is not supported.
pub fn init() -> Peripherals {
    // Attempt to take first so we panic before doing anything else
    let p = Peripherals::take();

    // In dual-hart, global interrupts are enabled in hart_main()
    // So for single-hart just enable them now
    // SAFETY: We're not worried about breaking any critical sections here
    #[cfg(feature = "single-hart")]
    unsafe {
        riscv::interrupt::enable()
    }

    #[cfg(feature = "time-driver")]
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

/* TEMPORARY until `csrrci` fix is released:
 * https://github.com/stnolting/neorv32/pull/1479
 *
 * This isn't ideal since we would like to cache MIE state and disable interrupts atomically,
 * but at least this actually will disable interrupts.
 */
#[cfg(feature = "single-hart")]
mod cs {
    use critical_section::{Impl, RawRestoreState, set_impl};

    struct SingleHartCriticalSection;
    set_impl!(SingleHartCriticalSection);

    unsafe impl Impl for SingleHartCriticalSection {
        unsafe fn acquire() -> RawRestoreState {
            let mie = riscv::register::mstatus::read().mie();
            riscv::interrupt::disable();
            mie
        }

        unsafe fn release(was_active: RawRestoreState) {
            // Only re-enable interrupts if they were enabled before the critical section.
            if was_active {
                unsafe { riscv::interrupt::enable() }
            }
        }
    }
}
