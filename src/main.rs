#![no_std]
#![no_main]

#![allow(internal_features)]
#![feature(core_intrinsics)]

mod buses;
mod common;
mod cpu;
mod graphics;
mod interrupt;
mod ipc;
mod memory;
mod net;
mod process;

use core::intrinsics::abort;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    // TODO: panic message print to console
    abort()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    loop {}
}
