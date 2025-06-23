#![no_std]
#![no_main]

#![allow(internal_features)]
#![feature(core_intrinsics)]

mod cpu;
mod graphics;
mod interrupt;
mod ipc;
mod mm;
mod net;
mod panic;
mod process;
mod util;

#[no_mangle]
pub extern "C" fn kernel_main() {
    loop {}
}
