#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &str = &"Yes, this is YesOS.";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_splash();
    println!("Yes, this is YesOS.");

    loop {}
}
