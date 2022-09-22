#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static TXT: &[u8] = b"Hello fuckers! Wassup, I love doing this cool shit!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buff = 0xb8000 as *mut u8;
    for (i, &byte) in TXT.iter().enumerate() {
        unsafe {
            *vga_buff.offset(i as isize * 2) = byte;
            *vga_buff.offset(i as isize * 2 + 1) = 0xd;
        }
    }

    loop {}
}
