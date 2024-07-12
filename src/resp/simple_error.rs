use std::ops::Deref;

use crate::resp::RespEncode;

#[derive(Debug, Clone)]
pub struct TError(String);

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
}
