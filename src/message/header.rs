pub use crate::{
    COMMAND_SIZE, START_STRING_SIZE, COMMAND_NAME_SIZE, PAYLOAD_SIZE_SIZE, CHECKSUM_SIZE, EMPTY_VERSION_SIZE, CUSTOM_VERSION_SIZE,
    traits::{
        EndianWrite,
        EndianRead,
    },
    message::{
        command::Command,
        payload::{
            VersionPayload,
            PingPayload,
        },
    },
    errors,
    helpers,
    NETWORK,
};

#[derive(Debug)]
pub struct MessageHeader {
    pub start_string: [u8;START_STRING_SIZE],
    pub command_name: [u8;COMMAND_NAME_SIZE],
    pub payload_size: [u8;PAYLOAD_SIZE_SIZE],
    pub checksum: [u8;CHECKSUM_SIZE],
}

pub const HEADER_SIZE: usize = START_STRING_SIZE + COMMAND_NAME_SIZE + PAYLOAD_SIZE_SIZE + CHECKSUM_SIZE;

impl EndianWrite for MessageHeader {
    type Output = [u8; COMMAND_SIZE];
    fn to_be_bytes(&self) -> Self::Output {
        let mut buf = [0;COMMAND_SIZE];
        let byte_sequence = [START_STRING_SIZE, COMMAND_NAME_SIZE, PAYLOAD_SIZE_SIZE, CHECKSUM_SIZE];     
          
        { // serialization rutine
            let mut cursor: usize = 0; 
            for i in 0..byte_sequence[0] {
                buf[i + cursor] = self.start_string[i]
            }
            cursor = cursor + byte_sequence[0];
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
    pub fn version(version_payload: [u8; CUSTOM_VERSION_SIZE]) -> Result<Self, Box<dyn errors::Error>> {
        // let version_payload = VersionPayload::default().to_be_bytes();
        let payload_size = helpers::u32_to_le_bytes(version_payload.len() as u32);
        let checksum = helpers::le_checksum(&version_payload);
        Ok(Self {
            start_string: NETWORK.to_le_bytes(),
            command_name: Command::Version(VersionPayload::default()).to_be_bytes(),
            payload_size,
            checksum,
        })
    }
    pub fn ping() -> Self {  // The Payload of Ping is its nonce.
        let ping_payload: PingPayload = PingPayload::default();
        let payload_size = helpers::u32_to_le_bytes(ping_payload.nonce.len() as u32);
        let checksum = helpers::le_checksum(&ping_payload.nonce);
        Self {
            start_string: NETWORK.to_le_bytes(),
            command_name: Command::Ping(ping_payload).to_be_bytes(),
            payload_size,
            checksum,
        }
    }
    pub fn verack() -> Self {
        Self {
            start_string: NETWORK.to_le_bytes(),
            command_name: Command::Verack.to_be_bytes(),
            payload_size: [0x00, 0x00, 0x00, 0x00],
            // checksum: [0x5d, 0xf6, 0xe0, 0xe2] // Empty checksum 0x5df6e0e2 big-endian
            checksum: [0x5d, 0xf6, 0xe0, 0xe2] // Empty checksum 0x5df6e0e2 little-endian
        }
    }
    pub fn to_le_bytes_with_payload(&mut self, payload: &[u8]) -> Result<[u8;COMMAND_SIZE], Box<dyn errors::Error>> {
        if helpers::u32_to_le_bytes(payload.len().try_into()?) != self.payload_size {
            return Err(Box::new(errors::ErrorSide::PayloadSizeMismatch(payload.len())))
        } else {
            self.payload_size = helpers::u32_to_le_bytes(payload.len().try_into()?);
            self.checksum = helpers::le_checksum(payload);
            Ok(self.to_le_bytes())
        }
    }
    pub fn to_be_bytes_with_payload(&mut self, payload: &[u8]) -> Result<[u8;COMMAND_SIZE], Box<dyn errors::Error>> {
        if helpers::u32_to_le_bytes(payload.len().try_into()?) != self.payload_size {
            return Err(Box::new(errors::ErrorSide::PayloadSizeMismatch(payload.len())))
        } else {
            self.payload_size = helpers::u32_to_le_bytes(payload.len().try_into()?);
            self.checksum = helpers::le_checksum(payload);
            Ok(self.to_be_bytes())
        }
    }
}

impl EndianRead for MessageHeader {
    type Input = [u8; START_STRING_SIZE + COMMAND_NAME_SIZE + PAYLOAD_SIZE_SIZE + CHECKSUM_SIZE];
    fn from_le_bytes(input: Self::Input) -> Self {
        let mut cursor = 0;
        let start_string: [u8; START_STRING_SIZE] = input[cursor..START_STRING_SIZE].try_into().expect("[cursor..cursor+SIZE] has size SIZE.");
        cursor += START_STRING_SIZE;
        let command_name: [u8; COMMAND_NAME_SIZE] = input[cursor..cursor + COMMAND_NAME_SIZE].try_into().expect("[cursor..cursor+SIZE] has size SIZE.");
        cursor += COMMAND_NAME_SIZE;
        let payload_size: [u8; PAYLOAD_SIZE_SIZE] = input[cursor..cursor + PAYLOAD_SIZE_SIZE].try_into().expect("[cursor..cursor+SIZE] has size SIZE.");
        cursor += PAYLOAD_SIZE_SIZE;
        let checksum: [u8; CHECKSUM_SIZE] = input[cursor..cursor + CHECKSUM_SIZE].try_into().expect("[cursor..cursor+SIZE] has size SIZE.");
        MessageHeader {
            start_string,
            command_name,
            payload_size,
            checksum,
        }
    }
    fn from_be_bytes(input: Self::Input) -> Self {
        let be_message_header = Self::from_le_bytes(input.into_iter().rev().collect::<Vec<u8>>().try_into().expect("[cursor..cursor+SIZE] has size SIZE."));
        /* These cases are expressed in a comment for further considerations.
        let start_string: [u8; START_STRING_SIZE] = le_message_header.start_string.into_iter().rev().collect::<Vec<u8>>().try_into().expect("[cursor..cursor+SIZE] has size SIZE.");
        let command_name: [u8; COMMAND_NAME_SIZE] = le_message_header.command_anme.into_iter().rev().collect::<Vec<u8>>().try_into().expect("[cursor..cursor+SIZE] has size SIZE.");
        let payload_size: [u8; PAYLOAD_SIZE_SIZE] = le_message_header.payload_size.into_iter().rev().collect::<Vec<u8>>().try_into().expect("[cursor..cursor+SIZE] has size SIZE.");
        let checksum: [u8; CHECKSUM_SIZE] = le_message_header.checksum.into_iter().rev().collect::<Vec<u8>>().try_into().expect("[cursor..cursor+SIZE] has size SIZE.");
        */
        be_message_header
    }
}

#[test]
fn HEADER_SIZE_is_the_sum_of_its_components_size() {
    assert_eq!(HEADER_SIZE, START_STRING_SIZE + COMMAND_NAME_SIZE + PAYLOAD_SIZE_SIZE + CHECKSUM_SIZE);
}