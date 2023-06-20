
[this is for running without an underlying OS]
rustup target add thumbv7em-none-eabihf

[buil the freestanding executable for this target]
cargo build --target thumbv7em-none-eabihf