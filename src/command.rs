use core::{
    fmt::{
        self,
        Display,
        Formatter,
    },
};
use crate::payload::{
    VersionPayload,
    PingPayload,
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