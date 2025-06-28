// static mut TEST_REG_32: u32 = 0;
// static mut TEST_REG_16: u16 = 0;
// static mut TEST_REG_8: u8 = 0;

use kernel::memory::mmio::RegSized;
use crate::{_prints_qemu, utils::console_assert_eq};

fn test_read_write_08(reg_addr: *mut u8, reg: u8) {
    unsafe {
        // TEST_REG_8 = 234;
        console_assert_eq(u8::mmio_read(reg_addr), 234);

        // TEST_REG_8 = 208;
        u8::mmio_write(reg_addr, 210);
        console_assert_eq(reg, 210);
    }
}

fn test_read_write_16(reg_addr: *mut u16, reg: u16) {
    unsafe {
        // TEST_REG_16 = 12345;
        console_assert_eq(u16::mmio_read(reg_addr), 12345);

        // TEST_REG_16 = 1234;
        u16::mmio_write(reg_addr, 1236);
        console_assert_eq(reg, 1236);
    }
}

fn test_read_write_32(reg_addr: *mut u32, reg: u32) {
        unsafe {
            // TEST_REG_32 = 1234567;
            console_assert_eq(u32::mmio_read(reg_addr), 1234567);
        
            // TEST_REG_32 = 1234567;
            u32::mmio_write(reg_addr, 1234569);
            console_assert_eq(reg, 1234569);
    }
}

pub(crate) fn memory_test_main() {
    
    unsafe {
        let mut u32t: u32 = 0;
        let mut u16t: u16 = 0;
        let mut u8t: u8 = 0;
        
        let test_reg_8_ptr: *mut u8 = &mut u8t;
        let test_reg_16_ptr: *mut u16 = &mut u16t;
        let test_reg_32_ptr: *mut u32 = &mut u32t;

        if test_reg_8_ptr.is_null() || test_reg_16_ptr.is_null() || test_reg_32_ptr.is_null() {
            _prints_qemu(b"An address is Null, aborting\n\0".as_ptr());
            return;
        }

        test_read_write_08(test_reg_8_ptr, u8t);
        test_read_write_16(test_reg_16_ptr, u16t);
        test_read_write_32(test_reg_32_ptr, u32t);
    }
}