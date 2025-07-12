// TODO: change self pull state
// TODO: pull up / down - clock ?
// TODO: pin level?
// TODO: pin event detect status?
// TODO: rising / falling edge detect - enable / disable
// TODO: detect high / low - enable / disable
// TODO: pin async - rising / falling edge - enable / disable

use crate::common::{get_reg_val, set_reg_val, delay_cycles};
use core::{{marker::Copy}, {clone::Clone}, {fmt::Debug}};

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub(crate) enum GpioPin {
    Pin00 = 0,
    Pin01,
    Pin02,
    Pin03,
    Pin04,
    Pin05,
    Pin06,
    Pin07,
    Pin08,
    Pin09,
    Pin10,
    Pin11,
    Pin12,
    Pin13,
    Pin14,
    Pin15,
    Pin16,
    Pin17,
    Pin18,
    Pin19,
    Pin20,
    Pin21,
    Pin22,
    Pin23,
    Pin24,
    Pin25,
    Pin26,
    Pin27,
    Pin28,
    Pin29,
    Pin30,
    Pin31,
    Pin32,
    Pin33,
    Pin34,
    Pin35,
    Pin36,
    Pin37,
    Pin38,
    Pin39,
    Pin40,
    Pin41,
    Pin42,
    Pin43,
    Pin44,
    Pin45,
    Pin46,
    Pin47,
    Pin48,
    Pin49,
    Pin50,
    Pin51,
    Pin52,
    Pin53
}

// defenitions by GPFSELn, page 91
#[derive(Copy, Clone, Debug)]
pub(crate) enum GpioState {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum GpioLevel {
    Low = 0,
    High = 1
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum PullState {
    Off = 0b00,
    PullDown = 0b01,
    PullUp = 0b10,
}

// GPIO configuration registers related, only 0 (add offsets where needed)
const GPFSEL0: u32 = 0x7E20_0000;
const GPSET0: u32 = 0x7E20_001C;
const GPCLR0: u32 = 0x7E20_0028;
const GPLEV0: u32 = 0x7E20_0034;
const GPEDS0: u32 = 0x7E20_0040;
const GPREN0: u32 = 0x7E20_004C;
const GPFEN0: u32 = 0x7E20_0058;
const GPHEN0: u32 = 0x7E20_0064;
const GPLEN0: u32 = 0x7E20_0070;
const GPAREN0: u32 = 0x7E20_007C;
const GPAFEN0: u32 = 0x7E20_0088;
const GPPUD: u32 = 0x7E20_0094;
const GPPUDCLK0: u32 = 0x7E20_0098;

const REG_SIZE: u32 = 4; // 32 bit registers

// TODO: write unit tests
pub(crate) trait GPIO: Sized + Copy + Clone {
    fn select_mode(self, state: GpioState);
    fn set_high(self);
    fn set_low(self);
    fn get_level(self) -> GpioLevel;
    fn consume_event(self) -> bool;
    fn set_rising_edge_detection(self, enable: bool);
    fn set_falling_edge_detection(self, enable: bool);
    fn set_high_detection(self, enable: bool);
    fn set_low_detection(self, enable: bool);
    fn set_async_rising_edge_detection(self, enable: bool);
    fn set_async_falling_edge_detection(self, enable: bool);
    fn set_pull_state(self, state: PullState);
}

impl GPIO for GpioPin {
    
    fn select_mode(self, state: GpioState) {
        let reg_offset: u32 = ((self as u32) / 10) * REG_SIZE; // 32 bit register, offset times 4 + base of GPFSEL0
        let reg = (GPFSEL0 + reg_offset) as *mut u32;

        let bit_offset = ((self as u8) % 10) * 3; // every mode is 3 bit, offset times 3 + base of register (bit 0)

        match set_reg_val(reg, state as u32, bit_offset, 3) {
            Ok(_) => {},
            Err(e) => todo!("console print error as debug {:?}", e)
        }
    }

    fn set_high(self) {
        let reg_offset: u32 = ((self as u32) / 32) * REG_SIZE;
        let reg = (GPSET0 + reg_offset) as *mut u32;

        let bit_offset: u8 = if (self as u8) > 31 {
            (self as u8) - 32
        } else {
            self as u8
        };

        // set the bit 1 to activate the output - set it high
        match set_reg_val(reg, 1, bit_offset, 1) {
            Ok(_) => {},
            Err(e) => todo!("console print error as debug {:?}", e)
        }
    }

    fn set_low(self) {
        let reg_offset: u32 = ((self as u32) / 32) * REG_SIZE;
        let reg = (GPCLR0 + reg_offset) as *mut u32;

        let bit_offset: u8 = if (self as u8) > 31 {
            (self as u8) - 32
        } else {
            self as u8
        };

        // set the bit 1 to clear the output - set it low
        match set_reg_val(reg, 1, bit_offset, 1) {
            Ok(_) => {},
            Err(e) => todo!("console print error as debug {:?}", e)
        }
    }

    fn get_level(self) -> GpioLevel {
        let reg_offset: u32 = ((self as u32) / 32) * REG_SIZE;
        let reg = (GPLEV0 + reg_offset) as *const u32;

        let bit_offset: u8 = if (self as u8) > 31 {
            (self as u8) - 32
        } else {
            self as u8
        };

        match get_reg_val(reg, bit_offset, 1) {
            Ok(val) => {
                return if val == 1 { GpioLevel::High } else { GpioLevel::Low };
            },
            Err(e) => todo!("console print error as debug {:?}", e)
        }
    }

