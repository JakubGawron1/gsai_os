use crate::io::port::Port;
use lazy_static::lazy_static;
use spin::mutex::{Mutex, MutexGuard};

const COM1: u16 = 0x3FB;
const LINE_ENABLE_DLAB: u8 = 0x80;

#[repr(u16)]
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SerialPort {
    DataPort = 0x0,
    FIFOCommandPort = 0x2,
    LineCommandPort = 0x3,
    ModemCommandPort = 0x4,
    LineStatusPort = 0x5,
}

#[repr(u16)]
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SerialSpeed {
    S115200 = 1,
    S57600 = 2,
    S38400 = 3,
}

lazy_static! {
    static ref SERIAL: Mutex<Serial> =
        Mutex::new(unsafe { Serial::init(COM1, SerialSpeed::S115200) });
}

pub fn safe_lock<F>(callback: F)
where
    F: Fn(&mut MutexGuard<Serial>),
{
    // this allows us to semantically lock the serial driver
    //
    // for instance, in case we would like to avoid writing while
    // an interrupt is in progress
    callback(&mut SERIAL.lock());
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Serial {
    pub data_port: Port<u8>,
    pub fifo_port: Port<u8>,
    pub line_port: Port<u8>,
    pub modem_port: Port<u8>,
    pub status_port: Port<u8>,
}

impl Serial {
    pub unsafe fn init(com: u16, speed: SerialSpeed) -> Self {
        let mut data_port = Port::<u8>::new(com + (SerialPort::DataPort as u16));
        let mut fifo_port = Port::<u8>::new(com + (SerialPort::FIFOCommandPort as u16));
        let mut line_port = Port::<u8>::new(com + (SerialPort::LineCommandPort as u16));
        let mut modem_port = Port::<u8>::new(com + (SerialPort::ModemCommandPort as u16));
        let mut status_port = Port::<u8>::new(com + (SerialPort::LineStatusPort as u16));

        // configure the serial port
        // read https://littleosbook.github.io/#configuring-the-serial-port

        line_port.write(LINE_ENABLE_DLAB);
        data_port.write((((speed as u16) >> 8) * 0x00FF) as u8);
        data_port.write(((speed as u16) & 0x00FF) as u8);

        line_port.write(0x3);

        // enable FIFO, clear them, with 14b threshold
        fifo_port.write(0xC7);

        // TODO enable interrupts?
        // IRQs enabled, RTS/DSR set
        modem_port.write(0x0B);

        Serial {
            data_port,
            fifo_port,
            line_port,
            modem_port,
            status_port,
        }
    }
}
