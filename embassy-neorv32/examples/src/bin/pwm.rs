#![no_std]
#![no_main]

use core::fmt::Write;
use embassy_neorv32::pwm::Pwm;
use embassy_neorv32::uart::UartTx;
use embassy_neorv32_examples::*;
use embassy_time::Timer;
use embedded_hal_02::Pwm as PwmTrait;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_neorv32::init();

    // Setup UART for display purposes
    let mut uart = UartTx::new_blocking(p.UART0, UART_BAUD, UART_IS_SIM, false);

    // Setup PWM channel 0
    let mut pwm0 = Pwm::new(p.PWM0, 42, false);
    pwm0.set_duty_cycle_percent(25);
    writeln!(
        &mut uart,
        "PWM 0 Frequency: {} Hz",
        1_000_000 / u32::from(pwm0.get_period())
    )
    .unwrap();
    writeln!(
        &mut uart,
        "PWM 0 Duty: {}%\n",
        (pwm0.get_duty(()) as u16 * 100) / 255
    )
    .unwrap();

    // Setup PWM channel 1
    let mut pwm1 = Pwm::new(p.PWM1, 1337, false);
    pwm1.set_duty_cycle_percent(66);
    writeln!(
        &mut uart,
        "PWM 1 Frequency: {} Hz",
        1_000_000 / u32::from(pwm1.get_period())
    )
    .unwrap();
    writeln!(
        &mut uart,
        "PWM 1 Duty: {}%",
        (pwm1.get_duty(()) as u16 * 100) / 255
    )
    .unwrap();

    // Keep PWM alive
    loop {
        Timer::after_micros(s_to_us(10)).await;
    }
}
