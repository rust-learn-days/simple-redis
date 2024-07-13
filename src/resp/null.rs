use anyhow::Result;
use bytes::BytesMut;

use crate::resp::{extract_fixed_data, RespDecode, RespEncode, RespError};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TNull;

// - null: "_\r\n"
impl RespEncode for TNull {
    fn encode(self) -> Vec<u8> {
        "_\r\n".to_string().into_bytes()
    }
}

impl RespDecode for TNull {
    const PREFIX: &'static str = "_";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        match extract_fixed_data(buf, "_\r\n", "TNull") {
            Ok(_) => Ok(TNull),
            Err(e) => Err(e),
        }
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(3)
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::RespFrame;

    use super::*;

    #[test]
    fn test_null_encode() {
        let frame: RespFrame = TNull.into();
        assert_eq!(frame.encode(), b"_\r\n");
    }

    #[test]
    fn test_null_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"_\r\n");

        let frame = TNull::decode(&mut buf)?;
        assert_eq!(frame, TNull);

        Ok(())
    }
}
