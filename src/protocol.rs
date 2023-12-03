use core::fmt::Display;

pub const COMMAND_NAME_SIZE: usize = 12;

pub enum StartString {
    Mainnet,
    Testnet,
}

impl StartString {
    pub fn value(&self) -> [u8; 4] {
        match self {
            StartString::Mainnet => {
                [0xf9, 0xbe, 0xb4, 0xd9]
            },
            StartString::Testnet => {
                [0x0b, 0x11, 0x09, 0x07]
            }
        }
    }
}

pub enum Command {
    Ping([u8; 8])
}

impl Command {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Command::Ping(nonce) => {
                let mut command_name_bytes = self.to_string().into_bytes();
                let command_bytes_len = command_name_bytes.len();
                for i in 0..(COMMAND_NAME_SIZE - command_bytes_len) {
                    
                    command_name_bytes.push(nonce[i]);
                };
                command_name_bytes
            }
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Command::Ping(_) => "ping",
        };

        write!(f, "{}", s)
    }
}

impl From<Command> for Vec<u8> {
    fn from(c: Command) -> Self {
        c.to_bytes()
    }
}