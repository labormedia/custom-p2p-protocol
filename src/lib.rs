pub mod errors;
pub mod payload;
pub mod traits;
pub mod command;

pub use command::Command;
pub use traits::endian::EndianWrite;
pub use payload::{
    VersionPayload,
    PingPayload,
};
use sha2::{Digest, Sha256};

pub enum StartString {
    Mainnet,
    Testnet,
}

impl EndianWrite for StartString {
    type Array = [u8;4];
    fn to_le_bytes(&self) -> Self::Array {
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
    fn to_be_bytes(&self) -> Self::Array {
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




// Size constants for version 70015
pub const COMMAND_SIZE: usize = 24;
pub const COMMAND_NAME_SIZE: usize = 12;
pub const START_STRING_SIZE: usize = 4;
pub const PAYLOAD_SIZE_SIZE: usize = 4;
pub const CHECKSUM_SIZE: usize = 4;
pub const MAX_PAYLOAD_SIZE: usize = 32 * 1024 * 1024;

pub const NETWORK: StartString = StartString::Mainnet;

impl EndianWrite for Command {
    type Array = [u8;COMMAND_NAME_SIZE];
    fn to_le_bytes(&self) -> Self::Array {
        let mut res = [0_u8;COMMAND_NAME_SIZE];
        let big_endian = self.to_be_bytes();
        res.copy_from_slice(&big_endian.into_iter().rev().collect::<Vec<u8>>()); // write as little endian
        res
    }
    fn to_be_bytes(&self) -> Self::Array {
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

pub struct MessageHeader {
    start_string: [u8;START_STRING_SIZE],
    command_name: [u8;COMMAND_NAME_SIZE],
    payload_size: [u8;PAYLOAD_SIZE_SIZE],
    checksum: [u8;CHECKSUM_SIZE],
}

impl MessageHeader {
    pub fn ping() -> Result<Self, Box<dyn errors::Error>> {
        let ping_payload: PingPayload = Default::default();
        let payload_size = u32_to_le_bytes(ping_payload.nonce.len().try_into()?);
        let checksum = le_checksum(&ping_payload.nonce);
        Ok(Self {
            start_string: NETWORK.to_le_bytes(),
            command_name: Command::Ping(ping_payload).to_le_bytes(),
            payload_size,
            checksum,
        })
    }
    pub fn verack() -> Result<Self, Box<dyn errors::Error>> {
        Ok(Self {
            start_string: NETWORK.to_le_bytes(),
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
        if u32_to_le_bytes(payload.len().try_into()?) != self.payload_size {
            return Err(Box::new(errors::ErrorSide::PayloadSizeMismatch(Box::new(self.payload_size))))
        };
        let payload_size = payload.len();
        self.payload_size = u32_to_le_bytes(payload_size.try_into()?); // repeats the complete rutine
        self.checksum = le_checksum(payload);
        let mut buf = [0;COMMAND_SIZE];
        let byte_sequence = [START_STRING_SIZE, COMMAND_NAME_SIZE, PAYLOAD_SIZE_SIZE, CHECKSUM_SIZE];

        
        { // serialization rutine
            let mut cursor: usize = 0;
            for i in 0..byte_sequence[0] {
                buf[i + cursor] = self.start_string[i]
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
        }

        Ok(buf)
    }
}

fn u32_to_le_bytes(size: u32) -> [u8; 4] {
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
fn check_u32_to_le_bytes_endianess() {
    let num: u32 = 42;
    assert_eq!(u32_to_le_bytes(num.try_into().unwrap()), num.to_le_bytes());
}