use core::{
    fmt::{
        self,
        Display,
    },
};
use crate::payload;

pub enum Command {
    Version(payload::Version),
    Ping([u8; 8]),
    Verack
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Command::Ping(_) => "ping",
            Command::Verack => "verack",
            Command::Version(_) => "version",
        };
        write!(f, "{}", s)
    }
}