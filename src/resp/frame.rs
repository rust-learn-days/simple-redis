use enum_dispatch::enum_dispatch;

use crate::resp::{
    TArray, TBulkString, TError, TMap, TNull, TNullArray, TNullBulkString, TSet, TSimpleString,
};

#[enum_dispatch(RespEncode)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RespFrame {
    SimpleString(TSimpleString),
    Error(TError),
    Integer(i64),
    BulkString(TBulkString),
    NullBulkString(TNullBulkString),
    Array(TArray),
    Null(TNull),
    NullArray(TNullArray),
    Boolean(bool),
    Double(f64),
    Map(TMap),
    Set(TSet),
}
