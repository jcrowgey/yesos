#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(yesos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use yesos::println;
use yesos::vga_buffer;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use yesos::memory::translate_addr;
    use x86_64::{structures::paging::PageTable, VirtAddr};

    vga_buffer::print_splash();
    println!("Yes, this is YesOS.");

    yesos::init();
    println!("Physical memory fully mapped at offset: {:?}", 
             boot_info.physical_memory_offset);

    /* let l4_table = unsafe {
        active_level_4_table(boot_info.physical_memory_offset)
    };
    */

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x20010a,
        // some stack page
        0x57ac_001f_fe48,
        // virtual address mapped to a physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe {
            translate_addr(virt, boot_info.physical_memory_offset)
        };
        println!("{:?} -> {:?}", virt, phys);
    }


    #[cfg(test)]
    test_main();

    println!("Ready...");
    yesos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    yesos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    yesos::test_panic_handler(info)
}
