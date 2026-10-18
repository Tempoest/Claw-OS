#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(claw_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{BootInfo, entry_point};
use claw_os::println;
use core::panic::PanicInfo;

extern crate alloc;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use claw_os::allocator;
    use claw_os::memory;
    use claw_os::memory::BootInfoFrameAllocator;
    // use x86_64::{VirtAddr, structures::paging::Translate};
    use x86_64::{VirtAddr, structures::paging::Page};

    println!("Hello Terra!{}", "!");
    claw_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    let x = Box::new(41);

    let heap_value = Box::new(41);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    core::mem::drop(reference_counted);

    #[cfg(test)]
    test_main();

    println!(" ");
    claw_os::shell::run();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    claw_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    claw_os::test_panic_handler(info)
}
