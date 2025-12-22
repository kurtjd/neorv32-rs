/// Macro to bind interrupts to handlers.
///
/// This defines the right interrupt handlers, and creates a unit struct (like `struct Irqs;`)
/// and implements the right \[`Binding`\]s for it. You can pass this struct to drivers to
/// prove at compile-time that the right interrupts have been bound.
///
/// Example of how to bind one interrupt:
///
/// ```rust,ignore
/// use embassy_neorv32::{bind_interrupts, trng, peripherals};
///
/// bind_interrupts!(struct Irqs {
///     TRNG => trng::InterruptHandler<peripherals::TRNG>;
/// });
/// ```
#[macro_export]
macro_rules! bind_interrupts {
    ($vis:vis struct $name:ident { $($irq:ident => $($handler:ty),*;)* }) => {
            #[derive(Copy, Clone)]
            $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[riscv_rt::core_interrupt($crate::pac::interrupt::CoreInterrupt::$irq)]
            fn $irq() {
                $(
                    // SAFETY: This macro ensures the given handler is being called from the correct IRQ
                    unsafe { <$handler as $crate::interrupt::typelevel::Handler<$crate::interrupt::typelevel::$irq>>::on_interrupt(); }
                )*
            }

            $(
                // SAFETY: This macro ensures the given IRQ is bounded to given handler
                unsafe impl $crate::interrupt::typelevel::Binding<$crate::interrupt::typelevel::$irq, $handler> for $name {}
            )*
        )*
    };
}

/// Generate a standard `mod interrupt` for a RISCV HAL.
#[macro_export]
macro_rules! interrupt_mod {
    ($($irqs:ident),* $(,)?) => {
        /// Interrupt definitions.
        pub mod interrupt {
            pub use $crate::pac::interrupt::CoreInterrupt;

            /// Type-level interrupt infrastructure.
            ///
            /// This module contains one *type* per interrupt. This is used for checking at compile time that
            /// the interrupts are correctly bound to HAL drivers.
            ///
            /// As an end user, you shouldn't need to use this module directly. Use the [`crate::bind_interrupts!`] macro
            /// to bind interrupts, and the [`crate::interrupt`] module to manually register interrupt handlers and manipulate
            /// interrupts directly (pending/unpending, enabling/disabling, setting the priority, etc...)
            pub mod typelevel {
                trait SealedInterrupt {}

                /// Type-level interrupt.
                ///
                /// This trait is implemented for all typelevel interrupt types in this module.
                #[allow(private_bounds)]
                pub trait Interrupt: SealedInterrupt {

                    /// Interrupt enum variant.
                    ///
                    /// This allows going from typelevel interrupts (one type per interrupt) to
                    /// non-typelevel interrupts (a single `Interrupt` enum type, with one variant per interrupt).
                    const IRQ: super::CoreInterrupt;

                    /// Enable the interrupt.
                    ///
                    /// # Safety
                    ///
                    /// Enabling interrupts might break critical sections or other synchronization mechanisms.
                    ///
                    /// Ensure that this is called in a safe context where interrupts can be enabled.
                    #[inline]
                    unsafe fn enable() {
                        // SAFETY: Caller must uphold safety guarantees
                        unsafe { riscv::interrupt::enable_interrupt(Self::IRQ) }
                    }

                    /// Disable the interrupt.
                    #[inline]
                    fn disable() {
                        riscv::interrupt::disable_interrupt(Self::IRQ);
                    }

                    /// Check if interrupt is enabled.
                    #[inline]
                    fn is_enabled() -> bool {
                        riscv::interrupt::is_interrupt_enabled(Self::IRQ)
                    }

                    /// Check if interrupt is pending.
                    #[inline]
                    fn is_pending() -> bool {
                        riscv::interrupt::is_interrupt_pending(Self::IRQ)
                    }
                }

                $(
                    #[allow(non_camel_case_types)]
                    #[doc=stringify!($irqs)]
                    #[doc=" typelevel interrupt."]
                    pub enum $irqs {}
                    impl SealedInterrupt for $irqs{}
                    impl Interrupt for $irqs {
                        const IRQ: super::CoreInterrupt = super::CoreInterrupt::$irqs;
                    }
                )*

                /// Interrupt handler trait.
                ///
                /// Drivers that need to handle interrupts implement this trait.
                /// The user must ensure `on_interrupt()` is called every time the interrupt fires.
                /// Drivers must use use [`Binding`] to assert at compile time that the user has done so.
                pub trait Handler<I: Interrupt> {
                    /// Interrupt handler function.
                    ///
                    /// Must be called every time the `I` interrupt fires, synchronously from
                    /// the interrupt handler context.
                    ///
                    /// # Safety
                    ///
                    /// This function must ONLY be called from the interrupt handler for `I`.
                    unsafe fn on_interrupt();
                }

                /// Compile-time assertion that an interrupt has been bound to a handler.
                ///
                /// For the vast majority of cases, you should use the `bind_interrupts!`
                /// macro instead of writing `unsafe impl`s of this trait.
                ///
                /// # Safety
                ///
                /// By implementing this trait, you are asserting that you have arranged for `H::on_interrupt()`
                /// to be called every time the `I` interrupt fires.
                ///
                /// This allows drivers to check bindings at compile-time.
                pub unsafe trait Binding<I: Interrupt, H: Handler<I>> {}
            }
        }
    };
}

// Disables interrupts (if enabled) and returns the interrupt enabled status before we disabled
pub(crate) fn disable() -> bool {
    let mut mstatus: usize;
    // SAFETY: This asm has the effect of disabling interrupts which is desired,
    // and it returns a value that is the correct bit representation of `Mstatus`
    unsafe {
        core::arch::asm!("csrrci {}, mstatus, 0b1000", out(reg) mstatus);
        core::mem::transmute::<usize, riscv::register::mstatus::Mstatus>(mstatus).mie()
    }
}

// Restores interrupt enabled status to previous state from a `disable()`
pub(crate) fn restore(was_active: bool) {
    if was_active {
        // SAFETY: We won't break any critical sections where this is used
        unsafe { riscv::interrupt::enable() }
    }
}

// A critical section that only disables interrupts, meant to synchronize on a single hart only
// Used internally for when we are only worried about being interrupted and not accessing state shared between harts
//
// This gives us slight performance gains and is also necessary for the times we need to call wfi
// within a critical section (which would block the other hart for far too long
// if we used the dual-core critical section)
pub(crate) fn free<R>(f: impl FnOnce() -> R) -> R {
    let was_active = disable();
    let ret = f();
    restore(was_active);
    ret
}
