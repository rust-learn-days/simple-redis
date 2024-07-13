use std::ops::Deref;

use anyhow::Result;
use bytes::{Buf, BytesMut};

use crate::resp::RespDecode;
use crate::resp::{
    calc_total_length, extract_fixed_data, parse_length, RespEncode, RespError, RespFrame, BUF_CAP,
    CRLF_LEN,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TArray(pub Vec<RespFrame>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TNullArray;

impl Deref for TArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TArray {
    pub fn new(data: impl Into<Vec<RespFrame>>) -> Self {
        TArray(data.into())
    }
}

// - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for TArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("*{}\r\n", self.0.len()).into_bytes());
        for frame in self.0 {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

impl RespDecode for TArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut BytesMut) -> anyhow::Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len() < total_len {
            return Err(RespError::NotCompleteFrame);
        }

        buf.advance(end + CRLF_LEN);

        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            frames.push(RespFrame::decode(buf)?);
        }

        Ok(TArray::new(frames))
    }

    fn expect_length(buf: &[u8]) -> anyhow::Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

// - null array: "*-1\r\n"
impl RespEncode for TNullArray {
    fn encode(self) -> Vec<u8> {
        "*-1\r\n".to_string().into_bytes()
    }
}

impl RespDecode for TNullArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "*-1\r\n", "NullArray")?;
        Ok(TNullArray)
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(4)
    }
}

impl From<Vec<RespFrame>> for TArray {
    fn from(s: Vec<RespFrame>) -> Self {
        TArray(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::TBulkString;

    use super::*;

    #[test]
    fn test_array_encode() {
        let frame: RespFrame = TArray::new(vec![
            TBulkString::new("set".to_string()).into(),
            TBulkString::new("hello".to_string()).into(),
            TBulkString::new("world".to_string()).into(),
        ])
        .into();
        assert_eq!(
            &frame.encode(),
            b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_null_array_encode() {
        let frame: RespFrame = TNullArray.into();
        assert_eq!(frame.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*-1\r\n");

        let frame = TNullArray::decode(&mut buf)?;
        assert_eq!(frame, TNullArray);

        Ok(())
    }

    #[test]
    fn test_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n$5\r\nhello\r\n");

        let frame = TArray::decode(&mut buf)?;
        assert_eq!(frame, TArray::new([b"set".into(), b"hello".into()]));

        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n");
        let ret = TArray::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotCompleteFrame);

        buf.extend_from_slice(b"$5\r\nhello\r\n");
        let frame = TArray::decode(&mut buf)?;
        assert_eq!(frame, TArray::new([b"set".into(), b"hello".into()]));

        Ok(())
    }
}
