use core::str::FromStr;

/// Time since capture start
#[derive(Debug, Clone, Copy)]
pub struct StartTime(pub f64);

impl TryFrom<&str> for StartTime {
    type Error = <f64 as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(value.parse()?))
    }
}

/// CSV Row type
#[derive(Debug, Clone, Copy)]
pub enum RowType {
    /// Address
    Address,
    /// Start
    Start,
    /// Stop
    Stop,
    /// Data
    Data,
}

impl TryFrom<&str> for RowType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().replace("\"", "").to_ascii_lowercase().as_str() {
            "address" => Ok(Self::Address),
            "start" => Ok(Self::Start),
            "stop" => Ok(Self::Stop),
            "data" => Ok(Self::Data),
            _ => Err(Error::InvalidRow),
        }
    }
}

/// Saleae Logic 2 Channel Name
#[derive(Debug, Clone)]
pub struct ChannelName(String);

impl<T: AsRef<str>> From<T> for ChannelName {
    fn from(value: T) -> Self {
        Self(value.as_ref().trim().replace("\"", ""))
    }
}

/// Error type
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// Failed to parse
    Parse,
    /// Invalid CSV row
    InvalidRow,
}

/// Parse hex number formatted as '0xFF'
pub(crate) fn parse_hex(input: &str) -> Option<u8> {
    let chars = input
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<char>>();
    match *chars.as_slice() {
        ['0', 'x', u, l] => Some(((hex_nibble_to_byte(u)?) << 4) | (hex_nibble_to_byte(l)?)),
        _ => None,
    }
}

/// Convert hex nibble to byte
pub(crate) fn hex_nibble_to_byte(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some((c as u8) - 48),
        'a'..='f' => Some((c as u8) - 55 - 32),
        _ => None,
    }
}
