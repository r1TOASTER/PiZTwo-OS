static mut TEST_REG_32: u32 = 0;
static mut TEST_REG_16: u16 = 0;
static mut TEST_REG_8: u8 = 0;

use kernel::memory::mmio::RegSized;
use crate::utils::console_assert_eq;

pub(crate) fn test_read_32() {
    unsafe {
        TEST_REG_32 = 1234567;
        let reg_addr: *mut u32 = &raw mut TEST_REG_32;
        console_assert_eq(u32::mmio_read(reg_addr), 1234567);
    }
}

pub(crate) fn test_read_16() {
    unsafe {
        TEST_REG_16 = 12345;
        let reg_addr: *mut u16 = &raw mut TEST_REG_16;
        console_assert_eq(u16::mmio_read(reg_addr), 12345);
    }
}

pub(crate) fn test_read_08() {
    unsafe {
        TEST_REG_8 = 234;
        let reg_addr: *mut u8 = &raw mut TEST_REG_8;
        console_assert_eq(u8::mmio_read(reg_addr), 234);
    }
}

pub(crate) fn test_write_32() {
    unsafe {
        TEST_REG_32 = 1234567;
        let reg_addr: *mut u32 = &raw mut TEST_REG_32;
        u32::mmio_write(reg_addr, 1234569);
        console_assert_eq(TEST_REG_32, 1234569);
    }
}

pub(crate) fn test_write_16() {
    unsafe {
        TEST_REG_16 = 1234;
        let reg_addr: *mut u16 = &raw mut TEST_REG_16;
        u16::mmio_write(reg_addr, 1236);
        console_assert_eq(TEST_REG_16, 1236);
    }
}

pub(crate) fn test_write_08() {
    unsafe {
        TEST_REG_8 = 208;
        let reg_addr: *mut u8 = &raw mut TEST_REG_8;
        u8::mmio_write(reg_addr, 210);
        console_assert_eq(TEST_REG_8, 210);
    }
}