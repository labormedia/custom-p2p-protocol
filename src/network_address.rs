use crate::traits::{
    EndianWrite,
    Length
};
use core::net::{
    Ipv4Addr,
    Ipv6Addr,
};
use core::mem;

// Network Data Layout Size Constants for runtime.
pub const NETWORK_TIME: usize = 4;
pub const NETWORK_SERVICES: usize = 8;
pub const NETWORK_IPvXX: usize = 16;
pub const NETWORK_PORT: usize = 2;

// 
pub const DEFAULT_IPADDR: [u8; NETWORK_IPvXX] = Ipv4Addr::new(127, 0, 0, 1).to_ipv6_mapped().octets();

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

#[derive(Clone)]
enum NetworkOptions {
    NetworkTime(Option<[u8;NETWORK_TIME]>),
    NetworkServices(Option<[u8;NETWORK_SERVICES]>),
    NetworkIpvXX(Option<[u8;NETWORK_IPvXX]>),
    NetworkPort(Option<[u8;NETWORK_PORT]>),
}

impl Length for NetworkOptions {
    fn len(&self) -> usize {
        match self {
            NetworkOptions::NetworkTime(option) => {
                match option {
                    None => 0_usize,
                    Some(serial_layout) => {
                        serial_layout.len()
                    }
                }
            },
            NetworkOptions::NetworkServices(option) => {
                match option {
                    None => 0_usize,
                    Some(serial_layout) => {
                        serial_layout.len()
                    }
                }
            },
            NetworkOptions::NetworkIpvXX(option) => {
                match option {
                    None => 0_usize,
                    Some(serial_layout) => {
                        serial_layout.len()
                    }
                }
            },
            NetworkOptions::NetworkPort(option) => {
                match option {
                    None => 0_usize,
                    Some(serial_layout) => {
                        serial_layout.len()
                    }
                }
            },
        }
    }
}

impl EndianWrite for NetworkOptions {
    type Output = Vec<u8>;
    fn to_le_bytes(&self) -> Self::Output {
        let mut options = self.to_be_bytes();
        options.reverse();
        options
    }
    fn to_be_bytes(&self) -> Self::Output {
        let options: Vec<u8> = match self {
            NetworkOptions::NetworkTime(option) => {
                match option {
                    None => Vec::new(),
                    Some(serial_layout) => {
                        (*serial_layout).to_vec()
                    }
                }
            },
            NetworkOptions::NetworkServices(option) => {
                match option {
                    None => Vec::new(),
                    Some(serial_layout) => {
                        (*serial_layout).to_vec()
                    }
                }
            },
            NetworkOptions::NetworkIpvXX(option) => {
                match option {
                    None => Vec::new(),
                    Some(serial_layout) => {
                        (*serial_layout).to_vec()
                    }
                }
            },
            NetworkOptions::NetworkPort(option) => {
                match option {
                    None => Vec::new(),
                    Some(serial_layout) => {
                        (*serial_layout).to_vec()
                    }
                }
            },
        };
        Vec::new()
    }
}

// Defines use of network address (they differ)
#[derive(Clone)]
pub enum NetworkAddress { 
    NonVersion(
        [NetworkOptions;4]
    ),
    Version(
        [NetworkOptions;4]
    ),
}

impl Length for NetworkAddress {
    fn len(&self) -> usize {
        match self {
            Self::NonVersion(options)
            | Self::Version(options) => {
                options
                    .into_iter()
                    .map(|x| {x.len()} )
                    .sum::<usize>()
            },
        }
    }
}

impl EndianWrite for NetworkAddress {
    type Output = Vec<u8>;
    fn to_le_bytes(&self) -> Self::Output {
        let mut options = self.to_be_bytes();
        options.reverse();
        options
    }
    fn to_be_bytes(&self) -> Self::Output {
        let mut options = match self {
            Self::NonVersion(options)
            | Self::Version(options)  => {
                options
                    .into_iter()
                    .map(|x| {x.to_be_bytes()} )  // TODO: check double endianess
                    .flatten()
                    .collect::<Vec<u8>>()
            },
        };
        options
    }
}

pub enum Services {
    NODE_NETWORK = 0x01,
    NODE_GETUTXO = 0x02,
    NODE_BLOOM = 0x04,
    NODE_WITNESS = 0x08,
    NODE_XTHIN = 0x10,
    NODE_COMPACT_FILTERS = 0x40,
    NODE_NETWORK_LIMITED = 0x0400,
}

impl EndianWrite for Services {
    type Output = [u8;NETWORK_SERVICES];
    fn to_le_bytes(&self) -> Self::Output {
        let mut buf: Self::Output = self.to_be_bytes();
        buf.reverse();
        buf
    }
    fn to_be_bytes(&self) -> Self::Output {
        let mut buf: Self::Output = Self::Output::default();
        let a: i32 = match self {
            Services::NODE_NETWORK => Services::NODE_NETWORK as i32,
            Services::NODE_GETUTXO => Services::NODE_GETUTXO as i32,
            Services::NODE_BLOOM => Services::NODE_BLOOM as i32,
            Services::NODE_WITNESS => Services::NODE_WITNESS as i32,
            Services::NODE_XTHIN => Services::NODE_XTHIN as i32,
            Services::NODE_COMPACT_FILTERS => Services::NODE_COMPACT_FILTERS as i32,
            Services::NODE_NETWORK_LIMITED => Services::NODE_NETWORK_LIMITED as i32,
        };
        buf.clone_from_slice(&a.to_le_bytes());
        buf
    }
}

 // Default for all variants
impl Default for NetworkAddress {
    fn default() -> Self {
        let network_address = DEFAULT_IPADDR;
        todo!()
    }
}


// #[test]
// fn check_network_ip_sizes() {
//     todo!()
// }