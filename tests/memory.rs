
static mut TEST_REG_32: u32 = 0;
static mut TEST_REG_16: u16 = 0;
static mut TEST_REG_8: u8 = 0;

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_32() {
        unsafe {
            TEST_REG_32 = 1234567;
            let reg_addr: *mut u32 = &mut TEST_REG_32;
            assert_eq!(mmio_read::<u32>(reg_addr), 1234567);
        }
    }

    #[test]
    fn test_read_16() {
        unsafe {
            TEST_REG_16 = 12345;
            let reg_addr: *mut u16 = &mut TEST_REG_16;
            assert_eq!(mmio_read::<u16>(reg_addr), 12345);
        }
    }

    #[test]
    fn test_read_08() {
        unsafe {
            TEST_REG_8 = 234;
            let reg_addr: *mut u8 = &mut TEST_REG_8;
            assert_eq!(mmio_read::<u8>(reg_addr), 234);
        }
    }

    #[test]
    fn test_write_32() {
        unsafe {
            TEST_REG_32 = 1234567;
            let reg_addr: *mut u32 = &mut TEST_REG_32;
            mmio_write::<u32>(reg_addr, 1234569);
            assert_eq!(TEST_REG_32, 1234569);
        }
    }

    #[test]
    fn test_write_16() {
        unsafe {
            TEST_REG_16 = 1234;
            let reg_addr: *mut u16 = &mut TEST_REG_16;
            mmio_write::<u16>(reg_addr, 1236);
            assert_eq!(TEST_REG_16, 1236);
        }
    }

    #[test]
    fn test_write_08() {
        unsafe {
            TEST_REG_8 = 208;
            let reg_addr: *mut u8 = &mut TEST_REG_8;
            mmio_write::<u8>(reg_addr, 210);
            assert_eq!(TEST_REG_8, 210);
        }
    }
}