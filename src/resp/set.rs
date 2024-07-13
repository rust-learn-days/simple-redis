use std::ops::Deref;

use crate::resp::{RespEncode, RespFrame};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TSet(Vec<RespFrame>);

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
            b"~2\r\n*2\r\n:+1234\r\n#t\r\n$5\r\nworld\r\n"
        );
    }
}
