#![no_std]
#![no_main]

#![allow(internal_features)]
#![feature(core_intrinsics)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use core::intrinsics::abort;

extern crate piztwo_os_lib;

// Custom test runner
pub fn test_runner(tests: &[&dyn Fn()]) {

    for (_, test) in tests.iter().enumerate() {
        test();
    }

    loop {}
}

// Panic handler for tests
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    abort()
}

// Include test modules
mod memory;