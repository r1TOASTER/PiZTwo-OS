use core::intrinsics::abort;
use core::panic::PanicInfo;

#[cfg_attr(not(test), panic_handler)]
pub fn panic(_info: &PanicInfo) -> ! {
    // TODO: panic message print to console
    abort()
}
