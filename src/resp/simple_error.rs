use std::ops::Deref;

use anyhow::Result;
use bytes::BytesMut;

use crate::resp::{extract_simple_frame_data, RespDecode, RespEncode, RespError, CRLF_LEN};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TError(pub(crate) String);

impl Deref for TError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// - error: "-Error message\r\n"
impl RespEncode for TError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

impl RespDecode for TError {
    const PREFIX: &'static str = "-";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        // split the buffer
        let data = buf.split_to(end + CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(TError::new(s.to_string()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

impl TError {
    pub fn new(data: impl Into<String>) -> Self {
        TError(data.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::RespFrame;

    use super::*;

    #[test]
    fn test_error_encode() {
        let frame: RespFrame = TError::new("Error message".to_string()).into();

        assert_eq!(frame.encode(), b"-Error message\r\n");
    }

    #[test]
    fn test_simple_error_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"-Error message\r\n");

        let frame = TError::decode(&mut buf)?;
        assert_eq!(frame, TError::new("Error message".to_string()));

        Ok(())
    }
}
