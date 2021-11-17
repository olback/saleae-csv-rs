use {
    crate::common::{parse_hex, ChannelName, Error, RowType, StartTime},
    std::time::Duration,
};

/// One I2C Frame
#[derive(Debug, Clone)]
pub struct I2CFrame {
    /// Channel name in Saleae Logic 2
    pub channel_name: ChannelName,
    /// Row type in Saleae Logic 2
    pub row_type: RowType,
    /// Start time relative to capture start time
    pub start_time: StartTime,
    /// Duration the transaction took
    pub duration: Duration,
    /// Transaction data
    pub data: I2CData,
}

/// I2C Frame data
#[derive(Debug, Clone, Copy)]
pub enum I2CData {
    /// I2C Start condition
    Start,
    /// I2C Stop condition
    Stop,
    /// I2C Address
    Address {
        /// I2C Address
        address: u8,
        /// Read/Write
        read: bool,
        /// Acknowledged
        ack: bool,
    },
    /// I2C Data
    Data {
        /// Data
        data: u8,
        /// Acknowledged
        ack: bool,
    },
}

impl TryFrom<&str> for I2CFrame {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cols = value.trim().split(',').collect::<Vec<_>>();
        if cols.len() != 8 {
            return Err(Error::InvalidRow);
        }

        let row_type = RowType::try_from(cols[1]).map_err(|_| Error::Parse)?;

        let data = match row_type {
            RowType::Start => I2CData::Start,
            RowType::Stop => I2CData::Stop,
            RowType::Address => I2CData::Address {
                address: parse_hex(cols[5]).ok_or(Error::Parse)?,
                read: cols[6].parse::<bool>().map_err(|_| Error::Parse)?,
                ack: cols[4].parse().map_err(|_| Error::Parse)?,
            },
            RowType::Data => I2CData::Data {
                data: parse_hex(cols[7]).ok_or(Error::Parse)?,
                ack: cols[4].parse().map_err(|_| Error::Parse)?,
            },
        };

        Ok(Self {
            channel_name: ChannelName::from(cols[0]),
            row_type,
            start_time: StartTime::try_from(cols[2]).map_err(|_| Error::Parse)?,
            duration: Duration::from_secs_f64(cols[3].parse::<f64>().map_err(|_| Error::Parse)?),
            data,
        })
    }
}

/// I2C Packet, Start to Stop
#[derive(Debug, Clone)]
pub struct I2CPacket {
    /// I2C address
    pub address: u8,
    /// Read/Write
    pub read: bool,
    /// Data associated with transaction
    pub data: Vec<u8>,
}

impl I2CPacket {
    /// Get packets from a slice of I2CFrames.
    /// require_ack will filter out data that did not receive an ack.
    pub fn from_frames(frames: &[I2CFrame], require_ack: bool) -> Vec<Self> {
        let mut ret = Vec::new();

        let mut packet_addr = None;
        let mut packet_read = None;
        let mut packet_data = None::<Vec<u8>>;

        for frame in frames {
            match frame.data {
                I2CData::Start => {
                    packet_addr = None;
                    packet_read = None;
                    packet_data = Some(Vec::new());
                }
                I2CData::Stop => {
                    if let (Some(pa), Some(pr), Some(pd)) = (packet_addr, packet_read, packet_data)
                    {
                        ret.push(I2CPacket {
                            address: pa,
                            read: pr,
                            data: pd.clone(),
                        })
                    }
                    packet_addr = None;
                    packet_read = None;
                    packet_data = None;
                }
                I2CData::Address { address, read, ack } => {
                    if let Some(ref _pd) = packet_data {
                        match (require_ack, ack) {
                            (true, true) | (false, _) => {
                                packet_addr = Some(address);
                                packet_read = Some(read);
                            }
                            _ => {}
                        }
                    }
                }
                I2CData::Data { data, ack } => {
                    if let Some(ref mut pd) = packet_data {
                        match (require_ack, ack) {
                            (true, true) | (false, _) => pd.push(data),
                            _ => {}
                        }
                    }
                }
            }
        }

        ret
    }
}
