use core::panic::PanicInfo;
use core::intrinsics::abort;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    // TODO: panic message print to console
    abort()
}