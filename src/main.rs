#![no_std]
#![no_main]

mod vga_buffers;

use core::panic::PanicInfo;

// Panics are handled by the STD library, so we need to define our own.
// Also in Cargo.toml panic is set to abort to prevent stack unwinding which also reduces binary size
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Override crt0
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world {}", 42);
    loop {}
}
