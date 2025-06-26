// This is to expose the crate for the tests dir

#![no_std]
#![no_main]

#![allow(internal_features)]
#![feature(core_intrinsics)]

pub mod buses;
pub mod common;
pub mod cpu;
pub mod graphics;
pub mod interrupt;
pub mod ipc;
pub mod memory;
pub mod net;
pub mod peripherals;
pub mod process;
pub mod timer;
pub mod kernel;