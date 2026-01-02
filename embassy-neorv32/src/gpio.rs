//! General-Purpose Input/Output (GPIO)
use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::peripherals::GPIO;
use core::convert::Infallible;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

// Max number of GPIO ports available
// Revisit: Do we want the user to configure this when not all 32 are available?
const MAX_PORTS: usize = 32;

/// GPIO interrupt handler binding.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let pending = T::reg().irq_pending().read().bits();
        let mut disabled = T::reg().irq_enable().read().bits();

        // Wake and disable every port that has IRQ pending
        for (i, waker) in T::wakers().iter().enumerate() {
            let port_bit = 1 << i;
            if (pending & port_bit) != 0 {
                waker.wake();
                disabled &= !port_bit;
            }
        }

        // Clear pending
        // SAFETY: Register is write 0 to clear, so we bitwise not `pending` to clear only those,
        // assuring if a port becomes pending in the meantime we don't clobber it
        T::reg()
            .irq_pending()
            .write(|w| unsafe { w.bits(!pending) });

        // Disable interrupts for ports that were just pending
        // SAFETY: We've ensured we've only cleared the bits of the interrupts we actually serviced
        T::reg().irq_enable().write(|w| unsafe { w.bits(disabled) });
    }
}

/// GPIO driver.
pub struct Gpio<'d, M: IoMode> {
    reg: &'static crate::pac::gpio::RegisterBlock,
    wakers: &'static [AtomicWaker; MAX_PORTS],
    _phantom: PhantomData<&'d M>,
}

impl<'d, M: IoMode> Gpio<'d, M> {
    fn new_inner<T: Instance>(_instance: Peri<'d, T>) -> Self {
        Self {
            reg: T::reg(),
            wakers: T::wakers(),
            _phantom: PhantomData,
        }
    }

