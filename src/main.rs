use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::time::Duration;

const PORT: &str = "/dev/ttyUSB0";

fn main() {
    let port_setting = SerialPortSettings {
        baud_rate: 115_200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(10),
    };
    if let Ok(mut port) = serialport::open_with_settings(PORT, &port_setting) {
        let mut buff = vec![0; 10];
        loop {
            let bytes = port.read(&mut buff);
            match bytes {
                Ok(_b) => {
                    // get bytes from serial
                    let raw = std::str::from_utf8(&buff).unwrap_or("bad string");
                    // first element is size of msg in bytes
                    let s = raw.split("\r\n").collect::<Vec<_>>();
                    let size = match s.get(0) {
                        Some(elem) => elem.parse::<u8>().unwrap_or(0),
                        None => 0,
                    };
                    // get length of the msg and then compare it with size
                    // l + 1 because we are considering \n at the end of msg
                    let (data, l) = match s.get(1) {
                        Some(elem) => {
                            let l = elem.len();
                            let d = elem.parse::<u16>().unwrap_or(999);
                            (d, l + 1)
                        }
                        None => (0, 0),
                    };
                    if size as usize == l {
                        println!("raw = {:?}", raw);
                        println!("size = {}", size);
                        println!("l = {}", l);
                        println!("data = {}", data);
                    }
                    // TODO: fint a better way
                    // clear buffer manually,
                    // because buff.clear() does not workk
                    buff.iter_mut().for_each(|x| *x = 0);
                }
                Err(_e) => {}
            }
        }
    } else {
        std::process::exit(0);
    }
}
