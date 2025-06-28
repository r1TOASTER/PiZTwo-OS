#![no_std]
#![no_main]

#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(ptr_metadata)]
#![feature(lang_items)]

// #[lang = "eh_personality"]
// extern fn eh_personality() {}

// #[no_mangle]
// pub extern fn __aeabi_unwind_cpp_pr0() {}

// #[allow(non_snake_case)]
// #[no_mangle]
// pub extern fn _Unwind_Resume() { loop {} }

// export?
pub mod buses;
pub mod common;
pub mod cpu;
pub mod graphics;
pub mod interrupt;
pub mod memory;
pub mod net;
pub mod peripherals;
pub mod process;
pub mod timer;

use core::intrinsics::abort;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO: panic message print to console
    abort()
}

// imports from raw assembly
unsafe extern {
    unsafe fn _prints_qemu(s: *const u8);
    unsafe fn _exit_qemu();
}

#[no_mangle]
#[inline(never)]
pub fn kernel_main() -> ! {
    // kernel main entry point, after entry.S initiallization
    unsafe {
        _prints_qemu(b"[Main Kernel]\n\0".as_ptr());
    }
    
    loop {}
}