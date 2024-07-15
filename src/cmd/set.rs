use crate::cmd::{
    extract_args, validate_command, CommandError, CommandExecute, SAddArgs, SismemberArgs,
    RESP_NULL, RESP_ONE, RESP_ZERO,
};
use crate::resp::{RespFrame, TArray};
use crate::Database;

impl CommandExecute for SAddArgs {
    fn execute(self, backend: &Database) -> RespFrame {
        backend.sadd(self.key, self.val);
        RESP_ONE.clone()
    }
}

impl TryFrom<TArray> for SAddArgs {
    type Error = CommandError;
    fn try_from(value: TArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["sadd"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(val))) => Ok(SAddArgs {
                key: String::from_utf8(key.0)?,
                val: String::from_utf8(val.0)?,
            }),
            _ => Err(CommandError::InvalidArgument(
                "Invalid key, value".to_string(),
            )),
        }
    }
}

impl CommandExecute for SismemberArgs {
    fn execute(self, backend: &Database) -> RespFrame {
        match backend.sall(&self.key) {
            Some(value) => {
                if value.contains(&self.val) {
                    RESP_ONE.clone()
                } else {
                    RESP_ZERO.clone()
                }
            }
            None => RESP_NULL.clone(),
        }
    }
}

impl TryFrom<TArray> for SismemberArgs {
    type Error = CommandError;
    fn try_from(value: TArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["sismember"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(val))) => {
                Ok(SismemberArgs {
                    key: String::from_utf8(key.0)?,
                    val: String::from_utf8(val.0)?,
                })
            }
            _ => Err(CommandError::InvalidArgument(
                "Invalid key, value".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use bytes::BytesMut;

    use crate::resp::RespDecode;

    use super::*;

    #[test]
    fn test_echo() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$4\r\nsadd\r\n$6\r\nmyhash\r\n$6\r\nfield1\r\n");

        let frame = TArray::decode(&mut buf)?;
        let result: SAddArgs = frame.try_into()?;
        assert_eq!(result.key, "myhash");
        assert_eq!(result.val, "field1");
        Ok(())
    }
}
