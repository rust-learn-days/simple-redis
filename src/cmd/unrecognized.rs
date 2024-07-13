use crate::cmd::{CommandExecute, UnrecognizedArgs, RESP_OK};
use crate::database::Database;
use crate::resp::RespFrame;

impl CommandExecute for UnrecognizedArgs {
    fn execute(self, _backend: &Database) -> RespFrame {
        RESP_OK.clone()
    }
}
