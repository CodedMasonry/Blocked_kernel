#![no_std] // Don't link the std library
#![no_main] // Disable default entry points

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use kernel_gaming::{println, hlt_loop};

// Handle panics because core doesn't have that
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kernel_gaming::hlt_loop();
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use kernel_gaming::memory::active_level_4_table;
    use x86_64::VirtAddr;
    kernel_gaming::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry: {}: {:?}", i, entry);
        }
    }

    println!("how are you?");
    println!("Does it work{}", "?");

    hlt_loop();
}
