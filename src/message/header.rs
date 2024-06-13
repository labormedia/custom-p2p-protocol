pub use crate::{
    COMMAND_SIZE, START_STRING_SIZE, COMMAND_NAME_SIZE, PAYLOAD_SIZE_SIZE, CHECKSUM_SIZE, EMPTY_VERSION_SIZE, CUSTOM_VERSION_SIZE,
    traits::EndianWrite,
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
            self.payload_size = helpers::u32_to_le_bytes(payload.len().try_into()?);
            self.checksum = helpers::le_checksum(payload);
            Ok(self.to_le_bytes())
        }
    }
    pub fn to_be_bytes_with_payload(&mut self, payload: &[u8]) -> Result<[u8;COMMAND_SIZE], Box<dyn errors::Error>> {
        if helpers::u32_to_le_bytes(payload.len().try_into()?) != self.payload_size {
            return Err(Box::new(errors::ErrorSide::PayloadSizeMismatch(Box::new(self.payload_size))))
        } else {
            self.payload_size = helpers::u32_to_le_bytes(payload.len().try_into()?);
            self.checksum = helpers::be_checksum(payload);
            Ok(self.to_be_bytes())
        }
    }
}