use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use thiserror::Error;

use crate::database::Database;
use crate::resp::{RespError, RespFrame, TArray, TNull, TSimpleString};

mod echo;
mod hmap;
mod map;
mod mget;
mod set;
mod unrecognized;

lazy_static! {
    static ref RESP_OK: RespFrame = TSimpleString::new("OK").into();
    static ref RESP_UNKNOW: RespFrame = TSimpleString::new("UNKNOWN").into();
    static ref RESP_NULL: RespFrame = TNull.into();
    static ref RESP_ZERO: RespFrame = 0.into();
    static ref RESP_ONE: RespFrame = 1.into();
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    RespError(#[from] RespError),
}

#[enum_dispatch]
pub trait CommandExecute {
    fn execute(self, backend: &Database) -> RespFrame;
}

#[enum_dispatch(CommandExecute)]
#[derive(Debug)]
pub enum Command {
    Get(GetArgs),
    Set(SetArgs),
    HGet(HGetArgs),
    HSet(HSetArgs),
    HGetAll(HGetAllArgs),
    Echo(EchoArgs),
    HMGet(HMGetArgs),
    SAdd(SAddArgs),
    Sismember(SismemberArgs),
    Unrecognized(UnrecognizedArgs),
}

#[derive(Debug)]
pub struct GetArgs {
    key: String,
}

#[derive(Debug)]
pub struct SetArgs {
    key: String,
    value: RespFrame,
}

#[derive(Debug)]
pub struct HGetArgs {
    key: String,
    field: String,
}

#[derive(Debug)]
pub struct HSetArgs {
    key: String,
    field: String,
    value: RespFrame,
}

#[derive(Debug)]
pub struct HMGetArgs {
    key: String,
    field: Vec<String>,
}

#[derive(Debug)]
pub struct HGetAllArgs {
    key: String,
    sort: bool,
}

#[derive(Debug)]
pub struct EchoArgs {
    val: String,
}

#[derive(Debug)]
pub struct SAddArgs {
    key: String,
    val: String,
}

#[derive(Debug)]
pub struct SismemberArgs {
    key: String,
    val: String,
}

#[derive(Debug)]
pub struct UnrecognizedArgs {}

impl TryFrom<RespFrame> for Command {
    type Error = CommandError;
    fn try_from(v: RespFrame) -> Result<Self, Self::Error> {
        match v {
            RespFrame::Array(array) => array.try_into(),
            _ => Err(CommandError::InvalidCommand(
                "Command must be an Array".to_string(),
            )),
        }
    }
}

impl TryFrom<TArray> for Command {
    type Error = CommandError;
    fn try_from(v: TArray) -> Result<Self, Self::Error> {
        match v.first() {
            Some(RespFrame::BulkString(ref cmd)) => match cmd.as_ref() {
                b"get" => Ok(GetArgs::try_from(v)?.into()),
                b"set" => Ok(SetArgs::try_from(v)?.into()),
                b"hget" => Ok(HGetArgs::try_from(v)?.into()),
                b"hset" => Ok(HSetArgs::try_from(v)?.into()),
                b"hgetall" => Ok(HGetAllArgs::try_from(v)?.into()),
                b"echo" => Ok(EchoArgs::try_from(v)?.into()),
                b"hmget" => Ok(HMGetArgs::try_from(v)?.into()),
                b"sadd" => Ok(SAddArgs::try_from(v)?.into()),
                b"sismember" => Ok(SismemberArgs::try_from(v)?.into()),
                _ => Ok(UnrecognizedArgs {}.into()),
            },
            _ => Err(CommandError::InvalidCommand(
                "Command must have a BulkString as the first argument".to_string(),
            )),
        }
    }
}

fn validate_command(
    value: &TArray,
    names: &[&'static str],
    n_args: usize,
) -> Result<(), CommandError> {
    if value.len() != n_args + names.len() {
        return Err(CommandError::InvalidArgument(format!(
            "{} command must have exactly {} argument",
            names.join(" "),
            n_args
        )));
    }
    for (i, name) in names.iter().enumerate() {
        match value[i] {
            RespFrame::BulkString(ref cmd) => {
                if cmd.as_ref().to_ascii_lowercase() != name.as_bytes() {
                    return Err(CommandError::InvalidCommand(format!(
                        "Invalid command: expected {}, got {}",
                        name,
                        String::from_utf8_lossy(cmd.as_ref())
                    )));
                }
            }
            _ => {
                return Err(CommandError::InvalidCommand(
                    "Command must have a BulkString as the first argument".to_string(),
                ))
            }
        }
    }
    Ok(())
}

fn extract_args(value: TArray, start: usize) -> Result<Vec<RespFrame>, CommandError> {
    Ok(value.0.into_iter().skip(start).collect::<Vec<RespFrame>>())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use bytes::BytesMut;

    use crate::database::Database;
    use crate::resp::RespDecode;
    use crate::resp::TNull;

    use super::*;

    #[test]
    fn test_command() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n");

        let frame = TArray::decode(&mut buf)?;

        let cmd: Command = frame.try_into()?;

        let backend = Database::new();

        let ret = cmd.execute(&backend);
        assert_eq!(ret, RespFrame::Null(TNull));

        Ok(())
    }
}
