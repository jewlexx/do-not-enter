//! Console sanity tests - RX, TX and statistics.

#![feature(format_args_nl)]
#![no_main]
#![no_std]

/// Console tests should time out on the I/O harness in case of panic.
mod panic_wait_forever;

use libkernel::{bsp, console, cpu, driver, exception, print};

#[no_mangle]
unsafe fn kernel_init() -> ! {
    use console::console;
    use driver::interface::DriverManager;

    exception::handling_init();
    bsp::driver::driver_manager().qemu_bring_up_console();

    // Handshake
    assert_eq!(console().read_char(), 'A');
    assert_eq!(console().read_char(), 'B');
    assert_eq!(console().read_char(), 'C');
    print!("OK1234");

    // 10
    // TODO: Figure out why it returns 10 and not 6
    print!("{}", console().chars_written());

    // 3
    print!("{}", console().chars_read());

    // The QEMU process running this test will be closed by the I/O test harness.
    cpu::wait_forever();
}
