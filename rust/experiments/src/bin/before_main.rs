#![no_std]
#![no_main]
#![feature(rustc_private)]
extern crate libc;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Note, that need to pass "-- -C panic=abort -C link-arg=-nostartfiles" build command.
