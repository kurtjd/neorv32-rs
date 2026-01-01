# Delete old files
rm neorv32.svd.patched
rm device.x
rm build.rs
rm -rf src

# Patch the upstream SVD file to remove interrupts (we define them in config.yml as Core Interrupts)
svdtools patch neorv32.svd-patch.yaml
# Then generate the PAC using the config and patched SVD
svd2rust --target riscv --edition=2024 --settings config.yml -i neorv32.svd.patched
# Finally properly split up the monolithic file into a nice folder structure
form -i lib.rs -o src
# And cleanup
rm lib.rs
cargo fmt