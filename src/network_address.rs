use crate::traits::EndianWrite;

// Network Data Layout Size Constants for runtime.
pub const NETWORK_TIME: usize = 4;
pub const NETWORK_SERVICES: usize = 8;
pub const NETWORK_IPvXX: usize = 16;
pub const NETWORK_PORT: usize = 2;

// Defines variants for IPv4 and IPv6.
pub enum IP {
    V4([u8; NETWORK_IPvXX]),
    V6([u8; NETWORK_IPvXX]),
}

impl EndianWrite for IP {
    type Output = [u8; NETWORK_IPvXX];
    fn to_le_bytes(&self) -> Self::Output {
        match self {
            Self::V4(network_address) | Self::V6(network_address) => {
                let mut buf: Self::Output = *network_address;
                buf.reverse();
                buf
            },
        }
    }
    fn to_be_bytes(&self) -> Self::Output {
        match self {
            Self::V4(network_address) | Self::V6(network_address) => {
                *network_address
            },
        }
    }
}

// Defines use of network address (they differ)
pub enum NetworkAddress { 
    General([u8;NETWORK_TIME], [u8;NETWORK_SERVICES], [u8;NETWORK_IPvXX], [u8;NETWORK_PORT]),
    Version([u8;NETWORK_SERVICES], [u8;NETWORK_IPvXX], [u8;NETWORK_PORT]),
}