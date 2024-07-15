use crate::cmd::{CommandExecute, UnrecognizedArgs, RESP_UNKNOW};
use crate::database::Database;
use crate::resp::RespFrame;

impl CommandExecute for UnrecognizedArgs {
    fn execute(self, _backend: &Database) -> RespFrame {
        RESP_UNKNOW.clone()
    }
}
