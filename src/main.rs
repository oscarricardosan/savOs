#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

//Enabled custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(sav_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sav_os::{println, test_panic_handler};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    sav_os::init(); // new

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();


    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    loop {}
}

/// This function is called on panic.
/// Execute in context not test
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}


