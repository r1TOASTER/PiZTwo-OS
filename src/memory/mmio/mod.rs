use core::sync::atomic::{AtomicPtr, Ordering};
use core::marker::Copy;

// TODO: think about unsigned 0-32 bit trait for T
// TODO: think about 8/16 bit scenerio where use ptr offsets / no offsets with bit-wise to keep other register bits info

pub fn mmio_read<T: Copy>(addr: *mut T) -> T {
    let reg = AtomicPtr::new(addr);
    unsafe { *reg.load(Ordering::Acquire) }
}

pub fn mmio_write<T: Copy>(addr: *mut T, mut data: T) {
    let reg = AtomicPtr::new(addr);
    reg.store(&mut data as *mut T, Ordering::Relaxed);
}