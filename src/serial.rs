use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut seral_port = unsafe {
            SerialPort::new(0x3F8)
        };
        seral_port.init();
        Mutex::new(seral_port)
    };
}
