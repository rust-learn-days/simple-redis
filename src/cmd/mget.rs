use log::info;

use crate::cmd::{extract_args, validate_command, CommandError, CommandExecute, HMGetArgs};
use crate::resp::{RespFrame, TArray, TBulkString};
use crate::Database;

impl CommandExecute for HMGetArgs {
    fn execute(self, backend: &Database) -> RespFrame {
        let mut data = Vec::with_capacity(self.field.len());
        for field in self.field.iter() {
            match backend.hget(&self.key, field) {
                Some(value) => data.push(value),
                None => data.push(RespFrame::BulkString(TBulkString::new(Vec::new()))),
            }
        }
        TArray::new(data).into()
    }
}

impl TryFrom<TArray> for HMGetArgs {
    type Error = CommandError;

    fn try_from(value: TArray) -> Result<Self, Self::Error> {
        let lens = value.len();
        validate_command(&value, &["hmget"], lens - 1)?;
        if value.len() < 2 {
            return Err(CommandError::InvalidArgument("Invalid args".to_string()));
        }

        let mut args = extract_args(value, 1)?.into_iter();

        let key = match args.next() {
            Some(RespFrame::BulkString(key)) => String::from_utf8(key.0)?,
            _ => return Err(CommandError::InvalidArgument("Invalid key".to_string())),
        };
        info!("key: {:?}", key);
        let field = args
            .map(|arg| match arg {
                RespFrame::BulkString(field) => Ok(String::from_utf8(field.0)?),
                _ => Err(CommandError::InvalidArgument("Invalid field".to_string())),
            })
            .collect::<Result<Vec<String>, CommandError>>()?;

        Ok(HMGetArgs { key, field })
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
        buf.extend_from_slice(b"*5\r\n$5\r\nhmget\r\n$6\r\nmyhash\r\n$6\r\nfield1\r\n$6\r\nfield2\r\n$7\r\nnofield\r\n");

        let frame = TArray::decode(&mut buf)?;
        let result: HMGetArgs = frame.try_into()?;
        assert_eq!(result.key, "myhash");
        assert_eq!(result.field, vec!["field1", "field2", "nofield"]);
        Ok(())
    }
}
