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
    use yesos::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    vga_buffer::disable_cursor();
    vga_buffer::print_splash();
    println!("Yes, this is YesOS.");


    yesos::init();
    println!("Physical memory fully mapped at offset: {:?}",
             boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map a page and write "new" on the vga buffer
    let page = Page::containing_address(VirtAddr::new(0xbadface));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

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
