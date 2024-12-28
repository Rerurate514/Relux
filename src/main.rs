#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop{}
}

static STRING: &[u8] = b"LIKE A PHOENIX!!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("HELLO WORLD{}", "!");

	#[cfg(test)]
	test_main();

	loop { }
}
