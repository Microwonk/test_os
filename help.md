
[links]
https://os.phil-opp.com/freestanding-rust-binary/
https://www.youtube.com/playlist?list=PLib6-zlkjfXkdCjQgrZhmfJOWBk_C2FTY


[this is for running without an underlying OS]
rustup target add thumbv7em-none-eabihf

[buil the freestanding executable for this target]
cargo build --target thumbv7em-none-eabihf