use anyhow::Result;
use bytes::{Buf, BytesMut};
use enum_dispatch::enum_dispatch;
use thiserror::Error;

pub use array::*;
pub use bulk_string::*;
pub use frame::*;
pub use map::*;
pub use null::*;
pub use set::*;
pub use simple_error::*;
pub use simple_string::*;

mod array;
mod boolean;
mod bulk_string;
mod double;
mod frame;
mod integer;
mod map;
mod null;
mod set;
mod simple_error;
mod simple_string;

/*
- 如何 serialize/deserialize Frame
    - simple string: "+OK\r\n"
    - error: "-Error message\r\n"
    - bulk error: "!<length>\r\n<error>\r\n"
    - integer: ":[<+|->]<value>\r\n"
    - bulk string: "$<length>\r\n<data>\r\n"
    - null bulk string: "$-1\r\n"
    - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
        - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
    - null array: "*-1\r\n"
    - null: "_\r\n"
    - boolean: "#<t|f>\r\n"
    - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
    - big number: "([+|-]<number>\r\n"
    - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
    - set: "~<number-of-elements>\r\n<element-1>...<element-n>"
    - ...
- enum RespFrame {}
- trait RespEncode / RespDecode (enum dispatch)
- bytes trait
*/

const BUF_CAP: usize = 4096;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum RespError {
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length: {0}")]
    InvalidFrameLength(isize),
    #[error("Invalid frame data: {0}")]
    InvalidFrameData(String),
    #[error("Frame is not complete")]
    NotCompleteFrame,
}

#[enum_dispatch]
pub trait RespEncode {
    //self 获取所有权, 如果你需要在方法内部完全消耗该对象
    //&self 借用所有权, 不需要修改调用者并且只需要读访问的方法
    //&mut self 可变借用所有权, 在方法内部修改对象，但仍希望在方法调用后继续使用该对象
    fn encode(self) -> Vec<u8>;
}

#[allow(dead_code)]
pub trait RespDecode: Sized {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
}

fn extract_fixed_data(
    buf: &mut BytesMut,
    expect: &str,
    expect_type: &str,
) -> Result<(), RespError> {
    if buf.len() < expect.len() {
        return Err(RespError::InvalidFrameData(format!(
            "expect: {}, got: {}",
            expect.len(),
            buf.len()
        )));
    }

    if !buf.starts_with(expect.as_bytes()) {
        return Err(RespError::InvalidFrameData(format!(
            "expect: {}, got: {:?}",
            expect_type, buf
        )));
    }

    buf.advance(expect.len());
    Ok(())
}