    /// Create a new instance of a port driver capable of simultaneous input and output.
    pub fn new_port<T: PortInstance>(&self, _instance: Peri<'d, T>) -> Port<'d, M> {
        Port::new(T::PORT, self.reg, &self.wakers[T::PORT as usize])
    }

    /// Create a new instance of an input-only port driver.
    pub fn new_input<T: PortInstance>(&self, _instance: Peri<'d, T>) -> Input<'d, M> {
        Input::new(T::PORT, self.reg, &self.wakers[T::PORT as usize])
    }

    /// Create a new instance of an output-only port driver.
    pub fn new_output<T: PortInstance>(&self, _instance: Peri<'d, T>) -> Output<'d> {
        Output::new(T::PORT, self.reg)
    }
}

impl<'d> Gpio<'d, Blocking> {
    /// Create a new instance of a blocking GPIO driver.
    pub fn new_blocking<T: Instance>(_instance: Peri<'d, T>) -> Self {
        Self::new_inner(_instance)
    }
}

impl<'d> Gpio<'d, Async> {
    /// Create a new instance of an async GPIO driver.
    pub fn new_async<T: Instance>(
        _instance: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        // SAFETY: It is valid to enable GPIO interrupt here
        unsafe { T::Interrupt::enable() }
        Self::new_inner(_instance)
    }
}

/// A GPIO port.
///
/// On the neorv32, ports are bidirectional represented by two (input/output) signals under the hood,
/// corresponding to bits PORT_IN(i) and PORT_OUT(i) respectively.
///
/// Thus, a single port allows simultaneous input and output.
pub struct Port<'d, M: IoMode> {
    input: Input<'d, M>,
    output: Output<'d>,
}

impl<'d, M: IoMode> Port<'d, M> {
    fn new(
        port: u32,
        reg: &'static crate::pac::gpio::RegisterBlock,
        waker: &'static AtomicWaker,
    ) -> Self {
        let input = Input::new(port, reg, waker);
        let output = Output::new(port, reg);

        Self { input, output }
    }

    /// Split the port into separate [Input] and [Output] ports for sharing between tasks.
    pub fn split(self) -> (Input<'d, M>, Output<'d>) {
        (self.input, self.output)
    }

    /// Split the port by mutable reference into separate [Input] and [Output] ports for sharing between tasks.
    pub fn split_ref(&mut self) -> (&mut Input<'d, M>, &mut Output<'d>) {
        (&mut self.input, &mut self.output)
    }

    /// Returns true if the port's input signal is low.
    pub fn is_low(&self) -> bool {
        self.input.is_low()
    }

    /// Returns true if the port's input signal is high.
    pub fn is_high(&self) -> bool {
        self.input.is_high()
    }

    /// Toggle the port's output signal between low and high.
    pub fn toggle(&mut self) {
        self.output.toggle();
    }

    /// Set the port's output signal low.
    pub fn set_low(&mut self) {
        self.output.set_low();
    }

    /// Set the port's output signal high.
    pub fn set_high(&mut self) {
        self.output.set_high();
    }

    /// Returns true if the port's output signal is set low.
    pub fn is_set_low(&self) -> bool {
        self.output.is_set_low()
    }

    /// Returns true if the port's output signal is set high.
    pub fn is_set_high(&self) -> bool {
        self.output.is_set_high()
    }
}

impl<'d> Port<'d, Async> {
    /// Wait until the port's input signal is low, returning immediately if it already is.
    pub fn wait_for_low(&mut self) -> impl Future<Output = ()> {
        self.input.wait_for_low()
    }

    /// Wait until the port's input signal is high, returning immediately if it already is.
    pub fn wait_for_high(&mut self) -> impl Future<Output = ()> {
        self.input.wait_for_high()
    }

    /// Wait for the port's input signal to transition from high to low.
    ///
    /// If the input signal is already low, this will not return until the signal transitions
    /// from low to high then back to low again.
    pub fn wait_for_falling_edge(&mut self) -> impl Future<Output = ()> {
        self.input.wait_for_falling_edge()
    }

    /// Wait for the port's input signal to transition from low to high.
    ///
    /// If the input signal is already high, this will not return until the signal transitions
    /// from high to low then back to high again.
    pub fn wait_for_rising_edge(&mut self) -> impl Future<Output = ()> {
        self.input.wait_for_rising_edge()
    }

    /// Wait for the port's input signal to undergo any state transition.
    pub fn wait_for_any_edge(&mut self) -> impl Future<Output = ()> {
        self.input.wait_for_any_edge()
    }
}

pub struct Input<'d, M: IoMode> {
    reg: &'static crate::pac::gpio::RegisterBlock,
    port: u32,
    waker: &'static AtomicWaker,
    _phantom: PhantomData<&'d M>,
}

// Allows for use in a Mutex (to share safely between harts and tasks)
unsafe impl<'d, M: IoMode> Send for Input<'d, M> {}

impl<'d, M: IoMode> Input<'d, M> {
    fn new(
        port: u32,
        reg: &'static crate::pac::gpio::RegisterBlock,
        waker: &'static AtomicWaker,
    ) -> Self {
        Self {
            reg,
            port,
            waker,
            _phantom: PhantomData,
        }
    }

    fn irq_disable(&mut self) {
        // SAFETY: We only clear our bit. This is only called in a critical section so no risk of clobbering others.
        self.reg
            .irq_enable()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.port)) });
    }

    /// Returns true if the port's input signal is low.
    pub fn is_low(&self) -> bool {
        (self.reg.port_in().read().bits() & (1 << self.port)) == 0
    }

    /// Returns true if the port's input signal is high.
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }
}

