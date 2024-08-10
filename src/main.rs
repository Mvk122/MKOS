#![no_std]
#![no_main]

use core::panic::PanicInfo;


// Panics are handled by the STD library, so we need to define our own.
// Also in Cargo.toml panic is set to abort to prevent stack unwinding which also reduces binary size
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Override crt0
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}