mod array;
mod boolean;
mod bulk_string;
mod double;
mod error;
mod integer;
mod map;
mod null;
mod null_array;
mod null_bulk_string;
mod set;
mod simple_string;
#[allow(unused_imports)]
pub use array::*;
#[allow(unused_imports)]
pub use boolean::*;
#[allow(unused_imports)]
pub use bulk_string::*;
#[allow(unused_imports)]
pub use double::*;
#[allow(unused_imports)]
pub use error::*;
#[allow(unused_imports)]
pub use integer::*;
#[allow(unused_imports)]
pub use map::*;
#[allow(unused_imports)]
pub use null::*;
#[allow(unused_imports)]
pub use null_array::*;
#[allow(unused_imports)]
pub use null_bulk_string::*;
#[allow(unused_imports)]
pub use set::*;
#[allow(unused_imports)]
pub use simple_string::*;

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

use std::collections::{HashMap, HashSet};
use std::ops::Deref;

use bytes::BytesMut;

#[allow(dead_code)]
pub trait RespEncode {
    //self 获取所有权, 如果你需要在方法内部完全消耗该对象
    //&self 借用所有权, 不需要修改调用者并且只需要读访问的方法
    //&mut self 可变借用所有权, 在方法内部修改对象，但仍希望在方法调用后继续使用该对象
    fn encode(self) -> Vec<u8>;
}

#[allow(dead_code)]
pub trait RespDecode {
    fn decode(buf: BytesMut) -> anyhow::Result<RespFrame>;
}

#[allow(dead_code)]
pub enum RespFrame {
    SimpleString(TSimpleString),
    Error(TError),
    Integer(TInteger),
    BulkString(TBulkString),
    NullBulkString(TNullBulkString),
    Array(Vec<RespFrame>),
    Null(TNull),
    NullArray(TNullArray),
    Boolean(TBoolean),
    Double(TDouble),
    Map(TMap),
    Set(TSet),
}

pub struct TSimpleString(String);

pub struct TError(String);

pub struct TInteger(i64);

pub struct TBulkString(Vec<u8>);

pub struct TNullBulkString;

#[allow(dead_code)]
pub struct TArray(Vec<RespFrame>);

pub struct TNull;

pub struct TNullArray;

pub struct TBoolean(());

pub struct TDouble(());

pub struct TMap(HashMap<String, RespFrame>);

pub struct TSet(HashSet<RespFrame>);

impl Deref for TSimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for TError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for TInteger {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for TBulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for TArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for TMap {
    type Target = HashMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for TSet {
    type Target = HashSet<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
