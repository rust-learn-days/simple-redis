use std::ops::Deref;

use anyhow::Result;
use bytes::BytesMut;

use crate::resp::{extract_simple_frame_data, RespDecode, RespEncode, RespError, CRLF_LEN};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TSimpleString(pub(crate) String);

//simple string: "+OK\r\n"
impl RespEncode for TSimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

impl RespDecode for TSimpleString {
    const PREFIX: &'static str = "~";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        // split the buffer
        let data = buf.split_to(end + CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(TSimpleString::new(s.to_string()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

impl Deref for TSimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TSimpleString {
    pub fn new(data: impl Into<String>) -> Self {
        TSimpleString(data.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::RespFrame;

    use super::*;

    #[test]
    fn test_simple_string_encode() {
        let frame: RespFrame = TSimpleString::new("OK".to_string()).into();

        assert_eq!(frame.encode(), b"+OK\r\n");
    }
}
