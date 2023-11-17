#![no_std] // Don't link the std library
#![no_main] // Disable default entry points

use core::panic::PanicInfo;

use kernel_gaming::{print, println};

// Handle panics because core doesn't have that
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kernel_gaming::hlt_loop();
}

#[no_mangle] // Don't skew the name
pub extern "C" fn _start() -> ! {
    kernel_gaming::init();
    
    println!("how are you?");
    println!("Does it work{}", "?");

    loop {}
}
