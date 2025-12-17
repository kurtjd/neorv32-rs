# neorv32-pac
To re-generate this PAC:

```
svd2rust --target riscv --edition=2024 --settings config.yml -i neorv32.svd
rm -rf src
form -i lib.rs -o src
rm lib.rs
cargo fmt
```