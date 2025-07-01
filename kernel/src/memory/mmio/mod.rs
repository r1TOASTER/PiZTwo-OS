use core::marker::Sized;

// TODO: maybe about 8/16 bit scenerio where use ptr offsets / no offsets with bit-wise to keep other register bits info

pub trait RegSized: Sized {
    unsafe fn mmio_read(addr: *const Self) -> Self {
        core::intrinsics::volatile_load(addr)
    }
    unsafe fn mmio_write(addr: *mut Self, value: Self) {
        core::intrinsics::volatile_store(addr, value);
    }
}

impl RegSized for u32 {}
impl RegSized for u64 {}

// macro_rules! rdwr {
//     ($t:ident($(reg: $reg_cls:ident, mmio_read: $patrd:literal, mmio_write: $patwr:literal),* $(,)?)) => {
//         #[cfg(target_arch = "aarch64")]
//         impl<T: Sized> RegSized<T> for $t {
//             unsafe fn mmio_read(addr: *mut T) -> $t {
//                 unsafe {
//                     $(#[cfg(target_arch = "aarch64")] {
//                         let val: $t;
//                         core::arch::asm!($patrd, addr = in(reg) addr, val = out($reg_cls) val);
//                         return val;
//                     })*
//                 }
//             }
//             unsafe fn mmio_write(addr: *mut T, val: $t) {
//                 unsafe {
//                     $(#[cfg(target_arch = "aarch64")] { 
//                         core::arch::asm!($patwr, addr = in(reg) addr, val = in($reg_cls) val);
//                     })*
//                 }
//             }
//         }
//     };
// }

// rdwr!(u8(
//     reg: reg, mmio_read: "ldrb {val:w}, [{addr}]", mmio_write: "strb {val:w}, [{addr}]",
// ));
// rdwr!(u16(
//     reg: reg, mmio_read: "ldrh {val:w}, [{addr}]", mmio_write: "strh {val:w}, [{addr}]",
// ));
// rdwr!(u32(
//     reg: reg, mmio_read: "ldr {val:w}, [{addr}]", mmio_write: "str {val:w}, [{addr}]",
// ));
// rdwr!(u64(
//     reg: reg, mmio_read: "ldr {val:x}, [{addr}]", mmio_write: "str {val:x}, [{addr}]",
// ));
