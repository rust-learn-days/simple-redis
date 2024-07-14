use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use anyhow::Result;
use bytes::{Buf, BytesMut};

use crate::resp::{
    calc_total_length, parse_length, RespDecode, RespEncode, RespError, RespFrame, TSimpleString,
    BUF_CAP, CRLF_LEN,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TMap(pub(crate) BTreeMap<String, RespFrame>);

impl Deref for TMap {
    type Target = BTreeMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
impl RespEncode for TMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (key, value) in self.0 {
            buf.extend_from_slice(&TSimpleString::new(key).encode());
            buf.extend_from_slice(&value.encode());
        }
        buf
    }
}

impl RespDecode for TMap {
    const PREFIX: &'static str = "%";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len() < total_len {
            return Err(RespError::NotCompleteFrame);
        }

        buf.advance(end + CRLF_LEN);

        let mut frames = TMap::new();
        for _ in 0..len {
            let key = TSimpleString::decode(buf)?;
            let value = RespFrame::decode(buf)?;
            frames.insert(key.0, value);
        }

        Ok(frames)
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

impl Default for TMap {
    fn default() -> Self {
        Self::new()
    }
}

impl TMap {
    pub fn new() -> Self {
        TMap(BTreeMap::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::TBulkString;

    use super::*;

    #[test]
    fn test_map_encode() {
        let mut map = TMap::new();
        map.insert(
            "hello".to_string(),
            TBulkString::new("world".to_string()).into(),
        );
        map.insert("foo".to_string(), (-123456.789).into());

        let frame: RespFrame = map.into();
        assert_eq!(
            &frame.encode(),
            b"%2\r\n+foo\r\n,-123456.789\r\n+hello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_map_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"%2\r\n+hello\r\n$5\r\nworld\r\n+foo\r\n$3\r\nbar\r\n");

        let frame = TMap::decode(&mut buf)?;
        let mut map = TMap::new();
        map.insert(
            "hello".to_string(),
            TBulkString::new(b"world".to_vec()).into(),
        );
        map.insert("foo".to_string(), TBulkString::new(b"bar".to_vec()).into());
        assert_eq!(frame, map);

        Ok(())
    }
}
