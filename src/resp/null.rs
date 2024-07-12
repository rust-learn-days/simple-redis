use crate::resp::RespEncode;

#[derive(Debug, Clone)]
pub struct TNull;

// - null: "_\r\n"
impl RespEncode for TNull {
    fn encode(self) -> Vec<u8> {
        "_\r\n".to_string().into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::RespFrame;

    #[test]
    fn test_null_encode() {
        let frame: RespFrame = TNull.into();
        assert_eq!(frame.encode(), b"_\r\n");
    }
}
