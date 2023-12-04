use core::{
    fmt::{
        self,
        Display,
    },
};
use sha2::{Digest, Sha256};

use crate::errors;

pub const COMMAND_SIZE: usize = 24;
pub const COMMAND_NAME_SIZE: usize = 12;
pub const START_STRING_SIZE: usize = 4;
pub const PAYLOAD_SIZE_SIZE: usize = 4;
pub const CHECKSUM_SIZE: usize = 4;
pub const MAX_PAYLOAD_SIZE: usize = 32 * 1024 * 1024;
pub const NETWORK: StartString = StartString::Testnet;

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
    Ping([u8; 8]),
    Verack
}

impl Command {
    pub fn to_bytes(&self) -> [u8;COMMAND_NAME_SIZE] {
        match self {
            Command::Ping(nonce) => {
                let mut command = [0; COMMAND_NAME_SIZE];
                let mut command_name_bytes = self.to_string().into_bytes();
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
            },
            Command::Verack => {
                let mut command = [0; COMMAND_NAME_SIZE];
                let mut command_name_bytes = self.to_string().into_bytes();
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
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Command::Ping(_) => "ping",
            Command::Verack => "varack",
        };
        write!(f, "{}", s)
    }
}

pub struct MessageHeader {
    start_string: [u8;START_STRING_SIZE],
    command_name: [u8;COMMAND_NAME_SIZE],
    payload_size: [u8;PAYLOAD_SIZE_SIZE],
    checksum: [u8;CHECKSUM_SIZE],
}

impl MessageHeader {
    // pub fn ping() -> Result<Self, Box<dyn errors::Error>> {
    //     Ok(Self {
    //         start_string: NETWORK.value(),
    //         command_name: Command::Ping([0,0,0,0,0,0,0,0]).to_bytes(),
    //         payload_size: [0x00, 0x00, 0x00, 0x00],
    //         checksum: [0x5d, 0xf6, 0xe0, 0xe2] // Empty checksum 0x5df6e0e2
    //     })
    // }
    pub fn verack() -> Result<Self, Box<dyn errors::Error>> {
        Ok(Self {
            start_string: NETWORK.value(),
            command_name: Command::Ping([0,0,0,0,0,0,0,0]).to_bytes(),
            payload_size: [0x00, 0x00, 0x00, 0x00],
            checksum: [0x5d, 0xf6, 0xe0, 0xe2] // Empty checksum 0x5df6e0e2
        })
    }
    pub fn to_bytes(&self) -> Result<[u8;COMMAND_SIZE], Box<dyn errors::Error>> {
        let mut buf = [0;COMMAND_SIZE];
        let mut cursor: usize = 0;
        for i in cursor..START_STRING_SIZE {
            buf[i] = self.start_string[i]
        }
        cursor = cursor + START_STRING_SIZE;
        for i in cursor..COMMAND_NAME_SIZE {
            buf[i] = self.command_name[i]
        }
        cursor = cursor + COMMAND_NAME_SIZE;
        for i in cursor..PAYLOAD_SIZE_SIZE {
            buf[i] = self.payload_size[i]
        }
        cursor = cursor + PAYLOAD_SIZE_SIZE;
        for i in cursor..CHECKSUM_SIZE {
            buf[i] = self.checksum[i]
        }
        Ok(buf)
    }
    pub fn to_bytes_with_payload(&mut self, payload: &[u8]) -> Result<[u8;COMMAND_SIZE], Box<dyn errors::Error>> {
        let payload_size = payload.len();
        self.payload_size = array_from_usize(payload_size);
        self.checksum = checksum(payload);
        let mut buf = [0;COMMAND_SIZE];
        let mut cursor: usize = 0;
        for i in cursor..START_STRING_SIZE {
            buf[i] = self.start_string[i]
        }
        cursor = cursor + START_STRING_SIZE;
        for i in cursor..COMMAND_NAME_SIZE {
            buf[i] = self.command_name[i]
        }
        cursor = cursor + COMMAND_NAME_SIZE;
        for i in cursor..PAYLOAD_SIZE_SIZE {
            buf[i] = self.payload_size[i]
        }
        cursor = cursor + PAYLOAD_SIZE_SIZE;
        for i in cursor..CHECKSUM_SIZE {
            buf[i] = self.checksum[i]
        }
        Ok(buf)
    }
}

fn array_from_usize(size: usize) -> [u8;4] {
    let b1 : u8 = ((size >> 24) & 0xff) as u8;
    let b2 : u8 = ((size >> 16) & 0xff) as u8;
    let b3 : u8 = ((size >> 8) & 0xff) as u8;
    let b4 : u8 = (size & 0xff) as u8;
    return [b1, b2, b3, b4]
}

pub fn checksum(data: &[u8]) -> [u8; 4] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(hash);
    let hash = hasher.finalize();

    let mut buf = [0u8; CHECKSUM_SIZE];
    buf.clone_from_slice(&hash[..CHECKSUM_SIZE]);

    buf
}

#[test]
fn check_command_size() {
    let total_size = START_STRING_SIZE + COMMAND_NAME_SIZE + PAYLOAD_SIZE_SIZE + CHECKSUM_SIZE;
    assert_eq!(total_size, COMMAND_SIZE)
}