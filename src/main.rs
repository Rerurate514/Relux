#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use x86_64::instructions::port;

mod vga_buffer;
mod serial;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

	exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion(){
	print!("trivial assertion... ");
	assert_eq!(1, 2);
	println!("[ok]");
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4);
		port.write(exit_code as u32);
	}
}
