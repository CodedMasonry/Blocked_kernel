#![no_std] // Don't link the std library
#![no_main] // Disable default entry points

mod vga_buffer;

use core::{panic::PanicInfo, fmt::Write};

// Handle panics because core doesn't have that
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // Don't skew the name
pub extern "C" fn _start() -> ! {
    
    vga_buffer::WRITER.lock().write_str("Hello there").unwrap();
    print!(", how are you?");
    println!("Does it work{}", "!");
    println!("Test new line...");
    
    loop {}
}