use {saleae_csv::*, std::fs};

fn main() -> Result<(), &'static str> {
    let raw_data = fs::read_to_string("data/arduino-tx.csv").map_err(|_| "Could not open file")?;
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
    } else {
        eprintln!("Invalid UTF-8 data");
    }

    Ok(())
}
