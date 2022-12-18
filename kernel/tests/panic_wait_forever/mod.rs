/// Overwrites libkernel's `panic_wait::_panic_exit()` with wait_forever.
#[no_mangle]
fn _panic_exit() -> ! {
    libkernel::cpu::wait_forever()
}