impl<'d> Input<'d, Async> {
    fn set_level_trigger(&mut self) {
        // SAFETY: We only clear our bit. This is only called in a critical section so no risk of clobbering others.
        self.reg
            .irq_type()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.port)) });
    }

    fn set_edge_trigger(&mut self) {
        // SAFETY: We only set our bit. This is only called in a critical section so no risk of clobbering others.
        self.reg
            .irq_type()
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.port)) });
    }

    fn set_trigger_polarity_low(&mut self) {
        // SAFETY: We only clear our bit. This is only called in a critical section so no risk of clobbering others.
        self.reg
            .irq_polarity()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.port)) });
    }

    fn set_trigger_polarity_high(&mut self) {
        // SAFETY: We only set our bit. This is only called in a critical section so no risk of clobbering others.
        self.reg
            .irq_polarity()
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.port)) });
    }

    fn irq_enable(&mut self) {
        // SAFETY: We only set our bit. This is only called in a critical section so no risk of clobbering others.
        self.reg
            .irq_enable()
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.port)) });
    }

    fn irq_enabled(&self) -> bool {
        (self.reg.irq_enable().read().bits() & (1 << self.port)) != 0
    }

    async fn wait(&mut self) {
        critical_section::with(|_| self.irq_enable());

        poll_fn(|cx| {
            self.waker.register(cx.waker());

            // If irq is disabled, we know interrupt actually fired
            if !self.irq_enabled() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }

    /// Wait until the port's input signal is low, returning immediately if it already is.
    pub async fn wait_for_low(&mut self) {
        if !self.is_low() {
            critical_section::with(|_| {
                self.set_level_trigger();
                self.set_trigger_polarity_low()
            });
            self.wait().await
        }
    }

    /// Wait until the port's input signal is high, returning immediately if it already is.
    pub async fn wait_for_high(&mut self) {
        if !self.is_high() {
            critical_section::with(|_| {
                self.set_level_trigger();
                self.set_trigger_polarity_high();
            });
            self.wait().await
        }
    }

    /// Wait for the port's input signal to transition from high to low.
    ///
    /// If the input signal is already low, this will not return until the signal transitions
    /// from low to high then back to low again.
    pub async fn wait_for_falling_edge(&mut self) {
        critical_section::with(|_| {
            self.set_edge_trigger();
            self.set_trigger_polarity_low();
        });
        self.wait().await
    }

    /// Wait for the port's input signal to transition from low to high.
    ///
    /// If the input signal is already high, this will not return until the signal transitions
    /// from high to low then back to high again.
    pub async fn wait_for_rising_edge(&mut self) {
        critical_section::with(|_| {
            self.set_edge_trigger();
            self.set_trigger_polarity_high();
        });
        self.wait().await
    }

    /// Wait for the port's input signal to undergo any state transition.
    pub async fn wait_for_any_edge(&mut self) {
        if self.is_low() {
            self.wait_for_rising_edge().await
        } else {
            self.wait_for_falling_edge().await
        }
    }
}

impl<'d, M: IoMode> Drop for Input<'d, M> {
    fn drop(&mut self) {
        critical_section::with(|_| self.irq_disable());
    }
}

pub struct Output<'d> {
    reg: &'static crate::pac::gpio::RegisterBlock,
    port: u32,
    _phantom: PhantomData<&'d ()>,
}

// Allows for use in a Mutex (to share safely between harts and tasks)
unsafe impl<'d> Send for Output<'d> {}

impl<'d> Output<'d> {
    fn new(port: u32, reg: &'static crate::pac::gpio::RegisterBlock) -> Self {
        Self {
            reg,
            port,
            _phantom: PhantomData,
        }
    }

    /// Toggle the port's output signal between low and high.
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high();
        } else {
            self.set_low();
        }
    }

    /// Set the port's output signal low.
    pub fn set_low(&mut self) {
        critical_section::with(|_| {
            // SAFETY: We only clear our bit. This is only called in a critical section so no risk of clobbering others.
            self.reg
                .port_out()
                .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.port)) })
        });
    }

    /// Set the port's output signal high.
    pub fn set_high(&mut self) {
        critical_section::with(|_| {
            // SAFETY: We only set our bit. This is only called in a critical section so no risk of clobbering others.
            self.reg
                .port_out()
                .modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.port)) })
        });
    }

    /// Returns true if the port's output signal is set low.
    pub fn is_set_low(&self) -> bool {
        (self.reg.port_out().read().bits() & (1 << self.port)) == 0
    }

    /// Returns true if the port's output signal is set high.
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }
}

trait SealedIoMode {}

/// GPIO IO mode.
#[allow(private_bounds)]
pub trait IoMode: SealedIoMode {}

/// Blocking GPIO.
pub struct Blocking;
impl SealedIoMode for Blocking {}
impl IoMode for Blocking {}

/// Async GPIO.
pub struct Async;
impl SealedIoMode for Async {}
impl IoMode for Async {}

trait SealedInstance {
    fn reg() -> &'static crate::pac::gpio::RegisterBlock;
    fn wakers() -> &'static [AtomicWaker; MAX_PORTS];
}

/// A valid GPIO peripheral.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    type Interrupt: Interrupt;
}

impl SealedInstance for GPIO {
    fn reg() -> &'static crate::pac::gpio::RegisterBlock {
        // SAFETY: We have exclusive access to the GPIO register block
        unsafe { &*crate::pac::Gpio::ptr() }
    }

    fn wakers() -> &'static [AtomicWaker; MAX_PORTS] {
        static WAKERS: [AtomicWaker; MAX_PORTS] = [const { AtomicWaker::new() }; MAX_PORTS];
        &WAKERS
    }
}
impl Instance for GPIO {
    type Interrupt = crate::interrupt::typelevel::GPIO;
}

