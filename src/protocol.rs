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
pub const NETWORK: StartString = StartString::Mainnet;

pub enum StartString {
    Mainnet,
    Testnet,
}

impl StartString {
    pub fn le_value(&self) -> [u8; 4] {
        match self {
            StartString::Mainnet => {
                [0xf9, 0xbe, 0xb4, 0xd9] // Little endian
                // [0xd9, 0xb4, 0xbe, 0xf9] // Big endian
            },
            StartString::Testnet => {
                [0x0b, 0x11, 0x09, 0x07] // Little endian
                // [0x07, 0x09, 0x11, 0x0b] // Big endian
            }
        }
    }
    pub fn be_value(&self) -> [u8; 4] {
        match self {
            StartString::Mainnet => {
                // [0xf9, 0xbe, 0xb4, 0xd9] // Little endian
                [0xd9, 0xb4, 0xbe, 0xf9] // Big endian
            },
            StartString::Testnet => {
                // [0x0b, 0x11, 0x09, 0x07] // Little endian
                [0x07, 0x09, 0x11, 0x0b] // Big endian
            }
        }
    }
}

pub enum Command {
    Ping([u8; 8]),
    Verack
}

impl Command {
    pub fn to_le_bytes(&self) -> [u8;COMMAND_NAME_SIZE] {
        let mut res = [0_u8;COMMAND_NAME_SIZE];
        let big_endian = match self {
            Command::Ping(_nonce) => {
                let mut command = [0_u8; COMMAND_NAME_SIZE];
                let command_name_bytes: Vec<_> = self.to_string().into_bytes();
                let command_bytes_len = command_name_bytes.len();
                #[cfg(debug_assertions)]
                println!("Command length {}", command_bytes_len);
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
                let command_name_bytes = self.to_string().into_bytes();
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
        };
        res.copy_from_slice(&big_endian.into_iter().rev().collect::<Vec<u8>>()); // write as little endian
        #[cfg(debug_assertions)]
        println!("Response : {:?}", res);
        res
    }
    pub fn to_be_bytes(&self) -> [u8;COMMAND_NAME_SIZE] {
        let mut res = [0_u8;COMMAND_NAME_SIZE];
        match self {
            Command::Ping(_nonce) => {
                let mut command = [0_u8; COMMAND_NAME_SIZE];
                let command_name_bytes: Vec<_> = self.to_string().into_bytes();
                let command_bytes_len = command_name_bytes.len();
                #[cfg(debug_assertions)]
                println!("Command length {}", command_bytes_len);
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
                let command_name_bytes = self.to_string().into_bytes();
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
            Command::Verack => "verack",
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
    pub fn ping() -> Result<Self, Box<dyn errors::Error>> {
        let nonce: [u8; 8] = [0,0,0,0,0,0,0,0];
        let payload_size = le_array_from_usize(nonce.len().try_into()?);
        let checksum = le_checksum(&nonce);
        Ok(Self {
            start_string: NETWORK.le_value(),
            command_name: Command::Ping(nonce).to_le_bytes(),
            payload_size,
            checksum,
        })
    }
    pub fn verack() -> Result<Self, Box<dyn errors::Error>> {
        Ok(Self {
            start_string: NETWORK.le_value(),
            command_name: Command::Verack.to_le_bytes(),
            payload_size: [0x00, 0x00, 0x00, 0x00],
            // checksum: [0x5d, 0xf6, 0xe0, 0xe2] // Empty checksum 0x5df6e0e2 big-endian
            checksum: [0xe2, 0xe0, 0xf6, 0x5d] // Empty checksum 0x5df6e0e2 little-endian
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
        if le_array_from_usize(payload.len().try_into()?) != self.payload_size {
            return Err(Box::new(errors::ErrorSide::PayloadSizeMismatch(Box::new(self.payload_size))))
        };
        let payload_size = payload.len();
        self.payload_size = le_array_from_usize(payload_size.try_into()?); // repeats the complete rutine
        self.checksum = le_checksum(payload);
        let mut buf = [0;COMMAND_SIZE];
        let mut cursor: usize = 0;
        // buf.write_all(self.start_string);
        // buf.append(self.command_name);
        let byte_sequence = [START_STRING_SIZE, COMMAND_NAME_SIZE, PAYLOAD_SIZE_SIZE, CHECKSUM_SIZE];
        for i in 0..byte_sequence[0] {
            buf[i] = self.start_string[i]
        }
        cursor = byte_sequence[0];
        for i in 0..(byte_sequence[1]) {
            buf[i + cursor] = self.command_name[i]
        }
        cursor = cursor + byte_sequence[1];
        for i in 0..(byte_sequence[2]) {
            buf[i + cursor] = self.payload_size[i]
        }
        cursor = cursor + byte_sequence[2];
        for i in 0..(byte_sequence[3]) {
            buf[i + cursor] = self.checksum[i]
        }
        // cursor = cursor + payload_size;
        // for i in cursor..(START_STRING_SIZE + COMMAND_NAME_SIZE + PAYLOAD_SIZE_SIZE + CHECKSUM_SIZE + payload_size) {
        //     buf[i] = payload[i]
        // }
        Ok(buf)
    }
}

fn le_array_from_usize(size: u32) -> [u8; 4] {
    let b1 : u8 = ((size >> 24) & 0xff) as u8;
    let b2 : u8 = ((size >> 16) & 0xff) as u8;
    let b3 : u8 = ((size >> 8) & 0xff) as u8;
    let b4 : u8 = (size & 0xff) as u8;
    return [b4, b3, b2, b1]  // Little Endianess
}

pub fn le_checksum(data: &[u8]) -> [u8; CHECKSUM_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(hash);
    let hash = hasher.finalize();

    let mut buf = [0u8; CHECKSUM_SIZE];
    buf.clone_from_slice(&hash[..CHECKSUM_SIZE]);

    [buf[3], buf[2], buf[1], buf[0]]
}

#[test]
fn check_command_size() {
    let total_size = START_STRING_SIZE + COMMAND_NAME_SIZE + PAYLOAD_SIZE_SIZE + CHECKSUM_SIZE;
    assert_eq!(total_size, COMMAND_SIZE)
}

#[test]
fn check_le_array_from_usize_endianess() {
    let num: u32 = 42;
    assert_eq!(le_array_from_usize(num.try_into().unwrap()), num.to_le_bytes());
}