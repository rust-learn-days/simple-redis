use std::ops::Deref;

use anyhow::Result;
use bytes::BytesMut;

use crate::resp::{extract_fixed_data, RespDecode, RespEncode, RespError};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TBulkString(Vec<u8>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TNullBulkString;

impl Deref for TBulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TBulkString {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        TBulkString(data.into())
    }
}

// - bulk string: "$<length>\r\n<data>\r\n"
impl RespEncode for TBulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(format!("${}\r\n", self.len()).as_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

// - null bulk string: "$-1\r\n"
impl RespEncode for TNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

impl RespDecode for TNullBulkString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        match extract_fixed_data(buf, "$-1\r\n", "TNullBulkString") {
            Ok(_) => Ok(TNullBulkString),
            Err(e) => Err(e),
        }
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
        let frame: RespFrame = TNullBulkString.into();
        assert_eq!(frame.encode(), b"$-1\r\n");
    }

    #[test]
    fn test_null_bulk_string_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"$-1\r\n");

        let frame = TNullBulkString::decode(&mut buf)?;
        assert_eq!(frame, TNullBulkString);

        Ok(())
    }
}