trait SealedPortInstance {}

/// A valid PORT peripheral.
#[allow(private_bounds)]
pub trait PortInstance: SealedPortInstance + PeripheralType {
    const PORT: u32;
}

macro_rules! impl_port {
    ($periph:ident, $port:expr) => {
        impl SealedPortInstance for crate::peripherals::$periph {}
        impl PortInstance for crate::peripherals::$periph {
            const PORT: u32 = $port;
        }
    };
}

impl_port!(PORT0, 0);
impl_port!(PORT1, 1);
impl_port!(PORT2, 2);
impl_port!(PORT3, 3);
impl_port!(PORT4, 4);
impl_port!(PORT5, 5);
impl_port!(PORT6, 6);
impl_port!(PORT7, 7);
impl_port!(PORT8, 8);
impl_port!(PORT9, 9);
impl_port!(PORT10, 10);
impl_port!(PORT11, 11);
impl_port!(PORT12, 12);
impl_port!(PORT13, 13);
impl_port!(PORT14, 14);
impl_port!(PORT15, 15);
impl_port!(PORT16, 16);
impl_port!(PORT17, 17);
impl_port!(PORT18, 18);
impl_port!(PORT19, 19);
impl_port!(PORT20, 20);
impl_port!(PORT21, 21);
impl_port!(PORT22, 22);
impl_port!(PORT23, 23);
impl_port!(PORT24, 24);
impl_port!(PORT25, 25);
impl_port!(PORT26, 26);
impl_port!(PORT27, 27);
impl_port!(PORT28, 28);
impl_port!(PORT29, 29);
impl_port!(PORT30, 30);
impl_port!(PORT31, 31);

impl<'d, M: IoMode> embedded_hal_02::digital::v2::InputPin for Port<'d, M> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }
}

impl<'d, M: IoMode> embedded_hal_02::digital::v2::OutputPin for Port<'d, M> {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<'d, M: IoMode> embedded_hal_02::digital::v2::StatefulOutputPin for Port<'d, M> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }
}

impl<'d, M: IoMode> embedded_hal_02::digital::v2::ToggleableOutputPin for Port<'d, M> {
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<'d, M: IoMode> embedded_hal_02::digital::v2::InputPin for Input<'d, M> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }
}

impl<'d> embedded_hal_02::digital::v2::OutputPin for Output<'d> {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<'d> embedded_hal_02::digital::v2::StatefulOutputPin for Output<'d> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }
}

impl<'d> embedded_hal_02::digital::v2::ToggleableOutputPin for Output<'d> {
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl<'d, M: IoMode> embedded_hal_1::digital::ErrorType for Port<'d, M> {
    type Error = Infallible;
}

impl<'d, M: IoMode> embedded_hal_1::digital::InputPin for Port<'d, M> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_low())
    }
}

impl<'d, M: IoMode> embedded_hal_1::digital::OutputPin for Port<'d, M> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<'d, M: IoMode> embedded_hal_1::digital::StatefulOutputPin for Port<'d, M> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_set_high())
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_set_low())
    }
}

impl<'d> embedded_hal_async::digital::Wait for Port<'d, Async> {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        self.wait_for_high().await;
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        self.wait_for_low().await;
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_rising_edge().await;
        Ok(())
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_falling_edge().await;
        Ok(())
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_any_edge().await;
        Ok(())
    }
}

impl<'d, M: IoMode> embedded_hal_1::digital::ErrorType for Input<'d, M> {
    type Error = Infallible;
}

impl<'d, M: IoMode> embedded_hal_1::digital::InputPin for Input<'d, M> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_low())
    }
}

impl<'d> embedded_hal_async::digital::Wait for Input<'d, Async> {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        self.wait_for_high().await;
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        self.wait_for_low().await;
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_rising_edge().await;
        Ok(())
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_falling_edge().await;
        Ok(())
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_any_edge().await;
        Ok(())
    }
}

impl<'d> embedded_hal_1::digital::ErrorType for Output<'d> {
    type Error = Infallible;
}

impl<'d> embedded_hal_1::digital::OutputPin for Output<'d> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<'d> embedded_hal_1::digital::StatefulOutputPin for Output<'d> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_set_high())
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_set_low())
    }
}
