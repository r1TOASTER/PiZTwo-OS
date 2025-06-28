#![no_std]
#![no_main]

#![allow(internal_features)]
#![feature(core_intrinsics)]

#![feature(custom_test_frameworks)]
#![test_runner(my_test_runner)]
#![reexport_test_harness_main = "kernel_main"]

// imports from raw assembly
unsafe extern "C" {
    unsafe fn _exit_qemu();
    unsafe fn _prints_qemu(s: *const u8);
}

// this crate's modules
mod memory;
mod utils;

#[no_mangle]
#[inline(never)]
pub fn kernel_main() {

    unsafe { _exit_qemu(); }
    
    // every mod's test main
    let main_test_functions: &[&dyn Fn()] = &[
        &memory::memory_test_main,
    ];

    test_runner(main_test_functions);
}

static STARTING_TESTS: &[u8] = b"Starting Tests\n\0";

// Custom test runner
fn test_runner(tests: &[&dyn Fn()]) {

    unsafe {
        _prints_qemu(STARTING_TESTS.as_ptr());
    }
    
    for test_mains in tests {
        // let current_test_str = "Doing test num: {i}\n\x00";
        // unsafe {
        //     TODO: implement a formatter to print shit
        //     _prints_qemu("Doing test num: \n\x00".as_ptr());
        // }
        test_mains();
    }

    loop {}
}