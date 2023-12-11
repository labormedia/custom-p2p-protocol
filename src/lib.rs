pub mod errors;
pub mod payload;
pub mod traits;
pub mod command;
pub mod magic_bytes;
pub mod helpers;

pub use command::Command;
pub use traits::EndianWrite;
pub use payload::{
    VersionPayload,
    PingPayload,
};
use magic_bytes::Network;

// Size constants for version 70015
pub const COMMAND_SIZE: usize = 24;
pub const COMMAND_NAME_SIZE: usize = 12;
pub const START_STRING_SIZE: usize = 4;
pub const PAYLOAD_SIZE_SIZE: usize = 4;
pub const CHECKSUM_SIZE: usize = 4;
pub const MAX_PAYLOAD_SIZE: usize = 32 * 1024 * 1024;

pub const NETWORK: Network = Network::Mainnet;

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

pub struct MessageHeader {
    start_string: [u8;START_STRING_SIZE],
    command_name: [u8;COMMAND_NAME_SIZE],
    payload_size: [u8;PAYLOAD_SIZE_SIZE],
    checksum: [u8;CHECKSUM_SIZE],
}

impl EndianWrite for MessageHeader {
    type Output = [u8; COMMAND_SIZE];
    fn to_be_bytes(&self) -> Self::Output {
        let mut buf = [0;COMMAND_SIZE];
        let byte_sequence = [START_STRING_SIZE, COMMAND_NAME_SIZE, PAYLOAD_SIZE_SIZE, CHECKSUM_SIZE];        
        for j in 0..4 { // serialization rutine
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

        buf
    }
    fn to_le_bytes(&self) -> Self::Output {
        let mut buf = self.to_be_bytes();
        buf.reverse();
        buf
    }
}

impl MessageHeader {
    pub fn version() -> Result<Self, Box<dyn errors::Error>> {
        let version_payload: VersionPayload = Default::default();
        let payload_size = helpers::u32_to_le_bytes(90);
        let checksum = helpers::le_checksum(&version_payload.to_le_bytes());
        Ok(Self {
            start_string: NETWORK.to_le_bytes(),
            command_name: Command::Version(version_payload).to_le_bytes(),
            payload_size,
            checksum,
        })
    }
    pub fn ping() -> Result<Self, Box<dyn errors::Error>> {  // The Payload of Ping is its nonce.
        let ping_payload: PingPayload = Default::default();
        let payload_size = helpers::u32_to_le_bytes(ping_payload.nonce.len().try_into()?);
        let checksum = helpers::le_checksum(&ping_payload.nonce);
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
    pub fn to_le_bytes_with_payload(&mut self, payload: &[u8]) -> Result<[u8;COMMAND_SIZE], Box<dyn errors::Error>> {
        if helpers::u32_to_le_bytes(payload.len().try_into()?) != self.payload_size {
            return Err(Box::new(errors::ErrorSide::PayloadSizeMismatch(Box::new(self.payload_size))))
        } else {
            self.payload_size = helpers::u32_to_le_bytes(payload.len().try_into()?); // repeats the complete rutine
            self.checksum = helpers::le_checksum(payload);
            Ok(self.to_le_bytes())
        }
    }
}