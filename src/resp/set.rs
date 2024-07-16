use std::ops::Deref;

use anyhow::Result;
use bytes::{Buf, BytesMut};

use crate::resp::{
    calc_total_length, parse_length, RespDecode, RespEncode, RespError, RespFrame, CRLF_LEN,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TSet(pub(crate) Vec<RespFrame>);

impl Deref for TSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// - set: "~<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for TSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());
        for frame in self.0 {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

impl RespDecode for TSet {
    const PREFIX: &'static str = "~";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;

        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len() < total_len {
            return Err(RespError::NotCompleteFrame);
        }

        buf.advance(end + CRLF_LEN);

        let mut frames = Vec::new();
        for _ in 0..len {
            frames.push(RespFrame::decode(buf)?);
        }

        Ok(TSet::new(frames))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

impl TSet {
    pub fn new(set: impl Into<Vec<RespFrame>>) -> Self {
        TSet(set.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::{TArray, TBulkString};

    use super::*;

    #[test]
    fn test_set_encode() {
        let frame: RespFrame = TSet::new([
            TArray::new([1234.into(), true.into()]).into(),
            TBulkString::new("world".to_string()).into(),
        ])
        .into();
        assert_eq!(
            frame.encode(),
            b"~2\r\n*2\r\n:1234\r\n#t\r\n$5\r\nworld\r\n"
        );
    }
    #[test]
    fn test_set_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"~2\r\n$3\r\nset\r\n$5\r\nhello\r\n");

        let frame = TSet::decode(&mut buf)?;
        assert_eq!(
            frame,
            TSet::new(vec![
                TBulkString::new(b"set".to_vec()).into(),
                TBulkString::new(b"hello".to_vec()).into()
            ])
        );

        Ok(())
    }
}
