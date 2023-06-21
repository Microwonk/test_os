# [Open this OS through QEMU]
> qemu-system-x86_64 -drive format=raw,file=target/x86_64-test_os/debug/bootimage-test_os.bin
> 'cargo run' to build and run

# [Build through this command]
> cargo bootimage

# [Filepath for the Image Binary]
target/x86_64-test_os/debug/bootimage-test_os.bin