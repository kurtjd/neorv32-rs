#![no_std]
#![no_main]

#[cfg(feature = "sim")]
compile_error!("PWM example not available in simulation.");

use core::fmt::Write;
use embassy_neorv32::pwm::{self, Pwm};
use embassy_neorv32::uart::UartTx;
use embassy_neorv32_examples::*;
use embassy_time::Timer;
use embedded_hal_02::PwmPin;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = UartTx::new_blocking(p.UART0, UART_BAUD, UART_IS_SIM, false);

    // Setup PWM peripheral
    let pwm = Pwm::new(p.PWM, pwm::ClkPrsc::_4096);

    // Setup PWM channel 0
    let mut chan0 = pwm.new_channel(p.PWMCHAN0, pwm::Mode::Fast, 42, false);
    chan0.set_duty_cycle(pwm::Percent::new(25).unwrap());
    writeln!(&mut uart, "PWM 0 Duty: {}%\n", chan0.get_duty().inner()).unwrap();

    // Setup PWM channel 1
    let mut chan1 = pwm.new_channel(p.PWMCHAN1, pwm::Mode::Fast, 1337, false);
    chan1.set_duty_cycle(pwm::Percent::new(66).unwrap());
    writeln!(&mut uart, "PWM 1 Duty: {}%", chan1.get_duty().inner()).unwrap();

    // Keep PWM alive
    loop {
        Timer::after_micros(s_to_us(10)).await;
    }
}
