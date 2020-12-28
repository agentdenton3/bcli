use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::path::Path;
use std::time::Duration;

pub fn save_data(fd: &str) {
    let port_setting = SerialPortSettings {
        baud_rate: 115_200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(10),
    };
    if let Ok(mut port) = serialport::open_with_settings(fd, &port_setting) {
        let mut buff = vec![0; 10];
        loop {
            let bytes = port.read(&mut buff);
            match bytes {
                Ok(_b) => {
                    // get bytes from serial
                    let raw =
                        std::str::from_utf8(&buff).unwrap_or("bad string");
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
                            let d = elem.parse::<u16>().unwrap_or(0);
                            (d, l + 1)
                        }
                        None => (0, 1),
                    };
                    if size as usize == l {
                        println!("raw = {:?}", raw);
                        println!("size = {}", size);
                        println!("l = {}", l);
                        println!("data = {}", data);
                    }
                    // TODO: fint a better way
                    // clear buffer manually to align data,
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

pub fn wtf() -> String {
    let mut count = 0;
    let name = "data";
    let file_path = "data/";
    if let Ok(dir) = std::fs::read_dir(file_path) {
        // get all files in the dir
        let mut v = dir.map(|d| d.unwrap().path()).collect::<Vec<_>>();
        if v.is_empty() {
            // create initial count file
            std::fs::File::create(format!("{}{}", file_path, count)).unwrap();
            // create initial data file
            std::fs::File::create(format!("{}{}_{}", file_path, name, count));
        } else {
            v.sort();
            // get the count file
            let first = v.first().unwrap();
            // parse and then increment count
            count = first.to_str().unwrap().split("/").collect::<Vec<_>>()[1]
                .parse::<u32>()
                .unwrap()
                + 1;
            // path of the new file
            let tmp = format!("{}{}_{}", file_path, name, count);
            if Path::new(&tmp).exists() {
                println!("path already exists!");
            } else {
                // rename(increment) count file
                std::fs::rename(first, format!("{}{}", file_path, count));
                // create data file with a new counter value
                std::fs::File::create(tmp).unwrap();
            }
        }
    } else {
        // if there is not dir, create one
        std::fs::create_dir(file_path).unwrap();
        // create initial count file
        std::fs::File::create(format!("{}{}", file_path, count)).unwrap();
        // create initial data file
        std::fs::File::create(format!("{}{}_{}", file_path, name, count));
    }
    format!("{}{}_{}", file_path, name, count)
}

fn main() {
    let s = wtf();
    println!("{:?}", s);
}
