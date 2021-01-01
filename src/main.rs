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

#[derive(Default, Debug)]
pub struct SerialData {
    opcode: Option<u8>,
    size: Option<u8>,
    data: Option<u16>,
}

/// parse bytes from serial, bytes [0..2] are opcode, [2..3] are size and
/// everything else is data, which is 4 bytes long max
pub fn parse_serial(bytes: &[u8]) -> SerialData {
    let mut sp = SerialData::default();
    let raw = std::str::from_utf8(&bytes).unwrap_or("bad string");

    sp.opcode = match raw[0..2].parse::<u8>() {
        Ok(opcode) => Some(opcode),
        Err(_e) => None,
    };
    sp.size = match raw[2..3].parse::<u8>() {
        Ok(size) => Some(size),
        Err(_e) => None,
    };

    let tmp = raw[3..].split("\r\n").collect::<Vec<_>>()[0];
    sp.data = match tmp.parse::<u16>() {
        Ok(data) => Some(data),
        Err(_e) => None,
    };
    sp
}

/// connect to device and print valid parsed data to terminal continuously,
/// if device is not available exit process.
pub fn test_serial(fd: &str) {
    if let Ok(mut port) = serialport::open_with_settings(fd, &PORT_SETTINGS) {
        let mut buff = vec![0; 10];
        loop {
            let bytes = port.read(&mut buff);
            match bytes {
                Ok(_b) => {
                    let sp = parse_serial(&buff);
                    if let Ok(raw) = std::str::from_utf8(&buff) {
                        println!("raw string {:?}", raw);
                    }
                    if sp.opcode.is_some() {
                        println!("opcode = {}", sp.opcode.unwrap());
                    }
                    if sp.size.is_some() {
                        println!("size = {}", sp.size.unwrap());
                    }
                    if sp.data.is_some() {
                        println!("data = {}", sp.data.unwrap());
                    }
                    println!("---------------------------------------------");
                    // TODO: find a better way
                    buff.iter_mut().for_each(|x| *x = 0);
                }
                Err(_e) => {}
            }
        }
    } else {
        println!("device is not available\n");
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
