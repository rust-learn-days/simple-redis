use crate::cmd::{extract_args, validate_command, CommandError, CommandExecute, EchoArgs};
use crate::resp::{RespFrame, TArray};
use crate::Database;

impl CommandExecute for EchoArgs {
    fn execute(self, _backend: &Database) -> RespFrame {
        RespFrame::BulkString(self.val.into())
    }
}

impl TryFrom<TArray> for EchoArgs {
    type Error = CommandError;

    fn try_from(value: TArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["echo"], 1)?;
        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(EchoArgs {
                val: String::from_utf8(key.0)?,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid args".to_string())),
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
        buf.extend_from_slice(b"*2\r\n$4\r\necho\r\n$12\r\nHello World!\r\n");

        let frame = TArray::decode(&mut buf)?;
        let result: EchoArgs = frame.try_into()?;
        assert_eq!(result.val, "Hello World!");

        Ok(())
    }
}
