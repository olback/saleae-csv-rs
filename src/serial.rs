use {
    crate::common::{parse_hex, ChannelName, Error, RowType, StartTime},
    std::time::Duration,
};

/// Serial packet
#[derive(Debug)]
pub struct SerialPacket {
    /// Saleae Logic 2 channel name
    pub channel_name: ChannelName,
    /// Row type
    pub row_type: RowType,
    /// Time since capture start
    pub start_time: StartTime,
    /// Transaction duration
    pub duration: Duration,
    /// Data sent/received
    pub data: u8,
}

impl TryFrom<&str> for SerialPacket {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cols = value.trim().split(',').collect::<Vec<_>>();
        if cols.len() != 5 {
            return Err(Error::InvalidRow);
        }

        Ok(Self {
            channel_name: ChannelName::from(cols[0]),
            row_type: RowType::try_from(cols[1]).map_err(|_| Error::Parse)?,
            start_time: StartTime::try_from(cols[2]).map_err(|_| Error::Parse)?,
            duration: Duration::from_secs_f64(cols[3].parse::<f64>().map_err(|_| Error::Parse)?),
            data: parse_hex(cols[4]).ok_or(Error::Parse)?,
        })
    }
}

/// Try to create a UTF-8 string from serial packets
pub fn serial_packets_to_utf8(
    packets: &[SerialPacket],
) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(packets.iter().map(|p| p.data).collect::<Vec<_>>())
}

// impl<A: AsRef<[SerialPacket]>> TryFrom<A> for String {
//     type Error = std::string::FromUtf8Error;

//     fn try_from(value: A) -> Result<Self, Self::Error> {
//         let a = value.as_ref().iter().map(|p| p.data).collect::<Vec<_>>();
//         String::from_utf8(a)
//     }
// }
