//! Dual-core support.
//!
//! This module provides a function for starting hart 1 and all the additional neccessary functionality
//! to safely support that (such as a custom executor and critical section).
//!
//! There are still open questions on the safety of Sending instantiated peripherals to the other hart,
//! (mainly due to interrupts being hart local) which I need to investigate more, so to be on the safe
//! side, for now it is recommended that peripherals are only instantiated in the hart they will be used
//! and not moved.
//!
//! Ensure the `dual-core` feature is enabled to use this.
use super::pac;
use core::mem::{ManuallyDrop, transmute};
use embassy_hal_internal::Peri;
pub use pac::interrupt::Hart;
use pac::interrupt::HartIdNumber;

// Number of harts on NEORV32
const NHARTS: usize = Hart::MAX_HART_ID_NUMBER + 1;

// Need a custom mp hook since the default will sleep hart 1 forever
// This just sleeps hart 1 until it is woken by hart 0 via a mswi
//
// After that, it returns 0 (which tells runtime to skip RAM init since hart 0 is responsible for that)
// and proceeds through rest of runtime until it gets to our hal_main
core::arch::global_asm!(
    r#".section .init.mp_hook, "ax"
    .global _mp_hook
    _mp_hook:
        // Enable software interrupt for both harts
        csrsi mie, 1 << 3
        // If hart 0, immediately return
        beqz a0, 3f
        // Otherwise if hart 1, wait for mswi from hart 0
        lui t0, 0xfff40
    1:  lw t1, 0x4(t0)
        bnez t1, 2f
        wfi
        j 1b
        // Hart 1 woken, so clear pending and return
    2:  sw zero, 0x4(t0)
        li a0, 0
        ret
    3:  li a0, 1
        ret
    "#
);

// We use this as a common point for both harts before they hit their user-provided entry points
// Hart 0's entry is provided statically via `main`, whereas hart 1's entry is provided at runtime
//
// SAFETY: No other symbol called `hal_main` is defined elsewhere
#[unsafe(export_name = "hal_main")]
fn hart_main(hart_id: usize, _: usize, _: usize) -> ! {
    // CLINT is used for software interrupts which is necessary for inter-hart communication
    if !crate::sysinfo::SysInfo::soc_config().clint() {
        panic!("CLINT must be supported for dual-core to work");
    }

    // Ensure global interrupts are enabled before entering user entry points
    // SAFETY: We're not worried about breaking any critical sections here
    unsafe { riscv::interrupt::enable() };

    // Hart 0 jumps directly to user main
    if hart_id == 0 {
        unsafe extern "Rust" {
            fn main() -> !;
        }
        // SAFETY: We are jumping to a user provided external entry so we assume it's safe
        unsafe { main() };
    // Hart 1 reads the pointer to the setup function placed by hart 0 and jumps there
    } else {
        let setup = (ihc::recv() >> 32) as usize;
        // SAFETY: It is guaranteed hart 0 has placed a valid function pointer in the mailbox
        let setup: fn() -> ! = unsafe { transmute(setup as *const ()) };
        setup();
    }
}

