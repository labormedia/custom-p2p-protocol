use core::{
    fmt::{
        self,
        Display,
        Formatter,
    },
};
use crate::{
    COMMAND_NAME_SIZE,
    traits::EndianWrite,
    message::payload::{
        VersionPayload,
        PingPayload,
    },
};

pub enum Command {
    Version(VersionPayload),
    Ping(PingPayload),
    Verack
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Command::Ping(_) => "ping",
            Command::Verack => "verack",
            Command::Version(_) => "version",
        };
        write!(f, "{}", s)
    }
}

impl EndianWrite for Command {
    type Output = [u8;COMMAND_NAME_SIZE];
    fn to_le_bytes(&self) -> Self::Output {
        let mut res = [0_u8;COMMAND_NAME_SIZE];
        let big_endian = self.to_be_bytes();
        res.copy_from_slice(&big_endian.into_iter().rev().collect::<Vec<u8>>()); // write as little endian
        res
    }
    fn to_be_bytes(&self) -> Self::Output {
        let mut command = [0_u8; COMMAND_NAME_SIZE];
        let command_name_bytes: Vec<_> = self.to_string().into_bytes();
        let command_bytes_len = command_name_bytes.len();
        // Fills the command with the appropiate size in bytes
        for i in 0..(COMMAND_NAME_SIZE) {
            if command_bytes_len > i {
                command[i] = command_name_bytes[i];
            } else {
                command[i] = 0x00;
            }
        };
        command
    }
}