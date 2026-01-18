#![no_std]
#![no_main]

#[cfg(feature = "sim")]
compile_error!("PWM example not available in simulation.");

use embassy_neorv32::pwm::{self, Percent, Pwm};
use embassy_neorv32::uart::UartTx;
use embassy_neorv32_examples::*;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    let mut uart = UartTx::new_blocking(p.UART0, UART_BAUD, UART_IS_SIM, false)
        .expect("UART must be supported");

    // Setup PWM peripheral with a clock prescaler of 4096
    let pwm = Pwm::new(p.PWM, pwm::ClkPrsc::_4096).expect("PWM must be supported");

    // Setup PWM channel 0 with frequency of 42
    let mut chan0 = pwm.new_channel(p.PWMCHAN0, pwm::Mode::Fast, 42, false);

    uart.blocking_write(b"Starting PWM LED breathing example...\n");
    loop {
        for duty in (0..=100).step_by(10).chain((0..=100).rev().step_by(10)) {
            let percent = Percent::new(duty).unwrap();
            chan0.set_duty_cycle(percent);
            Timer::after_millis(100).await;
        }
    }
}
