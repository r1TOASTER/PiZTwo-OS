use core::marker::Sized;

// TODO: maybe about 8/16 bit scenerio where use ptr offsets / no offsets with bit-wise to keep other register bits info

pub trait RegSized: Sized {
    unsafe fn mmio_read(addr: usize) -> Self;
    unsafe fn mmio_write(addr: usize, value: Self);
}

macro_rules! rdwr {
    ($t:ident($(reg: $reg_cls:ident, mmio_read: $patrd:literal, mmio_write: $patwr:literal),* $(,)?)) => {
        #[cfg(target_arch = "aarch64")]
        impl RegSized for $t {
            unsafe fn mmio_read(addr: usize) -> $t {
                unsafe {
                    $(#[cfg(target_arch = "aarch64")] {
                        let val: $t;
                        core::arch::asm!($patrd, addr = in(reg) addr, val = out($reg_cls) val);
                        return val;
                    })*
                }
            }
            unsafe fn mmio_write(addr: usize, val: $t) {
                unsafe {
                    $(#[cfg(target_arch = "aarch64")] { core::arch::asm!($patwr, addr = in(reg) addr, val = in($reg_cls) val); })*
                }
            }
        }
    };
}

rdwr!(u8(
    reg: reg, mmio_read: "ldrb {val}, [{addr}]", mmio_write: "strb {val}, [{addr}]",
));
rdwr!(u16(
    reg: reg, mmio_read: "ldrh {val}, [{addr}]", mmio_write: "strh {val}, [{addr}]",
));
rdwr!(u32(
    reg: reg, mmio_read: "ldrw {val}, [{addr}]", mmio_write: "strw {val}, [{addr}]",
));
rdwr!(u64(
    reg: reg, mmio_read: "ldr {val}, [{addr}]", mmio_write: "str {val}, [{addr}]",
));
