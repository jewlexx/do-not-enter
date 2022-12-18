/// Overwrites libkernel's `panic_wait::_panic_exit()` with the QEMU-exit version.
#[no_mangle]
fn _panic_exit() -> ! {
    libkernel::cpu::qemu_exit_success()
}
