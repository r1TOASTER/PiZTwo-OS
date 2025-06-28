use crate::_prints_qemu;
use core::cmp::PartialEq;

static FAILED_ASSERT: &[u8] = b"Failed Assert\n\0";

pub(crate) fn console_assert_eq<T: PartialEq>(left: T, right: T) {
    if left != right {
        unsafe { _prints_qemu(FAILED_ASSERT.as_ptr()); }
        loop {}
    }
}