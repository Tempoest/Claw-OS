#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(claw_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use claw_os::println;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    claw_os::init();

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("Everything works fine");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    claw_os::test_panic_handler(info)
}
