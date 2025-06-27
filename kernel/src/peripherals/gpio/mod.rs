// TODO: change pin pull state
// TODO: pull up / down - clock ?
// TODO: pin level?
// TODO: pin event detect status?
// TODO: rising / falling edge detect - enable / disable
// TODO: detect high / low - enable / disable
// TODO: pin async - rising / falling edge - enable / disable

use crate::memory::mmio::RegSized;
use core::{{marker::Copy}, {clone::Clone}, {fmt::Debug}};

const GPFSEL0: u32 = 0x7E20_0000;

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

// TODO: write unit tests
pub(crate) fn gpio_select_mode(pin: GpioPin, state: GpioState) {
    let reg_offset: u32 = ((pin as u32) / 10) * 4; // 32 bit register, offset times 4 + base of GPFSEL0
    let reg = (GPFSEL0 + reg_offset) as *mut u32;

    let bit_offset = ((pin as u8) % 10) * 3; // every mode is 3 bit, offset times 3 + base of register (bit 0)

    let mut og_val = unsafe { u32::mmio_read(reg) };
    
    og_val &= 0b111 << bit_offset;
    og_val |= ((state as u8) << bit_offset) as u32;

    unsafe { u32::mmio_write(reg, og_val); }
}