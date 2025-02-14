use std::ops::{Deref, DerefMut};

use anyhow::Result;
use bytes::{Buf, BytesMut};

use crate::resp::{extract_fixed_data, parse_length, RespDecode, RespEncode, RespError, CRLF_LEN};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TBulkString(pub Vec<u8>);

impl Deref for TBulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TBulkString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TBulkString {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        TBulkString(data.into())
    }
}

impl AsRef<[u8]> for TBulkString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<&str> for TBulkString {
    fn from(s: &str) -> Self {
        TBulkString(s.as_bytes().to_vec())
    }
}

impl From<String> for TBulkString {
    fn from(s: String) -> Self {
        TBulkString(s.into_bytes())
    }
}

impl From<&[u8]> for TBulkString {
    fn from(s: &[u8]) -> Self {
        TBulkString(s.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for TBulkString {
    fn from(s: &[u8; N]) -> Self {
        TBulkString(s.to_vec())
    }
}

// - bulk string: "$<length>\r\n<data>\r\n"
impl RespEncode for TBulkString {
    fn encode(self) -> Vec<u8> {
        if self.is_empty() {
            return b"$-1\r\n".to_vec();
        }
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(format!("${}\r\n", self.len()).as_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl RespDecode for TBulkString {
    const PREFIX: &'static str = "$";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        if extract_fixed_data(buf, "$-1\r\n", "TBulkString").is_ok() {
            return Ok(TBulkString::new(Vec::new()));
        }
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let remained = &buf[end + CRLF_LEN..];
        if remained.len() < len + CRLF_LEN {
            return Err(RespError::NotCompleteFrame);
        }

        buf.advance(end + CRLF_LEN);

        let data = buf.split_to(len + CRLF_LEN);
        Ok(TBulkString::new(data[..len].to_vec()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN + len + CRLF_LEN)
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::RespFrame;

    use super::*;

    #[test]
    fn test_bulk_string_encode() {
        let frame: RespFrame = TBulkString::new(b"hello".to_vec()).into();
        assert_eq!(frame.encode(), b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_null_bulk_string_encode() {
        let frame: RespFrame = TBulkString::new(Vec::new()).into();
        assert_eq!(frame.encode(), b"$-1\r\n");
    }

    #[test]
    fn test_bulk_string_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"$5\r\nhello\r\n");

        let frame = TBulkString::decode(&mut buf)?;
        assert_eq!(frame, TBulkString::new(b"hello"));

        buf.extend_from_slice(b"$5\r\nhello");
        let ret = TBulkString::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotCompleteFrame);

        buf.extend_from_slice(b"\r\n");
        let frame = TBulkString::decode(&mut buf)?;
        assert_eq!(frame, TBulkString::new(b"hello"));

        Ok(())
    }

    #[test]
    fn test_null_bulk_string_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"$-1\r\n");

        let frame = TBulkString::decode(&mut buf)?;
        let null_bulk_string = TBulkString::new(Vec::new());
        assert_eq!(frame, null_bulk_string);

        Ok(())
    }
}
