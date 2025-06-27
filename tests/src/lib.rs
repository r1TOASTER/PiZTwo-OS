#![no_std]
#![no_main]

#![allow(internal_features)]
#![feature(core_intrinsics)]

#![feature(custom_test_frameworks)]
#![test_runner(my_test_runner)]
#![reexport_test_harness_main = "kernel_main"]

// imports from raw assembly
#[allow(missing_abi)]
unsafe extern {
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
    
    // memory mod tests
    let memory_tests: &[&dyn Fn()] = &[
        &memory::test_read_08,
        &memory::test_read_16,
        &memory::test_read_32,
        &memory::test_write_08,
        &memory::test_write_16,
        &memory::test_write_32,
    ];

    test_runner(memory_tests);
}

// Custom test runner
fn test_runner(tests: &[&dyn Fn()]) {

    unsafe {
        _prints_qemu("Starting Tests\n\x00".as_ptr());
    }
    
    for test in tests {
        // let current_test_str = "Doing test num: {i}\n\x00";
        // unsafe {
        //     TODO: implement a formatter to print shit
        //     _prints_qemu("Doing test num: \n\x00".as_ptr());
        // }
        test();
    }

    loop {}
}