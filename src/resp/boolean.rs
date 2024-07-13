use anyhow::Result;
use bytes::BytesMut;

use crate::resp::{extract_fixed_data, RespDecode, RespEncode, RespError};

// - boolean: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        format!("#{}\r\n", if self { 't' } else { 'f' }).into_bytes()
    }
}

impl RespDecode for bool {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        match extract_fixed_data(buf, "#t\r\n", "Bool") {
            Ok(_) => Ok(true),
            Err(_) => match extract_fixed_data(buf, "#f\r\n", "Bool") {
                Ok(_) => Ok(false),
                Err(f) => Err(f),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::RespFrame;

    use super::*;

    #[test]
    fn test_boolean_encode() {
        let frame: RespFrame = true.into();
        assert_eq!(frame.encode(), b"#t\r\n");

        let frame: RespFrame = false.into();
        assert_eq!(frame.encode(), b"#f\r\n");
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    #[allow(clippy::assertions_on_constants)]
    fn test_boolean_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"#t\r\n");

        let frame = bool::decode(&mut buf)?;
        assert!(frame);

        buf.extend_from_slice(b"#f\r\n");

        let frame = bool::decode(&mut buf)?;
        assert!(!frame);

        buf.extend_from_slice(b"#f");
        if !bool::decode(&mut buf).is_err() {
            assert!(false);
        }
        Ok(())
    }
}
