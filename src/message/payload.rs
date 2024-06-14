use std::time::{Duration, SystemTime};
use core::net::Ipv4Addr;
use rand::prelude::*;
use crate::{
    protocol_builder::PayloadBuilder,
    START_STRING_SIZE,
    EMPTY_VERSION_SIZE,
    CUSTOM_VERSION_SIZE,
    USER_AGENT_SIZE,
    message::network_address::{
        self,
        NetworkAddress,
        NETWORK_SERVICES,
        DEFAULT_IPADDR,
        NETWORK_IPvXX,
        NetworkOptions,
    },
    traits::{
        EndianWrite,
        Length,
        Builder,
    },
    errors,
};

// Opaque types

#[derive(Clone, Debug)]
pub struct VersionPayload {
    version: [u8; 4],
    services: [u8; 8],
    timestamp: [u8; 8],
    // The three datum that composes addr_recv
    // are defined in NetworkAddress enum.
    addr_recv: NetworkAddress, // addr_recv: [u8; 26],
    // Fields below require version ≥ 106
    addr_from: [u8; 26],
    nonce: [u8; 8],
    user_agent: [u8; USER_AGENT_SIZE], // This variable size is fixated here for code simplicity.
    start_height: [u8; 4],
    // Fields below require version ≥ 70001
    relay: [u8; 1]
}

pub type VersionPayloadBuilder = PayloadBuilder<VersionPayload>;

impl VersionPayloadBuilder {
    pub fn with_addr_recv(mut self, ip: &[u8; NETWORK_IPvXX]) -> Result<Self, Box<dyn errors::Error>> {
        let ip_address: [u8; NETWORK_IPvXX] = (match ip {
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, ..] => Ok(ip.clone()), // Checks the binary format for IPv6 segments.
            _ => Err(Box::new(errors::ErrorSide::InvalidIPv6Segments)),
        })?;
        #[cfg(debug_assertions)]
        println!("To addr_recv address {:?}", ip_address);
        match self.payload_template.addr_recv {
            NetworkAddress::Version(mut options) => {
                options[0x02] = NetworkOptions::NetworkIpvXX(Some(ip_address));
                self.payload_template.addr_recv = NetworkAddress::Version(options);
                Ok(self)
            },
            NetworkAddress::NonVersion(mut options) => {
                options[2] = NetworkOptions::NetworkIpvXX(Some(ip_address));
                self.payload_template.addr_recv = NetworkAddress::NonVersion(options);
                Ok(self)
            },
            _ => {
                return Err(Box::new(errors::ErrorSide::Unreachable))
            }
        }
    }
    pub fn with_addr_recv_port(mut self, port: u16) -> Result<Self, Box<dyn errors::Error>> {
        self.payload_template.addr_recv.set_port(port);
        Ok(self)
    }
    pub fn with_addr_from(mut self, ip: &[u8; NETWORK_IPvXX]) -> Result<Self, Box<dyn errors::Error>> {
        let ip_address: [u8; NETWORK_IPvXX] = (match ip {
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, ..] => Ok(ip.clone()), // Checks the binary format for IPv6 segments.
            _ => Err(Box::new(errors::ErrorSide::InvalidIPv6Segments)),
        })?;
        let mut network_options = NetworkAddress::default();
        let _ = network_options.set_ip(&ip_address)?;
        self.payload_template.addr_from.clone_from_slice(&network_options.to_be_bytes());
        Ok(self)
    }
    pub fn with_addr_from_port(mut self, port: u16) -> Result<Self, Box<dyn errors::Error>> {
        let port_bytes = port.to_be_bytes();
        let port_bytes_length = port_bytes.len();
        let addr_from_length = self.payload_template.addr_from.len();
        self.payload_template.addr_from[addr_from_length - port_bytes_length..addr_from_length].clone_from_slice(&port_bytes);
        Ok(self)
    }
}

