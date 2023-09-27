#![no_std] // Don't link the std library
#![no_main] // Disable default entry points

mod vga_buffer;

use core::panic::PanicInfo;

// Handle panics because core doesn't have that
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // Don't skew the name
pub extern "C" fn _start() -> ! {
    vga_buffer::test_print();

    loop {}
}