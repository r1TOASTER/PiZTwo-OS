use crate::_prints_qemu;

pub(crate) fn console_assert_eq<T: core::cmp::PartialEq>(left: T, right: T) {
    if left != right {
        unsafe { _prints_qemu("Failed assert\n\x00".as_ptr()); }
        loop {}
    }
}