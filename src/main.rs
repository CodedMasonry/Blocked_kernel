#![no_std] // Don't link the std library
#![no_main] // Disable default entry points

use core::panic::PanicInfo;

use kernel_gaming::{print, println};

// Handle panics because core doesn't have that
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // Don't skew the name
pub extern "C" fn _start() -> ! {
    kernel_gaming::init();

    unsafe {
        *(0xdeadbeef as *mut u8) = 43;
    };
    print!("how are you?");
    println!("Does it work{}", "?");
    println!("Test new line...");

    loop {}
}
