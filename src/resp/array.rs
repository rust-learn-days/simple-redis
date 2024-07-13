use std::ops::Deref;

use crate::resp::{RespEncode, RespFrame, BUF_CAP};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TArray(Vec<RespFrame>);

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

// - null array: "*-1\r\n"
impl RespEncode for TNullArray {
    fn encode(self) -> Vec<u8> {
        "*-1\r\n".to_string().into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::TBulkString;

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
}
