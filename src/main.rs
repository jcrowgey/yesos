#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(yesos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use yesos::println;
use yesos::vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_splash();
    println!("Yes, this is YesOS.");

    yesos::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    yesos::test_panic_handler(info)
}
