#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(core_intrinsics)]

mod common;
mod cpu;
mod graphics;
mod interrupt;
mod ipc;
mod memory;
mod net;
mod panic;
mod process;

#[no_mangle]
pub extern "C" fn kernel_main() {
    loop {}
}
