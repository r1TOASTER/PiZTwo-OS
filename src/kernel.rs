#![allow(internal_features)]

use core::intrinsics::abort;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    // TODO: panic message print to console
    abort()
}


// TODO: define in entry.S
#[no_mangle]
#[inline(never)]
pub fn kernel_main() -> ! {

    // some semihosting API testing from the main kernel entry point (after entry.S initialliztions)

    let char = 'A' as u8;
    let char_addr = &char as *const u8;
    
    // try to write the char A to the console using SWI on ARM
    unsafe {
        core::arch::asm!("
            mov x0, #0x03
            mov x1, {}
            hlt #0xF000
        ", in(reg) char_addr)
    };

    // exit QEMU using SWI on ARM
    unsafe { 
        core::arch::asm!("
            mov x0, #0x18
            mov x1, #0x20000
            add x1, x1, #0x26
            hlt #0xF000
        ")
    };

    loop {}
}
