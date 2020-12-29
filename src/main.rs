use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::path::Path;
use std::time::Duration;

static FILE_NAME: &str = "data";
static FILE_PATH: &str = "data/";
static PORT_SETTINGS: SerialPortSettings = SerialPortSettings {
    baud_rate: 115_200,
    data_bits: DataBits::Eight,
    flow_control: FlowControl::None,
    parity: Parity::None,
    stop_bits: StopBits::One,
    timeout: Duration::from_millis(10),
};

// TODO: maybe add new(), and check on creation if data is valid
#[derive(Debug)]
pub struct SerialData {
    opcode: u8,
    size: u8,
    data: Option<u16>,
}

impl SerialData {
    pub fn new(opcode: u8, size: u8, data: Option<u16>) -> Self {
        Self { opcode, size, data }
    }
}

/// parse raw data from serial port
/// n - length with \n
/// l - length without \n
pub fn parse_serial(raw: &str) -> Option<SerialData> {
    // get bytes from serial
    let s = raw.split("\r\n").collect::<Vec<_>>();
    // first element is opcode
    let opcode = match s.get(0) {
        Some(elem) => elem.parse::<u8>().unwrap_or(56),
        None => 0,
    };
    // second element is size of msg in bytes
    let size = match s.get(1) {
        Some(elem) => elem.parse::<u8>().unwrap_or(0),
        None => 0,
    };
    // get length of the msg and then compare it with size
    // l + 1 because we are considering \n at the end of msg
    let (data, l) = match s.get(2) {
        Some(elem) => {
            let l = elem.len() as u8;
            let d = elem.parse::<u16>().unwrap_or(0);
            (d, l + 1)
        }
        None => (0, 1),
    };
    if size == l {
        Some(SerialData::new(opcode, size, Some(data)))
    } else {
        None
    }
}

pub fn test_serial(fd: &str) {
    if let Ok(mut port) = serialport::open_with_settings(fd, &PORT_SETTINGS) {
        let mut buff = vec![0; 10];
        loop {
            let bytes = port.read(&mut buff);
            match bytes {
                Ok(_b) => {
                    let raw =
                        std::str::from_utf8(&buff).unwrap_or("bad string");
                    println!("{:?}", raw);
                    if let Some(sd) = parse_serial(&raw) {
                        println!("st = {:?}", sd);
                    }
                    // if sd.size == sd.l {
                    //     println!("raw = {:?}", raw);
                    //     println!("size = {}", sd.size);
                    //     println!("data = {}", sd.data);
                    //     println!("--------------------");
                    // }
                    // // TODO: find a better way
                    // // clear buffer manually to align data,
                    // // because buff.clear() does not workk
                    // buff.iter_mut().for_each(|x| *x = 0);
                }
                Err(_e) => {}
            }
        }
    } else {
        std::process::exit(0);
    }
}

pub fn save_data() {}

/// this function is responsible for data file creation, user should not
/// create files by himself, it can lead to unpredictable behavior
// TODO: add ability to create file manually, without this breaking
pub fn create_data_file() -> String {
    let mut count = 0;
    if let Ok(dir) = std::fs::read_dir(FILE_PATH) {
        // get all files in the dir
        let mut v = dir.map(|d| d.unwrap().path()).collect::<Vec<_>>();
        if v.is_empty() {
            // create initial count file
            std::fs::File::create(format!("{}{}", FILE_PATH, count)).unwrap();
            // create initial data file
            std::fs::File::create(format!(
                "{}{}_{}",
                FILE_PATH, FILE_NAME, count
            ))
            .unwrap();
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
            let tmp = format!("{}{}_{}", FILE_PATH, FILE_NAME, count);
            if Path::new(&tmp).exists() {
                println!("path already exists!");
            } else {
                // rename(increment) count file
                std::fs::rename(first, format!("{}{}", FILE_PATH, count))
                    .unwrap();
                // create data file with a new counter value
                std::fs::File::create(tmp).unwrap();
            }
        }
    } else {
        // if there is not dir, create one
        std::fs::create_dir(FILE_PATH).unwrap();
        // create initial count file
        std::fs::File::create(format!("{}{}", FILE_PATH, count)).unwrap();
        // create initial data file
        std::fs::File::create(format!("{}{}_{}", FILE_PATH, FILE_NAME, count))
            .unwrap();
    }
    format!("{}{}_{}", FILE_PATH, FILE_NAME, count)
}

fn main() {
    test_serial("/dev/ttyUSB0");
}
