use core::result::Result;
use core::option::Option;

const UART1_ENABLE: u8 = 0; 
static mut UART1_ENABLED: bool = false;

enum UartErr {
    Init,
}

// TODO: bool if uart1 is enabled (maybe just global struct for peripherals info?)
// TODO: if not global struct and static bool, atomic read/write to prevent data race in multi core

/// brief - initalizing uart1 GPIO and setting the enabled flag
pub fn uart1_init() {
    // set gpio

    // check if clock is set

    // if error, return

    // maybe mutex here - for atomic read and write
    // atomic_store(UART1_ENABLED, true);
}

pub fn uart1_enable() -> Result<(), UartErr> {
    // check if gpio initialized first
    // if atomic_load::<bool, Acquire>(UART1_ENABLED) == false {
    //     return Err(UartErr::Init);
    // }

    // get AUXENB register - 0x7e21_5004
    // set bit UART1_ENABLE to 1

    return Ok(());
}

/// uart1 recv byte
pub fn uart1_recv() -> Option<u8> {
    None
}

/// uart1 send byte
pub fn uart1_send(_data: u8) {

}