impl Default for VersionPayload {
    fn default() -> VersionPayload {
        let multi_address = match NetworkAddress::non_version_with_ip(&DEFAULT_IPADDR).expect("Default not well defined.") {
            NetworkAddress::Version(multi_address) => multi_address,
            NetworkAddress::NonVersion(multi_address) => multi_address,
        };
        let version: [u8; 4] = 70001_u32.to_le_bytes();
        let services: [u8; NETWORK_SERVICES] = multi_address[1].to_be_bytes().into_iter().collect::<Vec<u8>>().try_into().expect("Default not well defined.");
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time System.").as_secs().to_le_bytes();
        let addr_recv = NetworkAddress::Version(multi_address);
        let addr_from = addr_recv.to_be_bytes().try_into().expect("Unexpected initial state.");
        let nonce: [u8; 8] = rand::thread_rng().gen::<u64>().to_le_bytes();
        let mut user_agent: [u8;USER_AGENT_SIZE] = [0_u8;USER_AGENT_SIZE];
        let start_height: [u8; START_STRING_SIZE] = 0_u32.to_le_bytes();
        let user_agent_size: [u8; 1] = [user_agent.clone().len() as u8 - 1];  // One byte size for the moment.
        user_agent[0..1].copy_from_slice(&user_agent_size);  // var_str <- var_int + char[]
        user_agent[1..].copy_from_slice(&"rust-example".as_bytes());
        let relay = [0_u8; 1];
        VersionPayload {
            version,
            services,
            timestamp,
            addr_recv,
            addr_from,
            nonce,
            user_agent,
            start_height,
            relay,
        }
    }
}

impl EndianWrite for VersionPayload {
    type Output = [u8; CUSTOM_VERSION_SIZE];
    fn to_le_bytes(&self) -> Self::Output {
        let mut buf = self.to_be_bytes();
        buf.reverse();
        buf
    }
    fn to_be_bytes(&self) -> Self::Output {
        let byte_sequence = [
            self.version.len(), 
            self.services.len(), 
            self.timestamp.len(), 
            self.addr_recv.len(),
            self.addr_from.len(),
            self.nonce.len(),
            self.user_agent.len(),
            self.start_height.len(),
            self.relay.len(),
        ];     
        let total_sequence: usize = byte_sequence.iter().sum();
        assert_eq!(total_sequence, CUSTOM_VERSION_SIZE); // This is hardcoded at this stage for convenience. TODO: implement dynamic size
        let mut buf = [0;CUSTOM_VERSION_SIZE];
        let mut start = 0;
        let mut end = start + byte_sequence[0];
        buf[start..end].copy_from_slice(&self.version);
        start = start + byte_sequence[0];
        end = start + byte_sequence[1];
        buf[start..end].copy_from_slice(&self.services);
        start = start + byte_sequence[1];
        end = start + byte_sequence[2];
        buf[start..end].copy_from_slice(&self.timestamp);
        start = start + byte_sequence[2];
        end = start + byte_sequence[3];
        buf[start..end].copy_from_slice(&self.addr_recv.to_be_bytes());
        start = start + byte_sequence[3];
        end = start + byte_sequence[4];
        buf[start..end].copy_from_slice(&self.addr_from);
        start = start + byte_sequence[4];
        end = start + byte_sequence[5];
        buf[start..end].copy_from_slice(&self.nonce);
        start = start + byte_sequence[5];
        end = start + byte_sequence[6];
        buf[start..end].copy_from_slice(&self.user_agent);
        start = start + byte_sequence[6];
        end = start + byte_sequence[7];
        buf[start..end].copy_from_slice(&self.start_height);
        start = start + byte_sequence[7];
        end = start + byte_sequence[8];
        buf[start..end].copy_from_slice(&self.relay);
        buf
    }
}

#[derive(Default, Clone)]
pub struct PingPayload {
    pub nonce: [u8;8],
}

#[test]
fn default_version_message_size_is_98() {
    assert_eq!(VersionPayload::default().to_le_bytes().len(), 98);
}