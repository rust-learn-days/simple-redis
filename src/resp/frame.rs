use bytes::BytesMut;
use enum_dispatch::enum_dispatch;

use crate::resp::{
    RespDecode, RespError, TArray, TBulkString, TError, TMap, TNull, TSet, TSimpleString,
};

#[enum_dispatch(RespEncode)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RespFrame {
    SimpleString(TSimpleString),
    Error(TError),
    Integer(i64),
    BulkString(TBulkString),
    Array(TArray),
    Null(TNull),
    Boolean(bool),
    Double(f64),
    Map(TMap),
    Set(TSet),
}

impl RespDecode for RespFrame {
    const PREFIX: &'static str = "";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => {
                let frame = TSimpleString::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'-') => {
                let frame = TError::decode(buf)?;
                Ok(frame.into())
            }
            Some(b':') => {
                let frame = i64::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'$') => {
                // try null bulk string first
                match TBulkString::decode(buf) {
                    Ok(frame) => Ok(frame.into()),
                    Err(e) => Err(e),
                }
            }
            Some(b'*') => {
                // try null array first
                match TArray::decode(buf) {
                    Ok(frame) => Ok(frame.into()),
                    Err(e) => Err(e),
                }
            }
            Some(b'_') => {
                let frame = TNull::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'#') => {
                let frame = bool::decode(buf)?;
                Ok(frame.into())
            }
            Some(b',') => {
                let frame = f64::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'%') => {
                let frame = TMap::decode(buf)?;
                Ok(frame.into())
            }
            Some(b'~') => {
                let frame = TSet::decode(buf)?;
                Ok(frame.into())
            }
            None => Err(RespError::NotCompleteFrame),
            _ => Err(RespError::InvalidFrameType(format!(
                "expect_length: unknown frame type: {:?}",
                buf
            ))),
        }
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'*') => TArray::expect_length(buf),
            Some(b'~') => TSet::expect_length(buf),
            Some(b'%') => TMap::expect_length(buf),
            Some(b'$') => TBulkString::expect_length(buf),
            Some(b':') => i64::expect_length(buf),
            Some(b'+') => TSimpleString::expect_length(buf),
            Some(b'-') => TError::expect_length(buf),
            Some(b'#') => bool::expect_length(buf),
            Some(b',') => f64::expect_length(buf),
            Some(b'_') => TNull::expect_length(buf),
            _ => Err(RespError::NotCompleteFrame),
        }
    }
}

impl From<&str> for RespFrame {
    fn from(s: &str) -> Self {
        TSimpleString(s.to_string()).into()
    }
}

impl From<&[u8]> for RespFrame {
    fn from(s: &[u8]) -> Self {
        TBulkString(s.to_vec()).into()
    }
}

impl<const N: usize> From<&[u8; N]> for RespFrame {
    fn from(s: &[u8; N]) -> Self {
        TBulkString(s.to_vec()).into()
    }
}
