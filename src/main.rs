#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop{}
}

static STRING: &[u8] = b"LIKE A PHOENIX!!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
	use core::fmt::Write;
	vga_buffer::WRITER.lock().write_str("LIKE A PHOENIX!").unwrap();
	write!(vga_buffer::WRITER.lock(), ", numbers: {} {}", 42, 1.3213).unwrap();
	vga_buffer::print_something();

	loop { }
}
