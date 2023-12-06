use crate::EndianWrite;

pub enum Network {
    Mainnet,
    Testnet,
}

impl EndianWrite for Network {
    type Array = [u8;4];
    fn to_le_bytes(&self) -> Self::Array {
        match self {
            Network::Mainnet => {
                [0xf9, 0xbe, 0xb4, 0xd9] // Little endian
                // [0xd9, 0xb4, 0xbe, 0xf9] // Big endian
            },
            Network::Testnet => {
                [0x0b, 0x11, 0x09, 0x07] // Little endian
                // [0x07, 0x09, 0x11, 0x0b] // Big endian
            }
        }
    }
    fn to_be_bytes(&self) -> Self::Array {
        match self {
            Network::Mainnet => {
                // [0xf9, 0xbe, 0xb4, 0xd9] // Little endian
                [0xd9, 0xb4, 0xbe, 0xf9] // Big endian
            },
            Network::Testnet => {
                // [0x0b, 0x11, 0x09, 0x07] // Little endian
                [0x07, 0x09, 0x11, 0x0b] // Big endian
            }
        }
    }
}