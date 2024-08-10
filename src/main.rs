#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Panics are handled by the STD library, so we need to define our own.
// Also in Cargo.toml panic is set to abort to prevent stack unwinding which also reduces binary size
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// Override crt0
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 +  1) = 0xb;
        }
    }

    loop {}
}