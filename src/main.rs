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

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    let heap_value = Box::new(41);
    // println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    // println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    // println!(
    //     "current reference count is {}",
    //     Rc::strong_count(&cloned_reference)
    // );
    core::mem::drop(reference_counted);
    // println!(
    //     "reference count is {} now",
    //     Rc::strong_count(&cloned_reference)
    // );

    #[cfg(test)]
    test_main();

    println!(" ");
    // claw_os::hlt_loop();
    // claw_os::init();
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
