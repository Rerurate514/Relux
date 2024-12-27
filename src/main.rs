#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop{}
}

static STRING: &[u8] = b"LIKE A PHOENIX!!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("HELLO WORLD{}", "!");

	loop { }
}
