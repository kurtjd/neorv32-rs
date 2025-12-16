# embassy-neorv32
An Embassy HAL for the RISCV-based [NEORV32](https://github.com/stnolting/neorv32) SoC/microcontroller

**NOTE**: Unfortunately, NEORV32 is changing faster than I can produce this HAL, so efforts are currently paused until NEORV32 is a bit more stabilized. This project was started on `v1.12.1` but as of the time of this writing NEORV32 is on `v1.12.4` which has already introduced breaking changes to the register interface.

*HOWEVER*, I'm continuing to improve the HALs in other ways, just not focusing on adding peripheral drivers
at the moment.

## Overview
This HAL currently supports the below peripherals for NEORV32 `v1.12.1`.

Please see `embassy-neorv32/examples` for guidance on how to use this HAL in your own projects. Instructions for running the examples can be found below.

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
- SysInfo

## Additional Features
- Dual-core support

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