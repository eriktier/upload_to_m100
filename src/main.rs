extern crate serial;
extern crate ascii;

use std::env;
use std::fs;
use std::io;
use std::time::Duration;

use std::thread;

use serial::prelude::*;


fn main() {
    let arg = env::args_os().skip(1).next().unwrap();

    let file = fs::read_to_string(arg).unwrap();

    let mut port = serial::open("/dev/tty.usbserial-1410").unwrap();
    interact(&mut port, &file).unwrap();
}

fn interact<T: SerialPort>(port: &mut T, file: &str) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud300)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(1000))?;

    let mut buf = file.to_string();
    buf.push_str("\u{001A}");
    print!("{}", buf);
    port.write_all(buf.as_bytes()).unwrap();
    let flush = port.flush();
    if let Err(e) = flush {
        println!("{}", e.to_string());
    }
    loop {
        thread::sleep(Duration::from_millis(100));
    }
}