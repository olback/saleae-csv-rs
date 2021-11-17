use {saleae_csv::*, std::fs};

fn main() -> Result<(), &'static str> {
    let raw_data = fs::read_to_string("data/arduino-i2c.csv").map_err(|_| "Could not open file")?;
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
