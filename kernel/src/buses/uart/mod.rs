use core::option::Option;

// GPIO 14 is UART1 TX
// GPIO 15 is UART1 RX

// TODO: maybe global struct for peripherals / buses info
// TODO uart1 functions - init gpio, init uart, enable / disable, recv / send, console wrapper?

// internal function to setup gpios 14 and 15
fn uart1_gpio_init() {
    // set pins direction

    // set alternate functions

    // set pull state
}

/// brief - initalizing uart1 GPIO and setting the enabled flag
pub(crate) fn uart1_init() {
    // set gpio pins

    // enable the peripheral on 0x7E21_5004 bit 0 -> turn on

    // MAYBE enable the DLAB access - on 0x7E21_504C bit 7 -> turn on
    // if enabled - disable at the end of baudrate config

    // set the data size to 8-bit mode - on 9x7E21_504C bits 0:1 -> turn on (0b11)

    // set baudrate - system_clock_freq / (8 * (baudrate_reg + 1))
    // for baudrate of 125,000 => 
    // set baudrate using baudrate_reg - on 0x7E21_5068 first 16 bit

    // if DLAB access enabled - disable now

    // enable interrupts
}

pub(crate) fn uart1_enable() -> bool {
    // check if gpio initialized first
    // if atomic_load::<bool, Acquire>(UART1_ENABLED) == false {
    //     return Err(UartErr::Init);
    // }

    // get AUXENB register - 0x7e21_5004
    // set bit UART1_ENABLE to 1

    return true;
}

pub(crate) fn uart1_disable() {
    panic!();
}

/// uart1 recv byte
pub(crate) fn uart1_recv() -> Option<u8> {
    todo!()
}

/// uart1 send byte
pub(crate) fn uart1_send(_data: u8) {
    todo!()
}