    fn consume_event(self) -> bool {
        let reg_offset: u32 = ((self as u32) / 32) * REG_SIZE;
        let reg = (GPEDS0 + reg_offset) as *mut u32;

        let bit_offset: u8 = if (self as u8) > 31 {
            (self as u8) - 32
        } else {
            self as u8
        };

        match get_reg_val(reg, bit_offset, 1) {
            Ok(val) => {
                if val == 1 {
                    // if there is an event, clear it and then return true
                    match set_reg_val(reg, 1, bit_offset, 1) {
                        Ok(_) => {},
                        Err(e) => todo!("console print error as debug {:?}", e)
                    }
                    return true;
                }
                // no event - just return
                return false;
            },
            Err(e) => todo!("console print error as debug {:?}", e)
        }
    }

    fn set_rising_edge_detection(self, enable: bool) {
        let reg_offset: u32 = ((self as u32) / 32) * REG_SIZE;
        let reg = (GPREN0 + reg_offset) as *mut u32;

        let bit_offset: u8 = if (self as u8) > 31 {
            (self as u8) - 32
        } else {
            self as u8
        };

        // set the bit 1 to enable rising edge detection
        match set_reg_val(reg, enable as u32, bit_offset, 1) {
            Ok(_) => {},
            Err(e) => todo!("console print error as debug {:?}", e)
        }
    }

    fn set_falling_edge_detection(self, enable: bool) {
        let reg_offset: u32 = ((self as u32) / 32) * REG_SIZE;
        let reg = (GPFEN0 + reg_offset) as *mut u32;

        let bit_offset: u8 = if (self as u8) > 31 {
            (self as u8) - 32
        } else {
            self as u8
        };

        // set the bit 1 to enable falling edge detection
        match set_reg_val(reg, enable as u32, bit_offset, 1) {
            Ok(_) => {},
            Err(e) => todo!("console print error as debug {:?}", e)
        }
    }

    fn set_high_detection(self, enable: bool) {
        let reg_offset: u32 = ((self as u32) / 32) * REG_SIZE;
        let reg = (GPHEN0 + reg_offset) as *mut u32;

        let bit_offset: u8 = if (self as u8) > 31 {
            (self as u8) - 32
        } else {
            self as u8
        };

        // set the bit 1 to enable high level detection (doesnt matter what signal)
        match set_reg_val(reg, enable as u32, bit_offset, 1) {
            Ok(_) => {},
            Err(e) => todo!("console print error as debug {:?}", e)
        }
    }

    fn set_low_detection(self, enable: bool) {
        let reg_offset: u32 = ((self as u32) / 32) * REG_SIZE;
        let reg = (GPLEN0 + reg_offset) as *mut u32;

        let bit_offset: u8 = if (self as u8) > 31 {
            (self as u8) - 32
        } else {
            self as u8
        };

        // set the bit 1 to enable high level detection (doesnt matter what signal)
        match set_reg_val(reg, enable as u32, bit_offset, 1) {
            Ok(_) => {},
            Err(e) => todo!("console print error as debug {:?}", e)
        }
    }
    
    fn set_async_rising_edge_detection(self, enable: bool) {
        let reg_offset: u32 = ((self as u32) / 32) * REG_SIZE;
        let reg = (GPAREN0 + reg_offset) as *mut u32;

        let bit_offset: u8 = if (self as u8) > 31 {
            (self as u8) - 32
        } else {
            self as u8
        };

        // set the bit 1 to enable high level detection (doesnt matter what signal)
        match set_reg_val(reg, enable as u32, bit_offset, 1) {
            Ok(_) => {},
            Err(e) => todo!("console print error as debug {:?}", e)
        }
    }
    
    fn set_async_falling_edge_detection(self, enable: bool) {
        let reg_offset: u32 = ((self as u32) / 32) * REG_SIZE;
        let reg = (GPAFEN0 + reg_offset) as *mut u32;

        let bit_offset: u8 = if (self as u8) > 31 {
            (self as u8) - 32
        } else {
            self as u8
        };

        // set the bit 1 to enable high level detection (doesnt matter what signal)
        match set_reg_val(reg, enable as u32, bit_offset, 1) {
            Ok(_) => {},
            Err(e) => todo!("console print error as debug {:?}", e)
        }
    }
    
    fn set_pull_state(self, state: PullState) {
        // write to GPPUD - state
        set_reg_val(GPPUD as *mut u32, state as u32, 0, 2).unwrap();

        // wait 150 cycles
        delay_cycles(150);

        let reg_offset: u32 = ((self as u32) / 32) * REG_SIZE;
        let reg = (GPPUDCLK0 + reg_offset) as *mut u32;

        let bit_offset: u8 = if (self as u8) > 31 {
            (self as u8) - 32
        } else {
            self as u8
        };

        // Write to GPPUDCLK0/1 - 1
        set_reg_val(reg, 1, bit_offset, 1).unwrap();

        // wait 150 cycles
        delay_cycles(150);

        // clear GPPUD - state
        set_reg_val(GPPUD as *mut u32, 0, 0, 2).unwrap();

        // clear GPPUDCLK0/1 - 0
        set_reg_val(reg, 0, bit_offset, 1).unwrap();
    }
}