/// Start hart 1 at the given entry point (must only be called from hart 0).
///
/// Hart 1's stack is expected to be configured via linker script and runtime.
/// By default, both harts have the same stack size and the runtime will have set hart 1's
/// stack pointer correctly before this is ever called.
///
/// # Panics
///
/// Panics if called from hart 1.
pub fn hart1_start<F>(_instance: Peri<'static, crate::peripherals::HART1>, entry: F)
where
    F: FnOnce() -> never::Never + Send + 'static,
{
    fn hart1_setup<F: FnOnce() -> never::Never>() -> ! {
        // Read the pointer to the entry
        let entry = ihc::recv() as usize;
        // SAFETY: It is guaranteed hart 0 has placed a valid closure in the mailbox
        let entry: *mut ManuallyDrop<F> = unsafe { transmute(entry as *mut ()) };

        // But then we need to actually copy the contents of the entry closure (not just the pointer),
        // since it currently resides on hart 0's stack
        //
        // SAFETY: We ensure we no longer use the old invalid contents at the original location
        let entry = unsafe { ManuallyDrop::take(&mut *entry) };

        // Let hart 0 know we've copied the entry closure and it can proceed, then jump to entry
        ihc::send(Hart::H0, ihc::HART1_ACTIVE);
        entry();
    }

    // Only hart 0 should be calling this
    assert_eq!(ihc::whoami(), Hart::H0);

    // We don't want to call Drop ourselves since we are transferring ownership to hart 1
    let mut entry = ManuallyDrop::new(entry);
    let entry = &raw mut entry as u64;
    let setup = hart1_setup::<F> as usize as u64;

    // Pack both setup and entry into u64 mtimecmp where hart 1 can easily see it
    let packed = (setup << 32) | entry;
    ihc::send(Hart::H1, packed);

    // Need to wait until hart 1 copies entry since it resides on our stack at the moment
    // If we returned too soon, hart 1 will encounter UB
    while ihc::recv() != ihc::HART1_ACTIVE {
        ihc::sleep();
    }
}

/// A convenience wrapper around [`hart1_start`] which creates a new executor for the hart,
/// then runs the given entry on that new executor.
///
/// # Panics
///
/// Panics if called from hart 1.
pub fn hart1_start_with_executor<F>(_instance: Peri<'static, crate::peripherals::HART1>, entry: F)
where
    F: FnOnce(embassy_executor::Spawner) + Send + 'static,
{
    let entry = move || {
        let mut executor = executor::Executor::new();
        let executor: &'static mut executor::Executor =
            // SAFETY: `run` never returns so `executor` will be valid for lifetime of entire program
            unsafe { core::mem::transmute(&mut executor) };
        executor.run(entry)
    };

    hart1_start(_instance, entry);
}

mod ihc {
    use super::*;

    pub(super) const HART1_ACTIVE: u64 = 0xC0FFEE;

    // Software interrupt is just used internally to wake harts, so all we do is clear it here
    #[riscv_rt::core_interrupt(pac::interrupt::CoreInterrupt::MachineSoft)]
    fn mswi_handler() {
        ihc::clint().mswi().msip_mhartid().unpend();
    }

    fn clint() -> pac::Clint {
        // SAFETY: We are the only users of clint and guarantee its safe use
        unsafe { pac::Clint::steal() }
    }

    pub(super) fn whoami() -> Hart {
        let id = riscv::register::mhartid::read();
        Hart::from_number(id).expect("NEORV32 should have harts id 0 and 1")
    }

    pub(super) fn sleep() {
        // We don't want the mswi_handler trapping right now,
        // since we want to read the flag before clearing ourselves
        riscv::interrupt::free(|| {
            while !clint().mswi().msip_mhartid().is_pending() {
                riscv::asm::wfi();
            }
            clint().mswi().msip_mhartid().unpend();
        })
    }

    pub(super) fn wake(hart: Hart) {
        clint().mswi().msip(hart).pend();
    }

    pub(super) fn send(hart: Hart, val: u64) {
        // We treat hart 1's mtimecmp reg as a convenient 'mailbox' since it is unused
        clint().mtimecmp1().write(val);
        wake(hart);
    }

    pub(super) fn recv() -> u64 {
        clint().mtimecmp1().read()
    }
}

mod cs {
    use super::*;
    use core::sync::atomic::AtomicU8;
    use core::sync::atomic::Ordering::{Acquire, Release};

    // Need to track this since critical sections can be nested
    const LOCK_UNOWNED: u8 = 0;
    const LOCK_OWNED: u8 = 2;
    static LOCK_OWNER: AtomicU8 = AtomicU8::new(LOCK_UNOWNED);

    fn acquire(spin: impl Fn()) -> u8 {
        // Hart 0 -> 1, Hart 1 -> 2
        let owner_id = ihc::whoami().number() as u8 + 1;

        // If we are the current owner of the lock, then we are in a nested cs,
        // so don't try to grab it again and return a sentinel value
        if owner_id == LOCK_OWNER.load(Acquire) {
            LOCK_OWNED
        // Otherwise spin until lock is free and return our current interrupt status
        } else {
            let mut mstatus;
            // SAFETY: This asm has the effect of disabling interrupts which is desired,
            // and it returns a value that is the correct bit representation of `Mstatus`
            unsafe {
                core::arch::asm!("csrrci {}, mstatus, 0b1000", out(reg) mstatus);
                core::mem::transmute::<usize, riscv::register::mstatus::Mstatus>(mstatus).mie();
            }
            spin();
            LOCK_OWNER.store(owner_id, Release);
            mstatus as _
        }
    }

    fn release(status: u8, free: impl Fn()) {
        // Only free the lock if we are calling release for the outer most cs
        // (as in, a nested cs should do nothing here since it didn't need to acquire the lock)
        if status != LOCK_OWNED {
            free();
            LOCK_OWNER.store(LOCK_UNOWNED, Release);
            if status == 1 {
                // SAFETY: We won't break any critical sections where this is used
                unsafe { riscv::interrupt::enable() }
            }
        }
    }

    // If on a NEORV32 with A extension, implement spinlock via atomic CAS
    #[cfg(target_has_atomic = "ptr")]
    mod cs_impl {
        use super::*;
        use core::sync::atomic::AtomicBool;
        use core::sync::atomic::Ordering::{Acquire, Relaxed, Release};

        static LOCK: AtomicBool = AtomicBool::new(false);

        struct CasLock;
        critical_section::set_impl!(CasLock);

        unsafe impl critical_section::Impl for CasLock {
            unsafe fn acquire() -> critical_section::RawRestoreState {
                acquire(|| {
                    while LOCK
                        .compare_exchange_weak(false, true, Acquire, Relaxed)
                        .is_err()
                    {
                        core::hint::spin_loop();
                    }
                })
            }

            unsafe fn release(status: critical_section::RawRestoreState) {
                release(status, || LOCK.store(false, Release));
            }
        }
    }

    // If A extension not available, implement spinlock via Peterson's algorithm
    //
    // NEORV32 only has 2 harts and SeqCst gives strong ordering between harts
    // so Peterson should be sound here, albeit less efficient than atomic CAS
    //
    // https://en.wikipedia.org/wiki/Peterson%27s_algorithm
    #[cfg(not(target_has_atomic = "ptr"))]
    mod cs_impl {
        use super::*;
        use core::sync::atomic::Ordering::SeqCst;
        use core::sync::atomic::{AtomicBool, AtomicUsize};

        static TURN: AtomicUsize = AtomicUsize::new(0);
        static FLAG: [AtomicBool; NHARTS] = [const { AtomicBool::new(false) }; NHARTS];

        struct PetersonLock;
        critical_section::set_impl!(PetersonLock);

        unsafe impl critical_section::Impl for PetersonLock {
            unsafe fn acquire() -> critical_section::RawRestoreState {
                acquire(|| {
                    let this = ihc::whoami().number();
                    let other = 1 - this;
                    FLAG[this].store(true, SeqCst);
                    TURN.store(other, SeqCst);

                    while FLAG[other].load(SeqCst) && TURN.load(SeqCst) == other {
                        core::hint::spin_loop();
                    }
                })
            }

            unsafe fn release(status: critical_section::RawRestoreState) {
                release(status, || FLAG[ihc::whoami().number()].store(false, SeqCst));
            }
        }
    }
}

pub mod executor {
    use super::*;
    use core::marker::PhantomData;
    use core::sync::atomic::AtomicBool;
    use core::sync::atomic::Ordering::{Acquire, Relaxed, Release};
    use embassy_executor::{Spawner, raw};

    static SEV_FLAG: [AtomicBool; NHARTS] = [const { AtomicBool::new(false) }; NHARTS];

    // Emulates a SEV by setting a flag for each hart and waking the other hart via MSWI
    fn sev() {
        for sev_flag in &SEV_FLAG {
            sev_flag.store(true, Release);
        }

        // We wake the other hart in case it now has work to do
        match ihc::whoami() {
            // Hart 1 might not have been started yet,
            // so if we trigger a SWI now it would prematurely try to start
            Hart::H0 if ihc::recv() == ihc::HART1_ACTIVE => ihc::wake(Hart::H1),
            Hart::H1 => ihc::wake(Hart::H0),
            _ => (),
        }
    }

    // Emulates a WFE by checking if flag is set before deciding to go to sleep
    fn wfe() {
        let hart_id = ihc::whoami().number();

        // We want to make sure we don't miss a MSWI between seeing the flag is false and wfi
        // We are not worried about doing an atomic CAS on the flag though
        riscv::interrupt::free(|| {
            if SEV_FLAG[hart_id].load(Acquire) {
                SEV_FLAG[hart_id].store(false, Relaxed);
            } else {
                riscv::asm::wfi();
            }
        });
    }

    // SAFETY: We ensure there is no other __pender symbol
    #[unsafe(export_name = "__pender")]
    fn __pender(_context: *mut ()) {
        sev();
    }

    /// Custom executor for dual-core NEORV32.
    ///
    /// This is necessary because the default rv32 executor provided by Embassy is not sufficient for dual-core use.
    /// It lacks the functionality to wake other harts from sleep when a task they are waiting on is ready to be polled again.
    ///
    /// This executor makes use of software interrupts to wake each hart when there is work to be done,
    /// mimicking the behavior of ARM's `sev/wfe` instructions to some extent.
    ///
    /// This can be used with the `#[embassy_executor::main]` macro like so:
    ///
    /// `#[embassy_executor::main(executor = "embassy_neorv32::dualcore::executor::Executor", entry = "riscv_rt::entry")]`
    pub struct Executor {
        inner: raw::Executor,
        not_send: PhantomData<*mut ()>,
    }

    impl Default for Executor {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Executor {
        /// Create a new dual-core executor.
        ///
        /// It is recommended to use the `#[embassy_executor::main]` macro for hart 0 and the
        /// [`hart1_start_with_executor`] function for hart 1 to avoid creating an executor manually.
        pub fn new() -> Self {
            Self {
                inner: raw::Executor::new(core::ptr::null_mut()),
                not_send: PhantomData,
            }
        }

        /// Run the executor.
        pub fn run(&'static mut self, init: impl FnOnce(Spawner)) -> ! {
            init(self.inner.spawner());

            loop {
                // SAFETY: We've guaranteed init has been called (above) and don't call poll re-entrantly
                unsafe { self.inner.poll() }
                wfe();
            }
        }
    }
}

// Used since the never (!) type is not stable
// https://github.com/nvzqz/bad-rs/blob/master/src/never.rs
mod never {
    pub(crate) type Never = <F as HasOutput>::Output;

    pub trait HasOutput {
        type Output;
    }

    impl<O> HasOutput for fn() -> O {
        type Output = O;
    }

    type F = fn() -> !;
}
