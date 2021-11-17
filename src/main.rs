use {
    saleae_csv::*,
    std::{env, fs},
};

pub enum FileType {
    I2C,
    Serial,
}

impl TryFrom<&str> for FileType {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "i2c" => Ok(Self::I2C),
            "serial" => Ok(Self::Serial),
            _ => Err(Self::Error::default()),
        }
    }
}

fn main() -> Result<(), &'static str> {
    let input_args = env::args().skip(1).collect::<Vec<_>>();
    let input_args_str = input_args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let mut input_args_str_slice = input_args_str.as_slice();

    let mut input_file = None;
    let mut input_mode = None;

    loop {
        match input_args_str_slice {
            ["-h" | "--help", ..] => {
                help();
                return Ok(());
            }
            ["-f" | "--file", filename, rest @ ..] => {
                input_file = Some(filename.to_string());
                input_args_str_slice = rest;
            }
            ["-t" | "--type", mode, rest @ ..] => {
                input_mode = FileType::try_from(*mode).ok();
                input_args_str_slice = rest;
            }
            [..] => break,
        }
    }

    match (input_file, input_mode) {
        (Some(input_file), Some(input_mode)) => match input_mode {
            FileType::I2C => {
                let _ = handle_i2c(&input_file);
            }
            FileType::Serial => {
                let _ = handle_serial(&input_file);
            }
        },
        _ => {
            help();
            return Err("Invalid or no argument");
        }
    }

    Ok(())
}

fn help() {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!(
        "Usage: {name} --file <filename> --type i2c|serial",
        name = env!("CARGO_PKG_NAME")
    );
}

fn handle_i2c(path: &str) -> Result<(), &'static str> {
    let raw_data = fs::read_to_string(path).map_err(|_| "Could not open file")?;
    let frames = raw_data
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .filter_map(|row| I2CFrame::try_from(row).ok())
        .collect::<Vec<_>>();

    if frames.is_empty() {
        return Err("File empty");
    }

    for packet in I2CPacket::from_frames(&frames, false) {
        if packet.read {
            print!("Read  from 0x{:02x}, data: ", packet.address);
        } else {
            print!("Write to   0x{:02x}, data: ", packet.address);
        }
        let data = packet
            .data
            .iter()
            .map(|b| format!("0x{:02x}", b))
            .collect::<Vec<_>>()
            .join(", ");
        println!("[{}]", data);
    }

    Ok(())
}

fn handle_serial(path: &str) -> Result<(), &'static str> {
    let raw_data = fs::read_to_string(path).map_err(|_| "Could not open file")?;
    let packets = raw_data
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .filter_map(|row| SerialPacket::try_from(row).ok())
        .collect::<Vec<_>>();

    if packets.is_empty() {
        return Err("File empty");
    }

    if let Ok(s) = serial_packets_to_utf8(&packets) {
        println!("{}", s)
    }

    Ok(())
}
