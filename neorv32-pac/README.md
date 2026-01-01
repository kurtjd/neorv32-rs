# neorv32-pac
A peripheral access crate (PAC) for the open-source [NEORV32](https://github.com/stnolting/neorv32)
RISC-V microcontroller.

## Overview
A PAC acts as a thin wrapper around raw peripheral register reads/writes
allowing maximum flexibility for interacting with the hardware. However, it is the user's responsbility
to ensure the hardware is used safely and correctly. For a higher-level, safer and more ergonomic
approach to accessing hardware, consider using the [NEORV32 Embassy HAL](../embassy-neorv32/) instead.

## Generation
This PAC is automatically generated using [svd2rust](https://crates.io/crates/svd2rust) and
[svdtools](https://crates.io/crates/svdtools). Additionally, [form](https://crates.io/crates/form)
is used to properly format and structure to generated Rust code.

To re-generate this PAC, install the above tools and simply run `./genpac.sh` from a terminal
(though this is only necessary if you modify `neorv32.svd` and/or `config.yml`).

## Version
This PAC has been generated for NEORV32 [v1.12.6](https://github.com/stnolting/neorv32/tree/v1.12.6).
There is no guarantee it will work for different versions.