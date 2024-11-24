#![no_std]
#![no_main]

fn main() { }

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop{}
}
