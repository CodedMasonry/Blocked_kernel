#![no_std] // Don't link the std library
#![no_main] // Disable default entry points

use core::panic::PanicInfo;

// Handle panics because core doesn't have that
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // Don't skew the name
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;


    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}