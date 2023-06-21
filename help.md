
[links]
https://os.phil-opp.com/freestanding-rust-binary/
https://www.youtube.com/playlist?list=PLib6-zlkjfXkdCjQgrZhmfJOWBk_C2FTY


[this is for running without an underlying OS]
rustup target add thumbv7em-none-eabihf

[build the freestanding executable for this target]
cargo build --target thumbv7em-none-eabihf

[build for our own custom system]
cargo build --target x86_64-test_os.json

[need this component for  this]
rustup component add rust-src

[this is for the bootloader to work]
cargo install bootimage
cargo bootimage // to create the bootable image

bootloader = "0.9.8" NEED this version.

[boot OS into QEMU]
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-test_os.bin

[put the image onto bootable USB-Drive]
dd if=target/x86_64-blog_os/debug/bootimage-blog_os.bin of=/dev/sdX && sync