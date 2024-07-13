use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use crate::resp::{RespEncode, RespFrame, TSimpleString, BUF_CAP};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TMap(BTreeMap<String, RespFrame>);

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
}
