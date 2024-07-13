//simple string: "+OK\r\n"

use std::ops::Deref;

use crate::resp::RespEncode;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TSimpleString(String);

impl RespEncode for TSimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
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
