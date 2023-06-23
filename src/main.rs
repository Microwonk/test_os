#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use test_os::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use test_os::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};

    println!("Hello World{}", "!");

    test_os::init();

    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        // new: use the `mapper.translate_addr` method
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    test_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    test_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_os::test_panic_handler(info)
}