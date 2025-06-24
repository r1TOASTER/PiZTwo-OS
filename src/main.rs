#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

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
mod panic;
mod process;

#[no_mangle]
pub extern "C" fn kernel_main() {
    loop {}
}
