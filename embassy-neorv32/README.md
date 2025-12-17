# Embassy NEORV32 HAL
HALs implement safe, idiomatic Rust APIs to use the hardware capabilities, so raw register manipulation is not needed.

The embassy-neorv32 HAL targets the open-source [NEORV32](https://github.com/stnolting/neorv32)
RISC-V microcontroller and implements both blocking and async APIs/drivers for most of the peripherals.
Additionally, async and blocking traits from [embedded-hal](https://crates.io/crates/embedded-hal)
are implemented where appropriate.

## Overview
NEORV32 is actively being developed and occasionally breaking changes are introduced (such as peripheral
register interfaces changing). This HAL will try to keep up-to-date with these changes, but may not
always support the latest and greatest.

At this time, the HAL currently supports the following peripherals and features for NEORV32 `v1.12.1`:

## Peripherals Currently Supported
- SPI
- TWI
- GPIO
- UART
- DMA
- PWM
- TRNG
- WDT
- GPTMR
- SYSINFO

## Additional Features
- Dual-core support

## Usage
Please see `embassy-neorv32/examples` for examples on how to use this HAL in your own projects.
Instructions for running the examples can be found below. *Note* that some other things may need to change
depending on running in simulation or on FPGA (such as time intervals for example, since a simulated
microsecond is much slower than a real second). Eventually would like to make this more streamlined.

## Run Simulation (no FPGA required)
- Clone [neorv32 v1.12.1](https://github.com/stnolting/neorv32/tree/v1.12.1)
- Update `examples/memory.x` with your configuration
- Uncomment `# runner = "./run-sim"` from `examples/.cargo/config.toml`
- Update `embassy-neorv32/examples/run-sim` to your `neorv32` path
- Install [GHDL](https://github.com/ghdl/ghdl) simulator
- Install [llvm-objcopy](https://llvm.org/docs/CommandGuide/llvm-objcopy.html)
- Run `cd embassy-neorv32/examples`
- Run `cargo run --release --bin hello-world`

## Run on FPGA over serial bootloader
- Clone [neorv32 v1.12.1](https://github.com/stnolting/neorv32/tree/v1.12.1)
- Update `examples/memory.x` with your configuration
- Update `embassy-neorv32/examples/run-bl` to your `neorv32` path
- Install `picocom` (or modify `run-bl` to use your preferred tool)
- Follow instructions in `run-bl` if using `picocom`

## References
- [NEORV32 Datasheet](https://stnolting.github.io/neorv32/)
