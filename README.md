# neorv32-rs
This repo provides Rust support for the open-source [NEORV32](https://github.com/stnolting/neorv32) RISC-V microcontroller in the form of two crates:
- [embassy-neorv32](embassy-neorv32): An async-friendly hardware abstraction layer (HAL) meant for use with [Embassy](https://github.com/embassy-rs/embassy) or any other lightweight async executor. This provides a safe, high-level library for accessing the various peripherals.
- [neorv32-pac](neorv32-pac): An auto-generated peripheral access crate (PAC) for low-level register access to various peripherals which `embassy-neorv32` depends on. This can be used alongside the HAL for when finer control is needed, however this is inherently unsafe and should only be done so with extreme caution.

## Future
NEORV32 is actively being developed which means breaking changes may be introduced to peripheral
register interfaces over time. See [Version](#version) for the currently supported version.
Additionally, the HAL does not yet offer support for all peripherals available.

Having said that, I will likely not develop drivers for additional peripherals or support newer
versions unless there is interest (please open an issue if so). Or better yet, open a PR!

## Contributing
PRs for bug fixes, new peripheral drivers, and new version support are highly encouraged and welcomed!

## Version
These crates have been developed for NEORV32 [v1.12.6](https://github.com/stnolting/neorv32/tree/v1.12.6).
There is no guarantee they will work for different versions.

## License
This project is licensed under the MIT license and is completely free to use and modify.
