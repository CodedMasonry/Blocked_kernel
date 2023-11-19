#![no_std] // Don't link the std library
#![no_main] // Disable default entry points

use core::panic::PanicInfo;

use kernel_gaming::{println, hlt_loop};

// Handle panics because core doesn't have that
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kernel_gaming::hlt_loop();
}

#[no_mangle] // Don't skew the name
pub extern "C" fn _start() -> ! {
    kernel_gaming::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table starts at: {:?}", level_4_page_table.start_address());

    println!("how are you?");
    println!("Does it work{}", "?");

    hlt_loop();
}
