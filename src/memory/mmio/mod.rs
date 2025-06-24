use core::marker::Sized;

// TODO: maybe about 8/16 bit scenerio where use ptr offsets / no offsets with bit-wise to keep other register bits info

trait RegSized: Sized {
    unsafe fn read(addr: usize) -> Self;
    unsafe fn write(addr: usize, value: Self);
}
macro_rules! rdwr {
    ($t:ident($(reg: $reg_cls:ident, read: $patrd:literal, write: $patwr:literal),* $(,)?)) => {
        #[cfg(target_arch = "aarch64")]
        impl RegSized for $t {
            unsafe fn read(addr: usize) -> $t {
                unsafe {
                    $(#[cfg(target_arch = "aarch64")] {
                        let val: $t;
                        core::arch::asm!($patrd, addr = in(reg) addr, val = out($reg_cls) val);
                        return val;
                    })*
                }
            }
            unsafe fn write(addr: usize, val: $t) {
                unsafe {
                    $(#[cfg(target_arch = "aarch64")] { core::arch::asm!($patwr, addr = in(reg) addr, val = in($reg_cls) val); })*
                }
            }
        }
    };
}

rdwr!(u8(
    reg: reg, read: "ldrb {val}, [{addr}]", write: "strb {val}, [{addr}]",
));
rdwr!(u16(
    reg: reg, read: "ldrh {val}, [{addr}]", write: "strh {val}, [{addr}]",
));
rdwr!(u32(
    reg: reg, read: "ldrw {val}, [{addr}]", write: "strw {val}, [{addr}]",
));
rdwr!(u64(
    reg: reg, read: "ldr {val}, [{addr}]", write: "str {val}, [{addr}]",
));
